use _trait::Summary;

mod generics;
mod _trait;
mod method;
mod pattern_match;
mod log;
mod option;
fn main() {
    let direction = pattern_match::Direction::East;
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
    let animal = _trait::Animal::new("moneky".to_string(), "buru".to_string());
    animal.summary();
}

