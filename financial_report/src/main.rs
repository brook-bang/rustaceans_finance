#[derive(Debug)]
struct FinancialReport {
    income: f64,
    expenses: f64,
}

fn main() {
    let report = FinancialReport {
        income: 10000.0,
        expenses: 7500.0,
    };

    println!("金融报告:\nincome: {:.2}\nexpenses: {:.2}",report.income,report.expenses);
    println!("{:?}",report);
}