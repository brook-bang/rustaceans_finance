struct BankAccount {
    account_holder: String,
    balance: Option<f64>,
}

impl BankAccount {
    fn new(account_holder: &str) -> BankAccount {
        BankAccount {
            account_holder: account_holder.to_string(),
            balance:None,
        }
    }

    fn deposit(&mut self,amount:f64) {
        if let Some(existing_balance) = self.balance {
            self.balance = Some(existing_balance + amount);
        } else {
            self.balance = Some(amount)
        }
    }

    fn withdraw(&mut self,amount: f64) -> Option<f64> {
        if let Some(existing_balance) = self.balance {
            if existing_balance >= amount {
                self.balance = Some(existing_balance - amount);
                Some(amount)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn check_balance(&self) -> Option<f64> {
        self.balance
    }
}

fn main() {
    let mut account = BankAccount::new("Alice");
    account.deposit(1000.0);
    println!("存款后的金额：{:?}",account.check_balance());

    if let Some(withdrawn_amount) = account.withdraw(500.0) {
        println!("成功取款：{:?}",withdrawn_amount);
    } else {
        println!("取款失败，余额不足或没有余额");
    }

    println!("最终余额：{:?}",account.check_balance());
}