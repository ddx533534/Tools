use std::thread;
use std::time::Duration;

// 函数性编程 - 闭包
// 闭包对内存的影响
// 当闭包从环境中捕获一个值时，会分配内存去存储这些值。
// 对于有些场景来说，这种额外的内存分配会成为一种负担。与之相比，函数就不会去捕获这些环境值，因此定义和使用函数不会拥有这种内存负担。
#[derive(Debug)]
enum ExerciseType {
    Run,
    JumpRope,
    PushUp,
}

pub fn exercise(time: i32, exercise_type: ExerciseType) -> i32 {
    println!(" i will {:?}!", exercise_type);
    thread::sleep(Duration::from_secs(time as u64));
    println!(" i have been  {:?} for {} seconds!", exercise_type, time);
    time
}

pub fn just_do_it(key: i32, time: i32) {
    match key {
        1 | 2 => { exercise(time, ExerciseType::Run) }
        3 | 4 => { exercise(time, ExerciseType::JumpRope) }
        5 | 6 => { exercise(time, ExerciseType::PushUp) }
        _ => { exercise(time, ExerciseType::Run) }
    };
}

pub fn just_do_it1(key: i32, time: i32) {
    let action = exercise;
    match key {
        1 | 2 => { action(time, ExerciseType::Run) }
        3 | 4 => { action(time, ExerciseType::JumpRope) }
        5 | 6 => { action(time, ExerciseType::PushUp) }
        _ => { action(time, ExerciseType::Run) }
    };
}

pub fn just_do_it2(key: i32, time: i32) {
    let exercise_type = match key {
        1 | 2 => { ExerciseType::Run }
        3 | 4 => { ExerciseType::JumpRope }
        5 | 6 => { ExerciseType::PushUp }
        _ => { ExerciseType::Run }
    };
    let action = || {
        println!(" i will {:?}!", exercise_type);
        thread::sleep(Duration::from_secs(time as u64));
        println!(" i have been  {:?} for {} seconds!", exercise_type, time);
        time
    };
    match key {
        1 | 2 => { action() }
        3 | 4 => { action() }
        5 | 6 => { action() }
        _ => { action() }
    };
}

pub fn test() {
    let key = 2;
    let time = 3;
    just_do_it2(key, time);
}

//
// struct Cacher<T>
//     where T: Fn(i32) -> i32 {
//     // T为泛型，Fn表示闭包，即query为一个入参和出参都为i32的闭包
//     query: T,
//     value: Option<i32>,
// }
//
// impl<T> Cacher<T>
//     where T: Fn(i32) -> i32 {
//     pub fn new(query: T, value: Option<i32>) -> Cacher<T> {
//         Cacher {
//             query,
//             value,
//         }
//     }
//
//     pub fn value(&mut self, arg: i32) -> i32 {
//         match self.value {
//             None => {
//                 let res = (self.query)(arg);
//                 self.value = Some(res);
//                 res
//             }
//             Some(v) => { v }
//         }
//     }
// }
//
// struct Cache<T, U>
// // U实现了Copy特征，后续所有权发生转移时不会报错
//     where T: Fn(U) -> U, U: Copy {
//     // T为泛型，Fn表示闭包，即query为一个入参和出参都为i32的闭包
//     query: T,
//     value: Option<U>,
// }
//
// impl<T, U> Cache<T, U>
//     where T: Fn(U) -> U, U: Copy {
//     pub fn new(query: T, value: Option<U>) -> Cache<T, U> {
//         Cache {
//             query,
//             value,
//         }
//     }
//
//     pub fn value(&mut self, arg: U) -> U {
//         // match self.value {
//         //     None => {
//         //         let res = (self.query)(arg);
//         //         // self.value = Some(res);
//         //         res
//         //     }
//         //     Some(v) => { v }
//         // }
//     }
// }
//
// struct Cache1<T, U>
// // U实现了Copy特征，后续所有权发生转移时可以copy
//     where T: Fn(U) -> U {
//     // T为泛型，Fn表示闭包，即query为一个入参和出参都为i32的闭包
//     query: T,
//     value: Option<U>,
// }
//
// impl<T, U> Cache1<T, U>
//     where T: Fn(U) -> U {
//     pub fn new(query: T, value: Option<U>) -> Cache1<T, U> {
//         Cache1 {
//             query,
//             value,
//         }
//     }
//
//     // pub fn value(&mut self, arg: U) -> U {
//     //     match self.value.take() {
//     //         None => {
//     //             let res = (self.query)(arg);
//     //             self.value = Some(res);
//     //             res
//     //         }
//     //         Some(v) => { v }
//     //     }
//     // }
// }
//
//
// pub fn test_closure() {
//     let mut s = "world ".to_string();
//     let update_string = |str| s.push_str(str);
//     exec(update_string);
//     println!("{:}", s);
// }
//
// fn exec<F: FnMut(&str)>(mut f: F) {
//     f("hello")
// }