pub fn match_demo(item: i32) {
    match item {
        0 => println!("match zero"),    //单值匹配
        10 ..= 20 => println!("10.. 20"), //范围匹配
        40 | 80 => println!("40|80"),   //匹配任意一个值
        _ => println!("all other cases") //匹配所有值
    }

    let a: i32 = 10000000;
    let b_: Result<i8, _> = a.try_into();
    match b_ {
        Ok(t) => println!("{}", t),
        Err(t) => println!("err: {}", t) //如果转换后数据有损失就会走到这
    }

}

pub fn match_demo_return_value(item: i32) {
    let msg = match item {
        0 => "match zero",    //单值匹配
        10 ..= 20 => "10.. 20", //范围匹配
        40 | 80 => "40|80",   //匹配任意一个值
        _ => "all other cases" //匹配所有值
    };
    println!("{}", msg);
}
