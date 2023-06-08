//rust是基于表达式的语言,如下：
fn is_even(n: i32) -> bool {
    n % 2 == 0 //因为表达式有返回值 这里可以省略return
}

//条件表达式
pub fn is_even_if_version() {
    let msg = if is_even(1) {
        "even"
    } else {
        "odd"
    };
    println!("even result is {}", msg);
}

pub fn is_even_match_version() {
    let msg = match is_even(1) {
      true => "even",
      false => "odd"  
    };
    println!("even result is {}", msg);
}

//break还特么能返回值？？？
pub fn break_return_a_value() {
    let n = loop {
        break 123;
    };
    println!("{}", n);
}

