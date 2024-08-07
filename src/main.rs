mod basic_type;
mod http_request;
mod advanced_level;

const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

static mut LEVEL: u32 = 5; // 不是线程安全
static CORRECT: u32 = 1; // 线程安全

fn test_print_and_format() {
    println!("{:.4}", 12.0);
    println!("{:?}", format!("{:08}", 989));
    println!(
        "Hello, world! sec={sec}, {sec}",
        sec = THREE_HOURS_IN_SECONDS
    );
    println!("-------------------");
}

fn test_print_string() {
    let company: &str = "我是人";
    println!("{}", company);
    let location: &'static str = "中国";
    println!("{}", location);
    println!("{}", CORRECT);
    unsafe {
        println!("{}", LEVEL);
    }
    println!("-------------------");
}

fn test_debug_string() {
    let i: f64 = std::f64::consts::PI;
    let s: String = String::from("from hello");
    let v = vec![1, 2, 3, 4];
    let p = Person {
        username: "sun_face".to_string(),
        age: 18,
    };
    println!("{:?}, {:?}, {:?}, {:?}", i, s, v, p.username);
    println!("-------------------");
}

fn test_data_type() {
    let guess: u32 = "42".parse().expect("not number!");
    println!("{:?}", guess);
    let str: &str = "sdfsfewfw";
    println!("{:?}", str.as_bytes());
    let s = b"sdfsdsdfsdfwsdf";
    println!("{:?}", s.as_slice());
    let bs = b'd';
    println!("is ascii: {:?}", bs.is_ascii());
    let not = r#######"sads\\n\r\ndf"#######;
    println!("{:?}", not);
    for i in 1..5 {
        print!("{}\t", i)
    }
    println!();
    for i in 1..=5 {
        print!("{}\t", i)
    }
    println!();
    for i in 'a'..'z' {
        print!("{}\t", i)
    }
    println!();
    for i in 'A'..'Z' {
        print!("{}\t", i)
    }
    println!("\n-------------------");
}

use num::complex::Complex;
use std::fmt::{Formatter, Write};

fn test_num_handle() {
    let a = Complex { re: 2.1, im: -1.2 };
    let b = Complex::new(11.1, 22.2);
    let result = a + b;
    println!("{} + {}i", result.re, result.im);
    println!("{}", 13.14_f32.round());
    let y: Complex<f64> = { a + Complex::new(20.1, 54.8) };
    println!("{:?}", y);
}

fn callback() -> (bool, String) {
    let mut st: String = String::new();
    st.write_str("hello world!").expect("TODO: panic message");
    ret_unit_type(32);
    return (true, st);
}

fn ret_unit_type(x: u32) {
    let z = if x % 2 == 1 { "odd" } else { "even" };
    println!("{:?}", z)
}

fn test_all_ownership() {
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);
    let mut s1 = String::from("hello ");
    s1.push_str("world!");
    let s2 = s1;
    // 所有权转移
    println!("s2: {}", s2);

    let mut s3 = String::from("你好， ");
    s3.push_str("老表s3.");
    let mut s4 = s3.clone();
    s4.push_str("老表s4.");

    println!("s3: {}, s4: {}", s3, s4);
    println!("-------------------");
}

fn takes_ownership(some_thing: String) {
    println!("{}", some_thing)
}
fn makes_copy(some_integer: i32) -> i32 {
    println!("{}", some_integer);
    some_integer + 1
}
fn test_function() {
    let s = String::from("hello world");
    takes_ownership(s);
    let x = 5;
    let y = makes_copy(x);
    println!("x: {}, y: {}, z: {}", x, y, x);
    println!("-------------------");
}

fn calculate_length(s: &mut String) -> usize {
    println!("&s1: {}", *s);
    s.push_str(", world!");
    s.len()
}

fn test_borrowing() {
    let x = 5;
    let y = &x;
    println!("x: {}, *y: {}", x, *y);

    let mut s1 = String::from("hello");
    let len = calculate_length(&mut s1);
    println!("s1:{}, len: {}", s1, len);
}

#[allow(unused_variables)]
type File = String;
fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    false
}
#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) {
    println!("{}, {}", f.len(), save_to.len())
}

fn test_file_read() {
    let mut f1 = File::from("f1.txt");
    open(&mut f1);
    close(&mut f1);
}

fn test_slice_string() {
    let s = String::from("hello world!");
    let s1 = &s[0..5];
    let len = s.len();
    let s2 = &s[6..len];
    let s3 = &s[..];
    println!("s: {}, s1: {}, s2: {}, s3: {}", s, s1, s2, s3);
    let mut s = String::from("hello world!");
    s.push_str(".");
    let word = first_word(&s);
    // s.clear(); // 借用的规则：当我们已经有了可变借用时，就无法再拥有不可变的借用
    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

fn test_utf8_slice() {
    // use utf8_slice lib
    for c in "asdfasf中国人adsfa".chars() {
        print!("{}\t", c)
    }
    println!("-------------------");
}

fn test_transfer_string_str() {
    let s = "hello world";
    println!("{}", s);
    let string = s.to_string();
    println!("&str to string: {}", string);
    let t_string = String::from("hello world");
    let t_string_to_str = &t_string[..];
    println!("string to str: {}", t_string_to_str);
}


struct Person {
    username: String,
    age: u8,
}
use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "username:{}, age:{}", self.username, self.age)
    }
}

