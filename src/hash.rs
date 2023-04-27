use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::{
    encode::{Encodable, VarInt},
    script::Script,
    transaction::Transaction,
};

pub fn sha256(bytes: Vec<u8>) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

pub fn ripemd160(bytes: Vec<u8>) -> [u8; 20] {
    let mut hasher = Ripemd160::new();
    hasher.update(bytes);
    hasher.finalize().into()
}

#[derive(Debug, Clone)]
pub enum SigHash {
    All,
    None,
    Single,
    AnyoneCanPay,
}

impl SigHash {
    pub fn hash(self, tx: &Transaction, input_index: usize, script: &Script) -> [u8; 32] {
        // TODO: subscript is the entire script only if non-segwit, non-P2SH script without OP_CODESEPARATOR
        let subscript = script;
        // TODO: remove signature in subscript
        let mut tx = tx.clone();
        for tx_in in &mut tx.inputs {
            tx_in.script_size = VarInt(0);
            tx_in.script_sig = Script(vec![]);
        }
        tx.inputs[input_index].script_size = VarInt(subscript.encode().len() as u64);
        tx.inputs[input_index].script_sig = subscript.clone();
        // TODO: handle None, Single, AnyoneCanPay
        let mut bytes = tx.encode();
        bytes.extend(self.to_four_bytes());
        sha256(sha256(bytes).to_vec())
    }

    pub fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            1 => Some(Self::All),
            2 => Some(Self::None),
            3 => Some(Self::Single),
            0x80 => Some(Self::AnyoneCanPay),
            _ => None,
        }
    }

    pub fn to_four_bytes(self) -> [u8; 4] {
        let value = match self {
            SigHash::All => 1u32,
            SigHash::None => 2u32,
            SigHash::Single => 3u32,
            SigHash::AnyoneCanPay => 128u32,
        };
        value.to_le_bytes()
    }
}
