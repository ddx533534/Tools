pub fn trait_test() {
    let c = C::new();
    c.method1();
    c.method2();

    let d = D::new();

}
trait A {
    fn method1(&self);
    fn method2(&self);
    // 添加更多假设的方法...
}
struct BaseA {}
impl A for BaseA {
    fn method1(&self) {
        println!("i am BaseA 1");
    }

    fn method2(&self) {
        println!("i am BaseA 2");
    }
}

struct C {
    base: BaseA,
}
impl C {
    fn new() -> Self {
        C { base: BaseA {} }
    }
}
struct D {
    base: BaseA,
}

impl D {
    fn new() -> Self {
        D { base: BaseA {} }
    }
}

// impl A for C {
//     fn method1(&self) {
//         println!("i am C 1");
//     }
//
//     fn method2(&self) {
//         println!("i am C 2");
//     }
// }

macro_rules! delegate_methods {
    (
        $type:ty, $base:ident, {
            override { $($override_method:ident),* }
            base { $($base_method:ident),* }
        }
    ) => {
        impl A for $type {
            $(
                fn $override_method(&self) {
                    println!("{}: custom {}", stringify!($type), stringify!($override_method));
                }
            )*

            $(
                fn $base_method(&self) {
                    self.$base.$base_method();
                }
            )*
        }
    };
}

delegate_methods!(
    C, base, {
        override { method1, method2 }
        base { }
    }
);