use std::thread;
use std::time::Duration;

pub fn thread(){
    // test_thread();
    test_thread_move();
}
fn test_thread() {
    let handle = thread::spawn(|| {
        for i in 0..10 {
            println!("hello, i am {}, from spawn", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 等所有子线程执行完毕
    handle.join().unwrap();
    for i in 0..5 {
        println!("hello, i am {}, from main", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn test_thread_move() {
    let vec = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("vec :{:?}", vec);
    });
    // 等所有子线程执行完毕
    handle.join().unwrap();
    // 所有权转移到子线程中，主线程不再拥有！
    // println!("vec :{:?}", vec);
}