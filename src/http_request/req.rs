use reqwest::Url;
use crate::http_request::ffi::ffi_mod::test_ffi;

struct TestDefault<'a> {
    td: &'a str,
    strs: String,
}

impl Default for TestDefault<'_> {
    fn default() -> Self {
        println!("test impl default for struct type--->>");
        TestDefault{strs: String::from("hello world"), td: "bye bye!"}
    }
}

// impl TestDefault {
//     fn set_td_value<'a>(&mut self, val: &'a str) -> &mut TestDefault<'a> {
//         self.td = val;
//         self
//     }
// }


fn req_http() {
    // let body = reqwest::get("https://www.rust-lang.org").await?
    //     .text().await?;
    // println!("body = {:?}", body);
    let url = Url::parse("https://www.baidu.com:8989/a/b/c?name=tom&hello=world").unwrap();
    println!("url:{:?}, \nscheme: {}", url, url.scheme());
    println!("===================>>>>=====================");
    let td = TestDefault::default();
    // td.set_td_value("body--->");
    println!("td: {}, stars: {}", td.td, td.strs);
}

struct EmptyStruct;

impl EmptyStruct {
    fn add(&self) -> i32 {
        return 3+2;
    }
    fn add_param(&self, val: i32) -> i32 {
        return 10 + val;
    }
    pub fn add_pub(&self) -> String {
        return String::from("hello rocky!");
    }
}

fn empty_struct_test() {
    let es = EmptyStruct;
    println!("{}", es.add());
    println!("{}", es.add_param(2));
    println!("string type: {}", es.add_pub());
}

trait AddSub<T> {
    fn add(&mut self, c2: T) ->T;
    fn sub(&mut self, c2: T) ->T;
}

#[derive(Debug)]
struct Complex{
    real: i32,
}

impl Complex{
    fn new(real: i32) -> Self {
        Complex{real}
    }
}

impl AddSub<i32> for Complex {
    fn add(&mut self, c2: i32) -> i32 {
        self.real + c2
    }
    fn sub(&mut self, c2: i32) -> i32 {
        self.real - c2
    }
}

trait AddSubMul<T>: AddSub<T> {
    fn mul(&mut self, v:T) -> T;
}

#[derive(Debug)]
struct ComplexMul;

impl AddSub<i32> for ComplexMul {
    fn add(&mut self, c2: i32) -> i32 {
        c2
    }

    fn sub(&mut self, c2: i32) -> i32 {
        c2
    }
}

impl AddSubMul<i32> for ComplexMul {
    fn mul(&mut self, val: i32) -> i32 {
        self.add(10) *2
    }
}

#[allow(unused)]
fn trait_test(){
    let mut c = Complex::new(100);
    println!("add:{}", c.add(10));
    println!("sub:{}", c.sub(10));
    println!("complex obj: {:?}", c);
    let mut c1 = ComplexMul;
    println!("add:{}", c1.add(10));
    println!("sub:{}", c1.sub(10));
    println!("mul:{}", c1.mul(2));
    println!("complexMul obj: {:?}", c1);

}
#[allow(unused)]
pub fn req_test() {
    req_http();
    // empty_struct_test();
    // trait_test();
    test_ffi();
}
