#[allow(unused_imports)]
use std::io::{self, stdout, Write};

fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    // Remove trailing newline
    if input.ends_with('\n') {
        input.pop();
        if input.ends_with('\r') {
            input.pop();
        }
    }
    Ok(input)
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        // Wait for user input
        let input = read_line().unwrap();
        match input.as_str() {
            "exit" => break,
            _ => println!("{}: command not found", input),
        }
    }
}
