pub enum StatementKind {
    Insert,
    Select,
}

pub struct Statement {
    kind: StatementKind,
}

pub type StatementError = String;

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
