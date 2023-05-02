use jsonrpc_core::{IoHandler, Result};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::ServerBuilder;

#[rpc]
pub trait Rpc {
    #[rpc(name = "hello")]
    fn hello(&self) -> Result<String>;
}

struct RpcImpl;

impl Rpc for RpcImpl {
    fn hello(&self) -> Result<String> {
        Ok("hello".to_owned())
    }
}

pub fn run_server() {
    let mut io = IoHandler::new();
    io.extend_with(RpcImpl.to_delegate());
    let server = ServerBuilder::new(io)
        .start_http(&"0.0.0.0:8000".parse().unwrap())
        .unwrap();
    server.wait();
}
