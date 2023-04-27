use std::collections::VecDeque;

use k256::ecdsa::{signature::hazmat::PrehashVerifier, VerifyingKey};

use crate::{
    encode::{Encodable, VarInt},
    hash::{ripemd160, sha256},
    script::{Script, StandardScript, StandardScriptType, UnlockingStandardScript},
};

pub type TxID = [u8; 32];

// TODO: flag, witness
#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: u32,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn txid(&self) -> TxID {
        let mut hash = sha256(sha256(self.encode()).to_vec());
        hash.reverse();
        hash
    }

    pub fn validate(&self, ind: usize, unlocking_script: &Script, locking_script: &Script) -> bool {
        match locking_script.to_standard() {
            Some(standard_script) => match standard_script {
                StandardScript::P2PK(pk) => {
                    match unlocking_script.to_unlocking_standard(StandardScriptType::P2PK) {
                        Some(UnlockingStandardScript::P2PK(signature, sighash)) => {
                            let hash = sighash.hash(self, ind, locking_script);
                            if let Ok(verifying_key) = VerifyingKey::from_sec1_bytes(&pk) {
                                verifying_key.verify_prehash(&hash, &signature).is_ok()
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                }
                StandardScript::P2PKH(pkh) => {
                    match unlocking_script.to_unlocking_standard(StandardScriptType::P2PKH) {
                        Some(UnlockingStandardScript::P2PKH(signature, sighash, pk)) => {
                            let pkh2 = ripemd160(sha256(pk.clone()).to_vec());
                            if pkh.len() != 20 {
                                return false;
                            }
                            for i in 0..20 {
                                if pkh[i] != pkh2[i] {
                                    return false;
                                }
                            }
                            let hash = sighash.hash(self, ind, locking_script);
                            if let Ok(verifying_key) = VerifyingKey::from_sec1_bytes(&pk) {
                                verifying_key.verify_prehash(&hash, &signature).is_ok()
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                }
                StandardScript::P2MS(m, _, pks) => {
                    match unlocking_script.to_unlocking_standard(StandardScriptType::P2MS) {
                        Some(UnlockingStandardScript::P2MS(sigs)) => {
                            let mut matches = 0;
                            let mut sigs = VecDeque::from(sigs);
                            let mut pks = VecDeque::from(pks);
                            while !sigs.is_empty() {
                                let (signature, sighash) = sigs.pop_front().unwrap();
                                let hash = sighash.hash(self, ind, locking_script);
                                while !pks.is_empty() {
                                    let pk = pks.pop_front().unwrap();
                                    if let Ok(verifying_key) = VerifyingKey::from_sec1_bytes(&pk) {
                                        if verifying_key.verify_prehash(&hash, &signature).is_ok() {
                                            matches += 1;
                                            break;
                                        }
                                    } else {
                                        return false;
                                    }
                                }
                            }
                            matches >= m
                        }
                        _ => false,
                    }
                }
                StandardScript::P2SH(sh) => {
                    match unlocking_script.to_unlocking_standard(StandardScriptType::P2SH) {
                        Some(UnlockingStandardScript::P2SH(unlocking_script, redeem_script)) => {
                            let sh2 = ripemd160(sha256(redeem_script.encode()).to_vec());
                            if sh.len() != 20 {
                                return false;
                            }
                            for i in 0..20 {
                                if sh[i] != sh2[i] {
                                    return false;
                                }
                            }
                            self.validate(ind, &unlocking_script, &redeem_script)
                        }
                        _ => false,
                    }
                }
                StandardScript::NullData(_) => todo!(),
            },
            None => unimplemented!("NON STANDARD SCRIPT"),
        }
    }
}

impl Encodable for Transaction {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.version.encode());
        bytes.append(&mut self.inputs.encode());
        bytes.append(&mut self.outputs.encode());
        bytes.append(&mut self.lock_time.encode());
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct TxIn {
    pub txid: TxID,
    pub output_index: u32,
    pub script_size: VarInt,
    pub script_sig: Script,
    pub sequence: u32,
}

impl Encodable for TxIn {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.txid.encode());
        bytes.append(&mut self.output_index.encode());
        bytes.append(&mut self.script_size.encode());
        bytes.append(&mut self.script_sig.encode());
        bytes.append(&mut self.sequence.encode());
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct TxOut {
    pub amount: u64,
    pub script_size: VarInt,
    pub script_pub_key: Script,
}

impl Encodable for TxOut {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.amount.encode());
        bytes.append(&mut self.script_size.encode());
        bytes.append(&mut self.script_pub_key.encode());
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct Witness;

impl Encodable for Witness {
    fn encode(&self) -> Vec<u8> {
        todo!()
    }
}
