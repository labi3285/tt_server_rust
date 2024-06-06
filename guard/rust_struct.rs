#[allow(unused)]
pub fn run() {

    // 普通结构体
    #[derive(Debug)]
    struct Student {
        name: String,
        age: i8,
    }
    impl Student {
        // 构造函数
        fn create(name: String, age: i8) -> Student {
            Student { name, age }
        }
        // 普通函数
        fn say_hello(&self) {
            println!("hello, my name is {}, {} years old.", self.name, self.age);
        }
    }

    let name = String::from("小明");
    let s1 = Student { name, age: 13 }; // 构建，类似js的简写也是支持的
    println!("{:?}", s1); // debug打印
    s1.say_hello(); // 调用方法
    println!("s1.name:{}", s1.name); // 访问变量
    let s2 = Student::create(String::from("小花"), 14); // 使用构造函数

    // 元组形成的结构体
    #[derive(Debug)]
    struct Color(u8, u8, u8);

    let c1 = Color(1, 1, 1);
    println!("{:?}", c1);

}
