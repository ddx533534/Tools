use std::mem;
use std::ops::Deref;

enum Node {}

#[derive(Debug)]
struct Person {
    id: i32,
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            id: self.id + 1
        }
    }
}

impl Copy for Person {}

pub fn test_box() {
    // it could Derefmove, so the box deallocate, but the Person is still alive
    let a = Box::new(Person { id: 123 });
    let b = *a;

    println!("a+1:{}", (a.id + 1));
    println!("b:{}", b.id);

    let c = &Person { id: 12 };
    // person 没有实现copy特征，无法在栈上转移所有权
    let d = *c;
    // auto deref
    println!("c+1:{}", (c.id + 1));
    println!("d:{}", d.id);

    let e = 1;

    // let b = *a + 1;
    // println!("a:{:?},b:{:?}", a, b);
}

pub fn test_box_in_zhan() {
    let arr = [0; 100];
    let arr1 = arr;
    // 数组，i32，f32这种的基础数据类型，一般都是栈上分配，所以重新赋值时都是发生深拷贝，所有权并未转移
    println!("arr len:{}", arr.len());
    println!("arr1 len:{}", arr1.len());

    // 堆上分配的，涉及重新赋值，只是浅拷贝了下指针，数据没复制，会发生所有权转移
    let a = Box::new([0; 1000]);
    let b = a.clone();
    println!("a len:{}", a.len());
    println!("b len:{}", b.len());
}

trait Draw {
    fn draw(&self);
}

struct Button {
    id: i32,
}

struct Text {
    id: i32,
}

impl Draw for Button {
    fn draw(&self) {
        println!("This is a Button! id is {}", self.id);
    }
}

impl Draw for Text {
    fn draw(&self) {
        println!("This is a Text! id is {}", self.id);
    }
}


pub fn test_box_in_dyn_trait() {
    let mut draws: Vec<Box<dyn Draw>> = Vec::new();
    draws.push(Box::new(Button { id: 1 }));
    draws.push(Box::new(Text { id: 2 }));

    for e in draws {
        e.draw();
    }
}

// box leak 可以在运行时将一个box消费掉，并将值泄露。什么玩意
// 运行时需要初始化一个值，并且活得和程序一样久，全局有效。
pub fn test_box_leak() -> &'static str {
    let mut a = String::new();
    a.push_str("123");
    Box::leak(a.into_boxed_str())
}

pub fn test_box_overflow() {
    // this will be crashed! "thread 'main' has overflowed its stack"
    // 难道说明Box的分配是在栈上？？？
    // 据说，debug模式下，是先分配到栈上，然后迁移到堆上；release模式下，是分配到堆上的。
    // 实验了下，[0; 1000000] 数组所占大小为3.8MB，当我以"RUST_MIN_STACK=4194304 cargo run " 即最小4MB运行时，没问题，不加这个参数
    // 就报栈溢出，说明debug模式下，栈分配的空间很少。但以"cargo run --release" release模式运行不报错，怎么能说明这数组没有分配到栈上
    // 还有可能是栈的空间很大？
    let a = Box::new([0; 1000000]);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(t: T) -> MyBox<T> {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    // 实现Deref特征 来为自定义智能指针解引用
    // 但是这种方式会导致栈溢出
    // self is &MyBox<T>
    // first * return MyBox<T>
    // second * return T
    // last & return &T
    fn deref(&self) -> &Self::Target {
        &**self
    }
}

pub fn test_box_custom() {
    let a = MyBox::new(123);
    println!("1 + a = {}", (1 + *a));
}


// impl<T: ?Sized> Deref for MyBox<T> {
//     type Target = T;
//
//     self is &MyBox<T>
//     first * return MyBox<T>
//     second * return T
//     last & return &T
//     fn deref(&self) -> &T {
//         &**self
//     }
// }

pub fn test_box_auto_deref() {
    // 自动解引用，以匹配函数参数类型
    let content = String::from("ddx");
    display(&content);

    // 自动链式解引用，以匹配函数参数类型
    let s = MyBox::new(String::from("dx"));
    display(&s);
}

pub fn display(s: &str) {}


pub fn test_box_raw() {
    let vec1 = vec![0;1000000];
    let box1 = Box::new(vec1);
    let box1_val = Box::into_raw(box1);

    // box1 的所有权已经发生转移，此处调用box1会报错
    // 但是有个问题，此处在这使用 LeakSanitizer 并不能检测出内存泄露！奇怪？
    // println!("box1:{}", box1);

    //内存泄漏的具体情况分析
//程序运行期间：

//内存泄漏发生时，程序分配的内存不会被释放，导致内存使用量不断增加。
//如果内存泄漏严重，可能会导致系统内存耗尽，进而引发程序崩溃或系统性能显著下降。
//程序运行结束：

//操作系统会回收程序在运行期间分配的所有内存，包括那些未被显式释放的内存。
//因此，从操作系统角度来看，进程结束后，所有内存（包括泄漏的内存）都会被系统回收，恢复以供其他进程使用。
    println!("box1_val: {:?}", box1_val);


    // unsafe {
    // 从一个指针恢复对应的Box实例
    //     let box1 = Box::from_raw(box1_val);
    //     println!("box1:{}", box1);
    // }
}

pub fn test_memory_leak(){
    let xs = vec![0;1000000];
    mem::forget(xs);
}
