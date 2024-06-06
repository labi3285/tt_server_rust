
#[allow(unused)]
pub fn run() {

    // Option是一个内置的枚举，不需要手动实现
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }


    let o1 = Option::Some("Hello"); // 正常写法
    let o2 = Some(123); // 简略写法

    show_option_val(&o1);
    show_option_val(&o2);

    if let Some(val) = o1 { // if let
        println!("let:{}", val)
    }
    let o3 = o2.unwrap(); // 强制解包，空的话会报错
    println!("unwrap:{}", o3);



    fn show_option_val<T: std::fmt::Display>(o: &Option<T>) {

        // 普通写法
        match o {
            Option::Some(val) => {
                println!("{}", val)
            },
            Option::None => {
                println!("None")
            }
        }
        
        // 简写
        match o {
            Some(val) => {
                println!("{}", val)
            },
            None => {
                println!("None")
            }
        }

        // if let
        if let Some(val) = o {
            println!("{}", val)
        } else {
            println!("None")
        }

    }
    
}
