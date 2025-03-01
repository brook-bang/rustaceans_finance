type Price = f64;
type Volume = u32;
type Date = String;

struct StockData {
    symbol: String,
    date: Date,
    price: Price,
    volume: Volume,
}

struct BondData {
    name: String,
    date: Date,
    price: Price,
}

fn main() {
    let apple_stock = StockData {
        symbol: String::from("AAPL"),
        date: String::from("2023-09-13"),
        price: 150.0,
        volume: 10000,
    };

    let us_treasury_bond = BondData {
        name: String::from("US Treasury Bond"),
        date: String::from("2023-09-13"),
        price: 1000.0,
    };

    println!("Stock Data:");
    println!("Symbol: {}", apple_stock.symbol);
    println!("Date: {}", apple_stock.date);
    println!("Price: ${}", apple_stock.price);
    println!("Volume: {}", apple_stock.volume);
    println!("");
    println!("Bond Data:");
    println!("Name: {}", us_treasury_bond.name);
    println!("Date: {}", us_treasury_bond.date);
    println!("Price: ${}", us_treasury_bond.price);


}
