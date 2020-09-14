mod btree;
mod cli;
mod cursor;
mod meta_command;
mod pager;
mod row;
mod statement;
mod table;

use std::env::args;

fn main() {
    let arguments: Vec<String> = args().collect();
    let _ = cli::start(&arguments[1]);
}
