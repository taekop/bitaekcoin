use ripemd::Ripemd160;
use sha2::{Digest, Sha256};

use crate::{
    encode::{Encodable, VarInt},
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxID},
};

pub fn merkle_root(mut txids: Vec<TxID>) -> [u8; 32] {
    for txid in &mut txids {
        txid.reverse();
    }
    while txids.len() > 1 {
        if txids.len() % 2 == 1 {
            txids.push(*txids.last().unwrap());
        }
        let mut new_txids = Vec::new();
        for i in 0..txids.len() / 2 {
            let h1 = txids[i * 2];
            let h2 = txids[i * 2 + 1];
            let h = sha256(sha256([h1, h2].concat().to_vec()).to_vec());
            new_txids.push(h);
        }
        txids = new_txids;
    }
    txids[0].reverse();
    txids[0]
}

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

#[derive(Debug, Clone, Copy)]
pub enum SigHash {
    All,
    None,
    Single,
    AnyoneCanPay,
}

impl SigHash {
    pub fn hash(
        self,
        tx: &Transaction,
        input_index: usize,
        script: &Script,
        amount: u64,
    ) -> [u8; 32] {
        if tx.is_segwit() {
            let hash_prevouts = tx.hash_prevouts(self);
            let hash_sequence = tx.hash_sequence(self);
            let hash_outputs = tx.hash_outputs(input_index, self);
            let script = match script.to_standard() {
                Some(StandardScript::P2WPKH(pkh)) => Script(vec![Instruction::PushBytes(
                    PushBytes::Bytes(0x19, StandardScript::P2PKH(pkh).encode()),
                )]),
                // TODO: handle not-yet-executed OP_CODESEPARATOR
                _ => Script(vec![Instruction::PushBytes(PushBytes::from_bytes(
                    script.encode(),
                ))]),
            };
            let mut bytes = Vec::new();
            bytes.extend(tx.version.encode());
            bytes.extend(hash_prevouts);
            bytes.extend(hash_sequence);
            bytes.extend(tx.inputs[input_index].txid.encode());
            bytes.extend(tx.inputs[input_index].output_index.encode());
            bytes.extend(script.encode());
            bytes.extend(amount.encode());
            bytes.extend(tx.inputs[input_index].sequence.encode());
            bytes.extend(hash_outputs);
            bytes.extend(tx.lock_time.encode());
            bytes.extend(self.to_four_bytes());
            sha256(sha256(bytes).to_vec())
        } else {
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

    pub fn to_byte(self) -> u8 {
        match self {
            SigHash::All => 1,
            SigHash::None => 2,
            SigHash::Single => 3,
            SigHash::AnyoneCanPay => 0x80,
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
