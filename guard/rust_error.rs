use std::{fs::File, io};

#[allow(unused)]
pub fn run() {

    // 第一种错误：异常（可捕获错误）

    // rust中的异常爽下面这样的结构体
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    struct Error {
        code: i8,
        message: String,
    }

    fn open_file(path: String) -> Result<File> {
        let f = File::open(path);
        match f {
            Ok(file) => {
                return Result::Ok(file);
            },
            Err(err) => {
                match err.kind() {
                    io::ErrorKind::NotFound => {
                        return Result::Err(Error { code: -1, message: String::from("文件未找到") });

                    },
                    _ => {
                        return Result::Err(Error { code: -1, message: String::from("文件打开失败") });
                    }
                }
            }
        }
    }









    
    // 第二种错误：崩溃，程序终止
    // panic!("程序崩溃了"); // 让程序崩溃
    // println!("这行不会打印") // 这行不会打印

}
