use std::path::PathBuf;

use crate::commands::Commands;
use crate::cli::cli_parser::{Object, Objects, ListObject, ListObjects};


pub fn add_deck(object: &Object) 
{
    // If deck exists, do nothing
    match object.object
    {
        Objects::Deck(ref deck) => Commands::add_deck(deck.deck_name.clone()),
    }
}

pub fn remove_deck(object: &Object) 
{
    // If deck doesn't exist, do nothing
    match object.object
    {
        Objects::Deck(ref deck) => Commands::remove_deck(deck.deck_name.clone()),
    }
}
pub fn edit_deck(object: &Object) 
{
    // If deck doesn't exist, do nothing
    match object.object
    {
        Objects::Deck(ref deck) => Commands::edit_deck(deck.deck_name.clone()),
    }
}

pub fn list_decks(object: &ListObject, deck_path: &PathBuf)
{
    match object.list_object
    {
        ListObjects::Decks => 
        {
            Commands::list_decks(deck_path)
                .iter()
                .for_each(|deck_name| println!("{deck_name}"));
        }
    }
}
