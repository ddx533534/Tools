// 生命周期


pub fn test_life_cycle() {
    let mut a = &1;
    {
        let b = 2;
        // a = &b;
        // borrowed value does not live long enough，悬垂引用问题，引用了一个生命周期早就结束的变量
    }
    println!("{}", a);
}

pub fn test_life_cycle_function() {
    // 用这个例子不是很好，因为直接定义字符串字面量 -> &str ，其生命周期是 static 即跟随整个应用程序！
    let item1 = "123";
    let item2 = "023";
    println!("{:?}", longest(item1, item2));


    // 这种直接使用字符串字面量的 &str 生命周期跟随整个应用程序，因此在这是没问题的。
    let a = "aab";
    let mut result = "";
    {
        let b = "bbc";
        result = longest(a, b);
    }
    println!("{:?}", result);


    // 通过使用 String 类型，转换成的&str 字符串切片类型的变量生命周期依赖于原先的 String！
    let a = String::from("aab");
    let mut result = "";
    {
        let b = String::from("bbc");
        result = longest(a.as_str(), b.as_str());
        // result 的生命周期长度虽然长于 b，但是最终生命周期结束地方是一致的。ٰ
        println!("{:?}", result);
    }
}

// 结构体中的字符串切片类型也必须使用生命周期标识符修饰！
// 要不然就是用 String 类型
struct Person {
    name: String,
}

struct Person1<'a> {
    name: &'a str,
}

// 这么玩没用，因为字符串字面量或者字符串切片类型的生命周期是 static ，跟随整个应用程序的
pub fn test_lifecycle_struct() {
    let mut person = Person1 { name:"dx" };
    {
        let new_name = "ddx";
        person.name = new_name;
    }
    println!("{:?}",person.name);
}

pub fn test_lifecycle_struct_string(){
    let mut person = Person1 { name:"dx" };
    {
        let new_name = String::from("ddx");
        person.name = new_name.as_str();
        // 放到这就没问题了~
        println!("{:?}",person.name);
    }
    // 这就会报错，因为 person 的生命周期是外边的大括号，而里边 new_name 的生命周期是小括号
    // println!("{:?}",person.name);
}


pub fn longest<'a>(item1: &'a str, item2: &'a str) -> &'a str {
    if (item1.len() > item2.len()) {
        item1
    } else {
        item2
    }
}