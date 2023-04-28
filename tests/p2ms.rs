use bitaekcoin::{
    encode::VarInt,
    script::{
        instruction::{Instruction, PushBytes},
        Script, StandardScript,
    },
    transaction::{Transaction, TxIn, TxOut},
};
use hex_literal::hex;

// Example from https://blockstream.info/tx/949591ad468cef5c41656c0a502d9500671ee421fadb590fbc6373000039b693
fn tx() -> Transaction {
    Transaction {
        version: 1,
        flag: None,
        inputs: vec![TxIn {
            txid: hex!("581d30e2a73a2db683ac2f15d53590bd0cd72de52555c2722d9d6a78e9fea510"),
            output_index: 0,
            script_size: VarInt(147),
            script_sig: Script(vec![
                Instruction::PushBytes(PushBytes::Empty),
                Instruction::PushBytes(PushBytes::Bytes(72, hex!("3045022100af204ef91b8dba5884df50f87219ccef22014c21dd05aa44470d4ed800b7f6e40220428fe058684db1bb2bfb6061bff67048592c574effc217f0d150daedcf36787601").to_vec())),
                Instruction::PushBytes(PushBytes::Bytes(72, hex!("3045022100e8547aa2c2a2761a5a28806d3ae0d1bbf0aeff782f9081dfea67b86cacb321340220771a166929469c34959daf726a2ac0c253f9aff391e58a3c7cb46d8b7e0fdc4801").to_vec()))
            ]),
            sequence: 0xffffffff,
        }],
        outputs: vec![
            TxOut {
                amount: 1680000,
                script_size: VarInt(25),
                script_pub_key: StandardScript::P2PKH(hex!("971802edf585cdbc4e57017d6e5142515c1e5028").to_vec()).into_script(),
            },
        ],
        witnesses: vec![],
        lock_time: 0,
    }
}

fn locking_script() -> Script {
    StandardScript::P2MS(
        2,
        3,
        vec![
            hex!("04d81fd577272bbe73308c93009eec5dc9fc319fc1ee2e7066e17220a5d47a18314578be2faea34b9f1f8ca078f8621acd4bc22897b03daa422b9bf56646b342a2").to_vec(),
            hex!("04ec3afff0b2b66e8152e9018fe3be3fc92b30bf886b3487a525997d00fd9da2d012dce5d5275854adc3106572a5d1e12d4211b228429f5a7b2f7ba92eb0475bb1").to_vec(),
            hex!("04b49b496684b02855bc32f5daefa2e2e406db4418f3b86bca5195600951c7d918cdbe5e6d3736ec2abf2dd7610995c3086976b2c0c7b4e459d10b34a316d5a5e7").to_vec(),
        ],
    )
    .into_script()
}

#[test]
fn test() {
    let tx = tx();
    assert_eq!(
        tx.txid(),
        hex!("949591ad468cef5c41656c0a502d9500671ee421fadb590fbc6373000039b693")
    );

    let locking_script = locking_script();
    assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script));
}
