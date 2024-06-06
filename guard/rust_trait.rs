
#[allow(unused)]
pub fn run() {

    // trait 特性，是一种比interface强大的抽象

    // 定义一个形状的trait
    trait Shape {
        fn area(&self) -> f64; // 形状trait需要实现方法area以获取面积
    }

    // 定义一个具体形状
    struct Square {
        width: f64,
        height: f64,
    }
    impl Shape for Square {
        fn area(&self) -> f64 { // 实现trait方法
            self.width * self.height
        }
    }

    // 定义另一个具体形状
    struct Circle {
        radius: f64,
    }
    impl Shape for Circle {
        fn area(&self) -> f64 { // 实现trait方法
            std::f64::consts::PI * self.radius * self.radius
        }
    }

    // fn show_area(shap: dyn Shape) {
    //     let area = shap::area();
    //     println("形状面积：{}", area)
    // }

    // let s1 = Square { width: 1.0, height: 2.0 };
    // let c1 = Circle { radius: 5.0 };

    // show_area(s1);
    // show_area(c1);










}
