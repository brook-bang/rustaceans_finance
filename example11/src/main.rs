use std::collections::HashMap;

#[derive(Debug)]
struct StockPrice {
    symbol: String,
    price: f64
}

fn main() {
    let mut stock_prices: HashMap<String,StockPrice> = HashMap::new();
    let stock1 = StockPrice {
        symbol: String::from("AAPL"),
        price: 150.0,
    };
    stock_prices.insert(String::from("AAPL"), stock1);

    let stock2 = StockPrice {
        symbol: String::from("GooGL"),
        price: 2800.0,
    };
    stock_prices.insert(String::from("GOOGL"), stock2);

    let stock3 = StockPrice {
        symbol: String::from("MSFT"),
        price: 300.0,
    };
    






}
