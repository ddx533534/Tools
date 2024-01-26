use crate::log;
// match target {
//     模式1 => 表达式1,
//     模式2 => {
//         语句1;
//         语句2;
//         表达式2
//     },
//     _ => 表达式3
// }

#[derive(Debug)]
pub(crate) enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug)]
enum ChinaCity {
    Qingdao(Direction),
    Beijing(Direction),
    Guangzhou(Direction),
    Lasa(Direction),
}
// match 和 java 中的 switch 很像，但每个分支不需要 break，_ 类似于 default
pub(crate) fn match_direction(diretcion: Direction) {
    match diretcion {
        Direction::West => println!("west!"),
        Direction::East | Direction::North => println!("East"),
        _ => println!("luanqibazao"),
    }
}

// match 作为返回值
pub(crate) fn match_statement() {
    let a = "W";
    let direction: Direction = match a {
        "W" => Direction::West,
        "E" => Direction::East,
        "N" => Direction::North,
        "S" => Direction::South,
        _ => Direction::East,
    };
    println!("{:?}", direction);
}

// 模式绑定
pub(crate) fn match_bind() {
    let qingdao = ChinaCity::Qingdao(Direction::East);
    log::log(qingdao);
}

// match default 默认值
pub(crate) fn match_default() {
    let city = ChinaCity::Lasa(Direction::West);
    match city {
        ChinaCity::Lasa(direction) => log::log(direction),
        other => log::log("other branch"),
    }
}

// 这两种匹配对于新手来说，可能有些难以抉择，但是只要记住一点就好：当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。
pub(crate) fn if_let() {
    let direction = Direction::East;
    let res = if let Direction::North = direction {
        "you are in North!"
    } else {
        "oh, i don't know where you are!"
    };
    log::log(res);
}

// matches！ 宏
pub(crate) fn matches() {
    let directions = vec![
        Direction::East,
        Direction::West,
        Direction::North,
        Direction::East
    ];
    // 表达式与模式匹配下无法直接与 == 比较，但可以通过宏来完成
    // log::log(directions.iter().filter(|x| x == Direction::East));
    log::log(directions.iter().filter(|x| matches!(x,Direction::East)));

    let char = 'v';
    assert!(matches!(char,'a'..='z' | 'A'..='Z'));
}

// 同名变量遮蔽
// const a = 1;
// let b;
// if((b = a) == 1){
//     console.log(b);
// }
pub(crate) fn variable_cover1(){
    let direction = ChinaCity::Beijing(Direction::North);
    // if let ChinaCity::Beijing(direction) = direction  {
    //     println!("111");
    // }
    println!("before match variable: {:?}" , direction);
    match direction {
        ChinaCity::Beijing(direction) => {
            println!("after match variable: {:?}" , direction);
        }
        _ => { log::log("no match!");}
    }
}


// 异名变量是否遮蔽？结论：异名变量不会遮蔽
pub(crate) fn variable_cover(){
    let direction = ChinaCity::Beijing(Direction::North);
    println!("before match variable: {:?}" , direction);
    match direction {
        ChinaCity::Beijing(dir) => {
            println!("after match variable: {:?}" , dir);
        }
        _ => { log::log("no match!");}
    }
}





