use crate::closure::{test, test_closure};
use crate::lifecycle::{test_life_cycle1, test_reborrow};

mod lifecycle;
mod closure;

fn main() {
    test_life_cycle1();
    test_reborrow();
    test();
    test_closure();
}
