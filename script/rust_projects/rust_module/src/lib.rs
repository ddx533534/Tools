mod back_of_house;

pub use crate::back_of_house::chicken;

// 模块 使用模块可以将包中的代码按照功能性进行重组，最终实现更好的可读性及易用性
mod front_of_house {
     pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

     mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat(){
    crate::front_of_house::hosting::add_to_waitlist();
    chicken::cook();
}