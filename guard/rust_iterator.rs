
#[allow(unused)]
pub fn run() {

    // iter，遍历集合
    let arr1 = [1, 2, 3, 4];
    for a1 in arr1.iter() {
        println!("a1: {a1}");
    }

    // iter，遍历集合
    let arr2 = [1, 2, 3, 4];
    for a2 in arr2.into_iter() {
        println!("a2: {a2}");
    }


}
