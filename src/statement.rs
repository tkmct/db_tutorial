use super::row::*;
use super::table::*;

#[derive(Debug, Eq, PartialEq)]
pub enum StatementKind {
    Insert,
    Select,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Statement {
    kind: StatementKind,
    row: Option<Row>,
}

pub type StatementError = String;

#[derive(Debug, Eq, PartialEq)]
pub enum ExecuteResult {
    InsertSuccess,
    SelectSuccess(Vec<Row>),
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

            if raw_args[2].len() > COLUMN_USERNAME_SIZE {
                return Err(String::from("Too long string."));
            }
            let username = String::from(raw_args[2]);

            if raw_args[3].len() > COLUMN_EMAIL_SIZE {
                return Err(String::from("Too long string."));
            }
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
            let mut cursor = table.table_end();
            let _ = cursor.insert_value(&row_to_insert);
            table.num_rows += 1;
            return ExecuteResult::InsertSuccess;
        }

        // This should not happen.
        ExecuteResult::EmptyRow
    }

    fn execute_select(&self, table: &mut Table) -> ExecuteResult {
        let mut res = Vec::new();

        let mut cursor = table.table_start();
        while !cursor.is_end() {
            if let Some(row) = cursor.get_value().and_then(|raw| Row::deserialize(raw)) {
                res.push(row);
            }
            cursor.advance_cursor();
        }

        ExecuteResult::SelectSuccess(res)
    }

    pub fn execute(&self, table: &mut Table) -> ExecuteResult {
        match self.kind {
            StatementKind::Insert => self.execute_insert(table),
            StatementKind::Select => self.execute_select(table),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::table::Table;
    use super::*;
    use std::error::Error;
    use std::fs;

    const TEST_FILE: &str = "db_test";

    #[test]
    fn test_insert_then_select() -> Result<(), Box<dyn Error>> {
        {
            let mut table = Table::open(TEST_FILE)?;
            let stmt = Statement::prepare("insert 1 user user@example.com")?;

            let result = stmt.execute(&mut table);
            assert_eq!(result, ExecuteResult::InsertSuccess);

            let stmt = Statement::prepare("select")?;
            let result = stmt.execute(&mut table);
            assert_eq!(
                result,
                ExecuteResult::SelectSuccess(vec![Row::new(
                    1,
                    String::from("user"),
                    String::from("user@example.com")
                )])
            );
        }
        let _ = fs::remove_file(TEST_FILE);
        Ok(())
    }

    #[test]
    fn test_table_is_full() -> Result<(), Box<dyn Error>> {
        {
            let mut table = Table::open(TEST_FILE)?;
            let mut i = 1;
            let result = loop {
                let stmt =
                    Statement::prepare(&format!("insert {i} user{i} user{i}@example.com", i = i))?;
                let result = stmt.execute(&mut table);

                if i == 1401 {
                    break result;
                }

                i += 1;
            };

            assert_eq!(result, ExecuteResult::TableFull);
        }
        let _ = fs::remove_file(TEST_FILE);
        Ok(())
    }

    #[test]
    fn test_insert_with_max_input_length() -> Result<(), Box<dyn Error>> {
        {
            let mut table = Table::open(TEST_FILE)?;

            let long_username: String = ['a'; 32].iter().collect();
            let long_email: String = ['a'; 255].iter().collect();

            let stmt = Statement::prepare(&format!("insert 1 {} {}", long_username, long_email))?;

            let result = stmt.execute(&mut table);
            assert_eq!(result, ExecuteResult::InsertSuccess);
        }
        let _ = fs::remove_file(TEST_FILE);
        Ok(())
    }

    #[test]
    fn test_insert_fails_with_too_long_string() -> Result<(), Box<dyn Error>> {
        let long_username: String = ['a'; 33].iter().collect();
        let long_email: String = ['a'; 256].iter().collect();

        let result = Statement::prepare(&format!("insert 1 {} {}", long_username, long_email));
        assert_eq!(result, Err(String::from("Too long string.")));

        Ok(())
    }

    #[test]
    fn test_persistence() -> Result<(), Box<dyn Error>> {
        {
            let mut table = Table::open(TEST_FILE)?;
            let stmt = Statement::prepare("insert 1 user user@example.com")?;

            let result = stmt.execute(&mut table);
            assert_eq!(result, ExecuteResult::InsertSuccess);
            table.close();
        }

        {
            let mut table = Table::open(TEST_FILE)?;
            println!("Table size: {}", table.num_rows);
            let stmt = Statement::prepare("select")?;
            let result = stmt.execute(&mut table);
            assert_eq!(
                result,
                ExecuteResult::SelectSuccess(vec![Row::new(
                    1,
                    String::from("user"),
                    String::from("user@example.com")
                )])
            );
        }

        let _ = fs::remove_file(TEST_FILE);
        Ok(())
    }
}
