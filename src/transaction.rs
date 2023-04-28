use std::collections::VecDeque;

use k256::ecdsa::{signature::hazmat::PrehashVerifier, VerifyingKey};

use crate::{
    encode::{Encodable, VarInt},
    hash::{ripemd160, sha256, SigHash},
    script::{
        instruction::PushBytes, Script, StandardScript, StandardScriptType, UnlockingStandardScript,
    },
    utils::signature_sighash,
};

pub type TxID = [u8; 32];

#[derive(Debug, Clone)]
pub struct Transaction {
    pub version: u32,
    pub flag: Option<u8>,
    pub inputs: Vec<TxIn>,
    pub outputs: Vec<TxOut>,
    pub witnesses: Vec<Witness>,
    pub lock_time: u32,
}

impl Transaction {
    pub fn txid(&self) -> TxID {
        let mut hash = sha256(sha256(self.encode_without_witness()).to_vec());
        hash.reverse();
        hash
    }

    pub fn wtxid(&self) -> TxID {
        let mut hash = sha256(sha256(self.encode()).to_vec());
        hash.reverse();
        hash
    }

    pub fn encode_without_witness(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.version.encode());
        bytes.append(&mut self.inputs.encode());
        bytes.append(&mut self.outputs.encode());
        bytes.append(&mut self.lock_time.encode());
        bytes
    }

    pub fn is_segwit(&self) -> bool {
        self.flag.is_some()
    }

    pub fn hash_prevouts(&self, sighash: SigHash) -> [u8; 32] {
        match sighash {
            SigHash::AnyoneCanPay => [0; 32],
            _ => {
                let mut prevouts = Vec::new();
                for tx_in in &self.inputs {
                    prevouts.extend(tx_in.txid.encode());
                    prevouts.extend(tx_in.output_index.encode());
                }
                sha256(sha256(prevouts).to_vec())
            }
        }
    }

    pub fn hash_sequence(&self, sighash: SigHash) -> [u8; 32] {
        match sighash {
            SigHash::All => {
                let mut sequences = Vec::new();
                for tx_in in &self.inputs {
                    sequences.extend(tx_in.sequence.encode());
                }
                sha256(sha256(sequences).to_vec())
            }
            _ => [0; 32],
        }
    }

    pub fn hash_outputs(&self, input_index: usize, sighash: SigHash) -> [u8; 32] {
        match sighash {
            SigHash::None => [0; 32],
            SigHash::Single if input_index < self.outputs.len() => {
                sha256(sha256(self.outputs[input_index].amount.encode()).to_vec())
            }
            _ => {
                let mut outputs = Vec::new();
                for tx_out in &self.outputs {
                    outputs.extend(tx_out.amount.encode());
                    outputs.extend(tx_out.script_size.encode());
                    outputs.extend(tx_out.script_pub_key.encode());
                }
                sha256(sha256(outputs).to_vec())
            }
        }
    }

    pub fn validate(
        &self,
        ind: usize,
        unlocking_script: &Script,
        locking_script: &Script,
        amount: u64,
    ) -> bool {
        match locking_script.to_standard() {
            Some(standard_script) => match standard_script {
                StandardScript::P2PK(pk) => {
                    match unlocking_script.to_unlocking_standard(StandardScriptType::P2PK) {
                        Some(UnlockingStandardScript::P2PK(signature, sighash)) => {
                            let hash = sighash.hash(self, ind, locking_script, amount);
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
                            let hash = sighash.hash(self, ind, locking_script, amount);
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
                                let hash = sighash.hash(self, ind, locking_script, amount);
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
                            self.validate(ind, &unlocking_script, &redeem_script, amount)
                        }
                        _ => false,
                    }
                }
                StandardScript::NullData(_) => false,
                StandardScript::P2WPKH(pkh) => {
                    if !self.is_segwit()
                        || self.witnesses.len() <= ind
                        || !unlocking_script.0.is_empty()
                    {
                        return false;
                    }
                    let witness = self.witnesses[ind].clone();
                    if witness.0.len() != 2 {
                        return false;
                    }
                    match witness.to_unlocking_standard(StandardScriptType::P2PKH) {
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
                            let hash = sighash.hash(self, ind, locking_script, amount);
                            if let Ok(verifying_key) = VerifyingKey::from_sec1_bytes(&pk) {
                                verifying_key.verify_prehash(&hash, &signature).is_ok()
                            } else {
                                false
                            }
                        }
                        _ => false,
                    }
                }
            },
            None => unimplemented!("NON STANDARD SCRIPT"),
        }
    }
}

impl Encodable for Transaction {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.append(&mut self.version.encode());
        if let Some(flag) = self.flag {
            bytes.push(0); // marker
            bytes.push(flag);
        }
        bytes.append(&mut self.inputs.encode());
        bytes.append(&mut self.outputs.encode());
        if self.flag.is_some() {
            for witness in &self.witnesses {
                bytes.append(&mut witness.encode());
            }
        }
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
pub struct Witness(pub Vec<PushBytes>);

impl Witness {
    pub fn to_unlocking_standard(&self, ty: StandardScriptType) -> Option<UnlockingStandardScript> {
        match ty {
            StandardScriptType::P2PK => None,
            StandardScriptType::P2PKH => {
                if self.0.len() != 2 {
                    return None;
                }
                let sig = self.0[0].bytes();
                if let Some((signature, sighash)) = signature_sighash(sig) {
                    let pk = self.0[1].bytes();
                    Some(UnlockingStandardScript::P2PKH(signature, sighash, pk))
                } else {
                    None
                }
            }
            StandardScriptType::P2MS => None,
            StandardScriptType::P2SH => todo!(),
            StandardScriptType::NullData => None,
        }
    }
}

impl Encodable for Witness {
    fn encode(&self) -> Vec<u8> {
        self.0.encode()
    }
}
