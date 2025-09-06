#![allow(dead_code)] //允许未使用的代码  注意这里全局有效的情况下,需要加!
#![allow(unused_variables)] //允许未使用的变量

//下面的属性可以屏蔽警告,需要放到指定代码前面
// #[allow(dead_code)] //允许未使用的代码 注意这里全局有效的情况下,需要加!
// #[warn(dead_code)] //对未使用的代码发出警告
// #[deny(dead_code)] //将未使用的代码视为错误
// #[allow(unused_variables)] //允许未使用的变量
// #[allow(unused_imports)] //允许未使用的导入
// #[allow(unused_mut)] //允许未使用的可变变量
// #[allow(unused_assignments)] //允许未使用的赋值
// #[allow(unused_must_use)] //允许未使用的必须使用的表达式
// #[allow(unused_parens)] //允许未使用的括号
// #[allow(unused_braces)] //允许未使用的花括号
// #[allow(unused_brackets)] //允许未使用的方括号
// #[allow(unused_macros)] //允许未使用的宏
// #[allow(unused_imports)] //允许未使用的导入
// #[allow(unused_labels)] //允许未使用的标签
// #[allow(unused_lifetimes)] //允许未使用的生活周期
// #[allow(unused_mut)] //允许未使用的可变变量
// #[allow(unused_parens)] //允许未使用的括号
// #[allow(unused_references)] //允许未使用的引用
// #[allow(unused_results)] //允许未使用的结果

fn main() {
    println!("Hello, world!");

    test_string_and_str();
    
    
}
/*作用域测试 */
fn test_scope(){
    {
        let s5 = String::from("this is s5");
        println!("s5 is {}",s5);
    }
    //println!("s5 is {}",s5);//这里s5已经不在作用域内,所以不能再次使用
}

/*
引用变量的可变性和引用数据的可变性
*/
fn test_reference_variable_mutability(){
    {
        let mut s1 = String::from("this is s1");
        let s2 = &mut s1;
        s2.push_str(" and s2");//s2是可变引用,可以修改s1 不需要let mut s2
        // println!("s1 is {}",s1);//注意这里不行,因为存在可变引用s2,s1不能再次被使用
        println!("s2 is {}",s2);
        println!("s1 is {}",s1);
    }

    //mut修饰引用时的作用
    {
        let mut s = String::from("hello");
        let mut s2 = String::from("world");
        let mut r1 = &mut s; // r1 可以重新绑定，并且可以通过r1修改指向的数据
        r1.push_str(" first"); // 修改s
        r1 = &mut s2; // 允许，因为r1是可变绑定，可以重新指向s2
        r1.push_str(" second"); // 现在修改的是s2
    }

    {
        let s= String::from("hello");
        let s2 = String::from("world");
        let mut r1 = &s; // r1 可以重新绑定，并且可以通过r1修改指向的数据
        println!("r1 is {}",r1);
        r1 = &s2;
        println!("r1 is {}",r1);
    }

}

