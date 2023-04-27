use std::collections::VecDeque;

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
