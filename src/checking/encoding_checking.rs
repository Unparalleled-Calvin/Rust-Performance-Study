use std::str;
use lazy_static::*;

const len: usize = 500;

lazy_static! {
    static ref s1: Vec<u8> = (0..len).map(|_| '0' as u8).collect();
    static ref s2: Vec<u8> = (0..(len / 3 + 1)).flat_map(|_| {
        [228 as u8, 184 as u8, 173 as u8] //'中'
    }).collect();

}

fn touch_data() -> usize {
    s1.len() + s2.len()
}

pub fn _1f() {
    let mut output: String = String::new();
    output.push_str(str::from_utf8(&s1).unwrap());
}

pub fn _1o() {
    let mut output: String = String::new();
    output.push_str(unsafe{ str::from_utf8_unchecked(&s1)} );
}

pub fn _2f() {
    let mut output: String = String::new();
    output.push_str(str::from_utf8(&s2).unwrap());
}

pub fn _2o() {
    let mut output: String = String::new();
     output.push_str(unsafe{ str::from_utf8_unchecked(&s2)} );
}