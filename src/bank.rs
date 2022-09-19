use std::fmt;
use std::fmt::Formatter;

pub struct Bank {
    balance: f32,
    name: String,
}

impl Bank {
    pub fn new(name: String, balance: f32) -> Bank {
        Bank { name, balance }
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn withdraw(&mut self, amount: f32) -> Result<f32, BankWithdrawAmountError> {
        if self.balance >= amount {
            self.balance -= amount;
            Ok(self.balance)
        } else {
            Err(BankWithdrawAmountError)
        }
    }
}

#[derive(Debug, Clone)]
pub struct BankWithdrawAmountError;

impl fmt::Display for BankWithdrawAmountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       write!(f, "withdraw amount exceeded bank account balance")
    }
}


