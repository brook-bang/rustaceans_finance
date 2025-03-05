use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Transaction {
    transaction_type: &'static str,
    amount: f64,
}

struct Account {
    transactions: Vec<Transaction>,
}

impl Account {
    fn new(transactions: Vec<Transaction>) -> Self {
        Account { transactions }
    }

    fn calculate_balance(&self) -> f64 {
        self.transactions
            .par_iter()
            .map(|transaction| match transaction.transaction_type {
                "Deposit" => transaction.amount,
                "Withdrawal" => -transaction.amount,
                _ => 0.0,
            })
            .sum()
    }
}

fn main() {
    let account1 = Account::new(vec![
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
    ]);

    let account2 = Account::new(vec![
        Transaction {
            transaction_type: "Deposit",
            amount: 300.0,
        },
        Transaction {
            transaction_type: "Withdrawal",
            amount: 75.0,
        },
    ]);

    let total_balance: f64 = vec![&account1, &account2]
        .par_iter()
        .map(|account| account.calculate_balance())
        .sum();

    println!("Total Account Balance: ${:.2}", total_balance);
}
