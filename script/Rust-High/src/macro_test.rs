macro_rules! as_bit {
    ($x:expr) => {
        {
            let bytes = $x.as_bytes();
            let mut vec = Vec::new();
            let mut cur_index = 0;
            for byte in bytes {
                let mut str = String::new();
                cur_index = 0;
                while cur_index < 8 {
                    let res = as_bool((*byte << cur_index) & 128);
                    cur_index += 1;
                    str.push_str(&res.to_string());
                }
                vec.push(str);
            }
            vec.join("")
        }
    };
}

pub fn use_macro_println() {
    let x = "a";
    let bytes = x.as_bytes();
    let mut vec = Vec::new();
    let mut cur_index = 0;
    for byte in bytes {
        let mut str = String::new();
        cur_index = 0;
        while cur_index < 8 {
            let res = as_bool((*byte << cur_index) & 128);
            cur_index += 1;
            str.push_str(&res.to_string());
        }
        vec.push(str);
    }
    println!("res: {:?}", vec);
    println!("macro bit: {:?}",as_bit!("1"));
    println!(); // prints just a newline
    println!("hello there!");
    println!("format {} arguments", "some");
    let local_variable = "some";
    println!("format {local_variable} arguments");
}

fn transfer_to_bit() {}

fn as_bool(n: u8) -> usize {
    match n {
        0 => 0,
        _ => 1
    }
}



