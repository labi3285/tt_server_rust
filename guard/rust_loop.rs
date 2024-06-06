
#[allow(unused)]
pub fn run() {

    // for
    for i1 in 0..5 {
        println!("i1: {i1}");
    }

    // while
    let mut i2 = 0;
    while i2 < 5 {
        println!("i2: {i2}");
        i2 += 1;
    }

    // loop
    let mut i3 = 0;
    loop {
        if i3 == 5 {
            break; // 也可以在break后面跟上一个返回结果，这个结果会作为loop的结果，例如“break i3;”
        }
        println!("i3: {i3}");
        i3 += 1;
    }

}
