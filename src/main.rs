mod bank;

use std::io::Write;

fn main() {
    println!("ATM interface");
    let mut buffer = String::new();
    let mut account = bank::Bank::new("Jeff Smith".to_owned(), 10924.89);
    loop {
        output_and_read_to_buffer(">> ", &mut buffer);

        match buffer.trim() {
            "balance" => println!("Your current balance is: ${}.", account.get_balance()),
            "transfer" => continue,
            "payments" => continue,
            "withdraw" => {
                output_and_read_to_buffer(
                    "Please enter the amount you wish to withdraw: $",
                    &mut buffer,
                );
                let amount = buffer
                    .trim()
                    .parse::<f32>()
                    .expect("Value entered could not be parsed as f32!");
                let new_balance = account.withdraw(amount)
                    .expect("Tried to withdraw too much money!");
                println!("You withdraw ${}!\nRemaining balance: ${}", amount, new_balance);
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
