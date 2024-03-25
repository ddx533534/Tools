use crate::iterator::{test_into_iterator, test_iterator, test_iterator_adapter, test_iterator_self};
use crate::lifecycle::{test_life_cycle1, test_reborrow};

mod lifecycle;
mod closure;
mod iterator;

fn main() {
    // test_life_cycle1();
    // test_reborrow();
    // test();
    // test_closure();
    // test_iterator();
    // test_into_iterator();
    // test_iterator_self();
    test_iterator_adapter();
}
