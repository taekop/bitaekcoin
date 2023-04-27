use bitaekcoin::{
    encode::VarInt,
    hash::SigHash,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxIn, TxOut},
};
use hex_literal::hex;

// Example from https://github.com/bitcoin/bitcoin/issues/8991
fn tx() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("40872a376e98a1f8b285827c2ad8c5b3eec7d779d752dc3a4adda5d9bb70f3b5"),
            output_index: 0,
            script_size: VarInt(72),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Bytes(71, hex!("3044022057a1234709270325e7215200f982546304cf465971cbd55d54231ead54ef1a7802207a82e93ef2b0f87188abe87bccb67ee9d5c650b1b58948e5b1c80ba1b4c43dc301").to_vec()))
            ]),
            sequence: 0xfffffffe,
        }],
        outputs: vec![
            TxOut {
                amount: 22999,
                script_size: VarInt(23),
                script_pub_key: StandardScript::P2SH(hex!("ea4e30950c2495beba1e75e035bedd55ccfe4c9b").to_vec()).into_script(),
            },
            TxOut {
                amount: 1955149,
                script_size: VarInt(25),
                script_pub_key: StandardScript::P2PKH(hex!("676de8484253cd355c37fc51ba34f2e5d62a9440").to_vec()).into_script(),
            },
        ],
        lock_time: 434102,
    }
}

fn locking_script() -> Script {
    StandardScript::P2PK(
        hex!("02d5ddb8c9a2bc17624baa51245ef3c07380f90b2e2f38217307a8a1869508ca01").to_vec(),
    )
    .into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("226a8b08dc46a00e9ecec5567a303a0b354bef3c1674476eb5e4b627b2ace493")
    );

    let locking_script = locking_script();
    let hash = SigHash::All.hash(&tx, 0, &locking_script);

    // equal to sha256(sha256(0x0100000001b5f370bbd9a5dd4a3adc52d779d7c7eeb3c5d82a7c8285b2f8a1986e372a874000000000232102d5ddb8c9a2bc17624baa51245ef3c07380f90b2e2f38217307a8a1869508ca01acfeffffff02d75900000000000017a914ea4e30950c2495beba1e75e035bedd55ccfe4c9b874dd51d00000000001976a914676de8484253cd355c37fc51ba34f2e5d62a944088acb69f060001000000))
    assert_eq!(
        hash,
        hex!("1fb32874e31febdd88dcbb6564499864543ad1378e580b68c09ebfe370b50588")
    );
    assert!(tx.validate(0, &locking_script));
}
