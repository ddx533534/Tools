use std::collections::HashMap;

pub fn test_map() {
    let name = String::from("ddx");
    let age = 18;
    let mut hashmap = HashMap::new();
    hashmap.insert(name, age);
    hashmap.insert(String::from("dx"),19);
    println!("{:?}", hashmap);

    //可以，类型有实现 copy 特征，不涉及所有权转移
    println!("{}", age);
    // 不可以，类型没有实现 copy 特征，所有权已经发生了转移
    // println!("{}", name);

    // 返回的是 Option<&i32>，值类型的不可变借用
    println!("{:?}",hashmap.get("ddx").unwrap());
    // 返回的是Some<&i32>，值类型的不可变借用，兜底一定返回 Some 类型
    println!("{:?}",hashmap.get("ddx").unwrap_or(&2));
    // 返回的是Some<i32>，直接是值类型，具备所有权，兜底一定返回 Some 类型
    println!("{:?}",hashmap.get("dx").copied().unwrap_or(3));
    // 返回的是Some<&i32>，值类型的不可变借用，兜底一定返回 Some 类型
    println!("{:?}",hashmap.get("x").unwrap_or(&4));

    for(key, value) in &hashmap{
        // 都是不可变借用
        println!("key:{},value:{}",key,value);
    }

    let mut str: &str = "hello world! ddx , i am your friend";
    let mut map = HashMap::new();
    for word in str.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}",map);
}