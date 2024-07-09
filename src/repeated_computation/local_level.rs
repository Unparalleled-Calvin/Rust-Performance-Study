use lazy_static::*;

use crate::utils::*;

lazy_static! {
    static ref lookup: String = (0..5)
        .map(|_| random_word(5))
        .collect::<Vec<String>>()
        .join("_");
    static ref iter_names: Vec<String> = (0..20)
        .map(|_| (0..5)
            .map(|_| random_word(5))
            .collect::<Vec<String>>()
            .join("_"))
        .collect();
}

pub fn touch_lookup() -> usize {
    lookup.len() + iter_names.len()
}

pub fn _1f() {
    let mut _iter_names = iter_names.clone();
    _iter_names.iter().fold(None, |result, candidate| {
        if sort_by_words(candidate.as_str()) == sort_by_words(&lookup) {
            Some(candidate.clone())
        } else {
            result
        }
    });
}

pub fn _1o() {
    let lookup_sorted_by_words = sort_by_words(&lookup);
    iter_names.iter().fold(None, |result, candidate| {
        if sort_by_words(candidate.as_str()) == lookup_sorted_by_words {
            Some(candidate.clone())
        } else {
            result
        }
    });
}

fn sort_by_words(name: &str) -> String {
    let mut split_words: Vec<&str> = name.split('_').collect();
    split_words.sort_unstable();
    split_words.join("_")
}
