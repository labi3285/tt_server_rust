
// use diesel::{Queryable, pg::Pg};
// use serde::{Serialize, Deserialize};

#[allow(unused)]
pub fn run() {
    // 普通枚举
    #[derive(Debug)]
    enum Gender {
        Man,
        Woman,
        Other,
    }

    let g1 = Gender::Man;
    println!("g1:{:?}", g1);

    // 带参数枚举
    #[derive(Debug)]
    enum Book {
        Papery { index: u32 }, // 带参数
        Electronic { url: String },
        Papery1(u32), // 这是另一种型式
        Electronic1(String),
    }

    let b1 = Book::Papery { index: 1 };
    let b2 = Book::Electronic {
        url: String::from("http://1"),
    };
    let b3 = Book::Papery1(2);
    let b4 = Book::Electronic1(String::from("http://2"));
    println!("b1:{:?}", b1);
    println!("b2:{:?}", b2);
    println!("b3:{:?}", b3);
    println!("b4:{:?}", b4);

    for (i, e) in [b1, b2, b3, b4].iter().enumerate() {
        match e {
            Book::Papery { index } => {
                println!("e{}:{}", i, index);
            }
            Book::Electronic { url } => {
                println!("e{}:{}", i, url);
            }
            Book::Papery1(number) => {
                println!("e{}:{}", i, number);
            }
            Book::Electronic1(url) => {
                println!("e{}:{}", i, url);
            }
        }
    }

    // // 问题：我如何让一个枚举可以保存到数据库？
    // #[derive(Queryable, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash, DeriveActiveEnum)]
    // enum UserRole {
    //     Admin,
    //     User,
    // }



}
