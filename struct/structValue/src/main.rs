#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("rect1 is {}", rect1); 报错
    println!("rect1 is {:?}", rect1); //添加#[derive(Debug)]解决报错
}
  