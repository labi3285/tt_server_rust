
#[allow(unused)]
pub fn run() {

    // 动态数组

    let mut arr1 = Vec::<i32>::new(); // 创建一个动态数组
    let arr2 = vec![1, 2, 3]; // 通过宏创建
    let arr3 = vec![1; 10]; // 通过宏创建，带10个1
    println!("arr1:{:?}", arr1);
    println!("arr2:{:?}", arr2);
    println!("arr3:{:?}", arr3);

    arr1.push(1); // 添加元素

    let a1 = arr1[0]; // 取下标
    arr1[0] = 3; // 操作下标
    println!("a1:{} arr1.0:{}", a1, arr1[0]); 

    let s1 = &arr2[0..1];
    println!("s1:{:?}", s1); 

    // 迭代
    for a in &arr1 {
        println!("arr1->{}", a);
    }

    // 迭代（带索引）
    for (i, a) in arr2.iter().enumerate() {
        println!("arr2.{}:{}", i, a);
    }

    // map
    let arr3 = arr2.iter().map(|a| {
        return a + 1;
    });

}

