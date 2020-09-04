use super::row::*;

pub enum StatementKind {
    Insert,
    Select,
}

pub struct Statement {
    kind: StatementKind,
    row: Option<Row>,
}

pub type StatementError = String;

// Hard coded table

impl Statement {
    pub fn prepare(input: &str) -> Result<Self, StatementError> {
        // TODO: refactor to use function to match first word in input

        if input.starts_with("select") {
            // scan arguments

            let raw_args: Vec<&str> = input.split_whitespace().collect();

            // check if length of arguments match length of the table
            // FIXME: hard coded length
            if raw_args.len() != 4 {
                return Err(String::from("arguments length does not match"));
            }

            let id = raw_args[1].parse::<u32>().unwrap();

            // TODO: add validation of username length
            let username = String::from(raw_args[2]);

            // TODO: add validation of email length
            let email = String::from(raw_args[3]);

            let row = Row::new(id, username, email);

            Ok(Statement {
                kind: StatementKind::Select,
                row: Some(row),
            })
        } else if input.starts_with("insert") {
            Ok(Statement {
                kind: StatementKind::Insert,
                row: None,
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
