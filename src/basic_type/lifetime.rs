use crate::basic_type::collection::longest;
use crate::basic_type::fn_example::complex_function;

pub(crate) fn test_lifetime() {
    let str1 = String::from("long string is long");
    let str2 = String::from("xyz");

    let result;
    {
        result = longest(str1.as_str(), str2.as_str());
        // println!("the longest str is: {}", result);
    };
    println!("the longest str is: {}", result);
}

pub(crate) fn test_lifetime2(){
    let a = 2;
    let my_num = complex_function(&a);
    println!("my_num:{}", my_num);
}