use bitaekcoin::{
    encode::VarInt,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxIn, TxOut},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/e65ad475a01384b086ce0d04199835fdd580739422ece1e0f1c4e362d43735d9
fn tx() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("486c887f2378feb1ea3cdc054cb7b6722e632ab1edac962a00723ea0240f2e9c"),
            output_index: 1,
            script_size: VarInt(106),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Bytes(71, hex!("304402203da9d487be5302a6d69e02a861acff1da472885e43d7528ed9b1b537a8e2cac9022002d1bca03a1e9715a99971bafe3b1852b7a4f0168281cbd27a220380a01b330701").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(33, hex!("02c9950c622494c2e9ff5a003e33b690fe4832477d32c2d256c67eab8bf613b34e").to_vec()))
            ]),
            sequence: 0xffffffff,
        }],
        outputs: vec![
            TxOut {
                amount: 390582,
                script_size: VarInt(25),
                script_pub_key: StandardScript::P2PKH(hex!("bdf63990d6dc33d705b756e13dd135466c06b3b5").to_vec()).into_script(),
            },
            TxOut {
                amount: 16932484,
                script_size: VarInt(25),
                script_pub_key: StandardScript::P2PKH(hex!("5fb0e9755a3424efd2ba0587d20b1e98ee29814a").to_vec()).into_script(),
            },
        ],
        lock_time: 0,
    }
}

fn locking_script() -> Script {
    StandardScript::P2PKH(hex!("5fb0e9755a3424efd2ba0587d20b1e98ee29814a").to_vec()).into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("e65ad475a01384b086ce0d04199835fdd580739422ece1e0f1c4e362d43735d9")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script));
}
