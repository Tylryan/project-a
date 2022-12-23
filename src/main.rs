mod cli;
mod deck;
mod decks;
mod card;
mod deck_reader;
use std::{env, path::PathBuf};

fn main() 
{
    if env::args().len() > 1 
    {
        return cli::Cli::run_clap();
    }

    let card1 = card::Card::new("Hello".into(), "There".into());
    let mut card2 = card::Card::new("hi".into(), "There".into());
    card2.set_difficulty(card::Difficulty::Hard);

    let path = PathBuf::from("./example.deck");
    let mut deck = deck::Deck::new(&path);
    deck.edit_deck();
    // deck.read_from_file(&path);
    // deck.add_card(card1).unwrap();
    // deck.add_card(card2).unwrap();

    println!("{deck:#?}");


    // println!("{:?}", card2.get_difficulty());
}
