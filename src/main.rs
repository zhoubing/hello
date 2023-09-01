use std::{thread, env::args};
use std::time::Duration;
use clap::{App, Arg};

mod statements;
mod match_demo;
mod reference;
mod for_loop;
mod type_declaration;
mod lifespan;
mod generics;
mod string;
mod array;
mod file;
mod compound_struct;
mod data;
mod emu;
mod memory;
mod windows;
mod serializer;
mod action_kv;
mod smart_pointer;
mod network;
mod traits;

fn main() {
    //命令行参数
    // let args = App::new("demo")
    // .version("1.0.0")
    // .about("my demo")
    // .arg(Arg::with_name("pattern").help("help me").takes_value(true).required(true))
    // .get_matches();

    // let pattern = args.value_of("pattern").unwrap();
    // print!("{}", pattern);



    // let mut data = 1;
    // thread::spawn(|| { data = 200;});
    // thread::spawn(|| { data = 300;});



    // let c: i32 = 1;
    // let c_: i16 = c.try_into().unwrap();


    type_declaration::type_declaration();

    for_loop::for_loop_demo();
    for_loop::enumerate_loop();


    statements::is_even_if_version();
    statements::is_even_match_version();
    statements::break_return_a_value();
    
    match_demo::match_demo(21);
    match_demo::match_demo_return_value(21);

    reference::ref_demo();

    let result = lifespan::add_with_lifetime(&1, &10);
    println!("{}", result);

    println!("{:?}", generics::add(Duration::new(5, 0), Duration::new(10, 0)));

    string::handle_string();

    array::handle_array();

    file::handle_file();

    println!("{}", 12u8.saturating_sub(22));
    println!("{}", 12i32 - 22);

    compound_struct::handle_struct();
    data::handle_data();

    emu::begin_emu();

    // memory::run();
    memory::run_1();
    memory::wrong_pointer();
    // memory::run();

    windows::run();

    serializer::run();

    action_kv::run();
    action_kv::run_akv_disk();
    action_kv::run_akv_mem();

    smart_pointer::run();

    network::run();

    traits::run();
}
