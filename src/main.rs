mod meta_command;
mod row;
mod statement;

use meta_command::*;
use statement::*;
use std::io::{self, prelude::*};

fn main() -> Result<(), io::Error> {
    use MetaCommandResult::*;

    loop {
        print!("db > ");
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut buffer = String::new();

        stdin.read_line(&mut buffer).expect("Could not read line");
        let buffer = buffer.trim();

        if buffer.starts_with('.') {
            match do_meta_command(buffer) {
                Success => continue,
                Exited => break,
                Fail(reason) => {
                    println!("Fail: {}", reason);
                    continue;
                }
            }
        }

        match Statement::prepare(buffer) {
            Ok(statement) => {
                statement.execute();
                println!("Executed.");
            }
            Err(e) => {
                println!("Error preparing statement. {}", e);
            }
        }
    }

    Ok(())
}
