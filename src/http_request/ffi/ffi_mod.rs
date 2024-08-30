use crate::http_request::req::req_test;

extern "C" {
    // fn Sleep(ms: u32);
    fn printf(format: *const u8, ...) -> i32;
}
#[allow(unused)]
fn test_ffi_c(){
    let message = "------->Hello world!";
    let format = b"->%s\0";
    unsafe {
        printf(format.as_ptr(), message.as_ptr());
    }
}

fn box_sized(){
    let s1:Box<str> = "Hello there!".into();
    println!("hello there-->:{}", s1);
}
#[allow(unused)]
pub fn test_ffi() {
    test_ffi_c();
    box_sized();
}