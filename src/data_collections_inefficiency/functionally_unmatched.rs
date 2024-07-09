use std::collections::HashSet;

use crate::{random_gen, utils::*};
use lazy_static::*;

lazy_static! {
    static ref names: Vec<String> = (0..100).map(|_| { random_word(3) }).collect();
    static ref df: Vec<String> = (0..100).map(|_| { random_word(3) }).collect();
}

pub fn touch_names() -> usize {
    df.len();
    names.len()
}

pub fn _1f() {
    let _names: Vec<&str> = names.iter().map(|s| s.as_ref()).collect();
    df.iter().for_each(|s| if !_names.contains(&s.as_str()) {})
}

pub fn _1o() {
    let _names: HashSet<&str> = names.iter().map(|s| s.as_ref()).collect();
    df.iter().for_each(|s| if !_names.contains(&s.as_str()) {})
}
