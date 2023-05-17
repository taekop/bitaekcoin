use k256::{
    ecdsa::{Signature, SigningKey, VerifyingKey},
    elliptic_curve::generic_array::GenericArray,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Account {
    pub index: usize,
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
    pub balance: u64,
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
        }
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
    fn test_account() {
        let account = Account::new(0, PRIVATE_KEY.to_vec());
        assert!(
            account.public_key.to_sec1_bytes().to_vec()
                == account.private_key.verifying_key().to_sec1_bytes().to_vec()
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountJson {
    pub index: usize,
    pub public_key: Vec<u8>,
    pub balance: u64,
}

impl From<Account> for AccountJson {
    fn from(value: Account) -> Self {
        Self {
            index: value.index,
            public_key: value.public_key.to_sec1_bytes().to_vec(),
            balance: value.balance,
        }
    }
}
