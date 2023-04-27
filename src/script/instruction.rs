use crate::encode::Encodable;

pub const OP_0: u8 = 0x00;
pub const OP_PUSHDATA1: u8 = 0x4c;
pub const OP_PUSHDATA2: u8 = 0x4d;
pub const OP_PUSHDATA4: u8 = 0x4e;
pub const OP_1NEGATE: u8 = 0x4f;

pub const OP_RETURN: u8 = 0x6a;
pub const OP_DUP: u8 = 0x76;
pub const OP_EQUAL: u8 = 0x87;
pub const OP_EQUALVERIFY: u8 = 0x88;
pub const OP_HASH160: u8 = 0xa9;
pub const OP_CODESEPARATOR: u8 = 0xab;
pub const OP_CHECKSIG: u8 = 0xac;
pub const OP_CHECKMULTISIG: u8 = 0xae;

#[derive(Debug, Clone)]
pub enum Instruction {
    Opcode(u8),
    PushBytes(PushBytes),
}

impl Instruction {
    pub fn opcode(&self) -> u8 {
        match self {
            Instruction::Opcode(op) => *op,
            Instruction::PushBytes(pb) => match pb {
                PushBytes::Empty => OP_0,
                PushBytes::Bytes(n, _) => *n,
                PushBytes::Data1(_, _) => OP_PUSHDATA1,
                PushBytes::Data2(_, _) => OP_PUSHDATA2,
                PushBytes::Data4(_, _) => OP_PUSHDATA4,
                PushBytes::Negate1 => OP_1NEGATE,
                PushBytes::OneToSixteen(n) => *n + 0x50,
            },
        }
    }

    pub fn is_push_bytes(&self) -> bool {
        match self {
            Instruction::Opcode(_) => false,
            Instruction::PushBytes(_) => true,
        }
    }
}

impl Encodable for Instruction {
    fn encode(&self) -> Vec<u8> {
        match self {
            Instruction::Opcode(opcode) => vec![*opcode],
            Instruction::PushBytes(pb) => pb.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PushBytes {
    Empty,
    Bytes(u8, Vec<u8>),
    Data1(u8, Vec<u8>),
    Data2(u16, Vec<u8>),
    Data4(u32, Vec<u8>),
    Negate1,
    OneToSixteen(u8),
}

impl PushBytes {
    pub fn bytes(&self) -> Vec<u8> {
        match self {
            PushBytes::Empty => vec![],
            PushBytes::Bytes(_, bytes) => bytes.clone(),
            PushBytes::Data1(_, bytes) => bytes.clone(),
            PushBytes::Data2(_, bytes) => bytes.clone(),
            PushBytes::Data4(_, bytes) => bytes.clone(),
            PushBytes::Negate1 => vec![0x81],
            PushBytes::OneToSixteen(b) => vec![*b],
        }
    }

    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let len = bytes.len();
        match len {
            0 => Self::Empty,
            0x01..=0x4b => Self::Bytes(len as u8, bytes),
            0x4c..=0xFF => Self::Data1(len as u8, bytes),
            0x100..=0xFFFF => Self::Data2(len as u16, bytes),
            _ => Self::Data4(len as u32, bytes),
        }
    }
}

impl Encodable for PushBytes {
    fn encode(&self) -> Vec<u8> {
        match self {
            PushBytes::Empty => vec![OP_0],
            PushBytes::Bytes(n, bytes) => {
                let mut ret = vec![*n];
                ret.extend(bytes);
                ret
            }
            PushBytes::Data1(n, bytes) => {
                let mut ret = vec![OP_PUSHDATA1, *n];
                ret.extend(bytes);
                ret
            }
            PushBytes::Data2(n, bytes) => {
                let mut ret = vec![OP_PUSHDATA2];
                ret.extend(n.to_le_bytes());
                ret.extend(bytes);
                ret
            }
            PushBytes::Data4(n, bytes) => {
                let mut ret = vec![OP_PUSHDATA4];
                ret.extend(n.to_le_bytes());
                ret.extend(bytes);
                ret
            }
            PushBytes::Negate1 => vec![OP_1NEGATE],
            PushBytes::OneToSixteen(n) => vec![0x50 + n],
        }
    }
}
