mod bank;

use std::io::Write;
use crate::bank::BankWithdrawAmountError;

fn main() {
    println!("ATM interface");
    let mut buffer = String::new();
    let mut account = bank::Bank::new("Jeff Smith".to_owned(), 10924.89);
    let mut other_accounts = vec![
        bank::Bank::new("Walter White".to_owned(), 882092.00),
        bank::Bank::new("Creed Bratton".to_owned(), 3000000.00),
    ];
    loop {
        output_and_read_to_buffer(">> ", &mut buffer);

        match buffer.trim() {
            "balance" => println!("Your current balance is: ${}.", account.get_balance()),
            "transfer" => {
                println!("Please choose the recipient you wish to send money to.");
                for (index, account) in other_accounts.iter().enumerate() {
                    println!("[{}] {}", index+1, account.get_name());
                }
                output_and_read_to_buffer(
                    format!("Corresponding number (1..{}): ", other_accounts.len()).as_str(),
                    &mut buffer,
                );
                let mut recipient = &mut other_accounts[buffer.trim().parse::<usize>().unwrap()-1];
                output_and_read_to_buffer(
                    format!("How much would you like to transfer to {}? $", recipient.get_name())
                        .as_str(),
                    &mut buffer,
                );
                let mut amount = buffer.trim().parse().unwrap();
                match account.transfer(recipient, amount) {
                    Ok(bal) => println!(
                        "Transferred ${} to {}!\nRemaining balance: ${}",
                        amount,
                        recipient.get_name(),
                        bal,
                    ),
                    Err(_) => println!("An issue occurred with your transfer!"),
                };
            },
            "payments" => continue,
            "withdraw" => {
                output_and_read_to_buffer(
                    "Please enter the amount you wish to withdraw: $",
                    &mut buffer,
                );
                let amount = buffer
                    .parse::<f32>()
                    .expect("Value entered could not be parsed as f32!");
                match account.withdraw(amount) {
                    Ok(bal) => {
                        println!("You withdrew ${}!\nRemaining balance: ${}", amount, bal);
                    },
                    Err(_) => println!("You cannot withdraw ${} as it exceeds your balance!", amount),
                };
            },
            _ => continue,
        }
        buffer.clear()
    }
}

fn output_and_read_to_buffer(output: &str, buffer: &mut String) {
    buffer.clear();
    print!("{}", output);
    std::io::stdout()
        .flush()
        .expect("Failed to flush output stream!");
    std::io::stdin()
        .read_line(buffer)
        .expect("Failed to read input stream into buffer!");
}