//字符串赋值,会发生移动
fn test_string() {
    let s1 = String::from("hello");//s1是堆上的字符串
    let s2 = s1; //String 不是基本类型，而且是存储在堆上的，因此不能自动拷贝  这里发生了移动操作
    //println!("s1 is {}",s1);//这里s1的所有权已经转移给s2，所以s1不能再次使用
    //s1 = String::from("world");//这里s1也不能再次使用了
    println!("s2 is {}",s2);

    //克隆字符串，会创建一个新的字符串，并复制其内容
    let s3 = s2.clone(); //深拷贝
    println!("s3 is {}",s3);
    println!("s2 is {}",s2);//s2依然存在,可以再次使用
}
//string使用引用
/*
引用的规则:
在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
引用必须总是有效的(编译器保证,不会发生引用还在但是引用所指的值已经不存在了,悬垂引用)
不能同时存在可变引用和原始变量的使用  防止数据竞争,保证线程安全
*/
fn test_string_reference() {
    let s1 = String::from("hello");

    //不可变引用允许你读取数据但不能修改它。同时，在同一个作用域内，一个数据可以同时有多个不可变引用
    let s2 = &s1;
    let s3 = &s1;
    println!("s1 is {},s2 is {},s3 is {}",s1,s2,s3);

    //可变引用允许你修改数据。但是，在同一个作用域内，一个数据只能有一个可变引用
    let mut sa =  String::from("world");
    let sb = &mut sa;

    //这里有问题,因为sb是可变引用,所以不能同时使用原始变量 sa
    // println!("sa is {}",sa);
    // println!("sb is {}",sb);

    //这里不会报错,因为sb是可变引用,所以可以同时使用原始变量 sa
    println!("sb is {}",sb);
    println!("sa is {}",sa);
}
/*
悬垂引用
*/
// fn test_string_reference_dangling() -> &String{  //返回引用,会发生悬垂引用
//     let s1 = String::from("hello");
//     &s1
// }//这里会发生悬垂引用,返回的引用在函数结束时会指向无效的内存
fn test_string_reference_dangling_2()->String{  //正确的做法,直接返回string,发生了移动操作
    let s1 = String::from("hello");
    s1
}

/*测试函数参数以及返回值的所有权 所有权转移
注意,基本类型不会发生所有权转移,会直接进行copy的操作(栈上数据,不牵扯堆)
*/
fn todo(){
    let s1 = String::from("hello");
    test_string_reference_ownership_transfer(s1);//所有权已经给了test_string_reference_ownership_transfer函数
    // println!("s1 is {}",s1); //错误

    let s2 = String::from("world");
    let s2 = test_string_reference_ownership_transfer_2(s2);//发生了2次转移,一次s2给函数,一次函数返回来
    println!("s2 is {}",s2);

    let s3 = String::from("this is s3");
    test_string_reference_ownership_transfer_3(&s3); //对原来是可变,引用成不可变,  如果let mut s3允许操作,但是warning
    println!("s3 is {}",s3); //和s1比较 这里s3没有发生所有权转移,所以可以再次使用

    let mut s4 = String::from("this is s4");
    test_string_reference_ownership_transfer_4(&mut s4); //可变引用,要求原本值就是可变变量
    println!("s4 is {}",s4); //和s3比较 这s4可被更改
}
fn test_string_reference_ownership_transfer(s:String){ //s:String 发生了所有权转移
    println!("s is {}",s);
}
fn test_string_reference_ownership_transfer_2(s:String)->String{ //return也有所有权的转移
    println!("s is {}",s);
    s
}
fn test_string_reference_ownership_transfer_3(s:&String){ //不可变引用
    println!("s is {}",s);
}
fn test_string_reference_ownership_transfer_4(s:&mut String){ //可变引用
    s.push_str(" and s4");
}

/*
String和str的区别和联系
String 是一个可增长的、可变的、拥有所有权的 UTF-8 编码字符串类型
堆分配：String 的数据存储在堆上
可变性：可以修改内容（添加、删除、替换字符）
所有权：拥有其内容的所有权
大小可变：可以根据需要增长或缩小

str 是一个固定大小的、不可变的、UTF-8 编码字符串类型
栈分配：str 的数据存储在栈上
不可变性：不能修改内容
所有权：借用（非拥有）
大小固定：在编译时确定

联系：
str 是 String 的不可变引用类型
str 通常用于字符串切片
String 可以包含动态增长的内容
*/
fn test_string_and_str(){
    //&str -> String
    let s1 = String::from("hello"); //&str -> String
    let s2 = "world".to_string(); 
    
    //String -> &str
    say_hello(&s1);
    say_hello(&s1[..]);
    say_hello(s2.as_str());
}
fn say_hello(s:&str){
    println!("hello, {}",s);
}

/*  hh
dfasdfdasf
*/

