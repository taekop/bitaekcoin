use std::collections::HashMap;

use primitive_types::U256;
use serde::{Deserialize, Serialize};

use crate::{
    hash::{merkle_root, sha256},
    transaction::{Transaction, TxID, TxOut},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn validate(&self, outpoints: HashMap<(TxID, u32), TxOut>) -> bool {
        // Validate transactions
        for i in 1..self.transactions.len() {
            let tx = &self.transactions[i];
            for i in 0..tx.inputs.len() {
                let sum_in: u64 = tx.outputs.iter().map(|tx_out| tx_out.amount).sum();
                let mut sum_out = 0;
                match outpoints.get(&(tx.inputs[i].txid, tx.inputs[i].output_index)) {
                    Some(tx_out) => {
                        sum_out += tx_out.amount;
                        if !tx.validate(
                            i,
                            &tx.inputs[i].script_sig,
                            &tx_out.script_pub_key,
                            tx_out.amount,
                        ) {
                            return false;
                        }
                    }
                    None => return false,
                }
                if sum_in < sum_out {
                    return false;
                }
            }
        }

        // Validate Merkle Root
        let txids = self.transactions.iter().map(|tx| tx.txid()).collect();
        if self.header.merkle_root != merkle_root(txids) {
            return false;
        }

        // Validate Block Hash
        self.header.validate()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    pub height: u64,
    pub version: u32,
    pub prev_block_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub timestamp: u32,
    pub bits: u32,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn hash(&self) -> [u8; 32] {
        let bytes = {
            let mut bytes: [u8; 80] = [0; 80];
            bytes[0..4].copy_from_slice(&self.version.to_le_bytes());

            let mut prev_block_hash_le = self.prev_block_hash;
            prev_block_hash_le.reverse();
            bytes[4..36].copy_from_slice(&prev_block_hash_le);

            let mut merkle_root_le = self.merkle_root;
            merkle_root_le.reverse();
            bytes[36..68].copy_from_slice(&merkle_root_le);

            bytes[68..72].copy_from_slice(&self.timestamp.to_le_bytes());
            bytes[72..76].copy_from_slice(&self.bits.to_le_bytes());
            bytes[76..80].copy_from_slice(&self.nonce.to_le_bytes());

            bytes
        };

        let mut hash = sha256(sha256(bytes.to_vec()).to_vec());
        hash.reverse();
        hash
    }

    pub fn validate(&self) -> bool {
        let target = {
            let bytes = self.bits.to_be_bytes();
            let index = bytes[0];
            let coef = U256::from_big_endian(&bytes[1..4]);
            coef << (8 * (index - 3))
        };

        let hash = self.hash();
        let hash_u256 = U256::from_big_endian(&hash);
        hash_u256 <= target
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    pub fn test_validate() {
        // https://blockchair.com/bitcoin/block/200000
        let block_header = BlockHeader {
            height: 200000,
            version: 2,
            prev_block_hash: hex!(
                "00000000000003a20def7a05a77361b9657ff954b2f2080e135ea6f5970da215"
            ),
            merkle_root: hex!("a08f8101f50fd9c9b3e5252aff4c1c1bd668f878fffaf3d0dbddeb029c307e88"),
            timestamp: 1348310759,
            bits: u32::from_be_bytes(hex!("1a05db8b")),
            nonce: 4158183488,
        };
        assert!(block_header.validate());
    }
}