impl fmt::Debug for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.username, self.age)
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn test_struct() {
    let p1 = Person {
        username: String::from("zhang_san"),
        age: 18,
    };
    println!("raw p1: {:#?}", p1);
    println!("username:{}, age: {}", p1.username, p1.age);
    let p2 = Person { age: 20, ..p1 };
    println!("p2->username:{}, p1->age: {}", p2.username, p1.age);

    let ct = Rectangle {
        width: dbg!(10 + 10),
        height: 12,
    };
    println!("ct->{:#?}", ct);
    dbg!(&ct);
    println!("-------------------");
}

#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
fn print_suit(card: PokerSuit) {
    println!("{:?}", card);
}
#[derive(Debug)]
struct PokerCard {
    suit: PokerSuit,
    value: u8,
}
#[derive(Debug)]
enum PU {
    Clubs(u8),
    Spades(u8),
    Diamonds(char),
    Hearts(char),
}
fn test_enum() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);
    let c1 = PokerCard {
        suit: PokerSuit::Clubs,
        value: 1,
    };
    let c2 = PokerCard {
        suit: PokerSuit::Diamonds,
        value: 12,
    };
    println!("c1: {:?}, c2: {:?}", c1, c2);
    let c3 = PU::Spades(5);
    let c4 = PU::Diamonds('A');
    println!("c3: {:?}, c4: {:?}", c3, c4);
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    let sum = x + y.unwrap();
    println!("sum: {}", sum);
    println!("-------------------");
}

fn test_array() {
    let a1 = [1, 2, 3, 4, 5];
    println!("{:?}", a1);
    let a2 = [3; 5];
    println!("{:?}", a2);
    let array: [String; 8] = std::array::from_fn(|i| String::from(format!("{} rust is good!", i + 1)));
    println!("{:#?}, len:{}", array, array.len());
    let slice: &[String] = &array[..3];
    println!("{:?}", slice);
    println!("-------------------");
    // 编译器自动推导出one的类型
    let one = [1, 2, 3];
    // 显式类型标注
    let two: [u8; 3] = [1, 2, 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3];
    let arrays: [[u8; 3]; 4] = [one, two, blank1, blank2];
    for a in &arrays {
        print!("{:?}: ", a);
        // 将a变成一个迭代器，用于循环
        // 你也可以直接用for n in a {}来进行循环
        for (i, n) in a.iter().enumerate() {
            print!("\t 第{}个元素: {} + 10 = {}", i + 1, n, n + 10);
        }
        let mut sum = 0;
        // 0..a.len,是一个 Rust 的语法糖，其实就等于一个数组，元素是从0,1,2一直增加到到a.len-1
        for i in 0..a.len() {
            sum += a[i];
        }
        println!("\t({:?} = {})", a, sum);
    }
    println!("-------------------");
}

#[derive(Debug)]
enum Direction {
    East,
    West,
    North,
    South,
}

enum Action {
    Say(String),
    MoveTo(i32, i32),
    ChangeColorRGB(u16, u16, u16),
}

fn test_match() {
    let dire = Direction::West;
    match dire {
        Direction::East => println!("East"),
        Direction::North | Direction::South => {
            println!("South or North");
        }
        _ => println!("West"),
    };

    let actions = [
        Action::Say("Hello Rust".to_string()),
        Action::MoveTo(1, 2),
        Action::ChangeColorRGB(255, 255, 0),
    ];
    for action in actions {
        match action {
            Action::Say(s) => {
                println!("{}", s);
            }
            Action::MoveTo(x, y) => {
                println!("point from (0, 0) move to ({}, {})", x, y);
            }
            Action::ChangeColorRGB(r, g, _) => {
                println!(
                    "change color into '(r:{}, g:{}, b:0)', 'b' has been ignored",
                    r, g
                );
            }
        };
    }
    let some_u8_value = 1u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }
    let v1 = vec![Direction::West, Direction::East, Direction::East];
    let filters = v1.iter().filter(|x| matches!(x, Direction::South));
    println!("flag: {:?}", filters);
    for f in filters {
        println!("{:?}", f);
    }
    println!("-------------------");
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(a) => Some(a+1),
    }
}
fn test_option(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    if none.is_none() {
        println!("none is nil.->{:?}", none);
    }
    println!("six: {:?}, none: {:?}", six, none);

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!("-------第二章节----------");
}

fn main() {
    // test_print_and_format();
    // test_print_string();
    // test_debug_string();
    // test_data_type();
    // test_num_handle();
    // let (flag, str) = callback();
    // println!("{:?}, {:?}", flag, str);
    // println!("-------------------");
    // test_all_ownership();
    // test_function();
    // test_borrowing();
    // test_file_read();
    // test_slice_string();
    // test_utf8_slice();
    // test_transfer_string_str();
    // test_handler_string();
    // test_struct();
    // test_enum();
    // test_array();
    // test_match();
    // test_option();
    // basic_type::test_fn_struct_enum_trait();
    // req::req_test();
    // type_transfer::test_type_transfer();
    // ffi_mod::test_ffi();
    // http_request::req::req_test();
    // http_request::ffi::ffi_mod::test_ffi();

    // basic_type::example::test_fn_struct_enum_trait();
    advanced_level::example::test_advanced_level();
}
