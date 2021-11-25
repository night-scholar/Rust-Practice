fn main() {
    let mut s = String::from("hello");
    let mut s2 = &mut s;
    change(&mut s2);
    println!("{}",&s)
}


//s,s2指向同一个地址
fn change(some_string: &mut String) {
    
    some_string.push_str(", world"); 
    println!("{}", &some_string);
}
