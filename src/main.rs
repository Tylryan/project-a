mod cli;
mod common;
mod tui;
mod user;
mod storage;

use chrono::{Duration, Local};
use cli::cli_parser;
use std::{env, io::Write, process::exit};
use crate::common::{
    study::ReviewSystem, 
    deck::Deck, 
    decks::Decks};

fn main() 
{
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    let mut storage = storage::db_handler::DbHandler::new("./test");

    // Get the deck
    let deck_name   = "idk";

    // Get current cards for that deck
    storage.sync_decks();

    let mut decks: Decks = storage.get_decks();
    let mut deck: Deck   = storage.get_decks()
        .get_deck(deck_name)
        .unwrap();

    let mut rs = ReviewSystem::new(&deck);
    rs.generate_study_deck();
    let current_card = rs.get_current_card();

    if current_card.is_none() 
    {
        return println!("No cards to study!");
    }

    let front = current_card.clone().unwrap().get_front();
    let back  = current_card.clone().unwrap().get_back();

    println!("{front}");
    let input = input();
    if input == "q" { return }
    else if input == back
    {
        println!("CORRECT!");

        // rs.mark_correct(&current_card.as_ref().unwrap());
        let mut new_card   = current_card.unwrap();
        let next_show_date = Local::now() + Duration::minutes(2);
        new_card.set_next_show_date(next_show_date);
        deck.update_card(&new_card).unwrap(); // Works
        decks.update_deck(&deck);             // Does works
        storage.save(&decks);
    }
    else if input != back 
    {
        println!("INCORRECT!");
        println!("Correct Answer: {back}");
        let mut new_card   = current_card.unwrap();
        let next_show_date = Local::now() + Duration::minutes(1);
        new_card.set_next_show_date(next_show_date);
        deck.update_card(&new_card).unwrap(); // Works
        decks.update_deck(&deck);             // Does works
        storage.save(&decks);
    }
}

fn input() -> String
{
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    std::io::stdout().flush().unwrap();

    return buffer.trim().to_string();
}
