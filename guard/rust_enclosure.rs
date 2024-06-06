
#[allow(unused)]
pub fn run() {

    // 闭包本身是变量，又可以当函数用
    
    // 不带参数
    let e1 = || println!("xxx");
    // 带参数（简写）
    let e2 = |a: i32, b: i32| a + b;
    // 带参数（完整写法）
    let e3 = |a: i32, b: i32| -> i32 {
        return a + b;
    };

    // 捕获外部变量
    let o4 = 5;
    let e4 = || {
        return o4 * 2; // 这里使用了外部变量o4
    };
    println!("o4:{} e4():{}", o4, e4());

    // 捕获外部变量（通过move，闭包获取了o5对应的模型的所有权）
    let o5 = vec![1, 2, 3];
    let equal_to_x = move |a: Vec<i32>| a == o5;
    // 由于o5所有权被闭包获取，此时o5已经失效

}
