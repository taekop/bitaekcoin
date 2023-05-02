use hex_literal::hex;

pub mod account;
pub mod database;
pub mod mempool;
pub mod node;
pub mod server;

const BITS: u32 = 0x1f00ffff;
const MINING_REWARD: u64 = 100000000;
const PUBLIC_KEY: [u8; 33] =
    hex!("037e96a57281401690c12c3509f2e7414163554a7029fe28e6e784dbaf2348882d");
const PRIVATE_KEY: [u8; 32] =
    hex!("7d37007eeadc8e7f924186b07bc0255e2de96d7d88d5d54dcd2a52711d068fba");
