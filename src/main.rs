mod bank;

use std::io::Write;
use std::ops::Deref;
use crate::bank::BankWithdrawAmountError;

fn main() {
    println!("ATM interface");

    // We use this for storing all inputs from the user throughout the program
    let mut buffer = String::new();

    // Stores the user's account
    let mut account = bank::Bank::new("Jeff Smith".to_owned(), 10924.89);

    // Other accounts (for transferring, and payment logs)
    let mut other_accounts = vec![
        bank::Bank::new("Walter White".to_owned(), 882092.00),
        bank::Bank::new("Creed Bratton".to_owned(), 3000000.00),
    ];

    loop {
        output_and_read_to_buffer(">> ", &mut buffer);

        match buffer.trim() {

            // If the user wants to view their balance.
            "balance" => println!("Your current balance is: ${}.", account.get_balance()),

            // For transferring money from the user's account to another
            "transfer" => {

                // List all the recipients the user can send to...
                println!("Please choose the recipient you wish to send money to.");
                for (index, account) in other_accounts.iter().enumerate() {
                    println!("[{}] {}", index+1, account.get_name());
                }

                // Get which recipient the user wants to send money to...
                output_and_read_to_buffer(
                    format!("Corresponding number (1..{}): ", other_accounts.len()).as_str(),
                    &mut buffer,
                );
                let recipient_number = match buffer.trim().parse::<usize>() {
                    Ok(number) => if number-1 < other_accounts.len() {
                        number
                    } else {
                        println!("Invalid! {} is not between {} and {}.", buffer.trim(), 1, other_accounts.len());
                        continue;
                    },
                    Err(_) => {
                        println!("\"{}\" is not a valid recipient number!", buffer.trim());
                        continue;
                    }
                };
                let mut recipient = &mut other_accounts[recipient_number];

                // Find the amount of money the user wants to send...
                output_and_read_to_buffer(
                    format!("How much would you like to transfer to {}? $", recipient.get_name())
                        .as_str(),
                    &mut buffer,
                );
                let mut amount = match buffer.trim().parse() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("\"{}\" is not a valid transfer amount!", buffer.trim());
                        continue;
                    }
                };

                // Transfer money to the recipient!
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

            // View payment history
            "payments" => {
                for (date, name, amount) in &account.payments {
                    if *name == *account.get_name() {
                        println!("[{}] Withdrew ${}.", date, amount);
                    } else {
                        println!("[{}] Transferred ${} to {}.", date, amount, name);
                    }
                }
            },

            // Withdraw money from the user's account (takes away from balance)
            "withdraw" => {

                // Getting the amount the user wants to withdraw
                output_and_read_to_buffer(
                    "Please enter the amount you wish to withdraw: $",
                    &mut buffer,
                );
                let amount = match buffer.trim().parse::<f32>() {
                    Ok(value) => value,
                    Err(_) => {
                        println!("\"{}\" is not a valid withdraw amount!", buffer.trim());
                        continue;
                    }
                };

                // Withdrawing the amount from the user's balance
                match account.withdraw(amount) {
                    Ok(bal) => {
                        println!("You withdrew ${}!\nRemaining balance: ${}", amount, bal);
                    },
                    Err(_) => println!("You cannot withdraw ${} as it exceeds your balance!", amount),
                };
            },

            "help" => {
                println!("Available commands:");
                println!("\thelp: show this message");
                println!("\tbalance: shows your current balance");
                println!("\ttransfer: transfer funds to another individual");
                println!("\twithdraw: withdraw funds from your account");
                println!("\tpayments: view payment history");
            },

            // If we ever get an unexpected input we report the issue to the user
            _ => println!(
                "error: \"{}\" is not an available command. \nUse \"help\" to view available commands.",
                buffer.trim()
            ),
        }
        buffer.clear()
    }
}

/// We do this a lot so I made a function for it.
/// This function clears the buffer
/// Prints the output, and then reads a line and slaps it into the buffer
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
