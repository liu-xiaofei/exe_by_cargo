#![allow(dead_code)] //允许未使用的代码
/*
库类型的 Package 只包含 src/lib.rs 文件
src/
├── lib.rs          ← 库包入口（固定名称）  是 Rust 库包的入口文件
├── main.rs         ← 二进制包入口（固定名称）
├── front_of_house.rs  ← 自定义模块文件
└── other_module.rs    ← 自定义模块文件

lib.rs作用:
定义库包的公共 API
作为库包的根模块
控制哪些模块和函数可以被外部使用

*/



mod front_of_house;//导入前厅模块

/*导出模块
将 front_of_house::hosting 模块重新导出到包的根级别
使得外部可以直接通过 exe_by_cargo::hosting 访问
*/
pub use crate::front_of_house::front_of_house::hosting;//注意,这里导入路径有2个front_of_house,一个是文件名,一个是模块名
pub use crate::front_of_house::hosting2;
// pub use crate::front_of_house::front_of_house::serving; 错误 提示serving是私有的数据
pub use crate::front_of_house::serving2;//导出服务模块

//lib.rs中必须有pub
pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::front_of_house::hosting::add_to_waitlist();

    //self 引用自身模块中的项
    self::front_of_house::front_of_house::hosting::add_to_waitlist();

    //测试模块私有但是函数pub的情况--->报错
    // front_of_house::front_of_house::serving::take_order();  提示serving是私有的数据

}






