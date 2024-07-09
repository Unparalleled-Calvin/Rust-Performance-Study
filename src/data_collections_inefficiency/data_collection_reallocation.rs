use std::collections::HashSet;

use crate::{random_gen, utils::*};
use lazy_static::*;

lazy_static! {
    static ref data: Vec<u32> = (0..1000).map(|_| random_gen!(u32)).collect();
}

pub fn touch_data() -> usize {
    data.len()
}

pub fn _1f() {
    let mut set: Vec<u32> = Vec::new();
    for i in data.iter() {
        set.push(i.clone());
    }
}

pub fn _1o() {
    let mut set: Vec<u32> = Vec::with_capacity(data.len());
    for i in data.iter() {
        set.push(i.clone());
    }
}

pub fn _2f() {
    let mut set: HashSet<u32> = HashSet::new();
    for i in data.iter() {
        set.insert(i.clone());
    }
}

pub fn _2o() {
    let mut set: HashSet<u32> = HashSet::with_capacity(data.len());
    for i in data.iter() {
        set.insert(i.clone());
    }
}
