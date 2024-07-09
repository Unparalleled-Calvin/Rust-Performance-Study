use std::collections::BTreeMap;
use std::collections::HashMap;
use ahash::AHashMap;
use fxhash::FxHashMap;

use crate::{random_gen, utils::*};
use lazy_static::*;

const len: usize = 400;

lazy_static! {
    static ref data: Vec<usize> = (0..2000).map(|_| random_gen!(usize)).collect();
}

pub fn touch_data() -> usize {
    data.len()
}

pub fn _1f() {
    let mut map: HashMap<u64, String> = HashMap::new();
    for i in 0..30 {
        map.insert(data[i] as u64, random_word(5));
    }
    for i in 0..30 {
        map.contains_key(&(data[2 * i] as u64));
    }
}

pub fn _1o() {
    let mut map: BTreeMap<u64, String> = BTreeMap::new();
    for i in 0..30 {
        map.insert(data[i] as u64, random_word(5));
    }
    for i in 0..30 {
        map.contains_key(&(data[2 * i] as u64));
    }
}

pub fn _2f() {
    let mut cnt: usize = 0;
    for i in 0..1000 {
        std::hint::black_box({
            let index: Vec<usize> = vec![data[i * 2], data[i * 2 + 1]];
            for _ in 0..100 {
                std::hint::black_box({
                    cnt += index[1];
                })
            }
        })
    }
}

pub fn _2o() {
    let mut cnt: usize = 0;
    for i in 0..1000 {
        std::hint::black_box({
            let index = [data[i * 2], data[i * 2 + 1]];
            for _ in 0..100 {
                std::hint::black_box({
                    cnt += index[1];
                })
            }
        })
    }
}


pub fn f1() {
    let mut map: HashMap<u64, String> = HashMap::new();
    for i in 0..len {
        map.insert(data[i] as u64, String::new());
    }
    for i in 0..len {
        map.contains_key(&(data[2 * i] as u64));
    }
}

pub fn f2() {
    let mut map: BTreeMap<u64, String> = BTreeMap::new();
    for i in 0..len {
        map.insert(data[i] as u64, String::new());
    }
    for i in 0..len {
        map.contains_key(&(data[2 * i] as u64));
    }
}

pub fn f3() {
    let mut map: AHashMap<u64, String> = AHashMap::new();
    for i in 0..len {
        map.insert(data[i] as u64, String::new());
    }
    for i in 0..len {
        map.contains_key(&(data[2 * i] as u64));
    }
}

pub fn f4() {
    let mut map: FxHashMap<u64, String> = FxHashMap::default();
    for i in 0..len {
        map.insert(data[i] as u64, String::new());
    }
    for i in 0..len {
        map.contains_key(&(data[2 * i] as u64));
    }
}