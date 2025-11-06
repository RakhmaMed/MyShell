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

/// Handles the `exit` command by terminating the shell process.
///
/// # Arguments
///
/// * `args` - Optional exit code (defaults to 0 if not provided)
///
/// # Note
///
/// This function does not return as it calls `std::process::exit()`.
fn handle_exit(args: &[&str]) -> ! {
    let code = args
        .first()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);
    std::process::exit(code);
}

fn main() -> io::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        // Wait for user input
        let input = read_line().unwrap();
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();
        match command {
            "exit" => handle_exit(&args),
            _ => println!("{}: command not found", command),
        }
    }
}
