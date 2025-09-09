/*
在这创建餐厅前厅模块
包含招待客人和服务两个子模块
使用 mod 关键字来创建新模块，后面紧跟着模块名称
模块可以嵌套，这里嵌套的原因是招待客人和服务都发生在前厅，因此我们的代码模拟了真实场景
模块中可以定义各种 Rust 类型，例如函数、结构体、枚举、特征等
所有模块均定义在同一个文件中

模块树
crate(包,包含二进制可执行文件和库)
 └── front_of_house (前厅) ->父模块
     ├── hosting(招待客人)
     │   ├── add_to_waitlist(添加到等待列表)
     │   └── seat_at_table(安排座位)
     └── serving(服务)->子模块
         ├── take_order(点菜) 
         ├── serve_order(上菜)
         └── take_payment(结账)


父子模块:
模块 A 包含模块 B，那么 A 是 B 的父模块，B 是 A 的子模块。在上例中，front_of_house 是 hosting 和 serving 的父模块，反之，后两者是前者的子模块

如何使用模块:
使用 use 关键字引入模块，后面紧跟着模块名称
使用 :: 操作符访问模块中的项
例如:
use front_of_house::hosting;
hosting::add_to_waitlist();
hosting::seat_at_table();
serving::take_order();
serving::serve_order();
serving::take_payment();

用路径引用模块
绝对路径: 从包根开始，路径名以包名或者 crate 作为开头
相对路径: 从当前模块开始，以 self，super 或当前模块的标识符作为开头
*/
pub mod front_of_house {//前厅
    pub mod hosting {//招待客人
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        } //添加到等待列表

        fn seat_at_table() { //私有函数
            println!("安排座位");
        } //安排座位
    }

    mod serving {//服务 私有模块
        pub fn take_order() {
            println!("点菜");
        } //点菜

        pub fn serve_order() {
            println!("上菜");
        } //上菜

        pub fn take_payment() {
            println!("结账");
        } //结账
    }
}


// 直接定义模块，不要嵌套 front_of_house
pub mod hosting2 {//招待客人
    pub fn add_to_waitlist() {
        println!("添加到等待列表");
    }

    pub fn seat_at_table() {  // ← 添加 pub
        println!("安排座位");
    }
}

pub mod serving2 {//服务  ← 添加 pub
    pub fn take_order() {  // ← 添加 pub
        println!("点菜");
    }

    pub fn serve_order() {  // ← 添加 pub
        println!("上菜");
    }

    pub fn take_payment() {  // ← 添加 pub
        println!("结账");
    }
}



