fn test_handler_string() {
    let mut s = String::from("hello w.orld");
    s.push_str("!");
    s.insert(5, ',');
    s.push('.');
    println!("s: {}", s);
    // 替换所有
    let new_all_string = s.replace(".", " I like you!");
    println!("new string: {}", new_all_string);
    // 替换指定个数
    let new_num_string = s.replacen(".", " I like you!", 1);
    println!("new string: {}", new_num_string);
    // 字符串范围的替换, 不会返回新的字符串
    s.replace_range(5..7, "==");
    println!("new string: {}", s);
    let s1 = s.pop();
    println!("s1: {}", s1.unwrap());
    println!("s占{}个字节", std::mem::size_of_val(s.as_str()));

    let string_append = String::from("hello ");
    let string_rust = String::from("rust"); //或者 "rust";
    let result = string_append + string_rust.as_str();
    println!("{}", string_rust);
    let mut result = result + "!?";
    result += "@!!!@";
    println!("concatenate string: {}", result);
    println!("-------------------");
}
