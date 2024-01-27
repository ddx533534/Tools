use std::fmt::{Display, Formatter};

// 调用同名方法，感觉就是有问题，在设计上就不应该设计出不同特征具备完全一样的方法。
// 多个特征方法冲突，通过完全限定解决
trait Animal {
    //方法
    fn name(&self) -> String;
    // 关联函数？
    fn age() -> i32;
}

trait Life {
    fn name(&self) -> String;
    fn age() -> i32;
}

struct Dog;

impl Dog {
    fn name(&self) -> String {
        "dddd".to_string()
    }
}

impl Animal for Dog {
    fn name(&self) -> String {
        "Animal:dog".to_string()
    }

    fn age() -> i32 {
        321
    }
}

impl Life for Dog {
    // 方法
    fn name(&self) -> String {
        "Life:dog".to_string()
    }

    fn age() -> i32 {
        123
    }
}

pub fn test_() {
    let dog = Dog;
    println!("{:?}", dog.name());
    println!("{:?}", Animal::name(&dog));
    println!("{:?}", Life::name(&dog));

    // 关联函数怎么办？关联函数更像静态函数，不依赖当前实例；方法则强绑定当前实例
    println!("{:?}", <Dog as Animal>::age());
    println!("{:?}", <Dog as Life>::age());
}


// 特征定义中的特征约束，特征套特征，，，类比接口实现接口，这串套娃都得具备
trait A{}
trait B:A{}

struct Data{}
impl A for Data{

}
impl B for Data{

}


// 在外部类型上实现外部特征，简而言之就是绕过孤儿规则，给外部类型套外部特征。
// 孤儿规则强调特征至少一个是属于内部作用域的，如下会报错
// impl std::fmt::Display for Vec<T>{
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         todo!()
//     }
// }

struct Wrapper<T>(Vec<T>);

impl<T:Display> Display for Wrapper<T>{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}]", self.0.iter().map(|item| item.to_string()).collect::<Vec<_>>().join(", "))
    }
}
struct Pix{}
pub fn test_out_trait(){
    let str =Wrapper(vec![String::from("ddx"),String::from("18")]);
    let str1 =Wrapper(vec![Pix{},Pix{}]);

    // println!("{}",str1);
}
