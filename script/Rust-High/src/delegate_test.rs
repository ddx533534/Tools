use std::cell::RefCell;
use std::rc::Rc;
use delegate::delegate;

pub fn delegate_test() {
    let mut stack = Stack::default();
    stack.add(12);
    stack.ref_add(13);
    println!("stack.inner: {:?}, ref_cell: {:?}", stack.inner, stack.ref_cell);

    let mut pai = Pai{inner:RoleImpl{}};
    let mut morgana = Morgana{inner:RoleImpl{}};

    println!("pai:{:?} morgana:{:?}",pai.inner.vote(),morgana.inner.vote());
}

struct Stack<A> {
    inner: Vec<A>,
    ref_cell: Rc<RefCell<Vec<A>>>,
}

impl<A> Stack<A> {
    delegate! {
        to self.inner{
            #[call(push)]
            pub fn add(&mut self,element: A);
        }
        to self.ref_cell.borrow_mut() {
           #[call(push)]
            pub fn ref_add(&mut self,element: A);
        }
    }
    pub fn default() -> Self {
        Self {
            inner: Vec::new(),
            ref_cell: Rc::new(RefCell::new(Vec::new())),
        }
    }
}


// delegate 实现 trait
pub trait Role {
    fn vote(&self) -> bool;
}

struct RoleImpl {}

impl Role for RoleImpl {
    fn vote(&self) -> bool {
        false
    }
}

struct Morgana {
    inner: RoleImpl,
}
impl Role for Morgana {
    delegate! {
        to &self.inner {
            #[through(Role)]
            fn vote(&self) -> bool;
        }
    }
}

struct Pai {
    inner: RoleImpl,
}
impl Role for Pai {
    delegate! {
        to &self.inner {
            #[through(Role)]
            fn vote(&self) -> bool;
        }
    }
}
