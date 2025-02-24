use std::{sync::{Arc, Mutex}, thread};

struct BankAccount {
    balance: f64
}

fn main() {
    let account = Arc::new(Mutex::new(BankAccount{ balance: 1000.0}));
    let mut handles = vec![];
    for _ in 0..5 {
        let account = Arc::clone(&account);
        let handle = thread::spawn(move ||{
            let mut account = account.lock().unwrap();

            let deposit_amount = 200.0;
            let withdrawal_amount = 150.0;

            account.balance += deposit_amount;

            if account.balance >= withdrawal_amount {
                account.balance -= withdrawal_amount;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let account = account.lock().unwrap();
    println!("Final Balance:${:.2}",account.balance);


}