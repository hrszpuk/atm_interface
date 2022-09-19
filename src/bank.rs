use std::fmt;
use std::fmt::Formatter;
use chrono::{DateTime, Utc};

/// Stores bank account information such as balance, name, and payment history!
pub struct Bank {
    balance: f32,
    name: String,
    pub payments: Vec<(i64, String, f32)>,
}

impl Bank {

    /// Constructor... OOP caveman brain ooga booga
    pub fn new(name: String, balance: f32) -> Bank {
        Bank { name, balance, payments: vec![] }
    }

    pub fn get_balance(&self) -> f32 {
        self.balance
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Withdraws a given amount from the balance and returns the result.
    /// We return an error if the amount exceeds the balance of the account.
    pub fn withdraw(&mut self, amount: f32) -> Result<f32, BankWithdrawAmountError> {
        if self.balance >= amount {
            self.balance -= amount;

            // Add to payment history
            self.payments.push((Utc::now().timestamp(), self.name.clone(), amount));
            Ok(self.balance)
        } else {
            Err(BankWithdrawAmountError)
        }
    }

    /// Transfers a given amount from (&mut self) to the given recipient.
    /// Will return a BankTransferError if any issues happen in the process.
    pub fn transfer(&mut self, recipient: &mut Bank, amount: f32) -> Result<f32, BankTransferError> {
        if self.balance < amount {
            return Err(BankTransferError)
        }
        self.balance -= amount;
        recipient.send(amount);

        // Add to payment history
        self.payments.push((Utc::now().timestamp(), recipient.name.clone(), amount));

        Ok(self.balance)
    }

    /// Private function used to give bank accounts money (used in transfer).
    fn send(&mut self, amount: f32) {
        self.balance += amount;
    }
}

/// This error occurs when the amount exceeds the balance during a withdraw request
#[derive(Debug, Clone)]
pub struct BankWithdrawAmountError;

impl fmt::Display for BankWithdrawAmountError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       write!(f, "withdraw amount exceeded bank account balance")
    }
}

/// This error occurs when the process of transferring money from one account to another fails.
#[derive(Debug, Clone)]
pub struct BankTransferError;

impl fmt::Display for BankTransferError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "bank transfer was unsuccessful")
    }
}


