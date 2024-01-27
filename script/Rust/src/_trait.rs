//### 特征,定义了一组可以被共享的行为，只要实现了特征，你就能使用这组行为。
use core::fmt::Display;
use std::{
    clone,
    ops::{Add, Mul},
};

pub trait Summary {
    fn summary(&self);

    fn to_string(&self) {
        println!("Summary.toString");
    }
}

pub struct Animal {
    name: String,
    catogery: String,
}
impl Animal {
    pub fn new(name: String, catogery: String) -> Animal {
        Animal {
            name: name,
            catogery: catogery,
        }
    }
}

impl Summary for Animal {
    fn summary(&self) {
        println!("animal name {:?},catogery: {:?}", self.name, self.catogery);
    }

    fn to_string(&self) {
        println!("Animal.toString:{:?}", self.name);
    }
}

// 再为Animal 实现另一个特征
impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.catogery)
    }
}

pub struct Sport {
    name: String,
    catogery: String,
}
impl Summary for Sport {
    fn summary(&self) {
        println!("animal name {},catogery: {}", self.name, self.catogery);
    }
}
impl Sport {
    pub fn new(name: String, catogery: String) -> Sport {
        Sport {
            name: name,
            catogery: catogery,
        }
    }
}

// 特征作为参数，多态，抽象->具体，传入的是不可变引用
pub(crate) fn trait_bound(summary: &impl Summary) {
    println!("let's see the summary");
    summary.summary();
}
// 上述写法为语法糖，完整写法如下 - 特征约束
pub(crate) fn trait_bound_expand<T: Summary>(item: &T) {}

pub(crate) fn trait_bound_multi_p(item1: &impl Summary, item2: &impl Summary) {
    item1.summary();
    item2.summary();
}
// 特征约束另一作用体现在约束多个参数具备同一类型，并实现同一特征。泛型约束同一类型+特征约束
pub(crate) fn trait_bound_multi_p_same<T: Summary>(item1: &T, item2: &T) {
    item1.summary();
    item2.summary();
}

//多个约束
pub(crate) fn trait_bound_multi_trait(item: &(impl Summary + Display)) {
    item.summary();
}

// 特征约束作为返回值，如果碰到不同的返回值，必须是同一类型，
// 特征约束在返回值上需要是同一类型，这点就有点奇怪，你参数可以是不同子类，怎么到返回值就必须是同一类型了
// pub(crate) fn trait_bound_return(switch: bool) -> impl Summary {
//     if switch {
//         Animal::new("name".to_string(), "catogery".to_string())
//     } else {
//         Sport::new("name".to_string(), "catogery".to_string())
//     }
// }

pub struct Pair<T> {
    x: T,
    y: T,
}
// 泛型+特征约束 实现了指定类型个特征的方法
impl<T: PartialOrd + Display> Pair<T> {
    pub fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }

    pub fn cmp_display(&self) {
        if self.x > self.y {
            println!("{} is the largest", self.x);
        } else {
            println!("{} is the largest", self.y);
        }
    }
}

// 修复泛型中不能直接比较的错误，加上特征约束即可，还得实现Copy特征， 要不所有权没法移动；借用的话没法返回，生命周期不正确。
pub fn largest<T: PartialOrd + Copy>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}
//  一种方案是实现clone特征，而不是copy特征，需要你自己手动克隆元素，这样你就有自己的所有权。
pub fn largest_clone<T: PartialOrd + Clone>(list: &[T]) -> Option<T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = list[0].clone();

    for item in list.iter() {
        if item > &largest {
            largest = item.clone();
        }
    }
    Some(largest)
}

// 还有一种方式，返回不可变引用，这样的话不需要实现copy，因为所有权没有发生移动，一直属于这个数组。
pub fn largest_ref<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    Some(largest)
}

pub struct Point<T: Add<T, Output = T>> {
    x: T,
    y: T,
}
// 为Point 实现Add 即 + 操作的特征
impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, item: Point<T>) -> Point<T> {
        Point {
            x: self.x + item.x,
            y: self.y + item.y,
        }
    }
}

pub fn add<T: Add<T, Output = T>>(item1: T, item2: T) -> T {
    item1 + item2
}

fn multiply<T: Mul<T, Output = T>>(a: T, b: T) -> T {
    a * b
}
