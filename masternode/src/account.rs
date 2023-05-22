use std::collections::HashMap;

use k256::{
    ecdsa::{Signature, SigningKey, VerifyingKey},
    elliptic_curve::generic_array::GenericArray,
};
use serde::{Deserialize, Serialize};

use bitaekcoin::{
    encode::{Encodable, VarInt},
    hash::SigHash,
    script::{Script, StandardScript, UnlockingStandardScript},
    transaction::{Transaction, TxID, TxIn, TxOut},
};

#[derive(Debug, Clone)]
pub struct Account {
    pub index: usize,
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
    pub balance: u64,
    pub utxos: HashMap<(TxID, u32), TxOut>,
}

impl Account {
    pub fn new(index: usize, private_key: Vec<u8>) -> Self {
        let private_key = SigningKey::from_bytes(&GenericArray::from_iter(private_key)).unwrap();
        let public_key = VerifyingKey::from(&private_key);
        Self {
            index,
            public_key,
            private_key,
            balance: 0,
            utxos: HashMap::new(),
        }
    }

    pub fn random(index: usize) -> Self {
        Account::new(
            index,
            SigningKey::random(&mut rand::thread_rng())
                .to_bytes()
                .to_vec(),
        )
    }

    pub fn transfer(&self, to: &VerifyingKey, amount: u64) -> Result<Transaction, String> {
        if amount > self.balance {
            return Err("Transfer amount is larger than balance".to_owned());
        }
        let mut sum = 0;
        let mut inputs = Vec::new();
        let mut data_for_sign = Vec::new();
        for ((txid, output_index), tx_out) in &self.utxos {
            inputs.push(TxIn {
                txid: *txid,
                output_index: *output_index,
                script_size: VarInt(0),
                script_sig: Script(Vec::new()),
                sequence: 0xffffffff,
            });
            data_for_sign.push((tx_out.script_pub_key.clone(), tx_out.amount));

            sum += tx_out.amount;
            if sum >= amount {
                break;
            }
        }
        let mut outputs = Vec::new();
        let locking_script = StandardScript::P2PK(to.to_sec1_bytes().to_vec()).into_script();
        outputs.push(TxOut {
            amount,
            script_size: VarInt(locking_script.encode().len() as u64),
            script_pub_key: locking_script,
        });
        if sum > amount {
            let locking_script =
                StandardScript::P2PK(self.public_key.to_sec1_bytes().to_vec()).into_script();
            outputs.push(TxOut {
                amount: sum - amount,
                script_size: VarInt(locking_script.encode().len() as u64),
                script_pub_key: locking_script,
            });
        }

        let mut tx = Transaction {
            version: 1,
            flag: None,
            inputs,
            outputs,
            witnesses: Vec::new(),
            lock_time: 0,
        };

        let mut signatures = Vec::new();
        for (i, (script, amount)) in data_for_sign.into_iter().enumerate() {
            let hash = SigHash::All.hash(&tx, i, &script, amount);
            let signature = self.sign(hash.to_vec());
            signatures.push(signature);
            tx.inputs[i].script_sig =
                UnlockingStandardScript::P2PK(signature, SigHash::All).into_script();
            tx.inputs[i].script_size = VarInt(tx.inputs[i].script_sig.encode().len() as u64);
        }

        Ok(tx)
    }

    pub fn sign(&self, msg: Vec<u8>) -> Signature {
        self.private_key.sign_prehash_recoverable(&msg).unwrap().0
    }
}

#[cfg(test)]
mod tests {

    use crate::PRIVATE_KEY;

    use super::*;

    #[test]
    fn test_miner_account() {
        let account = Account::new(0, PRIVATE_KEY.to_vec());
        assert!(
            account.public_key.to_sec1_bytes().to_vec()
                == account.private_key.verifying_key().to_sec1_bytes().to_vec()
        )
    }

    #[test]
    fn test_transfer() {
        let mut sender = Account::new(0, PRIVATE_KEY.to_vec());
        let receiver = Account::random(1);

        let locking_script =
            StandardScript::P2PK(sender.public_key.to_sec1_bytes().to_vec()).into_script();
        let utxos = {
            HashMap::from_iter([(
                ([0; 32], 0),
                TxOut {
                    amount: 1,
                    script_size: VarInt(locking_script.encode().len() as u64),
                    script_pub_key: locking_script.clone(),
                },
            )])
        };
        sender.balance = 1;
        sender.utxos = utxos;
        let tx = sender.transfer(&receiver.public_key, 1).unwrap();
        assert!(tx.validate(0, &tx.inputs[0].script_sig, &locking_script, 1));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountJson {
    pub index: usize,
    pub public_key: Vec<u8>,
    pub balance: u64,
    pub utxos: Vec<TxOutJson>,
}

impl From<Account> for AccountJson {
    fn from(value: Account) -> Self {
        Self {
            index: value.index,
            public_key: value.public_key.to_sec1_bytes().to_vec(),
            balance: value.balance,
            utxos: value
                .utxos
                .iter()
                .map(|((txid, output_index), tx_out)| TxOutJson {
                    txid: *txid,
                    output_index: *output_index,
                    amount: tx_out.amount,
                    script_size: tx_out.script_size.clone(),
                    script_pub_key: tx_out.script_pub_key.clone(),
                })
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TxOutJson {
    pub txid: TxID,
    pub output_index: u32,
    pub amount: u64,
    pub script_size: VarInt,
    pub script_pub_key: Script,
}
