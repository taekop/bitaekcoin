use bitaekcoin::{
    encode::VarInt,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxIn, TxOut, Witness},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/a38a683cbbd65856d8ec91dfbb5ac3a6c601e182cd49b018c31f07c0ddad85b4
fn tx() -> Transaction {
    Transaction {
        version: 1,
        flag: Some(1),
        inputs: vec![
            TxIn {
                txid: hex!("411a1cba7a65659b7e03982e61e5fe8662a6abc61ce54e732fb2423f2b36f32a"),
                output_index: 1,
                script_size: VarInt(35),
                script_sig: Script(vec![Instruction::PushBytes(PushBytes::Bytes(34, hex!("00204b02351ea14b39d86295a1878a1e1cebd0ba12cff24394224f2851d28e9738c8").to_vec()))]),
                sequence: 0xffffffff,
            },
        ],
        outputs: vec![
            TxOut {
                amount: 2500,
                script_size: VarInt(23),
                script_pub_key: StandardScript::P2SH(
                    hex!("219ee3cffaab831b97dd18ef8df2d536d94cbb75").to_vec(),
                )
                .into_script(),
            },
            TxOut {
                amount: 893542,
                script_size: VarInt(23),
                script_pub_key: StandardScript::P2SH(
                    hex!("7116934db63bfd021aa71144b9d1aa11c5d381b6").to_vec(),
                )
                .into_script(),
            },
        ],
        witnesses: vec![
            Witness(
                vec![
                    PushBytes::Empty,
                    PushBytes::Bytes(71, hex!("3044022076278cfdf6a8ae61247248da2557851001f01603b7a8a32502bb6fc416916022022019fd1a593f4c04ed8c3d2cdae666ce97b46fa49af30cc4e3473b1ac7c71740ff01").to_vec()),
                    PushBytes::Bytes(72, hex!("304502210097c6bdea165a0bca2df227b022d77270c66716a26284552f0885a14af7acd77e022048afe78aa69cca7a08f1d1c0d5aec4be6a5a76101a9d3ae9607242f25a9ab29001").to_vec()),
                    PushBytes::Bytes(71, hex!("522102b9cb57116e6fde616c70d38c79f755b4d55b098b764dba66027ddcd0bb20638e21035c1546bd07bc213aa73f683cf1010205f73b47d95b2aa12b9c1633495c82239552ae").to_vec()),
                ]
            ),
        ],
        lock_time: 0,
    }
}

fn locking_script() -> Script {
    StandardScript::P2SH(hex!("7116934db63bfd021aa71144b9d1aa11c5d381b6").to_vec()).into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("a38a683cbbd65856d8ec91dfbb5ac3a6c601e182cd49b018c31f07c0ddad85b4")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script, 898102));
}
