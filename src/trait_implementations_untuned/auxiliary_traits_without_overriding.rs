use crate::{random_gen, utils::*};
use lazy_static::*;

#[derive(Clone)]
struct Foo_1 {
    data: Vec<i32>,
    index: usize,
}

#[derive(Clone)]
struct Foo_2 {
    data: Vec<i32>,
    index: usize,
}

lazy_static! {
    static ref foo_1: Foo_1 = Foo_1 {
        data: (0..10000).map(|_| random_gen!(i32)).collect(),
        index: 0
    };
    static ref foo_2: Foo_2 = to_foo_2(foo_1.clone());
}

pub fn touch_foo() -> usize {
    foo_1.data.len() + foo_2.data.len()
}

pub fn _1f() {
    let mut x: Vec<i32> = Vec::new();
    let _foo: Foo_1 = foo_1.clone();
    x.extend(_foo);
}

pub fn _1o() {
    let mut x: Vec<i32> = Vec::new();
    let _foo: Foo_2 = foo_2.clone();
    x.extend(_foo);
}

fn to_foo_2(foo: Foo_1) -> Foo_2 {
    Foo_2 {
        data: foo.data,
        index: foo.index,
    }
}

impl Iterator for Foo_2 {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let value = self.data[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.data.len() - self.index;
        (remaining, None)
    }
}

impl Iterator for Foo_1 {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.data.len() {
            let value = self.data[self.index];
            self.index += 1;
            Some(value)
        } else {
            None
        }
    }
}
