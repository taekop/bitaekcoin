use std::sync::{Arc, RwLock};

use masternode::{database::DB, mempool::Mempool, node::Node, server::run_server};

fn main() {
    let mempool = Arc::new(RwLock::new(Mempool::new()));
    let mempool2 = mempool.clone();
    let db = Arc::new(RwLock::new(DB::new()));
    let db2 = db.clone();

    let server_thread = std::thread::spawn(move || {
        run_server(mempool, db);
    });

    let node_thread = std::thread::spawn(move || {
        let node = Node::new(mempool2, db2);
        node.run();
    });

    server_thread.join().unwrap();
    node_thread.join().unwrap();
}
