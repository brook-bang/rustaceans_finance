#[derive(Debug,PartialEq)]
struct Trade {
    price: f64,
    volume: i32,
}

fn main() {
    let trades = vec![
        Trade {price: 10.0,volume: 100},
        Trade {price: 20.0,volume: 200},
        Trade {price:30.0,volume: 300},
    ];

    let threshold = 25.0;

    let mut filtered_trades = trades.iter().filter(|trade|trade.price > threshold);

    match filtered_trades.next() {
        Some(&Trade {price: 30.0,volume:300}) => println!("第一个交易正确"),
        _ => println!("第一个交易不正确"),
    }

    match filtered_trades.next() {
        None => println!("没有更多的交易"),
        _ => println!("还有更多的交易"),
    }
}