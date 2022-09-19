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

    pub fn transfer(&mut self, recipient: &mut Bank, amount: f32) -> Result<f32, BankTransferError> {
        if self.balance < amount {
            return Err(BankTransferError)
        }
        self.balance -= amount;
        recipient.send(amount);

        Ok(self.balance)
    }

    fn send(&mut self, amount: f32) {
        self.balance += amount;
    }
}

#[derive(Debug, Clone)]
pub struct BankWithdrawAmountError;

impl fmt::Display for BankWithdrawAmountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       write!(f, "withdraw amount exceeded bank account balance")
    }
}

#[derive(Debug, Clone)]
pub struct BankTransferError;

impl fmt::Display for BankTransferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bank transfer was unsuccessful")
    }
}


