fn main() {
    let tup: (i32, f32, u8) = (500, 6.8, 1);
    println!("tup = {:?}", tup);

    // 通过模式匹配解构

    let (x, y, z) = tup;

    println!("x : {}, y : {}, z : {}", x, y, z);

    // 使用 . 访问元组

    let one = tup.0;
    let two = tup.1;
    let three = tup.2;
    println!("one : {}, two : {}, three : {}", one, two, three);
}
