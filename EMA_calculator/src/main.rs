fn main() {
    let stock_prices = [50.0, 52.0, 55.0, 60.0, 58.0, 62.0, 65.0, 70.0, 75.0, 80.0];

    let window_size = 5;
    let mut ema_values: Vec<f64> = Vec::new();

    let alpha = 2.0 / (window_size as f64 + 1.0);
    let mut ema = stock_prices[0];

    for price in &stock_prices {
        ema = (price - ema) * alpha + ema;
        ema_values.push(ema);
    }

    println!("指数移动平均线(EMA):");
    for (i,ema) in ema_values.iter().enumerate() {
        println!("Day {}: {:.2}", i + 1,ema);
    }

}
