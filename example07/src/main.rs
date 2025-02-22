struct Stock {
    symbol: String,
    price: f64,
    quantity: u32,
}

fn main() {

    let apple_stock = Stock {
        symbol: String::from("APPL"),
        price: 150.50,
        quantity: 1000,
    };

    println!("股票代码：{}",apple_stock.symbol);
    println!("股票价格：${:?}", apple_stock.price);
    println!("股票数量：{}",apple_stock.quantity);

    let total_value = apple_stock.price * apple_stock.quantity as f64;
    println!("总价值：${:.2}",total_value);

}