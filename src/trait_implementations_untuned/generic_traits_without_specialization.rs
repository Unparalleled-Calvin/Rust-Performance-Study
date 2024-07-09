use lazy_static::*;
use std::collections::BinaryHeap;

use crate::{random_gen, utils::*};

lazy_static! {
    static ref other: Vec<u32> = (0..100).map(|_| random_gen!(u32)).collect();
}

pub fn touch_others() -> usize {
    other.len()
}

pub fn _1f() {
    let mut a: BinaryHeap<u32> = BinaryHeap::new();
    let _other = other.clone();
    let iterator = _other.into_iter();
    let (lower, _) = iterator.size_hint();
    a.reserve(lower);
    iterator.for_each(move |elem| a.push(elem));
}

pub fn _1o() {
    let mut a: BinaryHeap<u32> = BinaryHeap::new();
    let _other = other.clone();
    let iterator = _other.into_iter();
    a.extend(iterator);
}
