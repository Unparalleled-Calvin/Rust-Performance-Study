use crate::{random_gen, utils::*};
use lazy_static::*;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Bar_1 {
    span: (u32, u16, u16),
    body_id: (u32, u32),
    code: NestedHashableEnum,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Bar_2 {
    span: (u32, u16, u16),
    body_id: (u32, u32),
    code: NestedHashableEnum,
}

impl Hash for Bar_2 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.body_id.hash(state);
        self.span.hash(state);
        std::mem::discriminant(&self.code).hash(state);
    }
}

lazy_static! {
    static ref bar_1s: Vec<Bar_1> = (0..100).map(|_| gen_bar_1()).collect();
    static ref bar_2s: Vec<Bar_2> = (0..100).map(|i| to_bar_2(&bar_1s[i])).collect();
}

pub fn touch_bars() -> usize {
    bar_1s.len() + bar_2s.len()
}

pub fn _1f() {
    let mut set: HashSet<Bar_1> = HashSet::new();
    for bar in bar_1s.clone() {
        set.insert(bar);
    }
}

pub fn _1o() {
    let mut set: HashSet<Bar_2> = HashSet::new();
    for bar in bar_2s.clone() {
        set.insert(bar);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum NestedHashableEnum {
    Integer(i32),
    Float(i64),
    Text(String),
    Tuple((i32, i64)),
}

fn gen_enum() -> NestedHashableEnum {
    match thread_rng().gen_range(0..=3) {
        0 => NestedHashableEnum::Integer(random_gen!(i32)),
        1 => NestedHashableEnum::Float(random_gen!(i64)),
        2 => NestedHashableEnum::Text(random_word(5)),
        3 => NestedHashableEnum::Tuple((random_gen!(i32), random_gen!(i64))),
        _ => panic!(),
    }
}

fn gen_bar_1() -> Bar_1 {
    Bar_1 {
        span: (random_gen!(u32), random_gen!(u16), random_gen!(u16)),
        body_id: (random_gen!(u32), random_gen!(u32)),
        code: gen_enum(),
    }
}

fn to_bar_2(bar: &Bar_1) -> Bar_2 {
    Bar_2 {
        span: bar.span.clone(),
        body_id: bar.body_id.clone(),
        code: bar.code.clone(),
    }
}
