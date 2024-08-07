use crate::basic_type::collection::gen_static_str;
use crate::basic_type::fn_example::some_function;
use crate::basic_type::struct_example::{SomeTFn, SomeUFn};
use crate::basic_type::trait_example::{notify, Post, returns_summarize, Summary, Weibo};

fn add<T:std::ops::Add<Output = T>>(a:T, b:T) -> T {
    a + b
}

#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_up<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y
        }
    }
}

fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T;N]) {
    println!("{:?}", arr);
}

pub(crate) fn test_generics(){
    println!("add i8: {}", add(2i8,3i8));
    println!("add i32: {}", add(20,30));
    println!("add f64: {}", add(1.23,2.14));
    let integer = Point{x:1,y:2};
    let float = Point{x:1.2, y: 3.2};
    println!("integer: {:?}, float: {:?}", integer,float);

    let p1 = Point{x: 5, y: 10.4};
    let p2 = Point{x: "Hello", y: 'c'};
    let p3 = p1.mix_up(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let arr: [i32; 3] = [1,2,3];
    display_array(arr);
    let arr: [i32;2] = [1,2];
    display_array(arr);

    let post = Post{
        title: "Rust语言简介".to_string(),
        author: "Post".to_string(),
        content: "Rust 棒极了！".to_string(),
    };
    let weibo = Weibo {
        username: "Weibo".to_string(),
        content: "好像微博没facebook好用".to_string()
    };
    println!("{}",post.summarize());
    println!("{}", weibo.summarize());
    notify(&weibo);
    notify(&post);
    let sf_u= SomeUFn{
        name: "Rust".to_string()
    };
    let sf_t = SomeTFn {
        name: "Go".to_string()
    };
    println!("some function len: {}", some_function(&sf_t, &sf_u));
    let entry = returns_summarize(true);
    println!("{}", entry.summarize());
    let entry_false = returns_summarize(false);
    println!("{}", entry_false.summarize_author());

    let arr = vec![Box::new(1), Box::new(2)];
    let (first, second) = (&arr[0], &arr[1]);
    let sum = **first + **second;
    println!("sum: {}, arr_debug_format: {:?}", sum, arr);

    let s = gen_static_str();
    println!("{}", s);
    println!("-------------------");
}
