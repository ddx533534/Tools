use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{io, panic};

// 一般情况下的错误处理，可以看到代码比较繁琐
pub fn test_panic() {
    panic::set_hook(Box::new(|panic_info| {
        // 在此处自定义 panic 处理逻辑
        println!("A panic occurred: {:?}",
                 panic_info.payload().downcast_ref::<&str>().unwrap_or(&",but  no panic message!"));
    }));
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => {
            println!("opening hello.txt success!");
            file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => {
                    println!("creating hello.txt success!");
                    file
                }
                Err(error) => {
                    println!("creating hello.txt error!");
                    panic!("create file error!{:?}", error)
                }
            },
            other => {
                println!("opening hello.txt error!");
                panic!("open file error!{:?}", other)
            }
        },
    };
}

// expect 和 unwrap 简化错误处理，如果有值就直接返回，错误就直接panic
pub fn test_panic_simple() {
    let f = File::open("ddx.txt").expect("open failed!");
}

// 错误传播
pub fn test_error_propagation() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(error) => return Err(error)
    };
    let mut str = String::new();
    match f.read_to_string(&mut str) {
        Ok(_) => Ok(str),
        Err(error) => Err(error)
    }
}

// ? 宏，相当于上面的 match 匹配，即有值就赋值，没有值就直接返回
pub fn test_error_propagation1() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

// ? 的链式调用
pub fn test_error_propagation2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
use std::num::ParseIntError;

fn multiply(n1_str: &str, n2_str: &str) -> Result<i32,ParseIntError> {
    let n1 = n1_str.parse::<i32>();
    let n2 = n2_str.parse::<i32>();
    Ok(n1.unwrap() * n2.unwrap())
}

fn test1() {
    let result = multiply("10", "2");
    assert_eq!(result.unwrap(), 20);

    let result = multiply("t", "2");
    assert_eq!(result.unwrap_or(8), 8);
}

