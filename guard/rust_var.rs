
#[allow(unused)]
pub fn run() {

    // 注意：
    // 变量前的“_”可以避免[unused variable]警告

    // let修饰的变量不可修改，但是可以重新let甚至不同类型，这种操作又称之为“重影”
    let _v1 = 123;
    let _v1 = "hello"; // 重影ßß

    // let mut 修饰的变量可以修改
    let mut _v2 = 123; 
    _v2 = 456; // mut可修改值

    // const修饰的变量不可变，且并且必须指定类型，const变量一般大写
    // const和let的区别在于const不能重新const
    const _V3: i32 = 123; 

}