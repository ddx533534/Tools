
pub(crate) fn option_addone(x:Option<i32>)->Option<i32>{
    match x{
        None => None,
        Some(i) => Some(i+1)
    }
}

pub(crate) fn option_stack(){
    let mut stack :Vec<Option<i32>>= Vec::new();    
    stack.push(Some(1));
    stack.push(Some(2));
    stack.push(Some(3));
    stack.push(None);
    stack.push(Some(4));

    // stack.pop 返回的是 Option<T> ， 此例中返回的 Option<Option<i32>>
    while let Some(top) = stack.pop()  {
        println!("{:?}",top);
    }
}