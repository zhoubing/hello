use std::ops::{Add};

pub fn add<T: Add<Output = T>>(i: T, j: T) -> T { //要求T必须实现Add 用来支持加法运算
    i + j
}