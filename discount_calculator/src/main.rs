use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    println!("折现计算器");
    print!("请输入本金额：");
    io::stdout().flush().expect("刷新失败");
    io::stdin().read_line(&mut input).expect("读取失败");
    let principal: f64 = input.trim().parse().expect("无效输入");
    input.clear();

    println!("请输入折现率(以小数形式):");
    io::stdin().read_line(&mut input).expect("读取失败");
    let discount_rate: f64 = input.trim().parse().expect("无效输入");

    input.clear();

    print!("请输入时间期限(以年为单位):");
    io::stdout().flush().expect("刷新失败");
    io::stdin().read_line(&mut input).expect("读取失败");
    let time_period: u32 = input.trim().parse().expect("无效输入");

    let result = calculate_present_value(principal, discount_rate, time_period);
    println!("现值为：{:.2}",result);
}

fn calculate_present_value(principal: f64,discount_rate: f64,time_period: u32) -> f64 {
    if discount_rate < 0.0 {
        eprint!("\n错误: 折现率不能为负数!");
        eprintln!("\n请提供有效的折现率");
        std::process::exit(1);
    }

    if time_period == 0 {
        eprint!("\n错误： 时间期限不能为零！");
        eprintln!("\n请提供有效的时间期限。");
        std::process::exit(1);
    }

    principal / (1.0 + discount_rate).powi(time_period as i32)
}
