fn main() {
    let stock_prices = [50.0, 52.0, 55.0, 60.0, 58.0, 62.0, 65.0, 70.0, 75.0, 80.0];
    let window_size = 5;
    let mut sma_values: Vec<f64> = Vec::new();

    for i in 0..stock_prices.len() -window_size + 1 {
        let window = &stock_prices[i..i+window_size];
        let sum: f64 = window.iter().sum();
        let sma = sum / window_size as f64;
        sma_values.push(sma);
    }

    println!("简单移动平均线(SMA):");
    for (i,sma) in sma_values.iter().enumerate() {
        println!("Day {}: {:.2}",i + window_size,sma);
    }
}