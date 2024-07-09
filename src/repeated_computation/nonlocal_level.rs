use crate::{random_gen, utils::*};
use md5::Digest;
use std::collections::HashMap;
fn foo(x: u64) -> Digest {
    let mut buffer = itoa::Buffer::new();
    let printed = buffer.format(x);
    let digest = md5::compute(printed.as_bytes());
    digest
}

use lazy_static::*;

lazy_static! {
    static ref data: Vec<u64> = (1..51).map(|i| 12345 / i).chain((0..50).map(|_| 1)).collect();
}

pub fn touch_data() -> usize {
    data.len()
}

pub fn _1f() {
    for i in data.iter() {
        foo(*i);
    }
}

pub fn _1o() {
    let mut cache: HashMap<u64, Digest> = HashMap::new();
    for i in data.iter() {
        if let Some(ty) = cache.get(i) {
            
        } else {
            cache.insert(*i, foo(*i));
        }
    }
}
