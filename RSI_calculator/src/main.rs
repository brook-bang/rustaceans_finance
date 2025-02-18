fn calculate_rsi(up_days: Vec<f64>,down_days: Vec<f64>) -> f64 {
    let up_sum = up_days.iter().sum::<f64>();
    let down_sum = down_days.iter().sum::<f64>();
    let rs = up_sum / down_sum;
    let rsi = 100.0 - (100.0 / (1.0 + rs));
    rsi
}

fn main() {
    let up_days = vec![2.0, 3.0, 3.0, 3.0, 2.0, 1.0, 1.0];
    let down_days = vec![2.0, 4.0, 5.0, 6.0, 4.0, 3.0, 3.0];
    let rsi = calculate_rsi(up_days, down_days);
    println!("RSI: {}", rsi);
}