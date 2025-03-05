struct Transaction {
    transaction_type: &'static str,
    amount: f64,
}

fn main() {
    let mut account_balance = 0.0;

    let transactions = vec![
        Some(Transaction {
            transaction_type: "Deposit",
            amount: 100.0,
        }),
        Some(Transaction {
            transaction_type: "Withdrawal",
            amount: 50.0,
        }),
        Some(Transaction {
            transaction_type: "Deposit",
            amount: 200.0,
        }),
        None,
    ];

    for transaction in transactions {
        if let Some(tx) = transaction {
            match tx.transaction_type {
                "Deposit" => {
                    account_balance += tx.amount;
                    println!("Deposited ${:.2}",tx.amount);
                }
                "Withdrawal" => {
                    account_balance -= tx.amount;
                    println!("Withdrawn ${:.2}",tx.amount);
                }
                _ => println!("Invalid transaction type"),
            }
        } else {
            break;
        }
    }
    println!("Account Balance: ${:.2}",account_balance);
}
