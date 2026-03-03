use std::io;

fn main() {
    println!("Fibonacci number finder v0.1.0");
    loop {
        println!("\nEnter the index (or Enter to exit):");
        let mut index = String::new();
        if io::stdin().read_line(&mut index).is_err() {
            update_terminal();
            println!("Failed to read line");
        }
        if index.trim().is_empty() {
            println!("Exiting...");
            break;
        }

        let index: u32 = if let Ok(num) = index.trim().parse() {
            num
        } else {
            update_terminal();
            println!("Invalid input! Try again.");
            continue;
        };

        match find_nth_fibonacci_number(index.into()) {
            Some(result) => println!("\nThe {index}th Fibonacci number is: {result}"),
            None => println!("\nOverflow! Index too large."),
        }
    }
}

fn find_nth_fibonacci_number(index: u128) -> Option<u128> {
    let mut prev: u128 = 0;
    let mut curr: u128 = 1;

    for _ in 1..=index {
        let temp = prev.checked_add(curr)?;
        prev = curr;
        curr = temp;
    }

    Some(prev)
}

fn update_terminal() {
    println!("\x1B[2J\x1B[1;1H");
}
