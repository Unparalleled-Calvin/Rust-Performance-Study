use crate::{random_gen, utils::*};
use std::ptr;

use lazy_static::*;
const len: usize = 100;

lazy_static! {
    static ref data: Vec<i32> = (0..len).map(|_| { random_gen!(i32) }).collect();
}

pub fn touch_data() -> usize {
    data.len()
}

pub fn _1f() {
    let mut v = Vec::<i32>::new();
    v.reserve(len);
    let mut line_start: i32 = 0;
    for i in 0..len {
        line_start = line_start + data[i];
        v.push(line_start);
    }
}

pub fn _1o() {
    let mut v = Vec::<i32>::new();
    v.reserve(len);
    let mut line_start: i32 = 0;
    v.extend((0..len).map(|i| {
        line_start = line_start + data[i];
        line_start
    }))
}

pub fn _2f() {
    let mut v = Vec::<i32>::new();
    v.reserve(len);
    let mut line_start: i32 = 0;
    for i in 0..len {
        line_start = line_start + data[i];
        v.push(line_start);
    }
}

pub fn _2o() {
    let mut v = Vec::<i32>::new();
    v.reserve(len);
    let mut line_start: i32 = 0;
    for i in 0..len {
        line_start = line_start + data[i];
        unsafe{ ptr::write(v.as_mut_ptr().add(i), line_start) }
    }
}
