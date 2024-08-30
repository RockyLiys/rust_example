use std::collections::HashSet;
#[allow(unused)]
pub(crate) fn is_unique(a_str: String) -> bool {
    let a_list: Vec<_> = a_str.bytes().collect();
    let mut unique_str = HashSet::new();
    for s in a_list.iter() {
        unique_str.insert(s);
    }
    let uni_length = unique_str.len();
    a_list.len() == uni_length
}
#[allow(unused)]
pub(crate) fn replace_spaces(s: String, length: i32) -> String {
    let mut s1 = String::new();
    for i in s.chars().take(length as usize) {
        if i == ' ' {
            s1.push_str("%20");
        }else {
            s1.push(i);
        }
    }
    s1
}
#[allow(unused)]
pub(crate) fn compress_string(s: String) -> String {
    let mut new_str = String::new();
    let mut count: usize = 0;
    let mut pre: char = ' ';
    for c in s.chars() {
        if pre == ' ' {
            new_str.push(c);
            count = count +1;
            pre = c;
            continue
        }
        if c == pre {
            count += 1;
            pre = c;
            continue
        }
        if c != pre {
            new_str.push_str(&count.to_string());
            pre = c;
            new_str.push(c);
            count = 1;
        }
    }
    new_str.push_str(&count.to_string());
    if s.len() < new_str.len() {
        return s
    }
    new_str
}

