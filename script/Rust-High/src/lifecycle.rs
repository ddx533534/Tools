#[derive(Debug)]
struct Food;

impl Food {
    pub fn mutable_share(&mut self) -> &Self {
        &*self
    }

    pub fn share(&self) {}
}

// &mut self 借用的生命周期和 _food1 的生命周期相同，将持续到 println 结束。
// 而在此期间 food1.share() 又进行了一次不可变 &food1 借用
// 违背了可变借用与不可变借用不能同时存在的规则，最终导致了编译错误。
// pub fn test_life_cycle() {
//     let mut food1 = Food;
//     let _food1 = food1.mutable_share();
//     food1.share();
//     println!("{:?}", _food1);
// }

pub fn test_life_cycle1() {
    let a: i32 = 1;
    let b;
    b = &a;
    println!("{}", b);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

pub fn test_reborrow() {
    let mut p1 = Point { x: 1, y: 1 };
    // p_b 为 p1的可变借用
    let p_b = &mut p1;
    // p_rb 为 p_b 的再借用！
    let p_rb = &*p_b;
    // 此时是对 p1 的可变借用
    println!("{:?}", p_rb);
    p_b.move_to(2, 2);
    println!("{:?}", p_b);
}