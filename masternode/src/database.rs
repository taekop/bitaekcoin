use std::collections::HashMap;

use bitaekcoin::{
    block::Block,
    script::StandardScript,
    transaction::{TxID, TxOut},
};

use crate::{account::Account, PRIVATE_KEY};

pub struct DB {
    pub accounts: Vec<Account>,
    pub blocks: Vec<Block>,
    pub utxos: HashMap<(TxID, u32), TxOut>,
}

impl DB {
    pub fn new() -> Self {
        let master = Account::new(0, PRIVATE_KEY.to_vec());
        Self {
            accounts: vec![master],
            blocks: vec![],
            utxos: HashMap::new(),
        }
    }

    pub fn push_block(&mut self, block: Block) {
        for tx in &block.transactions {
            let txid = tx.txid();
            for tx_in in &tx.inputs {
                if tx_in.txid != [0; 32] {
                    let (pubkey, amount) = {
                        let utxo = self.utxos.get(&(tx_in.txid, tx_in.output_index)).unwrap();
                        (unwrap_utxo_p2pk(utxo), utxo.amount)
                    };
                    let account = self.account_by_pubkey(pubkey);
                    account.balance -= amount;
                    account.utxos.remove(&(tx_in.txid, tx_in.output_index));
                }
            }
            for (i, tx_out) in tx.outputs.iter().enumerate() {
                let pubkey = unwrap_utxo_p2pk(tx_out);
                let account = self.account_by_pubkey(pubkey);
                account.balance += tx_out.amount;
                account.utxos.insert((txid, i as u32), tx_out.clone());
                self.utxos.insert((txid, i as u32), tx_out.clone());
            }
        }
        self.blocks.push(block);
    }

    pub fn blocks(&self) -> Vec<Block> {
        self.blocks.clone()
    }

    pub fn latest_block(&self) -> Option<Block> {
        self.blocks.last().cloned()
    }

    pub fn create_account(&mut self) -> Account {
        let index = self.accounts.len();
        let account = Account::random(index);
        self.accounts.push(account.clone());
        account
    }

    pub fn accounts(&self) -> Vec<Account> {
        self.accounts.clone()
    }

    pub fn account(&self, i: usize) -> Account {
        self.accounts[i].clone()
    }

    fn account_by_pubkey(&mut self, pubkey: Vec<u8>) -> &mut Account {
        for account in &mut self.accounts {
            if account.public_key.to_sec1_bytes().to_vec() == pubkey {
                return account;
            }
        }
        panic!("can't find account with public key");
    }
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
    }
}

fn unwrap_utxo_p2pk(utxo: &TxOut) -> Vec<u8> {
    if let Some(StandardScript::P2PK(pubkey)) = utxo.script_pub_key.to_standard() {
        pubkey
    } else {
        panic!("Trying to unwrap non p2pk utxo");
    }
}
