fn main() {
    
    let stock_prices = vec![100.0,105.0,110.0,115.0,120.0];

    let log_returns: Vec<f64> = stock_prices.iter().map(|&price|price / 100.0f64.ln()).collect();

    println!("{:?}",log_returns);

}
