use crate::{random_gen, utils::*};
use lazy_static::*;

lazy_static! {
    static ref pixels: Vec<[u8; 4]> = (0..100)
        .map(|_| {
            let mut rng = thread_rng();
            [
                random_gen!(u8),
                random_gen!(u8),
                random_gen!(u8),
                random_gen!(u8),
            ]
        })
        .collect();
    static ref d: Vec<Vec<u8>> = (0..10).map(|_| {
        (0..100).map(|_| {
            random_gen!(u8)
        }).collect()
    }).collect();
}

pub fn touch_pixels() -> usize {
    pixels.len() + d.len()
}

pub fn _1f() {
    let _pixels: Vec<u8> = pixels
        .iter()
        .flat_map(|srgba| [srgba[0], srgba[1], srgba[2], srgba[3]].into_iter())
        .collect();
}

pub fn _1o() {
    let mut data: Vec<u8> = Vec::new();
    for srgba in pixels.iter() {
        data.extend([srgba[0], srgba[1], srgba[2], srgba[3]].into_iter())
    }
}

pub fn _2f() { //单纯测试flatmap和extend的性能
    let mut x: Vec<&u8> = d.iter().flatten().collect();
}

pub fn _2o() {
    let mut x: Vec<&u8> = Vec::new();
    for i in d.iter() {
        x.extend(i);
    }
}