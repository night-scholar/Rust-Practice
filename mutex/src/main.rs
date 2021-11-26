use std::sync::{Mutex, Arc};
use std::thread;


#[allow(dead_code)]
fn main1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        //这里就已经解锁，离开闭包作用域就会释放锁 
        *num = 6;
    }

    println!("m = {:?}", m);
}

fn main() {
    //原子引用计数 Arc
    //使用 Arc 包装一个 Mutex 能够实现在多线程之间共享所有权
    //因为 counter 是不可变的，不过可以获取其内部值的可变引用
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1; 
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}