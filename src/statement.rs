use super::row::*;
use super::table::*;

pub enum StatementKind {
    Insert,
    Select,
}

pub struct Statement {
    kind: StatementKind,
    row: Option<Row>,
}

pub type StatementError = String;

pub enum ExecuteResult {
    Success,
    TableFull,
    EmptyRow,
}

// Hard coded table

impl Statement {
    pub fn prepare(input: &str) -> Result<Self, StatementError> {
        // TODO: refactor to use function to match first word in input

        if input.starts_with("insert") {
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
                kind: StatementKind::Insert,
                row: Some(row),
            })
        } else if input.starts_with("select") {
            Ok(Statement {
                kind: StatementKind::Select,
                row: None,
            })
        } else {
            Err(String::from("invalid input"))
        }
    }

    fn execute_insert(&self, table: &mut Table) -> ExecuteResult {
        let current_num_rows = table.num_rows;

        if current_num_rows >= TABLE_MAX_ROWS {
            return ExecuteResult::TableFull;
        }

        if let Some(row_to_insert) = &self.row {
            let serialized = row_to_insert.serialize();

            let (page, num) = table.row_slots(current_num_rows);
            let _ = page.insert_row(num, serialized);

            table.num_rows += 1;

            return ExecuteResult::Success;
        }

        // This should not happen.
        ExecuteResult::EmptyRow
    }

    fn execute_select(&self, table: &mut Table) -> ExecuteResult {
        println!("num_rows: {}", table.num_rows);
        for i in 0..table.num_rows {
            let (page, num) = table.row_slots(i);

            page.get_row(num)
                .and_then(|raw| Row::deserialize(raw))
                .and_then(|row| {
                    println!("{}", row);
                    Some(())
                });
        }

        ExecuteResult::Success
    }

    pub fn execute(&self, table: &mut Table) -> ExecuteResult {
        match self.kind {
            StatementKind::Insert => self.execute_insert(table),
            StatementKind::Select => self.execute_select(table),
        }
    }
}
