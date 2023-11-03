use std::{thread, time};
use std::time::Duration;
use libc::sleep;

pub fn run() {
    // lambda
    let c = 12;
    let add = |a: usize, b: usize| { a + b + c };
    let handler = thread::spawn(move || {
        // thread::sleep(Duration::from_secs(3));
        println!("thread1 spawn {}", c)
    });
    // handler.join().unwrap();
    println!("main run1");
    let handler = thread::spawn(move || {
        println!("thread2 spawn {}", c)
    });
    handler.join().unwrap();
    println!("main run2");

    let mut handlers: Vec<thread::JoinHandle<()>> = Vec::with_capacity(1000);
    for n in 1..50 {
        let handle = thread::spawn(move || {
            // thread::sleep(Duration::from_secs(3));
            let start = time::Instant::now();
            let pause = time::Duration::from_millis(20);

            while start.elapsed() < pause {
                // thread::yield_now()
                std::hint::spin_loop()
            }
            println!("thread {}", n);
        });
        handlers.push(handle)
    }

    println!("before join!");
    while let Some(handle) = handlers.pop() {
        handle.join();
        println!("after join!")
    }
}

