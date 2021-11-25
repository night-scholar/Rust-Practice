use std::env;
use std::process;
mod lib;
fn main() {
    //读取启动参数
    let args: Vec<String> = env::args().collect();
 
    //调用lib.rs文件下的Config的结构体的new方法
    //闭包读取失败后打印失败原因并退出
    let config = lib::Config::new(&args).unwrap_or_else(|error| {
        //eprintln打印到控制台而不打印到文件
        eprintln!("Problem parsing arguments: {}", error);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    //执行lib.rs文件的run方法，如果返回错误则打印失败原因并退出
    if let Err(e) = lib::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}