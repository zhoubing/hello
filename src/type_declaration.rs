pub fn type_declaration() {
    println!("{}", 32.5_f32.round()); //数字后面可以带类型
    println!("{}", 32.5f32.round()); //数字后面可以带类型
    println!("{}", 1_000_000); //数字可以分组 增强可读性

    //可以明确指定类型
    //有符号类型
    let i : i8 = 100;
    let i : i16 = 100;
    let i : i32 = 100;
    let i : i64 = 100;

    //无符号类型
    let i : u8 = 100;
    let i : u16 = 100;
    let i : u32 = 100;
    let i : u64 = 100;

    let i:f32 = 100.0;
    let i:f64 = 100.0;

    let i:isize = 100; //原生映射与当前CPU位宽相同宽度的整数类型，32位CPU就是32位 64位CPU就是64位
    let i:usize = 100;


    //二进制 八进制 十六进制字面量
    let binary = 0b1111;
    println!("{}", binary);

    let octal: i32 = 0o11;
    println!("{}", octal);

    let hex: i32 = 0x11;
    println!("{}", hex);

    //同类型才能比较 不同类型只能使用as转换为同一类型
    let a: i32 = 10000000;
    let b: i64 = 100;
    let c = 100i32;
    let d = 100_i32;

    if a == (b as i32) {
        println!("a is equal to b")
    }
}