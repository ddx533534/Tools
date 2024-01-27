// 特征对象
// 为什么需要特征对象？在很多语言中有继承的概念，通过多个子类继承同一父类，子类可以实现自己的行为
// 而可以通过统一调用父类来实现屏蔽子类的差异，即多态
// rust中没有继承，搞了个特征对象，即有些对象在编译期间无法确定其真正类型，直到运行时才能确定，就多态嘛
// 对于这种对象必须通过dyn关键字声明，明确为特征对象
// 实现方式为Box智能指针
pub trait Draw {
    fn draw(&self);
}

impl Draw for i32 {
    fn draw(&self) {
        println!("i am i32");
    }
}

impl Draw for f32 {
    fn draw(&self) {
        println!("i am f32");
    }
}

pub fn draw(item: &impl Draw) {
    item.draw();
}

pub fn draw1(item: Box<dyn Draw>) {
    item.draw();
}

pub fn draw2(item: &dyn Draw) {
    item.draw();
}

pub struct Button{

}
pub struct TextView{

}

impl Draw for Button{
    fn draw(&self) {
        println!("drawing button!");
    }
}

impl Draw for TextView {
    fn draw(&self) {
        println!("drawing textview!");
    }
}
//特征对象指向实现了 Draw 特征的类型的实例，也就是指向了 Button 或者 SelectBox 的实例
// 这种映射关系是存储在一张表中，可以在运行时通过特征对象找到具体调用的类型方法。
pub struct Screen{
    // 通过Box智能指针 完成特征对象
    components:Vec<Box<dyn Draw>>
}
impl Screen{
    pub fn new(components:Vec<Box<dyn Draw>>) -> Screen{
        Screen{
            components
        }
    }
    pub fn push(&mut self, item:Box<dyn Draw>){
        self.components.push(item);
    }
    pub fn draw_components(&self){
        for item in self.components.iter(){
            item.draw();
        }
    }
}
// Self 和 self ，前者用来指定当前特征或者类型的别名，是个类型，后者是具体的实例
trait Fly{
    fn fly(&self) -> Self;
}

// trait Swim<T>{
//     fn swim<T>(&self);
// }

//对象安全，特征对象不能用于 Self 和 泛型，下述代码报错，因为Fly中有个fly方法返回Self，非对象安全
// struct Bird{
//     animals:Vec<Box<dyn Fly>>
// }
// struct Fish<T>{
//     animals:Vec<Box<dyn Swim<T>>>
// }