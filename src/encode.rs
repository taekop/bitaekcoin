use std::collections::VecDeque;

use crate::utils::pop_front;

pub trait Encodable {
    fn encode(&self) -> Vec<u8>;
}

pub trait Decodable: Sized {
    fn decode(bytes: &mut VecDeque<u8>) -> Result<Self, DecodeError>;
}

pub struct DecodeError;

macro_rules! impl_int_encodable {
    ($ty:ident) => {
        impl Encodable for $ty {
            fn encode(&self) -> Vec<u8> {
                self.to_le_bytes().to_vec()
            }
        }
    };
}

impl_int_encodable!(u8);
impl_int_encodable!(u16);
impl_int_encodable!(u32);
impl_int_encodable!(u64);

impl Encodable for [u8; 32] {
    fn encode(&self) -> Vec<u8> {
        let mut bytes = *self;
        bytes.reverse();
        bytes.to_vec()
    }
}

impl<T: Encodable> Encodable for Vec<T> {
    fn encode(&self) -> Vec<u8> {
        let len = VarInt(self.len() as u64);
        let mut bytes = len.encode();
        for item in self {
            bytes.append(&mut item.encode());
        }
        bytes
    }
}

#[derive(Debug, Clone)]
pub struct VarInt(pub u64);

impl Encodable for VarInt {
    fn encode(&self) -> Vec<u8> {
        match self.0 {
            0..=0xFC => (self.0 as u8).to_be_bytes().to_vec(),
            0xFD..=0xFFFF => {
                let mut bytes = vec![0xFD];
                bytes.extend_from_slice(&(self.0 as u16).to_be_bytes());
                bytes
            }
            0x10000..=0xFFFFFFFF => {
                let mut bytes = vec![0xFE];
                bytes.extend_from_slice(&(self.0 as u32).to_be_bytes());
                bytes
            }
            _ => {
                let mut bytes = vec![0xFF];
                bytes.extend_from_slice(&self.0.to_be_bytes());
                bytes
            }
        }
    }
}

impl Decodable for VarInt {
    fn decode(bytes: &mut VecDeque<u8>) -> Result<Self, DecodeError> {
        let first_byte = pop_front(bytes, 1, DecodeError)?[0];
        let vi = match first_byte {
            0x00..=0xFC => VarInt(first_byte as u64),
            0xFD => {
                let value = pop_front(bytes, 2, DecodeError)?;
                VarInt(u16::from_be_bytes(value.try_into().unwrap()) as u64)
            }
            0xFE => {
                let value = pop_front(bytes, 4, DecodeError)?;
                VarInt(u32::from_be_bytes(value.try_into().unwrap()) as u64)
            }
            0xFF => {
                let value = pop_front(bytes, 8, DecodeError)?;
                VarInt(u64::from_be_bytes(value.try_into().unwrap()))
            }
        };
        Ok(vi)
    }
}

#[cfg(test)]
mod tests {
    use hex_literal::hex;

    use super::*;

    #[test]
    fn test_varint() {
        // https://wiki.bitcoinsv.io/index.php/VarInt
        assert_eq!(VarInt(187).encode(), hex!("BB"));
        assert_eq!(VarInt(255).encode(), hex!("FD00FF"));
        assert_eq!(VarInt(13337).encode(), hex!("FD3419"));
        assert_eq!(VarInt(14435729).encode(), hex!("FE00DC4591"));
        assert_eq!(VarInt(134250981).encode(), hex!("FE080081E5"));
        assert_eq!(VarInt(198849843832919).encode(), hex!("FF0000B4DA564E2857"));
        assert_eq!(
            VarInt(5473425651754713432).encode(),
            hex!("FF4BF583A17D59C158")
        );
    }
}
