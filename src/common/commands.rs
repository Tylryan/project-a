use std::path::PathBuf;

use crate::storage::db_handler::DbHandler;
use crate::user::deck_handler::DeckHandler;
use crate::common;

pub struct Commands { }

impl Commands 
{
    // Eventually will be able to add subdecks
    pub fn add_deck(deck_name: String)    
    { 
        let config_path = "./test";
        let deck_path   = format!("{config_path}/decks/{deck_name}.deck");
        let deck_buffer = PathBuf::from(format!("{deck_path}"));
        let new_deck    = common::deck::Deck::new(&deck_buffer);

        DeckHandler::add_deck(&new_deck, config_path);
        let storage = DbHandler::new(config_path);
        storage.add_deck(&new_deck);
    }

    // Eventually will be able to add subdecks
    pub fn remove_deck(deck_name: String) 
    { 
        let config_path = "./test";

        DeckHandler::remove_deck(&deck_name, config_path);
        let mut storage = DbHandler::new(config_path);
        storage.remove_deck(&deck_name);
    }

    // This should probably only list what's in the database
    pub fn list_decks() -> Vec<String>
    {
        let config_path = "./test";

        let deck_names: Vec<String>  = DeckHandler::list_user_decks(config_path);
        return deck_names;
    }

    pub fn list_cards(deck_path: &PathBuf) -> Vec<String> 
    {
        return DeckHandler::read_to_vec(deck_path).unwrap();
    }

    pub fn edit_deck(deck_name: String) 
    {
        let config_path       = "./test";
        DeckHandler::edit_deck(&deck_name, config_path);
        //TODO somehow sync the edited deck with the database
    }

    pub fn rename_deck(deck_name: String, new_name: String) 
    {
        let config_path       = "./test";
        DeckHandler::rename_deck(&deck_name, &new_name, config_path);
        let mut storage = DbHandler::new(config_path);
        storage.rename_deck(&deck_name, &new_name);
    }

    pub fn add_card(card: &common::card::Card, deck_name: &str) 
    { 
        let config_path   = "./test";
        DeckHandler::add_card(&card, &deck_name, config_path);
        let mut storage = DbHandler::new(config_path);
        storage.add_card(&card, &deck_name);
    }
    pub fn remove_card(front: String, deck_name: String) { todo!() }
    pub fn rename_card(front: String) { todo!() }
    pub fn add_subdeck(decks_names: String) { todo!()}
    pub fn remove_subdeck(deck_name: String) { todo!() }
    pub fn rename_subdeck(deck_name: String) { todo!() }
}
