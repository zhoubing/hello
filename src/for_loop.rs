use num::Complex; //::限定符 限定只导入num中的Complex包

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn add_1(a: i32, b: i32) -> i32 {
    a + b
}

pub fn for_loop_demo() {
    let names = [
        "1",
        "2",
        "3"
    ];
    for n in names.iter() {
        println!("{}", n);
        println!("{}", &n); //借用的写法，用于只读
    }

    let multi_lines = "
        aaa
        bbb
        ccc
        ddd
    ";
    for n in multi_lines.lines() {
        println!("{}", n);
    }
    eprintln!("{}", "err!"); //错误输出 

    let clx = Complex{ re: 2.1, im: -1.2}; //rust不支持构造函数，可以使用此语法来分配初始值

    Complex::new(2.1, -1.2); //根据惯例，很多类型习惯都会实现一个new方法，但这个不是rust语言的一部分。


    let container = vec!["1", "2", "3"];
    for item in container {
        println!("item: {}", item);
    }
    //上面迭代使用后，不能再次迭代该container，因为已经move
    //实际上rust将for循环扩展成了方法调用,从报错信息上能看出来是调用了into_iter
    // for item in container 扩展成了 IntoIterator::into_iter(container) 拥有所有权
    // for item in &container 扩展成了 for item in container.iter()     只读
    // for item in &mut container 扩展成了 for item in container.iter_mut() 读写

    // for item in container {
    //     println!("item: {}", item);
    // }

    //如果想再次使用的话 需要使用引用读取 
    // for item in &container {
    //     println!("item: {}", item);
    // }

    //匿名for循环
    for _ in vec!["1", "2", "3"] {
        println!("1");
    }

    println!("{}", add(1, 2));
    println!("{}", add_1(1, 2));

    //不推荐以下这种方式，因为需要做边界检查
    let v = vec![1, 2, 3];
    for i in 0..v.len() {
        println!("{}", v[i]);
    }

    //rust无限循环关键字
    // loop {
        
    // }

    for i in 0..=5 { //加一个等于号表示0到5包括5 而不是0到4
        println!("loop {} ", i );
    }
}


pub fn enumerate_loop() {
    let str = "\
        kkkkk
        ggggg
        hhhhh
        jjjjj";

    for li in str.lines() {
        println!("{}", li);
    } 

    for (i, li) in str.lines().enumerate() { //enumerate返回索引值以及内容
        println!("{} : {}", i, li);
    } 
}