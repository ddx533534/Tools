// 动态数组
// 在 Rust 中，最常用的数组有两种，第一种是速度很快但是长度固定的 array，
// 第二种是可动态增长的但是有性能损耗的 Vector


use std::fmt::{Display, Formatter};

pub fn test_dyn_vector() {
    let mut vec = Vec::new();
    vec.push(1);
    let mut vec1 = vec![1, 2, 3];
    let item1 = &vec1[1];
    println!("{}", item1);

    let item2 = vec1.get(2);
    println!("{}", item2.unwrap());

    println!("输出 vec1 数组");
    for i in &mut vec1 {
        *i += 1;
    }
    for i in vec1 {
        println!("{i}");
    }
}

#[derive(Debug)]
struct Person {
    age: i32,
    name: String,
}

impl Display for Person{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"age:{},name:{}",self.age,self.name)
    }
}

impl Person {
    fn new(age: i32, name: String) -> Self {
        Self {
            age,
            name,
        }
    }

}

// 孤儿规则
// impl<T:Display> Display for Vec<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

pub fn test_sort() {
    let mut vec = vec![3, 2, 5, 23, 1, -1];
    vec.sort();
    println!("{:?}", vec);

    let mut persons = vec![Person::new(18, "ddx".to_string()),
                           Person::new(15, "dx".to_string()),
                           Person::new(19, "dd".to_string())];
    persons.sort_unstable_by(|a,b| a.age.cmp(&b.age));
    println!("排序后:{:?}",persons);
}
fn is_vec(v: Vec<u8>) {}

fn test(){
    let arr: [u8; 3] = [1, 2, 3];

    let v = Vec::from(arr);
    is_vec(v);

    let v = vec![1, 2, 3];
    is_vec(v);

    // vec!(..) 和 vec![..] 是同样的宏，宏可以使用 []、()、{}三种形式，因此...
    let v = vec!(1, 2, 3);
    is_vec(v);

    let s= "1".to_string();
    let mut v = Vec::from([1, 2, 3]);
    for i in 0..5 {
        println!("{:?}", v[i])
    }

    for i in 0..5 {
        // 实现这里的代码...
        match v.get(i){
            Some(item) => v[i] = item + 1,
            None => v.insert(i,i+1)
        }
    }
    // ...在下面的代码中, v 是 Vec<[u8; 3]> , 而不是 Vec<u8>
    // 使用 Vec::new 和 `for` 来重写下面这段代码
    let mut v1 = Vec::new();
    for i in arr{
        v1.push(i);
    }
    // assert_eq!(v, v1);

    println!("Success!")
}

pub fn test_vec_reference() {
    let mut vec: Vec<i32> = Vec::new();
    let first = &mut vec[0];
    // 数组的大小是可变的，当旧数组的大小不够用时，Rust 会重新分配一块更大的内存空间，然后把旧数组拷贝过来。
    // 这种情况下，之前的引用显然会指向一块无效的内存，导致之前的 first 引用是无效的
    // vec.push(1);

    println!("{first}");
}

// 可变引用最多只能存在一个，不可变引用可以存在多个
#[derive(Debug)]
struct Data;

impl Data {
    fn test(&self) {}
}

// 同一时刻，对于同一数据，要么存在其多个不可变引用，要么存在其一个可变引用。
// pub fn test_reference(){
//     let a = 3;
//     let b = &a;
//     let c = &mut a;
//     // println!("{:?}",c);
//     // println!("{:?}",b);
//     // println!("{:?}",d);
// }


