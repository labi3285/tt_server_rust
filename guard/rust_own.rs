
#[allow(unused)]
pub fn run() {

    // rust通过变量所有权只属于某一个变量这样的方式从而达到很简单的内存管理

    {
        let i1 = 123; // i1变量指向一个值类型
        let i2 = i1; // 由于123是一个值类型，这里i2实际上获取的值是i1值的copy

        let o1 = create_heap_obj(); // o1变量持有一个堆对象

        let o2 = o1; // o1堆对象所有权转移给o2，o1失效

        own_heap_obj(o2); // own_heap_obj获取了o2指向的堆对象的所有权，o2失效，堆对象由own_heap_obj销毁

        let o3 = create_heap_obj(); // o3获取了一个新的堆对象的所有权
        let o4 = &o3; // o4只是引用o3的堆对象
        use_heap_obj(&o3); // use_heap_obj引用了o3的堆对象

        let o5 = o3; // o3的堆对象所有权转移到o5，o3失效，o3的引用o4失效


    } // 作用域结束，i1、i2失效，o5失效，o5持有的堆对象释放

}

fn create_heap_obj() -> String {
    let s = String::from("hello"); // s指向一个堆对象
    return s; // 在方法结束的时候，s失效，将堆对象return
}

fn own_heap_obj(obj: String) { // 这个方法中的参数obj持有了堆对象
    println!("own_heap_obj owned:{}", obj);   
} // 方法结束后，obj失效，堆对象销毁

fn use_heap_obj(obj: &String) { // 这个方法中的参数obj只是引用堆对象
    println!("use_heap_obj used:{}", obj);   
} // 方法结束后，obj失效

