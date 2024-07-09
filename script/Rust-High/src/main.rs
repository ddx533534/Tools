use crate::htmltest::html2html;
use crate::iterator::{test_into_iterator, test_iterator, test_iterator_adapter, test_iterator_self};
use crate::lifecycle::{test_life_cycle1, test_reborrow};
use crate::print_rcdom::print_rc_dom;
use crate::rc_test::rc_test_1;
use crate::test::{test_json, test_weak};

mod lifecycle;
mod closure;
mod iterator;
mod rc_test;
mod FileOp;
mod htmltest;
mod print_rcdom;
mod test;

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
    test_weak();
}




