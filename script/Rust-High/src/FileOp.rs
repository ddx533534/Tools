use std::cell::{Ref, RefCell};
use std::path::Path;
// 定义一个结构体，包含一个 RefCell
struct MyRefCell {
    data: RefCell<String>,
}

// 为 MyRefCell 实现 Drop 特征
impl Drop for MyRefCell {
    fn drop(&mut self) {
        // 当 MyRefCell 被销毁时执行的逻辑
        println!("Dropping MyRefCell with data: {}", self.data.borrow());
    }
}
// fn good(t: MyRefCell) -> bool {
//     t.borrow().len() == 12
// }

// error[E0597]: `t` does not live long enough
// fn bad(t: MyRefCell) -> bool {
//     let ddx = t;
//     ddx.borrow().len() == 12
// }
// pub fn file_test(){
//     let log_path = Path::new("log_dir").join("log.txt");
//     println!("log path: {:?}",log_path.file_name().unwrap());
// }