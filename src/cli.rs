use super::meta_command::*;
use super::statement::*;
use super::table::*;
use std::io::{self, prelude::*};

pub fn start() -> Result<(), io::Error> {
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
                Exited => break,
                Fail(reason) => {
                    println!("Fail: {}", reason);
                    continue;
                }
            }
        }

        match Statement::prepare(buffer) {
            Ok(statement) => match statement.execute(table) {
                ExecuteResult::InsertSuccess => println!("Insert succeed."),
                ExecuteResult::SelectSuccess(rows) => {
                    for row in rows.iter() {
                        println!("{}", row);
                    }
                }
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
