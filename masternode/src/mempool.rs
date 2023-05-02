use std::collections::{HashSet, VecDeque};

use bitaekcoin::transaction::Transaction;

pub struct Mempool {
    pub transactions: VecDeque<Transaction>,
}

impl Mempool {
    pub fn new() -> Self {
        Self {
            transactions: VecDeque::new(),
        }
    }

    pub fn push(&mut self, new_tx: Transaction) -> Result<(), String> {
        let mut tx_ins = HashSet::new();
        for pending_tx in &self.transactions {
            for tx_in in &pending_tx.inputs {
                tx_ins.insert((tx_in.txid, tx_in.output_index));
            }
        }
        for tx_in in &new_tx.inputs {
            if !tx_ins.insert((tx_in.txid, tx_in.output_index)) {
                return Err("Already spent utxo".to_owned());
            }
        }
        self.transactions.push_back(new_tx);
        Ok(())
    }

    pub fn pop(&mut self) -> Vec<Transaction> {
        self.transactions.drain(..).collect()
    }
}

impl Default for Mempool {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use bitaekcoin::{encode::VarInt, script::Script, transaction::TxIn};

    use super::*;

    #[test]
    fn test_mempool() {
        let mut mempool = Mempool::new();
        let tx = Transaction {
            version: 1,
            flag: None,
            inputs: vec![TxIn {
                txid: [0; 32],
                output_index: 0,
                script_size: VarInt(0),
                script_sig: Script(vec![]),
                sequence: 0,
            }],
            outputs: vec![],
            witnesses: vec![],
            lock_time: 0,
        };
        assert!(mempool.push(tx.clone()).is_ok());
        assert!(mempool.push(tx).is_err());
    }
}
