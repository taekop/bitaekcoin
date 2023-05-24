use std::{
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};

use bitaekcoin::{
    block::{Block, BlockHeader},
    encode::{Encodable, VarInt},
    hash::merkle_root,
    script::{Script, StandardScript},
    transaction::{Transaction, TxIn, TxOut},
};

use crate::{database::DB, mempool::Mempool, BITS, MINING_REWARD, PUBLIC_KEY};

pub struct Node {
    pub bits: u32,
    pub public_key: Vec<u8>,
    pub mempool: Arc<RwLock<Mempool>>,
    pub db: Arc<RwLock<DB>>,
}

impl Node {
    pub fn new(mempool: Arc<RwLock<Mempool>>, db: Arc<RwLock<DB>>) -> Self {
        Self {
            bits: BITS,
            public_key: PUBLIC_KEY.to_vec(),
            mempool,
            db,
        }
    }

    pub fn run(self) {
        loop {
            let block = self.db.read().unwrap().latest_block();
            let (height, prev_block_hash) = match block {
                Some(block) => (block.header.height + 1, block.header.hash()),
                None => (0, [0; 32]),
            };
            let transactions = self.mempool.write().unwrap().pop();
            let mut block = initialize_block(
                height,
                prev_block_hash,
                self.bits,
                self.public_key.clone(),
                transactions,
            );
            let utxos = self.db.read().unwrap().utxos.clone();
            loop {
                if block.validate(&utxos) {
                    self.db.write().unwrap().push_block(block);
                    break;
                }
                block.header.nonce += 1;
            }
        }
    }
}

fn initialize_block(
    height: u64,
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
            amount: MINING_REWARD,
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
            height,
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
    use std::collections::HashMap;

    use crate::{account::Account, PRIVATE_KEY};

    use super::*;

    #[ignore]
    #[test]
    fn test_node() {
        let node = Node::new(
            Arc::new(RwLock::new(Mempool::new())),
            Arc::new(RwLock::new(DB::new())),
        );
        node.run();
    }

    #[test]
    fn test_transfer() {
        let mut sender = Account::new(0, PRIVATE_KEY.to_vec());
        let receiver = Account::random(1);

        let locking_script =
            StandardScript::P2PK(sender.public_key.to_sec1_bytes().to_vec()).into_script();
        let utxos = {
            HashMap::from_iter([(
                ([0; 32], 0),
                TxOut {
                    amount: 1,
                    script_size: VarInt(locking_script.encode().len() as u64),
                    script_pub_key: locking_script.clone(),
                },
            )])
        };
        sender.balance = 1;
        sender.utxos = utxos.clone();
        let tx = sender.transfer(&receiver.public_key, 1).unwrap();
        assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script, 1));

        let transactions = vec![tx];
        let block = initialize_block(0, [0; 32], 0x22ffffff, PUBLIC_KEY.to_vec(), transactions);
        assert!(block.validate(&utxos));
    }
}
