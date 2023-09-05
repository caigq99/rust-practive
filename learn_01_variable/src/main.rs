fn main() {
    // 此处默认为不可变变量
    // let x = 5;
    // println!("The value of x is : {}", x);
    // 此处第二次进行赋值  报错
    // x = 6;
    // println!("The value of x is : {}", x);

    // let mut x = 5;
    // println!("The value of x is : {}", x);
    // x = 6;
    // println!("The value of x is : {}", x);
    // 此处如果不使用的话，编译时会提醒可以在变量前面加上_  这样编译器就会忽略该变量
    // let y = 10;
    // let _y = 10;

    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);

    println!("a = {:?}, b = {:?}", a, b);
    b = !b;
    assert_eq!(a, b);

    // 解构式赋值

    struct Struct {
        _e: i32,
    }

    let (_a, _b, _c, _d, _e);

    (_a, _b) = (1, 2);
    [_c, .., _d, _] = [1, 2, 3, 4, 5];

    Struct { _e, .. } = Struct { _e: 5 };

    assert_eq!([1, 2, 1, 4, 5], [_a, _b, _c, _d, _e]);
}
