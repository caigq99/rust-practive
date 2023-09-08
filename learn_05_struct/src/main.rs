#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
//  结构体User的关联函数
impl User {
    fn new(username: String, email: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

//  元组结构体 (希望有一个整体名称，但是又不关心里面字段的名称时就可以使用元组结构体)
struct Point(i32, i32, i32);

// 单元结构体 (定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体)
struct _Test;

#[derive(Debug)]
struct Rectangle {
    _width: u32,
    _height: u32,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("cgq"),
        email: String::from("123456789@example.com"),
        sign_in_count: 1,
    };
    println!("username is {}", user1.username);
    let mut user2 = User {
        active: user1.active,
        username: String::from("CC"),
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };
    println!("user2 is {:#?}", user2);
    // 在user2初始化时 user1.email 的所有权移动到user2里面了,导致user1不能使用， 所以这里会报错
    // println!("user1 is {:?}", user1);
    // 虽然 user1.email 的所有权移动到 user2里面，但是 其他的值依旧可以使用
    println!("user1.active is {}", user1.username);

    // 在下面这段代码中， 可以说明在对struct里面拥有所有权的数据进行更改实在同一块内存地址上修改数据的，并不是重新分配内存地址
    let a = &user2.email;
    println!("a 的引用地址 : {:p}", a);
    user2.email = String::from("test@qq.com");
    let b = &user2.email;
    println!("b 的引用地址 : {:p}", b);

    // 使用结构体的关联函数初始化User
    let username = String::from("cgq");
    let email = String::from("12312@qq.com");
    let user3 = User::new(username, email);
    println!("user3 is {:#?}", user3);

    // 初始化元组结构体
    let point = Point(10, 20, 30);

    println!("{}", point.0);

    let scale = 2;
    let rect1 = Rectangle {
        _width: dbg!(30 * scale),
        _height: 50,
    };
    dbg!(&rect1);
}
