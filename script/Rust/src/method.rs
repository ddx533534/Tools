pub(crate) struct Rectangle {
    x: f32,
    y: f32,
}

impl Rectangle {
    // 关联函数。
    // 函数和方法有区别的
    // 函数不依赖实例，不依赖当前类型
    // 方法依赖实例，开头必须 self 之类的，表示操作当前实例，可以应用在 struct 、enum 和 trait
    //Rust 中有一个约定俗成的规则，使用 new 来作为构造器的名称，出于设计上的考虑，Rust 特地没有用 new 作为关键字。
    pub fn new(x: f32, y: f32) -> Rectangle {
        Rectangle { x: x, y: y }
    }

    // self 类型为 Self， Self 表示当前结构体类型，而 self 表示当前结构体的实例
    // &self 类型为 &Self，&self表示当前结构体实例的不可变借用！！！
    // &mut self 类型为 &mut Self， &mut self 则表示当前结构体实例的可变借用
    pub fn area(&mut self) -> f32 {
        self.x+=1.0;
        self.y+=1.0;
        self.x * self.y
    }

    // 返回不可变引用吧
    pub fn width(&self) -> &f32 {&self.x}
    pub fn height(&self) -> &f32 {&self.y}

    pub fn set_width(&mut self, new_x:f32){self.x = new_x}
    pub fn set_height(&mut self, new_y:f32){self.y = new_y}
}

pub enum Sex {
    Male,
    Female   
}

impl Sex {
    pub fn is_male(&self) -> bool{
        match self {
            Sex::Male => true,
            _=> false
        }
    }
}


