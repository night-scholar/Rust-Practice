fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2); // 没问题
    let r3 = &mut s; // 没问题
    println!("{}", r3); // 没问题
                        // println!("{},{},{}",r1,r2,r3); //问题巨大，牢记：只有同道中人才能处于同一个作用域。
}
 