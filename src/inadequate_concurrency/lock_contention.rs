use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use std::thread;

pub fn _1f() {
    let data: HashSet<u32> = (0..20000).collect();
    let shared = Arc::new(RwLock::new(data));
    let mut handles = vec![];
    for _ in 0..10 {
        let lock = shared.clone();
        let handle = thread::spawn(move || {
            let mut x = lock.write().unwrap();
            let q = x.clone();
            let sum: u32 = q.iter().sum::<u32>();
            x.insert(sum);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn _1o() {
    let data: HashSet<u32> = (0..20000).collect();
    let shared = Arc::new(RwLock::new(data));
    let mut handles = vec![];
    for _ in 0..10 {
        let lock = shared.clone();
        let handle = thread::spawn(move || {
            let x = lock.read().unwrap();
            let q = x.clone();
            drop(x);
            let sum: u32 = q.iter().sum::<u32>();
            lock.write().unwrap().insert(sum);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
