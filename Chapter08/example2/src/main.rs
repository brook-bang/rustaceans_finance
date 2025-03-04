use std::str::FromStr;

struct StockPrice {
    ticker_symbol: String,
    price: f64,
}

impl ToString for StockPrice {
    fn to_string(&self) -> String {
        format!("{}:{}", self.ticker_symbol, self.price)
    }
}

impl FromStr for StockPrice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(":").collect();
        if components.len() != 2 {
            return Err(());
        }

        let ticker_symbol = String::from(components[0]);

        let price = components[1].parse::<f64>().unwrap();

        Ok(StockPrice {
            ticker_symbol,
            price,
        })
    }
}

fn main() {
    let price_string = "APPL:150.64";
    let stock_price = StockPrice::from_str(price_string).unwrap();
    println!("Ticker Symbol: {}", stock_price.ticker_symbol);
    println!("Price: {}", stock_price.price);
    let price_string_again = stock_price.to_string();
    println!("Price String:{}", price_string_again);
}
