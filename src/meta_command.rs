use super::table::Table;

pub enum MetaCommandResult {
    Exited,
    Fail(String),
}

pub fn do_meta_command(command: &str, table: &mut Table) -> MetaCommandResult {
    use MetaCommandResult::*;

    match command {
        ".exit" => {
            table.close();
            Exited
        }
        _ => Fail(format!("unrecognizable command `{}`", command)),
    }
}
