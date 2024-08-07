use std::fmt::{Debug, Display};
use crate::basic_type::enum_example::Message;
use crate::basic_type::struct_example::{Circle, Rectangle};

pub(crate) fn test_fn(){
    println!("test fn!");
    let circle = Circle::new(1.2,2.1,3.3);
    let area = circle.area();
    println!("radius: {}, area: {}", circle.radius(), area);

    let rectangle = Rectangle::new(3,4);
    println!("width: {}, height:{}, area: {}", rectangle.width, rectangle.height, rectangle.area());
    println!("can hold: {}", rectangle.can_hold(&circle));

    let m = Message::Write(String::from("hello world!"));
    m.call();
    println!("-------------------");
}

pub(crate) fn exec<F: FnOnce()>(f: F) {
    f()
}

pub(crate) fn exec1<F: FnMut()>(mut f: F){
    f()
}

pub(crate) fn exec2<F: Fn()>(f: F){
    f()
}

pub fn complex_function(a: &i32) -> i32 {
    let  b= 3;
    *match_only(a, &b)
}

fn match_only<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if *a > *b {
        a
    }else{
        b
    }
}



pub(crate) fn factory(x: i32) -> impl Fn(i32) -> i32{
    let num = 5;
    move  |y| y + x + num
    // move |y| x * y
}

pub(crate) fn factory1(x:i32) -> Box<dyn Fn(i32) -> i32> {
    let num = 5;

    if x > 1{
        Box::new(move |x| x + num)
    } else {
        Box::new(move |x| x - num)
    }
}






pub(crate) fn some_function<T, U>(t: &T, u: &U) -> i32
where   T: Display + Clone,
        U: Clone + Debug
{
    let su = format!("{:?}", u.clone());
    let st = format!("{}", t);
    println!("su: {}", su.len() as i32);
    println!("st: {}", st.len() as i32);
    (su.len() + st.len()) as i32
}


