use std::cell::RefCell;
use std::rc::Rc;

fn foo_1(a: Rc<RefCell<Vec<i32>>>) {
    a.borrow_mut().push(123);
}
fn foo_2(a: &RefCell<Vec<i32>>) {
    a.borrow_mut().push(456);
}

pub fn _1f() {
    let a = Vec::<i32>::new();
    let ptr = Rc::new(RefCell::new(a));
    foo_1(ptr.clone());
    ptr.clone().borrow_mut().push(123);
}

pub fn _1o() {
    let a = Vec::<i32>::new();
    let ptr = RefCell::new(a);
    foo_2(&ptr);
    ptr.borrow_mut().push(123);
}

pub fn _2f() {
    for _ in 0..1000 {
        let a = Vec::<i32>::new();
        let ptr = RefCell::new(a);
        ptr.borrow_mut().push(123);
    }
}

pub fn _2o() {
    for _ in 0..1000 {
        let a = Vec::<i32>::new();
        let mut ptr = RefCell::new(a);
        ptr.get_mut().push(123);
    }
}
