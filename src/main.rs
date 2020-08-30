use std::io::{self, prelude::*};

enum MetaCommandResult {
    Success,
    Exited,
    Fail(String),
}

fn do_meta_command(command: &str) -> MetaCommandResult {
    use MetaCommandResult::*;

    match command {
        ".exit" => Exited,
        _ => Fail(format!("unrecognizable command `{}`", command)),
    }
}

enum StatementKind {
    Insert,
    Select,
}

struct Statement {
    kind: StatementKind,
}

type StatementError = String;

impl Statement {
    pub fn prepare(input: &str) -> Result<Self, StatementError> {
        // TODO: refactor to use function to match first word in input
        if input.starts_with("select") {
            Ok(Statement {
                kind: StatementKind::Select,
            })
        } else if input.starts_with("insert") {
            Ok(Statement {
                kind: StatementKind::Insert,
            })
        } else {
            Err(String::from("invalid input"))
        }
    }

    pub fn execute(&self) {
        match self.kind {
            StatementKind::Insert => println!("This is where we do an insert."),
            StatementKind::Select => println!("This is where we do an select."),
        }
    }
}

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
