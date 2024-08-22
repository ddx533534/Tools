use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

pub fn event_test() {
    let event = Event::default();
    event.subscribe(|t| {
        println!("{} I am subscriber 1!", t);
    });
    event.subscribe(|t| {
        println!("{} I am subscriber 2!", t);
    });
    event.fire(&1);
}

struct Event<T> {
    subscribers: Rc<RefCell<Vec<Box<dyn Fn(&T)>>>>,
}

impl<T> Clone for Event<T> {
    fn clone(&self) -> Self {
        Self {
            subscribers: self.subscribers.clone()
        }
    }
}

impl<T> Default for Event<T> {
    fn default() -> Self {
        Self {
            subscribers: Default::default()
        }
    }
}

impl<T> Debug for Event<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} {{ {} subscribers }}",
            std::any::type_name::<Self>(),
            self.subscribers.borrow().len()
        )
    }
}
impl<T> Event<T> {
    fn subscribe<F>(&self, f: F)
    where
        F: Fn(&T) + 'static,
    {
        self.subscribers.borrow_mut().push(Box::new(f));
    }

    fn fire(&self, t: &T) {
        for func in self.subscribers.borrow_mut().iter() {
            func(t);
        }
    }
}