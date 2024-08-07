use std::collections::HashMap;
use std::slice::from_raw_parts;
use std::str::from_utf8_unchecked;

pub(crate) fn test_hashmap() {
    let teams_list = vec![
        ("china".to_string(), 100),
        ("usa".to_string(), 21),
        ("jp".to_string(), 32),
        ("ass".to_string(), 123),
    ];
    for v in &teams_list {
        println!("{}, {}", v.0, v.1);
    }
    let teams_map: HashMap<_, _> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score: Option<&i32> = scores.get(&team_name);
    println!("{:?}", score);
    drop(team_name);
    let _ = score;
    let team_name = String::from("ss");
    let score = scores.get(&team_name).copied().unwrap_or(100);
    println!("{}", score);
    println!("-------------------");
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }else {
        y
    }
}

fn get_memory_location() -> (usize,usize) {
    let string = "Hello world!";
    let pointer = string.as_ptr() as usize;
    let length = string.len();
    (pointer, length)
}

fn get_str_at_location(pointer: usize, length: usize) -> &'static str {
    unsafe {
        from_utf8_unchecked(from_raw_parts(pointer as *const u8, length))
    }
}

pub(crate) fn gen_static_str() ->&'static str {
    let mut s = String::new();
    s.push_str("hello world");
    Box::leak(s.into_boxed_str())
}

pub(crate) fn test_static() {
    let (pointer, length) = get_memory_location();
    let message = get_str_at_location(pointer, length);
    println!(
        "The {} bytes as 0x{:x} stored: {}",
        length, pointer,message
    );

    let r3: &str;
    {
        let r1 = "String";//.to_string();
        r3 = &r1;
        println!("r1: {}", r1);
    }
    println!("r3: {}", r3);
}
