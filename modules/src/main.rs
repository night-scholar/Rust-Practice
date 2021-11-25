// #[allow(non_snake_case)]
// #[allow(dead_code)]
// mod first{
//     //默认私有
//     fn firstPrivate(){
//         println!("firstPrivate");
//     }
//     //定义公有
//     pub fn firstPublic(){
//         println!("firstPublic");
//     }
//     //嵌套模块
//     pub mod second{ 
//         //默认私有
//         fn secondPrivate(){
//             println!("secondPrivate");
//         }
//         //定义公有
//         pub fn secondPublic(){
//             println!("secondPublic");
//         }
//         //只在当前模块可见
//         pub(self) fn secondPublicSelf(){
//             println!("secondPublicSelf");
//         }
//         //父模块可见
//         pub(super) fn secondPublicSuper(){
//             println!("secondPublicSuper");
//         }

//         pub mod third{
//             //父模块可见
//             pub(super) fn thirdPublicSuper(){
//                 println!("thirdPublicSuper");
//             }

//             // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
//             // `path` 必须是父模块（parent module）或祖先模块（ancestor module），这里first和second都可以调用
//             pub(in crate::first) fn thirdFunctionInFirst(){
//                 println!("thirdFunctionInFirst");
//             }
//         }
//         pub fn function(){
//             secondPrivate();
//             secondPublicSelf();
//             third::thirdFunctionInFirst();
//             third::thirdPublicSuper();
//         }
        
//     }
    
//     pub fn function(){
//         firstPublic();
//         firstPrivate();
//         //包含secondPublicSelf third::thirdFunctionInFirst();   third::thirdPublicSuper();
//         second::function();

//         second::secondPublic();
//         // secondPrivate(); error
//         second::secondPublicSuper();

//     }
// }   

// fn main(){
//     first::function();
// }




// fn main() {
//     mod my{
//         pub struct OpenBox<T>{
//             pub contents : T
//         }

//         pub struct ClosedBox<T>{
//             contents :T
//         }

//         impl<T> ClosedBox<T> {
//             // 一个公有的构造器方法
//             pub fn new(contents: T) -> ClosedBox<T> {
//                 ClosedBox {
//                     contents: contents,
//                 }
//             }
//         }
//     }
    
//     let open_box = my::OpenBox{contents:"open_box"};
//     println!("{}",open_box.contents);

//     // let closed_box = my::ClosedBox{contents:"close_box"}; error

//     let closed_box = my::ClosedBox::new("close_box");
//     // println!("{}",closed_box.contents);  error
// }


// fn main(){
//     use first::second::function as out_function;
//     fn function(){
//         println!("{}","function");
//     }

//     mod first{
//         pub mod second{
//             pub fn function(){
//                 println!("{}","first::second::function()");
//             }
//         }
//     }
//     out_function();
//     {
//         use first::second::function;
//         function();
//     }
//     function();
// }


// fn function() {
//     println!("called `function()`");
// }

// mod cool {
//     pub fn function() {
//         println!("called `cool::function()`");
//     }
// }

// mod my {
//     fn function() {
//         println!("called `my::function()`");
//     }
    
//     mod cool {
//         pub fn function() {
//             println!("called `my::cool::function()`");
//         }
//     }
    
//     pub fn indirect_call() {
//         // 让我们从这个作用域中访问所有名为 `function` 的函数！
//         print!("called `my::indirect_call()`, that\n> ");
        
//         // `self` 关键字表示当前的模块作用域——在这个例子是 `my`。
//         // 调用 `self::function()` 和直接调用 `function()` 都得到相同的结果，
//         // 因为他们表示相同的函数。
//         self::function();
//         function();
        
//         // 我们也可以使用 `self` 来访问 `my` 内部的另一个模块：
//         self::cool::function();
//         cool::function();
        
//         // `super` 关键字表示父作用域（在 `my` 模块外面）。
//         super::function();
        
//         // 这将在 *crate* 作用域内绑定 `cool::function` 。
//         // 在这个例子中，crate 作用域是最外面的作用域。
//         {
//             use crate::cool::function as root_function;
//             root_function();
//         }
//     }
// }

// fn main() {
//     my::indirect_call();
// }



// 多文件编程
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `my` 的模块里面。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}