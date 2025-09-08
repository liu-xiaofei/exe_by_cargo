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

    test_string_find();
    
    
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
    {//mut修饰引用,允许重新绑定
        let mut s = String::from("hello");
        let mut s2 = String::from("world");
        let mut r1 = &mut s; // r1 可以重新绑定，并且可以通过r1修改指向的数据
        r1.push_str(" first"); // 修改s
        r1 = &mut s2; // 允许，因为r1是可变绑定，可以重新指向s2
        r1.push_str(" second"); // 现在修改的是s2
    }

    {//mut修饰引用,允许重新绑定
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

    //可变引用允许你修改数据。但是，在同一个作用域内，一个数据只能有一个可变引用,也不能同时存在可变引用和不可变引用
    let mut sa =  String::from("world");
    let sb = &mut sa;//sb是sa的可变引用

    //这里有问题,因为sb是可变引用,所以不能同时使用原始变量 sa
    // println!("sa is {}",sa);//因为下面语句的存在,sb还在,这个时候只能用可变引用,所以报错
    // println!("sb is {}",sb);

    //这里不会报错,因为sb是可变引用,所以可以同时使用原始变量 sa
    println!("sb is {}",sb);//sb结束时,sa可以再次被使用
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
    // println!("s1 is {}",s1); //错误 这个一定要注意,所有权转移后,原始变量不能再次使用;所有权已经转移给了函数

    let s2 = String::from("world");
    let s2 = test_string_reference_ownership_transfer_2(s2);//发生了2次转移,一次s2给函数,一次函数返回来
    println!("s2 is {}",s2);

    //推荐使用引用作为函数的参数,这样不会发生所有权转移
    let s3 = String::from("this is s3");
    test_string_reference_ownership_transfer_3(&s3); //对原来是可变,引用成不可变
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

1."hello"是一个字符串字面量，它存储在程序的只读数据段（通常称为文本段）中。在运行时，字符串字面量通常作为指向这段只读内存的指针（&str）出现。
2.String::from是Rust标准库中的一个函数，它接受一个字符串切片（&str）作为参数，并返回一个String。String是一个可增长、可变、拥有所有权的UTF-8编码字符串类型。
3.String::from会为字符串内容在堆上分配一块内存。由于String拥有其内容，所以它需要将字符串字面量的内容复制到堆上分配的内存中。这样，String就拥有了这份数据的所有权，并且可以自由地修改它（比如追加、删除字符等），而不会影响原始的字符串字面量。

*/
fn test_string_and_str(){
    /*
    实际上发生:
    1.在堆上分配足够的内存来存储字符串内容（这里是"hello"），包括空字符（Rust的字符串不以空字符结尾，但会存储长度和容量）。
    2.将字符串字面量"hello"的内容（字节序列）复制到新分配的堆内存中。
    3.记录该字符串的长度（当前为5）和容量（至少为5，但可能根据分配策略略有不同）。
    4.返回一个String结构体的实例，该实例包含一个指向堆上数据的指针、长度和容量
    5.最后，这个String实例被绑定到变量s1。当s1离开作用域时，String的析构函数（Drop trait）会被调用，释放堆上的内存。
     */
    let s1 = String::from("hello"); //&str -> String
    println!("s1 = {}", s1);
    println!("长度: {}", s1.len());
    println!("容量: {}", s1.capacity());
    println!("内存地址: {:p}", s1.as_ptr());

    let s2 = "world".to_string(); 
    println!("s2 = {}", s2);
    println!("长度: {}", s2.len());
    println!("容量: {}", s2.capacity());
    println!("内存地址: {:p}", s2.as_ptr());

    //String -> &str,这里发生了deref 隐式强制转换
    say_hello(&s1);
    say_hello(&s1[..]);
    say_hello(s2.as_str());
    
}

fn say_hello(s:&str){
    println!("hello, {}",s);
}
/*其它语言中，使用索引的方式访问字符串的某个字符或者子串是很正常的行为，但是在 Rust 中就会报错：
字符串的底层的数据存储格式实际上是[ u8 ]，一个字节数组。对于 let hello = String::from("Hola"); 
这行代码来说，Hola 的长度是 4 个字节，因为 "Hola" 中的每个字母在 UTF-8 编码中仅占用 1 个字节
但是对于let hello = String::from("中国人");
字符串实际上是 9 个字节的长度，因为大部分常用汉字在 UTF-8 中的长度是 3 个字节，因此这种情况下对 hello 进行索引，访问 &hello[0] 没有任何意义，因为你取不到 中 这个字符，而是取到了这个字符三个字节中的第一个字节，这是一个非常奇怪而且难以理解的返回值。

总结:
在Rust中，字符串String不能直接使用索引来访问字符或子串，这主要是由于两方面的原因：
字符串是UTF-8编码的，这意味着一个字符可能由多个字节组成。
索引操作预期是常数时间O(1)的，但是对于UTF-8字符串，无法保证常数时间，因为必须从头开始扫描才能确定第n个字符的位置。
*/
fn test_string_index_access(){
    let s = String::from("hello");
    
    // 以下代码会编译错误：
    // let c = s[0];
    // println!("第一个字符: {}", c);

}
//字符串切片也是危险操作
fn test_string_slice_access(){
    let hellos = "中国人";
    let s = &hellos[0..2]; //程序崩溃,因为中文utf-8编码是3个字节,这个切片是2个字节,所以会崩溃

    println!("slice = {}", s);
}

/*
string操作的一些方法  String类型实际上是Vec<u8>的封装，但保证其中的内容是有效的UTF-8编码。因此，许多操作与Vec类似。
1.获取信息
len() 返回字符串的字节长度
capacity() 返回字符串的容量（已分配的内存大小）
is_empty() 检查字符串是否为空

2.追加操作
push(char) 追加单个字符
push_str(&str) 追加字符串切片
insert(usize, &str) 在指定位置插入字符串切片

3.删除操作
remove(usize) 删除并返回指定位置的字符
replace(old, new) 替换字符串中的子字符串

4.交换操作
swap_remove(usize) 删除并返回指定位置的字符，并将其与最后一个字符交换

5.调整容量操作
shrink_to_fit() 将字符串的容量缩小到与字符串的长度相同
reserve(usize) 预留指定大小的内存
truncate(usize) 截断字符串，保留前面的字节
clear() 清空字符串

6.删除字符串
pop() 删除并返回字符串的最后一个字符
truncate(usize) 截断字符串，保留前面的字节
clear() 清空字符串   注意:clear() 只是清空字符串，不会释放内存，需要调用shrink_to_fit() 来释放内存

7.获取信息
as_ptr() 返回字符串底层数据的指针
as_bytes() 返回字符串底层数据的字节数组
as_str() 返回字符串切片

8.转换其他形式
to_string() 将字符串切片转换为String
to_owned() 将字符串切片转换为String
*/

fn test_string_creat(){
    // ===== 创建和初始化 =====
    println!("=== 创建和初始化 ===");
    {
        // 1. 创建空字符串
        let mut s1 = String::new();//s1容量为0?所以不能直接使用s1.push_str("Hello");
        println!("空字符串: '{}'", s1);//打印'空' 是因为s1是空字符串,所以打印'空' 注意这里不会报错
        println!("容量: {}", s1.capacity());//容量为0
        println!("长度: {}", s1.len());//长度为0
        println!("是否为空: {}", s1.is_empty());//true
        println!("底层数据的指针: {:p}", s1.as_ptr());//0x1  -->可能是rust的优化,所以这里没有实际地址
        println!("底层数据的字节数组: {:?}", s1.as_bytes());//打印[]
        println!("底层数据的切片: {}", s1.as_str());//这个什么都没打印
        s1.push_str("this is s1");
        println!("追加字符串后: '{}'", s1);
        println!("追加字符串后容量: {}", s1.capacity());//10
        println!("追加字符串后长度: {}", s1.len());//10
        println!("追加字符串后底层数据的指针: {:p}", s1.as_ptr());//0x1ffe1492840
        println!("追加字符串后底层数据的字节数组: {:?}", s1.as_bytes());//打印[116, 104, 105, 115, 32, 105, 115, 32, 115, 49]
        println!("追加字符串后底层数据的切片: {}", s1.as_str());

        // 2. 从字面量创建
        let s2 = String::from("this is s2");
        println!("从字面量创建: '{}'", s2);
        println!("从字面量创建容量: {}", s2.capacity());//10
        println!("从字面量创建长度: {}", s2.len());//10
        println!("从字面量创建是否为空: {}", s2.is_empty());//false
        println!("从字面量创建底层数据的指针: {:p}", s2.as_ptr());//0x1ffe1492940
        println!("从字面量创建底层数据的字节数组: {:?}", s2.as_bytes());//打印[116, 104, 105, 115, 32, 105, 115, 32, 115, 50]
        println!("从字面量创建底层数据的切片: {}", s2.as_str());

        // 3. 从字符串切片创建
        let s3 = "this is s3".to_string();
        println!("从字符串切片创建: '{}'", s3);
        println!("从字符串切片创建容量: {}", s3.capacity());//10
        println!("从字符串切片创建长度: {}", s3.len());//10
        println!("从字符串切片创建是否为空: {}", s3.is_empty());//false
        println!("从字符串切片创建底层数据的指针: {:p}", s3.as_ptr());//0x1ffe14928c0
        println!("从字符串切片创建底层数据的字节数组: {:?}", s3.as_bytes());//打印[116, 104, 105, 115, 32, 105, 115, 32, 115, 51]
        println!("从字符串切片创建底层数据的切片: {}", s3.as_str());

        // 4. 使用 with_capacity 预分配容量
        let mut s4 = String::with_capacity(10);
        println!("从字符串切片创建: '{}'", s4);
        println!("从字符串切片创建容量: {}", s4.capacity());//10
        println!("从字符串切片创建长度: {}", s4.len());//0
        println!("从字符串切片创建是否为空: {}", s4.is_empty());//true
        println!("从字符串切片创建底层数据的指针: {:p}", s4.as_ptr());//0x1ffe1492740
        println!("从字符串切片创建底层数据的字节数组: {:?}", s4.as_bytes());//打印[]
        println!("从字符串切片创建底层数据的切片: {}", s4.as_str());
        s4.push_str("this is s4");
        println!("预分配容量后: '{}'", s4);
        println!("预分配容量后容量: {}", s4.capacity());//10
        println!("预分配容量后长度: {}", s4.len());//10
        println!("预分配容量后是否为空: {}", s4.is_empty());//false
        println!("预分配容量后底层数据的指针: {:p}", s4.as_ptr());//0x1ffe1492740  和上面一致
        println!("预分配容量后底层数据的字节数组: {:?}", s4.as_bytes());//打印[116, 104, 105, 115, 32, 105, 115, 32, 115, 52]
        println!("预分配容量后底层数据的切片: {}", s4.as_str());
    }

}

fn test_string_change(){
    // ===== 修改操作 =====
    println!("=== 修改操作 ===");
    {
        let mut s1 = String::from("this is s1");
        println!("修改操作前: '{}'", s1);
        s1.push_str(" and chang");
        println!("追加字符串后: '{}'", s1);
        s1.push('e');
        println!("追加字符后: '{}'", s1);

        s1.insert_str(10, " hah");
        println!("插入字符串后: '{}'", s1);
        s1.insert(14, 'a');
        println!("插入字符后: '{}'", s1);

        //拼接操作
        let s3_1 = String::from("this is s3_1");
        let s3_2 = String::from("this is s3_2");
        let s3 = s3_1 + &s3_2; //注意这里s3_1的所有权已经转移给了s3;注意这里s3_1和s3_2都不是必须mut的
        println!("拼接操作后: '{}'", s3);
        //println!("s3_1 cont used", s3_1);//这里s3_1的所有权已经转移给了s3,所以s3_1不能再次使用
        println!("s3_2 is {}", s3_2);//这里s3_2的所有权没有转移,所以可以再次使用

        //使用fromat!宏拼接
        let s4_1 = String:: from("this is s4_1");
        let s4 = format!("this is {} and {}", "s4",s4_1);
        println!("format!宏拼接后: '{}'", s4);


        //替换和截断
        let mut s5 = String::from("this is s5");
        s5.replace_range(0..3, "abcdefg");//替换范围
        println!("替换操作后: '{}'", s5);

        s5.truncate(10);//保留10个字符  容量不变
        println!("截断操作后: '{}'", s5);
        println!("截断操作后容量: {}", s5.capacity());//20
        println!("截断操作后长度: {}", s5.len());//10
        s5.shrink_to_fit();//缩小容量到长度
        println!("缩小容量操作后: '{}'", s5);
        println!("缩小容量操作后容量: {}", s5.capacity());//10
        println!("缩小容量操作后长度: {}", s5.len());//10

        //清除
        s5.clear();//清除内容  容量不变
        println!("清除操作后: '{}'", s5);
        println!("清除操作后容量: {}", s5.capacity());//10
        println!("清除操作后长度: {}", s5.len());//0

    }
}

fn test_string_find(){
    // ===== 查找操作 =====
    println!("=== 查找操作 ===");
    {
        let s1 = String::from("this is s1");
        //迭代器
        println!("迭代器遍历char: ");
        for c in s1.chars(){//打印t h i s i s s 1
            println!("{}", c);
        }
        println!("迭代器遍历u8: ");
        for b in s1.bytes(){//打印116 104 105 115 32 105 115 32 115 49 是utf-8编码
            println!("{}", b);
        }
        println!("迭代器遍历分隔符: ");
        for w in s1.split_whitespace(){//打印this is s1
            println!("{}", w);
        }
    }

    //检索操作
    {
        let s2 = String::from("this is s2");
        let index = s2.find("s2");//find返回的是Option<usize> option是一个枚举值,后面枚举会说明
        // println!("查找操作后: '{}'", index);//打印10  注意这里Option<usize> 需要解包  不能直接打印  解包后可以打印
        if let Some(index) = index{
            println!("查找操作后: '{}'", index);
        }else{
            println!("查找操作后: '{}'", "没有找到");
        }

        let index_2: usize = s2.find("s3").unwrap_or(0);//unwrap_or(0) 也解包  如果解包失败,则返回0
        println!("s2.find(\"s3\").unwrap_or(0) 查找操作后: '{}'", index_2);


        let index = s2.rfind("s2");
        if let Some(index) = index{
            println!("查找操作后: '{}'", index);
        }else{
            println!("查找操作后: '{}'", "没有找到");
        }
    }

    // let s2 = String::from("this is s2");
    // let index = s2.find("s2");
    // println!("查找操作后: '{}'", index);

    // let index = s2.rfind("s2");
    // println!("查找操作后: '{}'", index);
}

//测试github远程仓库是否完全同步,还是只同步我手动远程的

