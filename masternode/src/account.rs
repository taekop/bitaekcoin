use k256::{
    ecdsa::{Signature, SigningKey, VerifyingKey},
    elliptic_curve::generic_array::GenericArray,
};

#[derive(Debug, Clone)]
pub struct Account {
    pub public_key: VerifyingKey,
    pub private_key: SigningKey,
}

impl Account {
    pub fn new(private_key: Vec<u8>) -> Self {
        let private_key = SigningKey::from_bytes(&GenericArray::from_iter(private_key)).unwrap();
        let public_key = VerifyingKey::from(&private_key);
        Self {
            public_key,
            private_key,
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
        let account = Account::new(PRIVATE_KEY.to_vec());
        println!("{:X?}", account.public_key.to_sec1_bytes())
    }
}
