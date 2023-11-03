#![feature(asm)]
#![cfg(not(windows))]  //指出当前代码不能运行在Windows
#![feature(link_llvm_intrinsics)]//为了使用setjmp和longjmp所需要用到的注解
#![allow(non_camel_case_types)]

use std::arch::asm;
use std::{mem, process, time};
use std::cmp::max;
use std::thread::{sleep};
use std::time::Duration;
use libc::{raise, rand, SIG_DFL, SIG_IGN, signal, SIGTERM, SIGUSR1, unshare};

static mut SHUT_DOWN: bool = false;

pub fn interrupt_test() {
    //会产生segmentation fault
    // unsafe {
    //     asm!("int 42");
    // }

    //需要另外一个console给这个进程发信号
    // let delay = time::Duration::from_secs(1);
    // let pid = process::id();
    // println!("pid: {}", pid);
    // for i in 1..=60 {
    //     sleep(delay);
    //     println!(". {}", i);
    // }

    //
    // loop {
    //     unsafe {
    //         SHUT_DOWN = rand::random()
    //     }
    //     print!(".");
    //     if unsafe { SHUT_DOWN }
    //     {
    //         break;
    //     };
    // }
    // println!()

    // test_signal();


    // function_ptr_test();

    // ignore_signal_test();

    // dive(0, 5);

    jilj_test();
}

fn noop() {}

fn register_signal_handlers() {
    unsafe {
        libc::signal(SIGTERM, handle_sigterm as usize);
        libc::signal(SIGUSR1, handle_sigusr1 as usize);
    }
}

#[allow(dead_code)]
fn handle_sigterm(_signal: i32) {
    register_signal_handlers(); //尽可能重新注册信号，能够尽可能防止由于信号变化而影响信号处理程序本身

    println!("SIGTERM");

    unsafe {
        SHUT_DOWN = true;
    }
}

#[allow(dead_code)]
fn handle_sigusr1(_signal: i32) {
    register_signal_handlers();
    println!("SIGUSR1");
}

fn test_signal() {
    register_signal_handlers(); //必须尽早执行 否则信号不能得到正确处理
    let delay = Duration::from_secs(1);
    for i in 1_usize.. {
        println!("{}", i);
        unsafe {
            if SHUT_DOWN {//对可变静态变量访问是不安全的
                println!("*");
                return;
            }
        }
        sleep(delay);
        let signal = if i > 2 {
            SIGTERM
        } else {
            SIGUSR1
        };

        unsafe {
            libc::raise(signal);
        }
    }
}

fn function_ptr_test() {
    //函数指针
    let fn_ptr = noop as usize;
    let typed_fn_ptr = noop as *const fn() -> ();

    println!("noop as usize: 0x{:x}", fn_ptr);
    println!("noop as usize: *const T: {:p}", typed_fn_ptr);
}

fn ignore_signal_test() {
    unsafe {
        signal(SIGTERM, SIG_IGN); //忽略SIGTERM信号
        raise(SIGTERM); //让代码可以发出信号，这里这个信号是发给自己的
    }
    println!("ok!!!");
    unsafe {
        signal(SIGTERM, SIG_DFL); //把SIGTERM信号的行为重置为默认值 Default
        raise(SIGTERM); //体会一下与上面的区别 这里会让程序终止
    }
    println!("not ok!!!");//不会输出这行
}

fn print_depth(depth: usize) {
    for _ in 0..depth {
        print!("#");
    }
    println!();
}

fn dive(depth: usize, max_depth: usize) {
    print_depth(depth);
    if depth >= max_depth {
        return;
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

extern "C" {
    #[link_name = "llvm.eh.jilj.setjmp"]
    pub fn setjmp(_: *mut i8) -> i32;

    #[link_name = "llvm.eh.jilj.longjmp"]
    pub fn longjmp(_: *mut i8);
}

const JMP_BUF_WIDTH: usize = mem::size_of::<usize>() * 8;
const MOCK_SIGNAL_AT: usize = 3;

type jmp_buf = [i8; JMP_BUF_WIDTH];

static mut RETURN_HERE: jmp_buf = [0; JMP_BUF_WIDTH];

#[inline]
fn ptr_to_jump_buf() -> *mut i8 {
    unsafe {
        &RETURN_HERE as *const i8 as *mut i8
    }
}

fn jilj_dive(depth: usize, max_depth: usize) {
    unsafe {
        if SHUT_DOWN {
            println!();
            return;
        }
    }
    print_depth(depth);
    if depth >= max_depth {
        return;
    } else if depth == MOCK_SIGNAL_AT {
        unsafe {
            libc::raise(SIGUSR1);
        }
    } else {
        dive(depth + 1, max_depth);
    }
    print_depth(depth);
}

fn register_jilj_signal_handlers() {
    unsafe {
        libc::signal(SIGUSR1, handle_signals as usize);
    }
}

#[allow(dead_code)]
fn handle_signals(sig: i32) {
    register_jilj_signal_handlers();
    let should_shut_down = match sig {
        SIGHUP => false,
        SIGALRM => false,
        SIGTERM => true,
        SIGQUIT => true,
        SIGUSR1 => true,
        _ => false
    };
    unsafe {
        SHUT_DOWN = should_shut_down;
    }
    return_early();
}

#[inline]
fn return_early() {
    let franken_pointer = ptr_to_jump_buf();
    unsafe {
        longjmp(franken_pointer)
    };
}

fn jilj_test() {
    const JUMP_SET: i32 = 0;
    register_jilj_signal_handlers();

    let return_point = ptr_to_jump_buf();
    let rc = unsafe {
        setjmp(return_point)
    };
    if rc == JUMP_SET {
        jilj_dive(0, 10);
    } else {
        println!("early return!")
    }
    println!("finishing!")
}


