
#[allow(unused)]
pub fn run() {


    // 智能指针，用于包裹值，有各种指针，智能指针在失效的时候如指向的是堆对象，他会自动销毁

    // let obj1 = create_heap_obj();

    // [Box] 在堆上分配一个空间存放
    let b1 = Box::new(3);
    println!("b1:{}", b1);

    // [Rc] 采用引用计数可实现多个变量指向某值
    let r1 = std::rc::Rc::new(3); // r1是Rc包裹值3，3的引用计数为1
    let r2 = std::rc::Rc::clone(&r1); // 这里r2是3的引用，3的引用计数+1
    let r3 = std::rc::Rc::downgrade(&r1); // 这里r3是3的弱引用，3的引用计数不变
    println!("Reference count: {}", std::rc::Rc::strong_count(&r1));





} // b1销毁

