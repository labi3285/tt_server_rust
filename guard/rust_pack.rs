
#[allow(unused)]
pub fn run() {
        
    // 【Package】包
    // 指的是.toml文件管理的一个项目

    // 【Crate】箱
    // 二进制文件，可执行或者库

    // 【Module】模块
    // 1、rust组织代码的最小单元，比如一个文件就是一个模块
    // 2、文件中还可以定义mod（子mod），在下面
    nation::government::govern() // 调用
    
}

#[allow(unused)]
pub mod nation { // 这个mod是普遍，因此可以通过_pack::nation在外面调用
    pub mod government { // 共有模块，外部使用需要共有
        pub fn govern() {}
    }
    mod congress { // 私有模块
        pub fn legislate() {}
    }
    mod court {
        fn judicial() {
            super::congress::legislate(); // 模块内可以通过supper追溯，同一级mod可以访问
        }
    }
}