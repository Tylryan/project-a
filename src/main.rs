mod cli;
mod common;
mod tui;
mod user;
mod storage;

use cli::cli_parser;
use common::{card::Card, deck::Deck};
use std::{env, path::PathBuf};

use crate::storage::db_handler::DbHandler;

fn main() 
{
    let config_path = "./test";
    let deck_name = "bye";
    let deck_path = format!("{config_path}/{deck_name}.deck");
    let deck_buf = PathBuf::from(&deck_path);
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    // storage::db_handler::DbHandler::new(config_path);
    let storage   = DbHandler::new(config_path);
    let card      = Card::new("Hi".into(),"world".into());
    let mut decks = storage.get_decks();
    let mut deck = match decks.get_deck(deck_name.into())
    {
        Some(d) => d,
        None    => Deck::new(&deck_buf)
    };
    // println!("{:#?}", deck);
    // decks.remove_deck(&deck);
    // deck.add_card(card).unwrap();
    decks.add_deck(deck.clone()).unwrap();
    decks.update_deck(&deck);
    // println!("{:#?}", deck);
    storage.save(&decks);
    // for i in decks.list_decks() 
    // {
    //     println!("{}", i.get_name());
    // }

    

    // let deck_name = "new".into();
    // Commands::add_deck(deck_name);
    // let card1 = card::Card::new("Hello".into(), "There".into());
    // let mut card2 = card::Card::new("hi".into(), "There".into());
    // card2.set_difficulty(card::Difficulty::Hard);

    // let path = PathBuf::from("./example.deck");
    // let mut deck = deck::Deck::new(&path);
    // // deck.edit_deck();
    // deck.read_from_file(&path);
    // deck.add_card(card1).unwrap();
    // deck.add_card(card2).unwrap();

    // println!("{deck:#?}");


    // println!("{:?}", card2.get_difficulty());
}
