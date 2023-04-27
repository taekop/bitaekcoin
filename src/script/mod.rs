use std::collections::VecDeque;

use crate::{
    encode::{Decodable, DecodeError, Encodable},
    hash::SigHash,
    utils::pop_front,
};

pub mod instruction;

use instruction::*;
use k256::ecdsa::Signature;

#[derive(Debug, Clone)]
pub struct Script(pub Vec<Instruction>);

impl Script {
    pub fn to_standard(&self) -> Option<StandardScript> {
        let instructions = self.0.clone();
        let len = instructions.len();
        if len >= 3 {
            if let Instruction::PushBytes(PushBytes::OneToSixteen(m)) = instructions[0] {
                if let Instruction::PushBytes(PushBytes::OneToSixteen(n)) = instructions[len - 2] {
                    if len == (n as usize) + 3 && instructions[len - 1].opcode() == OP_CHECKMULTISIG
                    {
                        let mut pks = Vec::new();
                        // 1..len-2
                        for instruction in instructions.iter().take(len - 2).skip(1) {
                            if let Instruction::PushBytes(pb) = instruction {
                                pks.push(pb.bytes());
                            } else {
                                break;
                            }
                        }
                        if pks.len() == n as usize {
                            return Some(StandardScript::P2MS(m, n, pks));
                        }
                    }
                }
            }
        }
        match len {
            2 => {
                if let Instruction::PushBytes(pb) = &instructions[0] {
                    if instructions[1].opcode() == OP_CHECKSIG {
                        Some(StandardScript::P2PK(pb.bytes()))
                    } else {
                        None
                    }
                } else if let Instruction::PushBytes(pb) = &instructions[1] {
                    if instructions[0].opcode() == OP_RETURN {
                        Some(StandardScript::NullData(pb.bytes()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            3 => {
                if let Instruction::PushBytes(pb) = &instructions[1] {
                    if instructions[0].opcode() == OP_HASH160
                        && instructions[2].opcode() == OP_EQUAL
                    {
                        Some(StandardScript::P2SH(pb.bytes()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            5 => {
                if let Instruction::PushBytes(pb) = &instructions[2] {
                    if instructions[0].opcode() == OP_DUP
                        && instructions[1].opcode() == OP_HASH160
                        && instructions[3].opcode() == OP_EQUALVERIFY
                        && instructions[4].opcode() == OP_CHECKSIG
                    {
                        Some(StandardScript::P2PKH(pb.bytes()))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn to_unlocking_standard(&self) -> Option<UnlockingStandardScript> {
        let instructions = self.0.clone();
        let len = instructions.len();
        match len {
            1 => {
                if let Instruction::PushBytes(pb) = &instructions[0] {
                    let mut signature = pb.bytes();
                    if let Some(sighash) = signature.pop() {
                        if let Some(sighash) = SigHash::from_byte(sighash) {
                            if let Ok(signature) = Signature::from_der(&signature) {
                                Some(UnlockingStandardScript::P2PK(signature, sighash))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

impl Encodable for Script {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for instruction in &self.0 {
            bytes.append(&mut instruction.encode());
        }
        bytes
    }
}

impl Decodable for Script {
    fn decode(bytes: &mut VecDeque<u8>) -> Result<Self, DecodeError> {
        let mut instructions = Vec::new();
        while !bytes.is_empty() {
            let opcode = bytes.pop_front().unwrap();
            let instruction = match opcode {
                OP_0 => Instruction::PushBytes(PushBytes::Empty),
                0x01..=0x4b => {
                    let value = pop_front(bytes, opcode as usize, DecodeError)?;
                    Instruction::PushBytes(PushBytes::Bytes(opcode, value))
                }
                OP_PUSHDATA1 => {
                    let n = pop_front(bytes, 1, DecodeError)?[0];
                    let value = pop_front(bytes, n as usize, DecodeError)?;
                    Instruction::PushBytes(PushBytes::Data1(n, value))
                }
                OP_PUSHDATA2 => {
                    let n =
                        u16::from_le_bytes(pop_front(bytes, 2, DecodeError)?.try_into().unwrap());
                    let value = pop_front(bytes, n as usize, DecodeError)?;
                    Instruction::PushBytes(PushBytes::Data2(n, value))
                }
                OP_PUSHDATA4 => {
                    let n =
                        u32::from_le_bytes(pop_front(bytes, 4, DecodeError)?.try_into().unwrap());
                    let value = pop_front(bytes, n as usize, DecodeError)?;
                    Instruction::PushBytes(PushBytes::Data4(n, value))
                }
                OP_1NEGATE => Instruction::PushBytes(PushBytes::Negate1),
                0x51..=0x60 => Instruction::PushBytes(PushBytes::OneToSixteen(opcode - 0x50)),
                _ => Instruction::Opcode(opcode),
            };
            instructions.push(instruction);
        }
        Ok(Script(instructions))
    }
}

#[derive(Debug, Clone)]
pub enum StandardScript {
    P2PK(Vec<u8>),              // public key
    P2PKH(Vec<u8>),             // public key hash
    P2MS(u8, u8, Vec<Vec<u8>>), // m-of-n with n public keys
    P2SH(Vec<u8>),              // script hash
    NullData(Vec<u8>),          // data
}

impl StandardScript {
    pub fn into_script(&self) -> Script {
        let instructions = match self.clone() {
            StandardScript::P2PK(pk) => vec![
                Instruction::PushBytes(PushBytes::from_bytes(pk)),
                Instruction::Opcode(OP_CHECKSIG),
            ],
            StandardScript::P2PKH(pkh) => vec![
                Instruction::Opcode(OP_DUP),
                Instruction::Opcode(OP_HASH160),
                Instruction::PushBytes(PushBytes::from_bytes(pkh)),
                Instruction::Opcode(OP_EQUALVERIFY),
                Instruction::Opcode(OP_CHECKSIG),
            ],
            StandardScript::P2MS(m, n, pks) => {
                let mut instructions = vec![Instruction::PushBytes(PushBytes::OneToSixteen(m))];
                for pk in pks {
                    instructions.push(Instruction::PushBytes(PushBytes::from_bytes(pk)));
                }
                instructions.push(Instruction::PushBytes(PushBytes::OneToSixteen(n)));
                instructions.push(Instruction::Opcode(OP_CHECKMULTISIG));
                instructions
            }
            StandardScript::P2SH(sh) => vec![
                Instruction::Opcode(OP_HASH160),
                Instruction::PushBytes(PushBytes::from_bytes(sh)),
                Instruction::Opcode(OP_EQUAL),
            ],
            StandardScript::NullData(data) => vec![
                Instruction::Opcode(OP_RETURN),
                Instruction::PushBytes(PushBytes::from_bytes(data)),
            ],
        };
        Script(instructions)
    }
}

impl Encodable for StandardScript {
    fn encode(&self) -> Vec<u8> {
        self.into_script().encode()
    }
}

#[derive(Debug, Clone)]
pub enum UnlockingStandardScript {
    P2PK(Signature, SigHash),
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_to_unlocking_standard() {
        let script = Script(vec![Instruction::PushBytes(PushBytes::Bytes(72, hex!("3045022100be47eec0d762891eb9beb8f4557551b8fbc3fbaa206ea9cd99a4931a0afdd13c022033919442e1cdf93bd268dbf5ca41cfa17b083510f1fa3edea40fbe34acd6fbd201").to_vec()))]);
        let unlocking_standard_script = script.to_unlocking_standard().unwrap();
        if let UnlockingStandardScript::P2PK(signature, SigHash::All) = unlocking_standard_script {
            assert_eq!(
                signature.r().to_string(),
                "BE47EEC0D762891EB9BEB8F4557551B8FBC3FBAA206EA9CD99A4931A0AFDD13C"
            );
            assert_eq!(
                signature.s().to_string(),
                "33919442E1CDF93BD268DBF5CA41CFA17B083510F1FA3EDEA40FBE34ACD6FBD2"
            );
        } else {
            panic!()
        }
    }
}
