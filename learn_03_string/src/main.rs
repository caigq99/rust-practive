fn main() {
    // 此时为 &str 类型
    let hello = "hello, world";
    // 此方法接收一个 String 类型的值  所以会报错
    // print_hello(hello);
    // to_string 方法会将 &str 类型的字符串切片转换成 String 类型的字符串
    print_hello(hello.to_string());

    //  切片
    let s = String::from("hello world");

    // .. range语法
    let hello = &s[0..5];
    let world = &s[6..11];
    let all = &s[0..11];
    println!("h1 : {}, w1 : {}, all1 : {}", hello, world, all);
    // 简写
    let hello = &s[..5];
    let world = &s[6..];
    let all = &s[..];
    println!("h1 : {}, w1 : {}, all1 : {}", hello, world, all);

    // 注意：在对字符串使用切片语法时需小心，切片的索引必须落在字符之间的边界位置，也就是UTF-8字符的边界，否则代码会崩溃
    // 例 ： 中文一般在UTF-8中占用三个字节
    // let s = "中国";
    // 此时程序会panic
    // let a = &s[0..2];
    // println!("a = {}", a);

    // 其他切片 因为切片是对集合的部分引用，因此不仅仅是字符串有切片，其他集合类型也有，例如数组：
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[0..2];
    println!("slice : {:?}", slice);

    //  String 与 &str 之间的转换 (这种灵活用法是因为 deref 隐式转换， 通过 Deref 特征实现)
    let string = String::from("hello,world");
    say_hello(&string);
    say_hello(&string[0..5]);
    say_hello(string.as_str());

    // 操作字符串
    // 1、push  追加字符   push_str  追加字符串字面量（字符串切片）
    // 2、insert  插入字符   insert_str  插入字符串字面量（字符串切片）
    // 3、replace  替换  适用于 String 和 &str 类型   返回新的字符串
    // 4、replacen  替换  适用于 String 和 &str 类型  返回新的字符串  比replace多一个参数  用于设置替换的个数
    // 4、replace_range  替换  仅适用于 String  直接操作原来的字符串

    // 删除  （仅适用于String）直接操作原来的字符串
    // 1、pop  删除并返回字符串的最后一个字符
    // 2、remove  删除并返回指定位置的字符
    // 3、truncate  删除指定位置字符之后的全部字符
    // 4、clear 清空字符串

    // 使用 +/+= 连接字符串

    let str_append = String::from("hello ");
    let str_rust = String::from("rust");
    // 此时会报错，  因为 + 后面需要跟 &str 类型
    // let res = str_append + str_rust;
    // 现在的 + 后面是 &String 类型， 但是在 + 号执行时会将 &String 隐式转换成 &str 类型， 背后其实就是调用了 std::string下面的add()方法
    let res = str_append + &str_rust;
    println!("res = {}", res);
    let mut result = res + "!";
    result += "!!!";
    println!("result = {}", result);
}

fn print_hello(str: String) {
    println!("{}", str);
}

fn say_hello(str: &str) {
    println!("say: {}", str);
}
