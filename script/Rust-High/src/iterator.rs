use std::vec::IntoIter;

pub fn test_iterator() {
    let arr = vec![1, 2, 3];
    let mut iterator = arr.into_iter();
    let a = iterator.next();
    println!("a : {}", a.unwrap());
    for i in iterator {
        println!("{}", i);
    }
}

pub fn test_into_iterator() {
    let arr = vec![4, 5, 6];
    let iter = arr.into_iter().into_iter().into_iter().into_iter().into_iter();
    for i in iter {
        println!("{}", i);
    }
}

pub fn test_iterator_self() {
    let arr = vec![11, 12, 13];
    match IntoIterator::into_iter(arr) {
        mut iter => {
            loop {
                match iter.next() {
                    None => { break; }
                    Some(i) => { println!("{}", i); }
                }
            }
        }
    }
}

pub fn test_iterator_lifecycle() {
    let mut arr = vec![1, 1, 1];
    // arr所有权直接转移
    let iter = arr.into_iter();
    // arr 不可变借用
    // let iter0 = arr.iter();
    // arr 可变借用
    // let iter1 = arr.iter_mut();
}

// 消费者适配器，顾名思义每 next 一下，就会消耗掉当前元素
pub fn test_consumer_adapter() {
    let arr = vec![123, 123, 132];
    // arr 所有权转移到 sum 中
    let sum: i32 = arr.into_iter().sum();
    println!("{:?}",  sum);
}

// 迭代器适配器
pub fn test_iterator_adapter() {
    let arr = vec![1, 0, 0];
    let arr1: Vec<i32> = arr.iter().map(|x| x + 1).collect();
    println!("arr1: {:?}", arr1);
}

// 倒计时
struct CountDowner {
    value: u32,
}

impl Iterator for CountDowner {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value >= 0 {
            self.value -= 1;
            Some(self.value)
        } else {
            None
        }
    }
}