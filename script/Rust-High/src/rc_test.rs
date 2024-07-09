use std::ops::Deref;
use std::rc::Rc;

pub fn rc_test_1() {
    let a = Rc::new([1; 100]);
    // clone 时并没有发生深拷贝，只是指针的计数+1
    let b = Rc::clone(&a);

    // 看下这个引用计数指针，不同变量的引用计数值， 此时 a 和 b 还没有释放
    println!("rc a count: {:?}", Rc::strong_count(&a));
    println!("rc b count: {:?}", Rc::strong_count(&b));

    println!("a len : {}, b len :{}", a.len(), b.len());
    println!("a[0] + b[0] = :{}", a[0] + b[0]);

    println!("a deref: {:?}", a.deref());
    println!("rc a count: {:?}", Rc::strong_count(&a));
}