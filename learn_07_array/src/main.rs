use std::array;
fn main() {
    // array 是存储在栈上的，这里的 array 不是 vector
    // 数组三要素：
    //  1、长度固定
    //  2、所有元素类型必须相同
    //  3、依次线性排列
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("first = {}, second = {}", first, second);
    // 创建元素不为基础类型的数组
    // 这个例子会报错， 因为之前是使用的基础数据类型， 他们都实现了 Copy 这个Trait，而 String 并没有实现，所以这种生成方式的底层就是利用的 Copy
    // let arr = [String::from("nihao"); 5];
    // 正确的创建
    let arr: [String; 8] = array::from_fn(|_| String::from("nihao"));
    println!("add = {:#?}", arr);

    // 总结
    // 数组类型容易跟数组切片混淆，[T;n]描述了一个数组的类型，而[T]描述了切片的类型， 因为切片是运行期的数据结构，它的长度无法在编译期得知，因此不能用[T;n]的形式去描述
    // [u8; 3]和[u8; 4]是不同的类型，数组的长度也是类型的一部分
    // 在实际开发中，使用最多的是数组切片[T]，我们往往通过引用的方式去使用&[T]，因为后者有固定的类型大小
}
