// 类似地，`mod inaccessible` 和 `mod nested` 将找到 `nested.rs` 和
// `inaccessible.rs` 文件，并在它们放到各自的模块中。
//inaccessible为私有，mian函数不能调用
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    println!("called `my::indirect_access()` ");

    private_function();
    inaccessible::public_function();
}
