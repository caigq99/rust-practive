fn main() {
    // 整数类型

    //   长度     有符号   无符号
    //   8位       i8	   u8
    //  16位	  i16	   u16
    //  32位	  i32	   u32
    //  64位	  i64	   u64
    // 128位      i128     u128
    // 视架构而定  isize    usize

    // 数字字面量
    // 十进制          98_222
    // 十六进制        0xff
    // 八进制         0o77
    // 二进制         0b1111_0000
    // 字节（仅限于u8）b'A'

    // 浮点类型
    //  f64 双精度浮点数
    // let a: f64 = 0.1;
    // let b: f64 = 0.2;
    // f32 单精度浮点数
    // let c: f32 = 0.1;
    // let d: f32 = 0.2;
    // 注意：
    // assert!(a + b == 0.3); // 双精度浮点数相加  会panic
    // assert!(c + d == 0.3);  //  不会panic

    // practive  验证上方assert
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);
    println!("abc   f32");
    println!("0.1 + 0.2 = {:x}", (abc.0 + abc.1).to_bits());
    println!("      0.3 = {:x}", (abc.2).to_bits());

    println!("xyz   f64");
    println!("0.1 + 0.2 = {:x}", (xyz.0 + xyz.1).to_bits());
    println!("      0.3 = {:x}", (xyz.2).to_bits());

    // NaN
    let x: f32 = (-40.1_f32).sqrt();
    println!("x = {}", x);

    if x.is_nan() {
        println!("未定义的数学行为");
    }

    // 字符
    let str1: char = 'c';
    let str2: char = '国';
    let heart_eyed_cat: char = '😻';

    println!(
        "字符‘c’占用了{}字节的内存大小",
        std::mem::size_of_val(&str1)
    );
    println!(
        "字符‘国’占用了{}字节的内存大小",
        std::mem::size_of_val(&str2)
    );
    println!(
        "字符‘😻’占用了{}字节的内存大小",
        std::mem::size_of_val(&heart_eyed_cat)
    );

    //  boolean
    let _bool: bool = false; // true or false

    // 单元类型 ()
}

//  函数 语句和表达式 (语句会执行一些操作，但是并不会返回值，而表达式会在求值之后返回一个值)
fn add_with_extra(x: i32, y: i32) -> i32 {
    let x = x + 1; // 语句
    let y = y + 1; // 语句
    x + y //  表达式
}
