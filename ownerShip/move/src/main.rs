fn main() {
    // String是引用类型
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); 报错，因为这属于深拷贝，称之为移动
    println!("{}, world!", s2); //不报错
}
 