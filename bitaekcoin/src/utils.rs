use std::collections::VecDeque;

use k256::ecdsa::Signature;

use crate::hash::SigHash;

pub fn pop_front<E>(bytes: &mut VecDeque<u8>, mut n: usize, err: E) -> Result<Vec<u8>, E> {
    let mut ret = Vec::new();
    while n > 0 {
        match bytes.pop_front() {
            Some(byte) => ret.push(byte),
            None => return Err(err),
        }
        n -= 1;
    }
    Ok(ret)
}

pub fn signature_sighash(mut bytes: Vec<u8>) -> Option<(Signature, SigHash)> {
    if let Some(byte) = bytes.pop() {
        let signature = Signature::from_der(&bytes);
        let sighash = SigHash::from_byte(byte);
        match (signature, sighash) {
            (Ok(signature), Some(sighash)) => Some((signature, sighash)),
            _ => None,
        }
    } else {
        None
    }
}
