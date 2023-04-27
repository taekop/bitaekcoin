use std::collections::HashMap;

use bitaekcoin::{
    block::{Block, BlockHeader},
    encode::VarInt,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxID, TxIn, TxOut},
};
use hex_literal::hex;

// Example from https://blockstream.info/block/000000000003ba27aa200b1cecaad478d2b00432346c3f1f3986da1afd33e506
fn block() -> Block {
    Block {
        header: BlockHeader {
            version: 1,
            prev_block_hash: hex!(
                "000000000002d01c1fccc21636b607dfd930d31d01c3a62104612a1719011250"
            ),
            merkle_root: hex!("f3e94742aca4b5ef85488dc37c06c3282295ffec960994b2c0d5ac2a25a95766"),
            timestamp: 1293623863,
            bits: 0x1b04864c,
            nonce: 0x10572b0f,
        },
        transactions: vec![tx1(), tx2(), tx3(), tx4()],
    }
}

// 8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87
fn tx1() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("0000000000000000000000000000000000000000000000000000000000000000"),
            output_index: 0xffffffff,
            script_size: VarInt(8),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Bytes(4, hex!("4c86041b").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(2, hex!("0602").to_vec())),
            ]),
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOut {
            amount: 5000000000,
            script_size: VarInt(67),
            script_pub_key: StandardScript::P2PK(hex!("041b0e8c2567c12536aa13357b79a073dc4444acb83c4ec7a0e2f99dd7457516c5817242da796924ca4e99947d087fedf9ce467cb9f7c6287078f801df276fdf84").to_vec()).into_script(),
        }],
        lock_time: 0,
    }
}

// fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4
fn tx2() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("87a157f3fd88ac7907c05fc55e271dc4acdc5605d187d646604ca8c0e9382e03"),
            output_index: 0,
            script_size: VarInt(140),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Bytes(73, hex!("3046022100c352d3dd993a981beba4a63ad15c209275ca9470abfcd57da93b58e4eb5dce82022100840792bc1f456062819f15d33ee7055cf7b5ee1af1ebcc6028d9cdb1c3af774801").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(65, hex!("04f46db5e9d61a9dc27b8d64ad23e7383a4e6ca164593c2527c038c0857eb67ee8e825dca65046b82c9331586c82e0fd1f633f25f87c161bc6f8a630121df2b3d3").to_vec())),
            ]),
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOut {
            amount: 556000000,
            script_size: VarInt(25),
            script_pub_key: StandardScript::P2PKH(hex!("c398efa9c392ba6013c5e04ee729755ef7f58b32").to_vec()).into_script(),
        }, TxOut {
            amount: 4444000000,
            script_size: VarInt(25),
            script_pub_key: StandardScript::P2PKH(hex!("948c765a6914d43f2a7ac177da2c2f6b52de3d7c").to_vec()).into_script(),
        }],
        lock_time: 0,
    }
}

// 6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4
fn tx3() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("cf4e2978d0611ce46592e02d7e7daf8627a316ab69759a9f3df109a7f2bf3ec3"),
            output_index: 1,
            script_size: VarInt(138),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Bytes(71, hex!("30440220032d30df5ee6f57fa46cddb5eb8d0d9fe8de6b342d27942ae90a3231e0ba333e02203deee8060fdc70230a7f5b4ad7d7bc3e628cbe219a886b84269eaeb81e26b4fe01").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(65, hex!("04ae31c31bf91278d99b8377a35bbce5b27d9fff15456839e919453fc7b3f721f0ba403ff96c9deeb680e5fd341c0fc3a7b90da4631ee39560639db462e9cb850f").to_vec())),
            ]),
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOut {
            amount: 1000000,
            script_size: VarInt(25),
            script_pub_key: StandardScript::P2PKH(hex!("b0dcbf97eabf4404e31d952477ce822dadbe7e10").to_vec()).into_script(),
        }, TxOut {
            amount: 299000000,
            script_size: VarInt(25),
            script_pub_key: StandardScript::P2PKH(hex!("6b1281eec25ab4e1e0793ff4e08ab1abb3409cd9").to_vec()).into_script(),
        }],
        lock_time: 0,
    }
}

// e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d
fn tx4() -> Transaction {
    Transaction {
        version: 1,
        inputs: vec![TxIn {
            txid: hex!("f4515fed3dc4a19b90a317b9840c243bac26114cf637522373a7d486b372600b"),
            output_index: 0,
            script_size: VarInt(140),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Bytes(73, hex!("3046022100bb1ad26df930a51cce110cf44f7a48c3c561fd977500b1ae5d6b6fd13d0b3f4a022100c5b42951acedff14abba2736fd574bdb465f3e6f8da12e2c5303954aca7f78f301").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(65, hex!("04a7135bfe824c97ecc01ec7d7e336185c81e2aa2c41ab175407c09484ce9694b44953fcb751206564a9c24dd094d42fdbfdd5aad3e063ce6af4cfaaea4ea14fbb").to_vec())),
            ]),
            sequence: 0xffffffff,
        }],
        outputs: vec![TxOut {
            amount: 1000000,
            script_size: VarInt(25),
            script_pub_key: StandardScript::P2PKH(hex!("39aa3d569e06a1d7926dc4be1193c99bf2eb9ee0").to_vec()).into_script(),
        }],
        lock_time: 0,
    }
}

fn prev_outpoints() -> HashMap<(TxID, u32), Script> {
    HashMap::from_iter([
        (
            (
                hex!("87a157f3fd88ac7907c05fc55e271dc4acdc5605d187d646604ca8c0e9382e03"),
                0,
            ),
            StandardScript::P2PKH(hex!("71d7dd96d9edda09180fe9d57a477b5acc9cad11").to_vec())
                .into_script(),
        ),
        (
            (
                hex!("cf4e2978d0611ce46592e02d7e7daf8627a316ab69759a9f3df109a7f2bf3ec3"),
                1,
            ),
            StandardScript::P2PKH(hex!("35fbee6a3bf8d99f17724ec54787567393a8a6b1").to_vec())
                .into_script(),
        ),
        (
            (
                hex!("f4515fed3dc4a19b90a317b9840c243bac26114cf637522373a7d486b372600b"),
                0,
            ),
            StandardScript::P2PKH(hex!("c4eb47ecfdcf609a1848ee79acc2fa49d3caad70").to_vec())
                .into_script(),
        ),
    ])
}

#[test]
fn test() {
    let block = block();
    let outpoints = prev_outpoints();
    assert!(block.validate(outpoints));
}
