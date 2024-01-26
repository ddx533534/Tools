//### 泛型 ###
// 原理是单态化，说白了就是在编译期间给每个具体的类型进行具体实现，然后替换为具体的实现
// 比如Option<T>
// Some(3)
// Some(true)
// 那么 Option 就会在编译期间生成两种特定类型的 Option -> Option_i32 和 Option_bool，然后替换掉原来的泛型模板即可。

// 枚举泛型
pub enum Result<T, E> {
    // 返回正确值
    Ok(T),
    // 返回错误值
    Err(E),
}

// 结构体泛型
pub struct Point<T: std::cmp::PartialOrd> {
    x: T,
    y: T,
}

impl<T: std::cmp::PartialOrd> Point<T> {
    pub fn is_in_first_area(&self, zero: T) -> bool {
        self.x > zero && self.y > zero
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }
}

// 多个泛型
pub struct Person<T, W> {
    age: T,
    name: W,
}

impl<T, W> Person<T, W> {
    pub fn swap(&mut self, other: &mut Person<T,W>){
        // 这种使用 temp 交换两个值的方法有问题，因为你必须创建一个 temp 实例出来，但此时你是不知道 temp泛型的具体类型，所以就很尴尬
        // let mut temp = Person{
        //     age: T,
        //     name:W
        // };
        // temp.age = self.age;
        // temp.name = self.name;
        // other.age = self.age;
        // other.name = self.name;
        // self.age = temp.age;
        // self.name = temp.name;
        std::mem::swap(&mut self.age, &mut other.age);
        std::mem::swap(&mut self.name, &mut other.name);
    }
}

// 值泛型 const 1.51版本引入

// 通常写法，通过传入数组切片使用
pub fn display_array<T:std::fmt::Debug>(array:&[T]){
    println!("{:?}",array);
}

pub fn display_array_const<T:std::fmt::Debug, const N: usize> (array:[T;N]){
    println!("{:?}",array);
}

pub fn test_array(){
    let arr1 = [3;1];
    display_array_const(arr1);
    display_array(&arr1);
    let arr2 = [4;4];
    display_array(&arr2);
}
