
#[allow(unused)]
pub fn run() {

    let s1 = "hello1"; // 切片类型（&str）
    let s2 = String::from("hello2"); // String对象

    let s3 = &s1[0..2]; // 切片可以进一步切片
    let s4 = &s1[0..2]; // String对象也可以切片

    let s5 = &s2[..]; // String对象通过切片可以转化为切片类型

    let mut s6 = String::from("hello6"); // String对象
    let s7 = &s6[0..3]; // 切片引用
    s6.push_str(" add2"); // 愿字符串变化，切片s7失效

    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = &arr1[0..2]; // 数组也支持切片

}


