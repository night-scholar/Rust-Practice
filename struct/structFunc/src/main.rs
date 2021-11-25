#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//这里声明area方法为结构体Rectangle的方法
impl Rectangle {
    //self为Rectangle结构体类型的数据
    fn area(&self) -> u32 {
        //返回类型为u32，返回值为长*宽
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
