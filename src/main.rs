use std::io::{self, prelude::*};

fn main() -> Result<(), io::Error> {
    loop {
        print!("db > ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).expect("Could not read line");

        match buffer.trim() {
            ".exit" => break,
            _ => {
                println!("Unrecognized command `{}`.", buffer.trim());
                continue;
            }
        }
    }

    Ok(())
}
