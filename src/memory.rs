use std::borrow::Cow;
use std::ffi::CStr;
use std::os::raw::c_char;

static B: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
static C: [u8; 11] = [116, 104, 97, 110, 107, 115, 102, 105, 115, 104, 0];

pub fn run() {
    println!("=== memory run === ");
    let a = 40;
    let b: String;
    let c: Cow<str>;

    unsafe {
        let b_ptr: *mut u8 = &B as *const u8 as *mut u8;
        b = String::from_raw_parts(b_ptr, 10, 10);
        let c_ptr = &C as *const u8 as *const c_char;
        c = CStr::from_ptr(c_ptr).to_string_lossy();
    }

    let t = MyType { name: String::from("value")};
    let t1: String = t.into();
    println!("{}", t1);

    let heap_str = Box::new("123");
    println!("{}", *heap_str);


    let a = "123";
    let b: Box<&str> = Box::new(a);
    println!("{}", *b);
    println!("=== memory run === ");
}

pub fn run_1() {
    let a = 42;
    let a_ptr = &a as *const i32;
    let a_addr: usize = unsafe {
        std::mem::transmute(a_ptr)
    };
    println!("{:p} {:x}", a_ptr, a_addr);

    {
        let str: String = String::from("value");
    }
}

pub fn wrong_pointer() {
    let ptr = 42 as *const Vec<String>;
    unsafe {
        println!("{:p}, {:p}", ptr, ptr.offset(4));
    }
}

struct MyType {
    name: String
}

impl Into<String> for MyType {
    fn into(self) -> String {
        self.name + "123"
    }
}