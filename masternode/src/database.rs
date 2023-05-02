use bitaekcoin::block::Block;

use crate::{account::Account, PRIVATE_KEY};

pub struct DB {
    pub accounts: Vec<Account>,
    pub blocks: Vec<Block>,
}

impl DB {
    pub fn new() -> Self {
        let master = Account::new(PRIVATE_KEY.to_vec());
        Self {
            accounts: vec![master],
            blocks: vec![],
        }
    }

    pub fn push_block(&mut self, block: Block) {
        self.blocks.push(block);
    }

    pub fn latest_block(&self) -> Option<Block> {
        self.blocks.last().cloned()
    }

    pub fn push_account(&mut self, account: Account) {
        self.accounts.push(account);
    }

    pub fn account(&self, i: usize) -> Account {
        self.accounts[i].clone()
    }
}

impl Default for DB {
    fn default() -> Self {
        Self::new()
    }
}
