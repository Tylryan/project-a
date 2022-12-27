mod cli;
mod common;
mod tui;
mod user;
mod storage;

use cli::cli_parser;
use std::env;

fn main() 
{
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    let mut storage = storage::db_handler::DbHandler::new("./test");
    storage.sync_decks();
}
