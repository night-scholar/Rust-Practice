use std::thread;
use std::sync::mpsc;
use std::time::Duration;


#[allow(dead_code)]
fn main1() {
    //创建一个通道，并将其两端赋值给 tx 和 rx
    //mpsc::channel 函数返回一个元组：第一个元素是发送端，而第二个元素是接收端。
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val is {}", val); error 在通道中已经发送了val，所有权移动至接收者
    });
    let received = rx.recv().unwrap();
    println!("{}",received);
}


fn main() {
    let (tx, rx) = mpsc::channel();
    //克隆一个发送者
    // let tx1  = tx.clone();
    let tx1 = mpsc::Sender::clone(&tx);


    thread::spawn(move || {
        let vals = vec![
            String::from("1hi"),
            String::from("1from"),
            String::from("1the"),
            String::from("1thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    

    for received in rx {
        println!("Got: {}", received);
    }
}