#[derive(Debug)]
struct ConversionError;

struct StockPrice {
    price: f64,
}

impl TryFrom<StockPrice> for f64 {
    type Error = ConversionError; 

    fn try_from(stock_price: StockPrice) -> Result<Self, Self::Error> {
        if stock_price.price > 0.0 {
            Ok(stock_price.price.ln())
        } else {
            Err(ConversionError)
        }
    }
}

fn main() {
    let valid_price = StockPrice { price: 50.0 };
    let result: Result<f64, ConversionError> = valid_price.try_into();
    println!("{:?}", result);

    let invalid_price = StockPrice { price: -10.0 };
    let result: Result<f64, ConversionError> = invalid_price.try_into();
    println!("{:?}", result);
}
