use std::panic;
use _trait::Summary;

mod _trait;
mod _trait_object;
mod _trait_test;
mod generics;
mod log;
mod method;
mod option;
mod pattern_match;
mod _trait_deep;
mod dyn_vec;
mod _hashmap;
mod lifecycle;
mod _panic;

fn main() {
    // let direction = pattern_match::Direction::East;
    // pattern_match::match_direction(direction);
    // pattern_match::match_statement();
    // pattern_match::match_bind();
    // pattern_match::match_default();
    // pattern_match::matches();
    // pattern_match::variable_cover();
    // let a = Some(1);
    // let res = option::option_addone(a);
    // println!("{:?}",res);
    // option::option_stack();

    // let mut rectangle = method::Rectangle::new(12.0,13.0);
    // rectangle.set_width(1.0);
    // rectangle.set_height(1.0);
    // let rec_ref = &mut rectangle;
    // // 此时 rec_ref 是对 Rectangle 的一个可变引用，但由于打印面积时，rectangle.area()实际上已经持有了不可变引用
    // // rec_ref 作用域还没有结束，导致违背原则。可以将 rec_ref.area()调到前面，提前结束rec_ref 的生命周期即可。
    // // println!("变更前面积为: {:?} 变更后面积 :{:?}",rectangle.area(),rec_ref.area());
    // println!("变更前面积为: {:?} 变更后面积 :{:?}",rec_ref.area(),rectangle.area());

    // println!("当前长为:{:?},宽为:{:?}",rectangle.height(),rectangle.width());

    // let sex = method::Sex::Female;
    // println!("是男的吗？{:?}",sex.is_male());
    // generics::test_array();
    // let animal = _trait::Animal::new("moneky".to_string(), "buru".to_string());
    // let animal1 = _trait::Animal::new("pig".to_string(), "buru".to_string());
    // let sport = _trait::Sport::new("basketball".to_string(), "ball".to_string());
    // _trait::report_summary(&animal);
    // _trait::trait_bound_multi_p(&animal, &sport);
    // _trait::trait_bound_multi_p_same(&animal1, &animal);
    // _trait::trait_bound_multi_trait(&animal);
    // let pair = _trait::Pair::new("animal", "animal1");
    // pair.cmp_display();

    // let largest = _trait::largest(&vec![1, 2, 4, 6, 8, -1, 3]);

    // let largest = _trait::largest(&vec!["1", "2", "4", "6", "8", "3"]);
    // let arry = &vec!["1".to_string(), "2".to_string()];
    // let largest = _trait::largest_clone(arry);
    // let largest_ref = _trait::largest_ref(arry);
    // println!("{}", largest.unwrap());

    // let file = _trait_test::File::new(
    //     "ddx.txt".to_string(),
    //     vec![32.0; 8],
    //     _trait_test::FileState::CLOSED,
    // );
    // println!("{}", file);

    // let a = 8i32;
    // let b = 9.0f32;
    //
    // _trait_object::draw(&a);
    // _trait_object::draw(&b);
    // _trait_object::draw1(Box::new(a));
    // _trait_object::draw1(Box::new(b));
    // _trait_object::draw2(&a);
    // _trait_object::draw2(&b);

    // let button = _trait_object::Button{};
    // let textview = _trait_object::TextView{};
    // let mut screen = _trait_object::Screen::new(vec![]);
    // screen.push(Box::new(button));
    // screen.push(Box::new(textview));
    // screen.draw_components();

    // _trait_deep::test_();
    // _trait_deep::test_out_trait();


    // dyn_vec::test_dyn_vector();
    // dyn_vec::test_dyn_vector();
    // dyn_vec::test_sort();
    // _hashmap::test_map();

    // lifecycle::test_life_cycle();
    // lifecycle::test_life_cycle_function();
    // lifecycle::test_lifecycle_struct();


    // _panic::test_panic();
    // _panic::test_panic_simple();
    // let res = _panic::test_error_propagation().expect("oh no ~");
    let res = _panic::test_error_propagation1().expect("on on~");
    println!("file content is {:?}", res);
}


