#[derive(Debug)]
enum PokerSuit {
    Clubs,
    Spades,
    Diamonds,
    Hearts,
}
// 带有类型的枚举
#[derive(Debug)]
enum PokerCard {
    Clubs(u8),
    Spades(u8),
    Diamonds(u8),
    Hearts(u8),
}

// Option 枚举
// struct Options<T> {
//     Some(T),
//     None,
// }
fn main() {
    let heart = PokerSuit::Hearts;
    let diamond = PokerSuit::Diamonds;
    print_suit(heart);
    print_suit(diamond);

    let c1 = PokerCard::Clubs(8);
    let c2 = PokerCard::Clubs(10);

    println!("{:?}", c1);
}

fn print_suit(suit: PokerSuit) {
    println!("{:?}", suit);
}
