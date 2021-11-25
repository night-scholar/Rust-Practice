//trait特性 定义一个实现某些目的所必需的行为的集合，可以用于标识哪些类有哪些方法。
// 格式是：impl <特性名> for <所实现的类型名>
// 大致可以理解为Golang中的结构体方法,trait添加一些方法，使用impl <特性名> for <所实现的类型名>实现。
pub trait Summary {
    fn summarize(&self) -> String;
}

// pub trait Display{
//     fn getNum(&self) -> i32;
// }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

//提供Summary里的方法给Tweet
impl Summary for Tweet {  
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// impl Display for Tweet{
//     fn getNum(&self) -> i32{
//         1
//     }
// }
//trait 作为参数,item 需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现
// pub fn notify(item: impl Summary + Display) {
//     println!("{}", item.summarize());
//     println!("{}",item.getNum());
// }


pub fn notify(item: impl Summary) {
    println!("{}", item.summarize());
}

//返回trait类型
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}


fn main(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    
    // println!("{}", tweet.summarize());
    notify(tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    // println!("{}", article.summarize());
    notify(article);
    
    let summarizable = returns_summarizable();
    // println!("{}", summarizable.summarize());
    notify(summarizable);



}