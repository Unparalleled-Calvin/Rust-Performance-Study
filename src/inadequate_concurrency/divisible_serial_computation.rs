use crate::{random_gen, utils::*};
use lazy_static::*;
use rayon::prelude::*;

const X: usize = 100;

lazy_static! {
    static ref m_a: Vec<u32> = (0..X * X).map(|_| random_gen!(u32)).collect();
    static ref m_b: Vec<u32> = (0..X * X).map(|_| random_gen!(u32)).collect();
}

pub fn touch_m() -> usize {
    m_a.len() + m_b.len()
}

pub fn _1f() -> Vec<u32> {
    let mut m_c: Vec<u32> = vec![0; X * X];
    for i in 0..X {
        for j in 0..X {
            for k in 0..X {
                m_c[i * X + j] += m_a[i * X + k] * m_b[k * X + j];
            }
        }
    }
    m_c
}

pub fn _1o() -> Vec<u32> {
    let mut m_c: Vec<u32> = vec![0; X * X];
    m_c.par_chunks_mut(X).enumerate().for_each(|(i, row)| {
        for j in 0..X {
            for k in 0..X {
                row[j] += m_a[i * X + k] * m_b[k * X + j];
            }
        }
    });
    m_c
}
