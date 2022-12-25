use std::path::PathBuf;
use std::process::exit;

use crate::commands::Commands;
use crate::cli::cli_parser::{Object, Objects, ListObject, ListObjects, Deck, Card};
use crate::deck_reader::Reader;


pub fn add(object: &Object) 
{
    match object.object
    {
        Objects::Deck(ref deck) => { add_deck(deck) },
        Objects::Card(ref card) => { add_card(&card.card_name.clone(), &card.deck_name) }
    }
}

// exe add deck <deck>
fn add_deck(deck: &Deck) 
{
    let deck_name  = deck.deck_name.clone();
    let decks_path = format!("./decks/{deck_name}.deck");
    if PathBuf::from(&decks_path).exists() 
    {
        return eprintln!("Error: Deck `{deck_name}` already exists!");
    }

    Commands::add_deck(deck.deck_name.clone());
}

// exe add card front::back deck_name
fn add_card(front_back: &str, deck_name: &str) 
{
    let deck_path_str = format!("./decks/{deck_name}.deck");
    let deck_path = PathBuf::from(&deck_path_str);
    // check if deck exists
    if !deck_path.exists() 
    {
        return eprintln!("Error: Deck `{deck_name}` not found!");
    }
    // check if card exists
    let split_f_b: Vec<String> = front_back.split("::").map(|x| x.to_string()).collect();
    // No front would be checked by clap
    let front = split_f_b[0].clone();
    let back  = split_f_b.get(1);
    if back.is_none()
    {
        eprintln!("Error: {front} has no definition!");
        return;
    }
    if back.unwrap().eq("") 
    {
        eprintln!("Error: {front} has no definition!");
        return;
    }

    Reader::add_cli_card(&front, back.unwrap(), &deck_path);
}

pub fn remove(object: &Object) 
{
    match object.object
    {
        Objects::Deck(ref deck) => { remove_deck(deck) },
        Objects::Card(ref card) => {todo!()}
    }
}

// exe remove deck <deck>
fn remove_deck(deck: &Deck)
{
    let deck_name  = deck.deck_name.clone();
    let decks_path = format!("./decks/{deck_name}.deck");
    if !PathBuf::from(&decks_path).exists() 
    {
        return eprintln!("Error: Deck `{deck_name}` not found!");
    }

    Commands::remove_deck(deck.deck_name.clone())
}

pub fn edit(object: &Object) 
{
    match object.object
    {
        Objects::Deck(ref deck) => { edit_deck(deck) },
        Objects::Card(ref card) => {todo!()}
    }
}

// exe edit deck <deck>
fn edit_deck(deck: &Deck) 
{
    let deck_name  = deck.deck_name.clone();
    let decks_path = format!("./decks/{deck_name}.deck");
    if !PathBuf::from(&decks_path).exists() 
    {
        return eprintln!("Error: Deck `{deck_name}` not found!");
    }

    Commands::edit_deck(deck_name)
}

// exe edit card <front> <deck>
fn edit_card() {todo!()}

// exe list decks
pub fn list(object: &ListObject, deck_path: &PathBuf)
{
    match object.list_object
    {
        ListObjects::Decks => { list_decks(deck_path) }
    }
}

// exe list decks
fn list_decks(deck_path: &PathBuf) 
{
    Commands::list_decks(deck_path)
        .iter()
        .for_each(|deck_name| println!("{deck_name}"))
}

// exe list cards <deck>
fn list_cards(){todo!()}

