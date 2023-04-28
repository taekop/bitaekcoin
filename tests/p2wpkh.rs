use bitaekcoin::{
    encode::VarInt,
    script::{instruction::PushBytes, Script, StandardScript},
    transaction::{Transaction, TxIn, TxOut, Witness},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd
fn tx() -> Transaction {
    Transaction {
        version: 1,
        flag: Some(1),
        inputs: vec![TxIn {
            txid: hex!("dfcec48bb8491856c353306ab5febeb7e99e4d783eedf3de98f3ee0812b92bad"),
            output_index: 0,
            script_size: VarInt(0),
            script_sig: Script(vec![]),
            sequence: 0xffffffff,
        }],
        outputs: vec![
            TxOut {
                amount: 81530,
                script_size: VarInt(22),
                script_pub_key: StandardScript::P2WPKH(
                    hex!("8d7a0a3461e3891723e5fdf8129caa0075060cff").to_vec(),
                )
                .into_script(),
            },
            TxOut {
                amount: 81530,
                script_size: VarInt(22),
                script_pub_key: StandardScript::P2WPKH(
                    hex!("8d7a0a3461e3891723e5fdf8129caa0075060cff").to_vec(),
                )
                .into_script(),
            },
            TxOut {
                amount: 0,
                script_size: VarInt(37),
                script_pub_key: StandardScript::NullData(
                    hex!("42697462616e6b20496e632e204a6170616e20737570706f7274732053656757697421")
                        .to_vec(),
                )
                .into_script(),
            },
        ],
        witnesses: vec![
            Witness(
                vec![
                    PushBytes::Bytes(72, hex!("3045022100a6e33a7aff720ba9f33a0a8346a16fdd022196862796d511d31978c40c9ad48b02206fb8f67bd699a8c952b3386a81d122c366d2d36cd08e2de21207e6aa6f96ce9501").to_vec()), 
                    PushBytes::Bytes(33, hex!("0283409659355b6d1cc3c32decd5d561abaac86c37a353b52895a5e6c196d6f448").to_vec())
                ]
            )
        ],
        lock_time: 0,
    }
}

fn locking_script() -> Script {
    StandardScript::P2WPKH(hex!("8d7a0a3461e3891723e5fdf8129caa0075060cff").to_vec()).into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("f91d0a8a78462bc59398f2c5d7a84fcff491c26ba54c4833478b202796c8aafd")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script, 194300));
}
