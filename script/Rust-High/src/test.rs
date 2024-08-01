use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::atomic::{AtomicU32, Ordering};

pub fn test_json() {
    let result = "{}";
    let json_str = format!("{{\"status\":1, \"code\":200, \"data\":\"{}\"}}", result);
    let json_str1 = String::from("{{\"status\":0, \"code\":-1, \"data\":\"\"}}");
    println!("{}", json_str);
    println!("{}", json_str1);
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: RefCell<Weak<Node>>,
}

pub fn test_weak() {
    let node1 = Rc::new(Node { value: 1, next: RefCell::default() });
    let node2 = Rc::new(Node { value: 2, next: RefCell::default() });
    *node1.next.borrow_mut() = Rc::downgrade(&node2);
    println!("node1: {:?}, node2: {:?}", node1, node1.next.take().upgrade());
    println!("node1: {:?}, node2: {:?}", node1, node1.next.take().upgrade());

    let weak1 = Rc::downgrade(&node1);
    println!("node1: {:?}", weak1.upgrade());
    println!("node1: {:?}", weak1.upgrade());


    let cell1 = Cell::new("123");
    println!("{}", cell1.take());
    println!("{}", cell1.take());

    let atomic = AtomicU32::new(1);
    println!("fetch_add : {}", atomic.fetch_add(1, Ordering::SeqCst));
    println!("fetch_add : {}", atomic.fetch_add(1, Ordering::SeqCst));
    println!("fetch_add : {}", atomic.fetch_add(1, Ordering::SeqCst));

    let str = "添加招商银行卡支付";
    println!("{}",str.chars().count());
}


pub fn poker_game(){
    let count = 52;
    let weight = 1;
}