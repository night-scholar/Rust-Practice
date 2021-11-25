#[derive(Debug)]
struct Shoe{
    size: u32,
    style: String,
}


fn iterator_filter(){
    let mut v_struct = vec![
        Shoe{size:10,style:String::from("Red")},
        Shoe{size:11,style:String::from("Green")}
    ];
    v_struct.push(Shoe{size:12,style: String::from("Black")});
    let shoes  = shoes_in_mysize(v_struct, 12);
    println!("{:?}",shoes);
} 

fn shoes_in_mysize(shoes:Vec<Shoe>,shoes_size:u32) -> Vec<Shoe>{

    shoes.into_iter().filter(|s| s.size == shoes_size).collect()

}


fn iterator_iter(){
    println!("iterator_iter");
    let v = vec![1,2,3];

    for num in v.iter(){
        println!("{}",num);
    }
}

fn iterator_next(){
    let v = vec![1,2,3];

    let mut v_iter = v.iter();
    println!("iterator_next");
    //每一次next都会消费一个值
    println!("{:?}",v_iter.next());
    println!("{:?}",v_iter.next());
    println!("{:?}",v_iter.next());

}

fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    //所有权转移
    let total: i32 = v1_iter.sum();

    println!("iterator_sum");
    println!("{}",total);

}

#[derive(Debug)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new(i:u32) -> Counter {
        Counter { count: i }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}


fn using_other_iterator_trait_methods(i:u32)  {
    //Counter:new(1) -> [2,3,4,5]   Counter::new(i).skip(1) -> [3,4,5]
    //zip -> [(2,3),(3,4),(4,5)]
    //map -> [6,12,20]
    //filter -> [6,12]
    //sum -> 18
    let sum: u32 = Counter::new(i).zip(Counter::new(i).skip(1))
                                 .map(|(a, b)| a * b)
                                 .filter(|x| x % 3 == 0)
                                 .sum();
    println!("{}",sum);
}

fn main() {

    iterator_iter();

    iterator_next();

    iterator_sum();

    iterator_filter();

    println!("{:?}",Counter::new(4).next());

    using_other_iterator_trait_methods(1);

}