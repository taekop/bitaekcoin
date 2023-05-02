use bitaekcoin::{
    encode::VarInt,
    script::{instruction::PushBytes, Script, StandardScript},
    transaction::{Transaction, TxIn, TxOut, Witness},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/cab75da6d7fe1531c881d4efdb4826410a2604aa9e6442ab12a08363f34fb408
fn tx() -> Transaction {
    Transaction {
        version: 1,
        flag: Some(1),
        inputs: vec![TxIn {
            txid: hex!("bd430d52f35166a7dd6251c73a48559ad8b5f41b6c5bc4a6c4c1a3e3702f4287"),
            output_index: 0,
            script_size: VarInt(0),
            script_sig: Script(vec![]),
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOut {
            amount: 73182,
            script_size: VarInt(22),
            script_pub_key: StandardScript::P2WPKH(
                hex!("5d6f02f47dc6c57093df246e3742cfe1e22ab410").to_vec(),
            )
            .into_script(),
        }],
        witnesses: vec![
            Witness(
                vec![
                    PushBytes::Empty,
                    PushBytes::Bytes(72, hex!("3045022100a9a7b273afe54da5f087cb2d995180251f2950cb3b08cd7126f3ebe0d9323335022008c49c695f8951fbb6837e157b9a243dc8a6c79334af529cde6af20a1749efef01").to_vec()),
                    PushBytes::Bytes(37, hex!("512103534da516a0ab32f30246620fdfbfaf1921228c1e222c6bd2fcddbcfd9024a1b651ae").to_vec()),
                ]
            )
        ],
        lock_time: 0,
    }
}

fn locking_script() -> Script {
    StandardScript::P2WSH(
        hex!("916ff972855bf7589caf8c46a31f7f33b07d0100d953fde95a8354ac36e98165").to_vec(),
    )
    .into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("cab75da6d7fe1531c881d4efdb4826410a2604aa9e6442ab12a08363f34fb408")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script, 86591));
}
