#![allow(unused_variables)] //可以让编译器放宽要求，在一些情况下不产生编译警告
#![allow(dead_code)]

use std::{fmt::Display, rc::Rc, cell::RefCell};

static mut ERROR: isize = 0;

type File = String; //类型别名

#[derive(Debug)] //必须放在struct上面 让struct支持println!
struct FileEx {
    name: String,
    data: Vec<u8>,
}

impl FileEx {
    fn new(name: &str) -> FileEx {
        FileEx {name: File::from("/home/rust/hello/README.md"), data:Vec::new()}
    }
    
    fn new_with_data(name: &str, data: &Vec<u8>) -> FileEx {
        let mut f = FileEx::new(name);
        f.data = data.clone();
        f
    }
}

struct FileStr(String, u8);

trait Ops {
    fn sudo();
}

impl Ops for FileEx {
    fn sudo() {
        
    }
}

impl Display for FileEx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "123") 

    }
}

fn open(f: &mut File) {
    unsafe {
        ERROR = 10;  //修改静态可变变量必须使用unsafe
    }
}

fn open1(f: FileEx) -> Result<FileEx, String>{
    return Ok(f);
}

fn close(f: &mut File) {

}

pub fn handle_struct() -> () { //不写返回值就等于返回一个空的元组
    let mut f = File::from("/home/rust/hello/README.md"); //接受参数是字符串切片 &str
    open(&mut f);

    let fe: FileEx = FileEx {name: File::from("/home/rust/hello/README.md"), data:Vec::new()};
    let name = &fe.name; //使用.和引用符号避免发生移动后的使用问题

    println!("{:?} {}", fe, name); //#[derive(Debug)] 因为添加了这个才不会报错

    let fs = FileStr(String::from("123"), 10);
    println!("{} {}", fs.0, fs.1); //用数字索引方式访问底层数据

    let fs1 = FileEx {
        name: String::from("hello"),
        data: vec![1, 2, 3, 4, 5, 6]
    };
    let mut buf = Vec::new();
    let len = read(&fs1, &mut buf);
    println!("{:?}, len is {:?}", buf, len);


    let f = open1(fs1).unwrap();
    println!("FileEx trait is {}", f);
    let f1 = open1(f).is_err();


    //引用计数器
    let fs1 = FileEx {
        name: String::from("hello"),
        data: vec![1, 2, 3, 4, 5, 6]
    };
    let fs1_rc: Rc<FileEx> = Rc::new(fs1); //不可变
    let fs1 = FileEx {
        name: String::from("hello"),
        data: vec![1, 2, 3, 4, 5, 6]
    };
    let fs1_rc = Rc::new(RefCell::new(fs1)); //可变
    {
        let mut fs1_rc_mut = fs1_rc.borrow_mut();
        fs1_rc_mut.name = String::from("123");
    }
    println!("{:?}", fs1_rc);

}

pub fn never_returns_from_me() -> ! { //这种返回值类型叫做Never类型，永不返回。如果函数中不是死循环之类的话 会报错。
    loop {
        
    }
}

fn read(fs: &FileEx, buf: &mut Vec<u8>) -> usize {
    let mut data = fs.data.clone();

    buf.reserve(data.len());
    println!("data len before append: {}", data.len());
    buf.append(&mut data);
    println!("data len after append: {}", data.len()); //如果想保留传入的数据，可以使用Vec的Vec::extend_from_slice方法

    let mut data = fs.data.clone();

    buf.reserve(data.len());
    println!("data len before append: {}", data.len());
    buf.extend_from_slice(&mut data);
    println!("data len after append: {}", data.len()); //如果想保留传入的数据，可以使用Vec的Vec::extend_from_slice方法

    data.len()
}


