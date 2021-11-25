#![allow(non_snake_case)]
#![allow(unused_assignments)]
fn main(){
    main1();
    main2();
    main3();
} 



fn main1() {
    let spaces = "        ";
    // spaces = "hello world"; 报错，在没有使用mut和let关键字的情况下，spaces被称为不可变的变量
    println!("spaces is {}", spaces);
}

fn main2() {
    let mut spaces = "      ";
    // spaces = spaces.len();  报错，在使用mul关键字的情况下，我们不能改变变量的类型
    spaces = "hello world"; //不报错
    println!("spaces is {}", spaces);
}

fn main3() {
    let spaces = "      ";
    let spaces = spaces.len();
    //不报错 let不在乎类型,使用这种方式修改变量称之为隐藏
    println!("spaces is {}", spaces);
}
