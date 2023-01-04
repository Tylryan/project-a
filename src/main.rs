mod cli    ;
mod common ;
mod tui    ;
mod user   ;
mod storage;

use std::env;
use cli::cli_parser;
use tui::tui::App;

fn main() 
{
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    App::run();
}
