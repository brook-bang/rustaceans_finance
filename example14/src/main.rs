use std::{error::Error, fmt::Display};


#[derive(Debug)]
enum FinancialError {
    InvalidInput,
    DivisionByZero,
}

impl Display for FinancialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FinancialError::InvalidInput => write!(f,"Invalid input"),
            FinancialError::DivisionByZero => write!(f,"Division by zero"),
        }
    }
}

impl Error for FinancialError {}

fn main() -> Result<(),Box<dyn Error>>{

    let input = "10";

    let num: i32 = input.parse().map_err(|_| Box::new(FinancialError::InvalidInput))?;

    if num == 0 {
        return Err(Box::new(FinancialError::DivisionByZero));
    }

    let result = 100 / num;

    println!("Result:{}",result);

    Ok(())
}
