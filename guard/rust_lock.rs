
#[allow(unused)]
pub fn run() {

    println!("xxxxxx");


    // RwLock 读取-写入锁
    {
        let lock = std::sync::RwLock::new(5); // 创建锁
        let r = lock.read().unwrap(); // 可多线程读取
        {
            let mut w = lock.write().unwrap();

            *w = 6;
            *w = 7;
        }
        println!("r:{}", r);
    }






}
