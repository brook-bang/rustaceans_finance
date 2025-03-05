#[derive(Clone,Debug)]
struct Transaction {
    transaction_type: &'static str,
    amount: f64,
}

fn main() {
    let transactions = vec![
        Transaction {
            transaction_type: "Deposit",
            amount: 100.0,
        },
        Transaction {
            transaction_type: "Withdrawal",
            amount: 50.0,
        },
        Transaction {
            transaction_type: "Deposit",
            amount: 200.0,
        },
        Transaction {
            transaction_type: "Withdrawal",
            amount: 75.0,
        },
    ];

    let deposits: Vec<Transaction> = transactions
    .iter()
    .filter(|&transaction| transaction.transaction_type == "Deposit")
    .cloned()
    .collect();

    println!("Deposit Transacions:{:?}",deposits);



}
