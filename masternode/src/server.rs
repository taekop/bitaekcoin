use std::sync::{Arc, RwLock};

use bitaekcoin::block::Block;
use jsonrpc_core::{Error, IoHandler, Result};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;

use crate::{account::AccountJson, database::DB, mempool::Mempool};

#[rpc]
pub trait Rpc {
    #[rpc(name = "getBlocks")]
    fn get_blocks(&self) -> Result<Vec<Block>>;

    #[rpc(name = "getLatestBlock")]
    fn get_latest_block(&self) -> Result<Block>;

    #[rpc(name = "getAccounts")]
    fn get_accounts(&self) -> Result<Vec<AccountJson>>;
}

struct RpcImpl {
    pub mempool: Arc<RwLock<Mempool>>,
    pub db: Arc<RwLock<DB>>,
}

impl Rpc for RpcImpl {
    fn get_blocks(&self) -> Result<Vec<Block>> {
        let mut blocks = self.db.read().unwrap().blocks();
        blocks.reverse();
        Ok(blocks)
    }

    fn get_latest_block(&self) -> Result<Block> {
        match self.db.read().unwrap().latest_block() {
            Some(block) => Ok(block),
            None => Err(Error::internal_error()),
        }
    }

    fn get_accounts(&self) -> Result<Vec<AccountJson>> {
        Ok(self
            .db
            .read()
            .unwrap()
            .accounts()
            .into_iter()
            .map(|a| a.into())
            .collect())
    }
}

pub fn run_server(mempool: Arc<RwLock<Mempool>>, db: Arc<RwLock<DB>>) {
    let mut io = IoHandler::new();
    io.extend_with(RpcImpl { mempool, db }.to_delegate());
    let server = ServerBuilder::new(io)
        .start_http(&"0.0.0.0:8000".parse().unwrap())
        .unwrap();
    server.wait();
}
