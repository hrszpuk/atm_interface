mod bank;

use std::io::Write;
use colored::Colorize;

fn main() {
    println!("{}", "ATM interface".green().bold());

    // We use this for storing all inputs from the user throughout the program
    let mut buffer = String::new();

    // Stores the user's account
    let mut account = register();

    // Other accounts (for transferring, and payment logs)
    let mut other_accounts = vec![
        bank::Bank::new("Walter White".to_owned(), 882092.00),
        bank::Bank::new("Creed Bratton".to_owned(), 3000000.00),
    ];

    loop {
        output_and_read_to_buffer(">> ", &mut buffer);

        match buffer.trim() {

            // If the user wants to view their balance.
            "balance" => println!("Your current balance is: ${}.", account.get_balance().to_string().green()),

            // For transferring money from the user's account to another
            "transfer" => transfer(&mut buffer, &mut account, &mut other_accounts),

            // View payment history
            "payments" => {
                for (date, name, amount) in &account.payments {
                    if *name == *account.get_name() {
                        println!("Timestamp: {}", date);
                        println!("           └ Withdrew ${}", amount.to_string().red());
                    } else {
                        println!("Timestamp: {}", date);
                        println!("           └ Transferred ${} to \"{}\"", amount.to_string().red(), name.blue());
                    }
                }
            },

            // Withdraw money from the user's account (takes away from balance)
            "withdraw" => withdraw(&mut buffer, &mut account),

            // Closes the application
            "exit"|"quit" => {
                break;
            },

            // Shows name and balance
            "info" => {
                println!(
                    "Account information\n{} {}\n{} ${}",
                    "Name:".bold(),
                    account.get_name().blue(),
                    "Balance:".bold(),
                    account.get_balance().to_string().green(),
                )
            }

            //  Show help message
            "help" => {
                println!("Available commands:");
                println!("\t{}: show this message", "help".underline().purple());
                println!("\t{}: shows your current balance", "balance".underline().purple());
                println!("\t{}: transfer funds to another individual", "transfer".underline().purple());
                println!("\t{}: withdraw funds from your account", "withdraw".underline().purple());
                println!("\t{}: view payment history", "payments".underline().purple());
                println!("\t{}: view account information", "info".underline().purple());
                println!("\t{}: quits the program (quit also works)", "exit".underline().purple());
            },

            // If we ever get an unexpected input we report the issue to the user
            _ => println!(
                "{}: {} is not an available command. \nUse \"help\" to view available commands.",
                "Error".red(),
                buffer.trim().underline().bright_red()
            ),
        }
        buffer.clear()
    }
    println!("{}", "Closed ATM interface.".red().bold());
}

/// For transferring money from the user's bank account to another bank account
fn transfer(mut buffer: &mut String, account: &mut bank::Bank, other_accounts: &mut Vec<bank::Bank>) {

    // List all the recipients the user can send to...
    println!("Please choose the recipient you wish to send money to.");
    for (index, account) in other_accounts.iter().enumerate() {
        println!("[{}] {}", index+1, account.get_name().blue());
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
            return;
        },
        Err(_) => {
            println!("{} is not a valid recipient number!", buffer.trim().red().underline());
            return;
        }
    };
    let recipient = &mut other_accounts[recipient_number];

    // Find the amount of money the user wants to send...
    output_and_read_to_buffer(
        format!("How much would you like to transfer to {}? $", recipient.get_name().blue())
            .as_str(),
        &mut buffer,
    );
    let amount = match buffer.trim().parse() {
        Ok(value) => value,
        Err(_) => {
            println!("{} is not a valid transfer amount!", buffer.trim().red().underline());
            return;
        }
    };

    // Transfer money to the recipient!
    match account.transfer(recipient, amount) {
        Ok(bal) => println!(
            "Transferred ${} to {}!\nRemaining balance: ${}",
            amount.to_string().green(),
            recipient.get_name().blue(),
            bal.to_string().green(),
        ),
        Err(_) => println!("{}", "An issue occurred with your transfer!".red()),
    };
}

/// For withdrawing money from the user's account
fn withdraw(mut buffer: &mut String, account: &mut bank::Bank) {

    // Getting the amount the user wants to withdraw
    output_and_read_to_buffer(
        "Please enter the amount you wish to withdraw: $",
        &mut buffer,
    );
    let amount = match buffer.trim().parse::<f32>() {
        Ok(value) => value,
        Err(_) => {
            println!("{} is not a valid withdraw amount!", buffer.trim().red().underline());
            return;
        }
    };

    // Withdrawing the amount from the user's balance
    match account.withdraw(amount) {
        Ok(bal) => {
            println!("You withdrew ${}!\nRemaining balance: ${}", amount.to_string().blue(), bal.to_string().green());
        },
        Err(_) => println!("You cannot withdraw ${} as it exceeds your balance!", amount.to_string().red()),
    };
}

fn register() -> bank::Bank {
    let mut name = String::new();
    let mut balance_buffer = String::new();
    let balance: f32;

    println!("Register a {} and {} to continue...", "name".underline(), "starting balance".underline());

    output_and_read_to_buffer(
        "Name: ",
        &mut name,
    );

    output_and_read_to_buffer(
        "Starting balance: ",
        &mut balance_buffer,
    );

    balance = match balance_buffer.trim().parse::<f32>() {
        Ok(bal) => bal,
        Err(_) => {
            println!("{}",
                format!("{} is not a valid starting balance!", balance_buffer.trim()).red());
            println!("Re-starting registry process.");
            return register();
        }
    };

    bank::Bank::new(name.trim().to_string(), balance)
}

/// We do this a lot so I made a function for it.
/// This function clears the buffer
/// Prints the output, and then reads a line and slaps it into the buffer
fn output_and_read_to_buffer(output: &str, buffer: &mut String) {
    buffer.clear();
    print!("{}", output.bold());
    std::io::stdout()
        .flush()
        .expect("Failed to flush output stream!");
    std::io::stdin()
        .read_line(buffer)
        .expect("Failed to read input stream into buffer!");
}
