use crate::{random_gen, utils::*};
use lazy_static::*;
use std::collections::HashSet;

lazy_static! {
    // static ref visited: Mutex<HashSet<u32>> = Mutex::new(HashSet::new());
    static ref blocks: Vec<u32> = (0..30).map(|_| random_gen!(u32)).collect();
}
pub fn touch_blocks() -> usize {
    blocks.len()
}

fn foo_1() {
    let mut visited: HashSet<u32> = HashSet::new();
    for current in blocks.iter() {
        if visited.contains(current) {}
        visited.insert(*current);
    }
}

fn foo_2(visited: &mut HashSet<u32>) {
    visited.clear();
    for current in blocks.iter() {
        if visited.contains(current) {}
        visited.insert(*current);
    }
}

pub fn _1f() {
    for _ in 0..10 {
        foo_1();
    }
}

pub fn _1o() {
    let mut visited: HashSet<u32> = HashSet::new();
    for _ in 0..10 {
        foo_2(&mut visited);
    }
}

pub fn _2f() -> Vec<u32> {
    let cap = 10000;
    let mut x = std::vec::from_elem(0u32, cap);
    x[2] = 12;
    x
}

pub fn _2o() -> Vec<u32> {
    let cap = 10000;
    let mut buf = Vec::with_capacity(cap);
    unsafe {
        buf.set_len(cap);
    }
    buf[2] = 12;
    buf
}
