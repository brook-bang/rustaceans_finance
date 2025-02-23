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
    stock_prices.insert(String::from("MSFT"), stock3);

    if let Some(price) = stock_prices.get("AAPL") {
        println!("The price of AAPL is ${}",price.price);
    } else {
        println!("AAPL not found in the stock prices");
    }

    for (symbol,price) in &stock_prices {
        println!("{}:${}",symbol,price.price);
    }
    






}
