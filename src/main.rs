mod cli;
mod common;
mod tui;
mod user;
mod storage;

use chrono::{Duration, Local};
use cli::cli_parser;
use std::{env, io::Write, process::exit};
use crate::common::{
    study::{ReviewSystem, Message},
    deck::Deck, 
    decks::Decks
};

use crate::tui::tui::App;
fn main() 
{
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    App::run();

    // Sync deck

}
