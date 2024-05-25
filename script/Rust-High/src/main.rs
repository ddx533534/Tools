// #![default_stack_size = "4194304"]
use crate::BoxTest::{test_box, test_box_custom};

mod lifecycle;
mod closure;
mod BoxTest;

fn main() {
    // test_life_cycle1();
    // test_reborrow();
    // test();
    // test_closure();
    test_box();
    // test_box_in_zhan();
    // test_box_in_dyn_trait();
    // test_box_overflow();
    // test_box_custom();
}


