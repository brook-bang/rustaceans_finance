#[allow(dead_code)]
#[derive(Debug)]
struct Transaction {
    amount: f64,
    date: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Account {
    name: String,
    transactions: Vec<Transaction>,
}

fn main() {
    let mut financial_data: Vec<Box<Account>> = Vec::new();
    let account1 = Account {
        name: "Account 1".to_string(),
        transactions: vec![
            Transaction {
                amount: 1000.0,
                date: "2023-09-14".to_string(),
            },
            Transaction {
                amount: -500.0,
                date: "2023-09-15".to_string(),
            },
        ],
    };

    let account2 = Account {
        name: "Account 2".to_string(),
        transactions: vec![
            Transaction {
                amount: 2000.0,
                date: "2023-09-14".to_string(),
            },
            Transaction {
                amount: -1000.0,
                date: "2023-09-15".to_string(),
            },
        ],
    };

    financial_data.push(Box::new(account1));
    financial_data.push(Box::new(account2));

    for account in financial_data.iter() {
        println!("{:?}",account)
    }


}
