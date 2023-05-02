use std::{
    collections::HashMap,
    time::{SystemTime, UNIX_EPOCH},
};

use bitaekcoin::{
    block::{Block, BlockHeader},
    encode::{Encodable, VarInt},
    hash::merkle_root,
    script::{Script, StandardScript},
    transaction::{Transaction, TxIn, TxOut},
};

pub struct Node {}

impl Node {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(self) {
        let mut block = initialize_block([0; 32], 0x1f00ffff, vec![0], vec![]);
        loop {
            if block.validate(HashMap::new()) {
                println!("{}", block.header.nonce);
                break;
            }
            block.header.nonce += 1;
        }
    }
}

fn initialize_block(
    prev_block_hash: [u8; 32],
    bits: u32,
    miner_pk: Vec<u8>,
    mut transactions: Vec<Transaction>,
) -> Block {
    let output_script = StandardScript::P2PK(miner_pk).into_script();
    let coinbase_transaction = Transaction {
        version: 1,
        flag: None,
        inputs: vec![TxIn {
            txid: [0; 32],
            output_index: 0xffffffff,
            script_size: VarInt(0),
            script_sig: Script(vec![]),
            sequence: 0,
        }],
        outputs: vec![TxOut {
            amount: 100000000,
            script_size: VarInt(output_script.encode().len() as u64),
            script_pub_key: output_script,
        }],
        witnesses: vec![],
        lock_time: 0,
    };
    transactions.insert(0, coinbase_transaction);

    let txids = transactions.iter().map(|tx| tx.txid()).collect();
    let merkle_root = merkle_root(txids);
    Block {
        header: BlockHeader {
            version: 1,
            prev_block_hash,
            merkle_root,
            timestamp: get_timestamp(),
            bits,
            nonce: 0,
        },
        transactions,
    }
}

fn get_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node() {
        let node = Node::new();
        node.run();
    }
}
