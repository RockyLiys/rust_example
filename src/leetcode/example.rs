use crate::leetcode::str_unique::{compress_string, is_unique, replace_spaces};

pub(crate) fn test_leetcode() {
    // let is_uni= is_unique("leetcode".to_string());
    // println!("{}", is_uni);
    // println!("{}", replace_spaces("adf asdfa adf ".to_string(), 13))
    let n = compress_string("bb".to_string());
    println!("{}", n)
}