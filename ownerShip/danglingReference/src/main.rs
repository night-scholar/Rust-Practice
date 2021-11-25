fn main() {
    let reference_to_nothing = dangle();
    println!("{}",reference_to_nothing);
}

fn dangle() -> String {
    //&String 报错 因为String类型离开了作用域 那么返回值&s将指向一个空地址，因此不用取地址符
    let s = String::from("hello");

    // &s 报错 因为String类型离开了作用域 那么返回值&s将指向一个空地址，因此不用取地址符
    s
}
