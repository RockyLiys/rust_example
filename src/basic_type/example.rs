use std::thread;
use std::time::Duration;
use crate::basic_type::collection::{test_hashmap, test_static};
use crate::basic_type::fn_example;
use crate::basic_type::fn_example::{exec, exec1, exec2, factory, factory1, test_fn};
use crate::basic_type::format_example::test_println_format;
use crate::basic_type::generic_example::test_generics;
use crate::basic_type::lifetime::{test_lifetime, test_lifetime2};
use crate::http_request;

fn workout(intensity: u32, random_number: u32) {
    let action = || {
        println!("muuuu.....");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!(
            "今天活力满满，先做 {} 个俯卧撑!",
            action()
        );
        println!(
            "旁边有妹子在看，俯卧撑太low，再来 {} 组卧推!",
            action()
        );
    } else if random_number == 3 {
        println!("昨天练过度了，今天还是休息下吧！");
    } else {
        println!(
            "昨天练过度了，今天干干有氧，跑步 {} 分钟!",
            action()
        );
    }
}


fn test_workout(){
    let intensity = 10;
    let random_number = 7;
    workout(intensity, random_number);
    println!("-------------------");
    let sum = |x, y| x+y;
    let val = sum(1,2);
    println!("sum: {}", format!("{:?}",val));
    println!("-------------------");

    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);

    update_string("hello world");
    update_string(" rocky!");
    println!("{:?}", s);
    println!("-------------------");

    let s1 = String::from("exec");
    let up_string = || println!("{}", s1);

    exec(up_string);
    exec1(up_string);
    exec2(up_string);
    println!("-------------------");
    let f = factory(2);
    let answer = f(5);
    println!("{}, {}", answer, factory1(4)(5));
}

pub fn test_fn_struct_enum_trait() {
    // test_fn();
    // test_generics();
    // test_hashmap();
    // test_lifetime();
    // test_static();
    // test_workout();
    // test_lifetime2();
    test_println_format();
}

fn test_pub_super(){
    println!("{}", "test_pub_super");
}
