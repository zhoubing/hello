use std::{fs::{File, read}, io::{BufReader, Read, BufRead}};
use std::env::current_dir;

pub fn handle_file() {
    println!("{:?}", current_dir());
    let f = File::open("/home/rust/hello/README.md").unwrap();
    let mut reader: BufReader<File> = BufReader::new(f);
    // let mut buf = String::new();
    // loop {
    //     let len = reader.read_line(&mut buf).unwrap();
    //     if len == 0 {
    //         break;
    //     }
    //     println!("{}", buf);
    //     buf.truncate(0);
    // }


    for l in reader.lines() {
        let s = l.unwrap();
        println!("{}", s);
    }
}