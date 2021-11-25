use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    //生成随机数
    let secret_number = rand::thread_rng().gen_range(1..101);
    //循环
    loop {
        println!("Please input your guess.");
        //创建地址空间
        let mut guess = String::new();
        //存储用户输入
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        //处理用户输入类型
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        //比较大小
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
