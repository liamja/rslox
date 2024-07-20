mod token;
mod token_type;
mod scanner;

use std::io::Write;
use std::{env, fs, io, process};

fn main() -> process::ExitCode {
    // Get the arguments passed to the interpreter.
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    // If there are no arguments, print the usage and exit.
    if args.len() > 2 {
        println!("Usage: rslox [script]");
        process::ExitCode::from(64)
    } else if args.len() == 2 {
        run_file(&args[1]);
        process::ExitCode::SUCCESS
    } else {
        run_prompt();
        process::ExitCode::SUCCESS
    }
}

fn run_file(file_path: &String) {
    // todo: Read the file and run it.
    println!("Running file: {}", file_path);

    let file_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("💣 Error reading file `{}`: {}", file_path, err);
        process::exit(66);
    });

    println!("Read contents:\n{}", file_content);

    // todo: run(file_content)
}

fn run_prompt() {
    // Set up a long-lived buffer to read input into, so we can work with it.
    let mut buffer = String::new();

    // Get a shared handle to standard input.
    let stdin = io::stdin();

    println!("👋 Hello!");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        // Read a line from standard input.
        match stdin.read_line(&mut buffer) {
            Ok(0) => {
                // No bytes were read, so we've reached the end of the input.
                println!();
                println!("👋 Goodbye!");
                return;
            }
            Ok(n) => {
                // Some bytes were read in from the input.
                println!("ℹ️ {n} bytes read into buffer.");
                let line = buffer.lines().last().unwrap();
                // println!("ℹ️ Last line: {}", line);
                run(line);
            }
            Err(err) => {
                // There was an error reading from the input.
                eprintln!("💣 Error reading input: {}", err);
                process::exit(74);
            }
        }

        // Print the line that was read, for debugging.
        // println!("ℹ️ You entered: {}", buffer);
    }
}

fn run(source: &str) {
    let mut scanner = scanner::Scanner::new(source.to_string());
    let tokens = scanner.scan_tokens();

    for token in tokens {
        println!("{:?}", token.token_type);
    }
    
    // todo: for each token, print the token.
    // println!("ℹ️ Source:");
    // println!("{}", source);
}
