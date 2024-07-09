use crate::{random_gen, utils::*};
use lazy_static::*;

fn new_routes(num: usize, len: usize) -> Vec<String> {
    (0..num).map(|_| random_word(len)).collect()
}

lazy_static! {
    static ref routes: Vec<String> = new_routes(100, 2);
}

pub fn touch_routes() -> usize {
    routes.len()
}

pub fn _1f() {
    let _routes = routes.clone();
    let routes_: Vec<String> = _routes
        .iter()
        .map(|s| {
            if s.is_empty() {
                return "/".to_string();
            } else {
                s.to_string()
            }
        })
        .collect();
}

pub fn _1o() {
    let _routes = routes.clone();
    let routes_: Vec<String> = _routes
        .into_iter()
        .map(|s| if s.is_empty() { "/".to_string() } else { s })
        .collect();
}
