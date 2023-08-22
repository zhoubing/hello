// chapter5
#![allow(arithmetic_overflow)] //需要加这行代码 才会出现overflow panic

pub fn handle_data() {
    let a: u16 = 50115;
    let b: i16 = -15421;
    println!("{:016b}", a);
    println!("{:016b}", b);

    transform_data();
}

fn transform_data() {
    let a:f32 = 42.42;
    let b:u32 = unsafe {
        std::mem::transmute(a)
    };
    println!("{}", b);
    println!("{:032b}", b);

    //let i: u8 = 200 + 200;   panic
    float_store();
}

fn float_store() {
    let n:f32 = 42.42;
    let n_bits = n.to_bits();
    let sign_bit = n_bits >> 31;
    println!("{}", sign_bit);
    let exponent = n_bits >> 23;
    let bias = 127;
    println!("{}", ((exponent & 0xff) as i32) - bias);

    for i in 0..23 {
        let mask = 1 << i;
        let one_at_bit = n_bits & mask;
        if one_at_bit != 0 {
            
        }
    }
}