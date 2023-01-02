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
    decks::Decks};

use crate::tui::tui::App;

fn main() 
{
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    // Sync deck

    App::run();
    // let mut rs = ReviewSystem::new(&deck, &storage);
    // loop 
    // {
    //     let mut storage = storage::db_handler::DbHandler::new("./test");
    //     // storage.sync_decks();
    //     let deck_name        = "idk";
    //     let deck: Deck   = storage.get_decks()
    //         .get_deck(deck_name)
    //         .unwrap();
    //     let mut rs = ReviewSystem::new(&deck, &storage);
    //     rs.study_cli();

    // }
}
