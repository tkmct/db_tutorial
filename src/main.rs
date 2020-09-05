mod meta_command;
mod row;
mod statement;
mod table;

use meta_command::*;
use statement::*;
use std::io::{self, prelude::*};
use table::*;

fn main() -> Result<(), io::Error> {
    use MetaCommandResult::*;

    let table = &mut Table::new();

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
            Ok(statement) => match statement.execute(table) {
                ExecuteResult::Success => println!("Execution succeed."),
                ExecuteResult::TableFull => println!("Error: Table full"),
                _ => println!("Something went wrong."),
            },
            Err(e) => {
                println!("Error preparing statement. {}", e);
            }
        }
    }

    Ok(())
}
