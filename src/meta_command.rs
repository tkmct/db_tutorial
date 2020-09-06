pub enum MetaCommandResult {
    Exited,
    Fail(String),
}

pub fn do_meta_command(command: &str) -> MetaCommandResult {
    use MetaCommandResult::*;

    match command {
        ".exit" => Exited,
        _ => Fail(format!("unrecognizable command `{}`", command)),
    }
}
