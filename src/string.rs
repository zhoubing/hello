use std::ffi::OsString;
use std::path::Path;

pub fn handle_string() {
    let str = "Rust in Action"; //字面量字符串的类型是 &str
    println!("{}", str);
    let a: char = 'A';

    println!("{}", a);
    let mut v: Vec<u8> = Vec::with_capacity(8);
    v.push(12);

    println!("{:?}", v);

    let mut os = OsString::new();
    os.push("hello");
    println!("{:?}", os);

    let p = Path::new("index.html");
    println!("{:?}", p);

    
}