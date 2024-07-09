use lazy_static::*;
use smallvec::SmallVec;

lazy_static! {
    static ref s: SmallVec<[u8; 20]> =
        SmallVec::from_vec("12345678901234567890".as_bytes().to_vec());
    static ref l: Vec<u32> = (0..10000).collect();
}

pub fn touch_s() -> usize {
    s.len() + l.len()
}

pub fn _1f() -> SmallVec<[u8; 20]> {
    let mut _s = s.clone();
    fn forgiving_base64_decode(mut input: Vec<u8>) -> Vec<u8> {
        let len = input.len() / 2;
        input.truncate(len);
        input
    }
    forgiving_base64_decode(_s.into_vec()).into()
}

pub fn _1o() -> SmallVec<[u8; 20]> {
    let mut _s = s.clone();
    fn forgiving_base64_decode(input: &mut [u8]) -> usize {
        input.len() / 2
    }
    let len = forgiving_base64_decode(&mut _s);
    _s.truncate(len);
    _s
}


pub fn _2f() {
    let mut v = l.as_slice().to_owned();
    v[1] = 1;
}

pub fn _2o() {
}