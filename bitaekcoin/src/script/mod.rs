use std::collections::VecDeque;

use crate::{
    encode::{Decodable, DecodeError, Encodable},
    hash::SigHash,
    utils::{pop_front, signature_sighash},
};

pub mod instruction;

use instruction::*;
use k256::ecdsa::Signature;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
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
                if instructions[0].opcode() == OP_0 {
                    match &instructions[1] {
                        Instruction::PushBytes(PushBytes::Bytes(20, bytes)) => {
                            Some(StandardScript::P2WPKH(bytes.clone()))
                        }
                        Instruction::PushBytes(PushBytes::Bytes(32, bytes)) => {
                            Some(StandardScript::P2WSH(bytes.clone()))
                        }
                        _ => None,
                    }
                } else if let Instruction::PushBytes(pb) = &instructions[0] {
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

    pub fn to_unlocking_standard(&self, ty: StandardScriptType) -> Option<UnlockingStandardScript> {
        let instructions = self.0.clone();
        let len = instructions.len();
        match ty {
            StandardScriptType::P2PK => {
                if len == 1 {
                    if let Instruction::PushBytes(pb) = &instructions[0] {
                        if let Some((signature, sighash)) = signature_sighash(pb.bytes()) {
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
            }
            StandardScriptType::P2PKH => {
                if len == 2 {
                    match (&instructions[0], &instructions[1]) {
                        (Instruction::PushBytes(pb1), Instruction::PushBytes(pb2)) => {
                            if let Some((signature, sighash)) = signature_sighash(pb1.bytes()) {
                                Some(UnlockingStandardScript::P2PKH(
                                    signature,
                                    sighash,
                                    pb2.bytes(),
                                ))
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            StandardScriptType::P2MS => {
                if len >= 2 && instructions[0].opcode() == OP_0 {
                    let mut sigs = Vec::new();
                    // 1..len
                    for instruction in instructions.iter().take(len).skip(1) {
                        if let Instruction::PushBytes(pb) = instruction {
                            if let Some(sig) = signature_sighash(pb.bytes()) {
                                sigs.push(sig);
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    if sigs.len() == len - 1 {
                        return Some(UnlockingStandardScript::P2MS(sigs));
                    }
                    None
                } else {
                    None
                }
            }
            StandardScriptType::P2SH => {
                if len >= 2 {
                    if let Instruction::PushBytes(pb) = &instructions[len - 1] {
                        if let Ok(redeem_script) = Script::decode(&mut VecDeque::from(pb.bytes())) {
                            let unlocking_script = Script(instructions[0..len - 1].to_vec());
                            Some(UnlockingStandardScript::P2SH(
                                unlocking_script,
                                redeem_script,
                            ))
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
            StandardScriptType::NullData => None,
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
pub enum StandardScriptType {
    P2PK,
    P2PKH,
    P2MS,
    P2SH,
    NullData,
}

#[derive(Debug, Clone)]
pub enum StandardScript {
    P2PK(Vec<u8>),              // public key
    P2PKH(Vec<u8>),             // public key hash
    P2MS(u8, u8, Vec<Vec<u8>>), // m-of-n with n public keys
    P2SH(Vec<u8>),              // script hash
    NullData(Vec<u8>),          // data
    P2WPKH(Vec<u8>),            // public key hash
    P2WSH(Vec<u8>),             // script hash
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
            StandardScript::P2WPKH(pkh) => vec![
                Instruction::PushBytes(PushBytes::Empty),
                Instruction::PushBytes(PushBytes::from_bytes(pkh)),
            ],
            StandardScript::P2WSH(sh) => vec![
                Instruction::PushBytes(PushBytes::Empty),
                Instruction::PushBytes(PushBytes::from_bytes(sh)),
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
    P2PKH(Signature, SigHash, Vec<u8>),
    P2MS(Vec<(Signature, SigHash)>),
    P2SH(Script, Script),
}

impl UnlockingStandardScript {
    pub fn into_script(&self) -> Script {
        match self {
            UnlockingStandardScript::P2PK(signature, sighash) => {
                let mut bytes = signature.to_der().as_bytes().to_vec();
                bytes.push(sighash.to_byte());
                Script(vec![Instruction::PushBytes(PushBytes::from_bytes(bytes))])
            }
            UnlockingStandardScript::P2PKH(signature, sighash, pubkey) => {
                let mut bytes = signature.to_der().as_bytes().to_vec();
                bytes.push(sighash.to_byte());
                Script(vec![
                    Instruction::PushBytes(PushBytes::from_bytes(bytes)),
                    Instruction::PushBytes(PushBytes::from_bytes(pubkey.clone())),
                ])
            }
            UnlockingStandardScript::P2MS(multisig) => {
                let mut instructions = vec![Instruction::Opcode(0)];
                for (signature, sighash) in multisig {
                    let mut bytes = signature.to_der().as_bytes().to_vec();
                    bytes.push(sighash.to_byte());
                    let instruction = Instruction::PushBytes(PushBytes::from_bytes(bytes));
                    instructions.push(instruction);
                }
                Script(instructions)
            }
            UnlockingStandardScript::P2SH(unlocking_script, redeem_script) => {
                let mut instructions = vec![Instruction::Opcode(0)];
                instructions.extend(unlocking_script.0.clone());
                instructions.push(Instruction::PushBytes(PushBytes::from_bytes(
                    redeem_script.encode(),
                )));
                Script(instructions)
            }
        }
    }
}
