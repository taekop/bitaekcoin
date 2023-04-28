use bitaekcoin::{
    encode::VarInt,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxIn, TxOut, Witness},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/fc64c1d65626a459220faeb257e0cab2c3eee26c2e5ba60a36840e882ab48e49
fn tx() -> Transaction {
    Transaction {
        version: 2,
        flag: Some(1),
        inputs: vec![
            TxIn {
                txid: hex!("543f319e52123528847681ffc62941219196983b6b2daa51aa7fd44c9bc5c981"),
                output_index: 0,
                script_size: VarInt(23),
                script_sig: Script(vec![Instruction::PushBytes(PushBytes::Bytes(22, hex!("001414579720bd206d1ac86f65e48c384a4414a4adb0").to_vec()))]),
                sequence: 0xfffffffe,
            },
            TxIn {
                txid: hex!("ea735b8cdaa6b13db5d87ba417c75f693aecf01d48fd9e46ed8ddb40d8e2cfa0"),
                output_index: 1,
                script_size: VarInt(23),
                script_sig: Script(vec![Instruction::PushBytes(PushBytes::Bytes(22, hex!("001416527a765084e22089a72610be687b4d7c749e36").to_vec()))]),
                sequence: 0xfffffffe,
            },
            TxIn {
                txid: hex!("769e8be2a9a14aadecdc6a8c943f2659b0aa7003af3428b17ac428c6d01263e3"),
                output_index: 0,
                script_size: VarInt(23),
                script_sig: Script(vec![Instruction::PushBytes(PushBytes::Bytes(22, hex!("00143b14bd1f24003805c5b9348e5bb85274c55b058f").to_vec()))]),
                sequence: 0xfffffffe,
            },
        ],
        outputs: vec![
            TxOut {
                amount: 2543781,
                script_size: VarInt(25),
                script_pub_key: StandardScript::P2PKH(
                    hex!("09bbf3b7c72973929e7ad323b9c7d33f48c56592").to_vec(),
                )
                .into_script(),
            },
            TxOut {
                amount: 1106542,
                script_size: VarInt(23),
                script_pub_key: StandardScript::P2SH(
                    hex!("fd4d295b037b26455510bba0fee9ffd50e5c0cd6").to_vec(),
                )
                .into_script(),
            },
        ],
        witnesses: vec![
            Witness(
                vec![
                    PushBytes::Bytes(71, hex!("304402201cc5ce31b66e9242fca56ee19a5b427d00a912dd5adcd7c3d403777099f2b0d30220689d76f3f03b04794adaf3aab62867018def6aa2c3cf1b28b1b4b3b35b478ef701").to_vec()),
                    PushBytes::Bytes(33, hex!("035be12a751c60ac9975e7c245527690d2fc550f8d385b15f8f25eeedf0f4995b6").to_vec()),
                ]
            ),
            Witness(
                vec![
                    PushBytes::Bytes(71, hex!("3044022002b91d7a5d6a7bf31583f8be4ab94753bbf2ee38da11d2ce558954c9330fefcc02205c6b6a5403823d368ea66e13ea6314fb866578ba2f61f33c9c2cc10fcef1edfa01").to_vec()),
                    PushBytes::Bytes(33, hex!("0386400b2ee88bc53147e702bc5bddd4cef9ac5cecd7d64109babb5437715b433c").to_vec()),
                ]
            ),
            Witness(
                vec![
                    PushBytes::Bytes(72, hex!("3045022100c3efb855ff5ea4e8d448c2cf09d660c3892f880592c0f88a49236d998aad1e8b02201cd91b449a1bc59d377d74c3636da9e6c3d2b51569e1f2ae455a869205e7c20301").to_vec()),
                    PushBytes::Bytes(33, hex!("02c0b09f8c1dbfacae649c92ebb032d8d1d06014426f498e6a9690a6aded29f904").to_vec()),
                ]
            ),
        ],
        lock_time: 534738,
    }
}

fn locking_script() -> Script {
    StandardScript::P2SH(hex!("3a753d05730395ed74da57041617f76c5aa4f831").to_vec()).into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("fc64c1d65626a459220faeb257e0cab2c3eee26c2e5ba60a36840e882ab48e49")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script, 1215000));
}
