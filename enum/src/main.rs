#[allow(dead_code)]

enum Book{
    Go {index : u32}, 
    Rust {index : u32},
    Solidity {index : u32}
}

fn main(){
    let my_book = Book::Go{index :1};
    println!("{:?}",find_book(my_book));
}

fn find_book(book:Book)->(String,u32){
    match book {
        Book::Go{index}=>{
            ("Go".to_string(),index)
        }
        Book::Rust{index}=>{
            ("Rust".to_string(),index)
        }
        Book::Solidity{index}=>{
            ("Solidity".to_string(),index)
        }
    }
}









// enum Book{
//     Go {index : u32},
//     Rust {index : u32},
//     C {index : u32},
//     Java {index : u32},
// }


// fn main(){
//     let my_book = Book::Go{index : 1};
//     let fin_book = find_book(my_book);
//     println!("{}",fin_book.0);
//     println!("{}",fin_book.1);
// }


// fn find_book(book:Book) -> (String,u32){
//     match book {
//         Book::Go{index} => {
//             ("Go".to_string(),index)
//         }
//         Book::Rust{index} => {
//             ("Rust".to_string(),index)
//         }
//         Book::C{index} => {
//             ("C".to_string(),index)
//         }
//         Book::Java{index} => {
//             ("Java".to_string(),index)
//         }
//     }
// }