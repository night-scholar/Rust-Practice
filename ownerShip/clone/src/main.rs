fn main() {
    let s1 = String::from("hello");
    let mut s2 = s1.clone();
    s2.push_str(", world") ;
    println!("s1 = {}, s2 = {}", s1, s2);
}
 