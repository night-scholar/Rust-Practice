#![allow(non_snake_case)]
use std::convert::From;

fn main() {
    //From
    let num = Number::from(30);
    println!("My number is {:?}", num);

    //Info
    let int = 5;
    // 试试删除类型说明
    let num: Number = int.into();
    println!("My number is {:?}", num);
}

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}