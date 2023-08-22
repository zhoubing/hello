use std::env;
use std::io::Read;
use std::path::Path;
use std::path::PathBuf;

use serde_json::to_string;
use serde_json::to_vec;
use bincode::serialize;
use serde_derive::Serialize;

use std::fs::File;

#[derive(Serialize)]
struct User {
    name: String,
    age: u16
}

pub fn run() {
    let u = User {name: String::from("zhoubing"), age : 16};
    let ju = to_string(&u).unwrap();
    println!("{:?}", ju);
    let vu = to_vec(&u).unwrap();
    println!("{:?}", vu);
    let bs = serialize(&u).unwrap();
    println!("{:?}", bs);

    println!("UTF8 {:?}", String::from_utf8_lossy(ju.as_bytes()));
    println!("UTF8 {:?}", String::from_utf8_lossy(&vu));
    println!("UTF8 {:?}", String::from_utf8_lossy(&bs));

    let mut INPUT: &[u8] = br#"
        fn main() {
            println("Hello World");
        }
    "#;

    let mut buf: Vec<u8> = vec![];
    println!("{:?}", buf);
    INPUT.read_to_end(&mut buf);
    println!("{:?}", buf);

    for line in buf.chunks(16) {
        // println!("{:08x}", position_in_input);
        for b in line {
            println!("{:02x}", b);  // 输出的这种形式 02就是输出2位 如果不够左边用0补齐
        }
    }

    let p = env::args().nth(1).unwrap(); //读取参数
    println!("{}", p);

    // let f = File::open("c:\\Users\\zhoub\\Downloads\\Python深度学习（第2版）.pdf");
    println!("{:?}", env::current_dir());
    let f: Result<File, std::io::Error> = File::open("/home/chat-shot.png");
    let r = match f {
        Ok(mut v) => {
            let mut buf = [0; 1280];
            let r = v.read_exact(&mut buf).unwrap();
            println!("{:?}", r);
            println!("{:?}", buf);
        },
        Err(e) => {
            println!("{:?}", e);
        }
    };
    println!("{:?}", r);
    let mut f = File::open("/home/chat-shot.png").expect("msg");
    println!("{:?}", f);

    while let Ok(_) = f.read_exact(&mut buf) {
        println!("===> {:?}", buf);
    }
    if let Ok(f) = File::open("/home/chat-shot1.png") {
        println!("---> {:?}", f);
    } else {
        println!("end!")
    }

    let pb = PathBuf::from("c:/tmp/hello.txt");
    println!("{}", pb.extension().unwrap().to_str().unwrap()); //获取扩展名
}