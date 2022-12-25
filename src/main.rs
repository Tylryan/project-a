mod cli;
mod deck;
mod decks;
mod card;
mod deck_reader;
mod commands;

use cli::cli_parser;

use commands::Commands;
use std::env;
fn main() 
{
    if env::args().len() > 1 
    {
        return cli_parser::Cli::run_clap();
    }

    let deck_name = "new".into();
    Commands::add_deck(deck_name);
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
