use bitaekcoin::{
    encode::VarInt,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxIn, TxOut},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/7edb32d4ffd7a385b763c7a8e56b6358bcd729e747290624e18acdbe6209fc45
fn tx() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("40eee3ae1760e3a8532263678cdf64569e6ad06abc133af64f735e52562bccc8"),
            output_index: 0,
            script_size: VarInt(144),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Empty),
                Instruction::PushBytes(PushBytes::Bytes(72, hex!("3045022100ad0851c69dd756b45190b5a8e97cb4ac3c2b0fa2f2aae23aed6ca97ab33bf88302200b248593abc1259512793e7dea61036c601775ebb23640a0120b0dba2c34b79001").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(69,  hex!("5141042f90074d7a5bf30c72cf3a8dfd1381bdbd30407010e878f3a11269d5f74a58788505cdca22ea6eab7cfb40dc0e07aba200424ab0d79122a653ad0c7ec9896bdf51ae").to_vec()))
            ]),
            sequence: 0xfffffffe,
        }],
        outputs: vec![
            TxOut {
                amount: 980000,
                script_size: VarInt(25),
                script_pub_key: StandardScript::P2PKH(hex!("1d30342095961d951d306845ef98ac08474b36a0").to_vec()).into_script(),
            },
        ],
        lock_time: 272295,
    }
}

fn locking_script() -> Script {
    StandardScript::P2SH(hex!("e9c3dd0c07aac76179ebc76a6c78d4d67c6c160a").to_vec()).into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("7edb32d4ffd7a385b763c7a8e56b6358bcd729e747290624e18acdbe6209fc45")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script));
}
