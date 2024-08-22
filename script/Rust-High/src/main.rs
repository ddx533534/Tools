use crate::BoxTest::{test_box_raw, test_memory_leak};
use crate::event::event_test;
use crate::htmltest::html2html;
use crate::iterator::{test_into_iterator, test_iterator, test_iterator_adapter, test_iterator_self};
use crate::lifecycle::{test_life_cycle1, test_reborrow};
use crate::macro_test::use_macro_println;
use crate::print_rcdom::print_rc_dom;
use crate::rc_test::rc_test_1;
use crate::test::{test_json, test_weak};
use crate::thread_test::{test_thread, thread};

mod lifecycle;
mod closure;
mod iterator;
mod rc_test;
mod FileOp;
mod htmltest;
mod print_rcdom;
mod test;

mod BoxTest;
mod macro_test;
mod thread_test;
mod event;

fn main() {
    // test_life_cycle1();
    // test_reborrow();
    // test();
    // test_closure();
    // test_iterator();
    // test_into_iterator();
    // test_iterator_self();
    // test_iterator_adapter();
    // rc_test_1();
    // file_test();
    // html2html();
    // print_rc_dom();
    // test_weak();
    // test_box_raw();
    //  test_memory_leak();
    //  use_macro_println();
    //  thread();
     event_test();
}
