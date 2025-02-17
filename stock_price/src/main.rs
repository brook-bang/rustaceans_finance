use std::fmt::Display;

struct StockPrice{
    symbol: String,
    price: f64,
}

impl Display for StockPrice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"股票：{} - 价格： {:.2}",self.symbol,self.price)
    }
}

fn main() {
    let price = StockPrice {
        symbol: "AAPL".to_string(),
        price: 150.25,
    };
    println!("[INFO]:{}",price);
}
