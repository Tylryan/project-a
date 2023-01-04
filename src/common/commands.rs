use std::path::PathBuf;

use crate::{
    common::{self, traits::*}      ,
    storage::db_handler::DbHandler ,
    user::deck_handler::DeckHandler,
};

pub struct Commands { }

impl Commands 
{
    pub fn add_deck(deck_name: String)    
    { 
        let config_path = "./test";
        let deck_buffer = format!("{config_path}/decks/{deck_name}.deck").to_pathbuf();
        let new_deck    = common::deck::Deck::new(&deck_buffer);

        DeckHandler::add_deck(&new_deck, config_path);
        let storage = DbHandler::new(config_path);
        storage.add_deck(&new_deck);
    }

    // Just a POC, not going to implement it for awhile
    pub fn add_subdeck(deck_name: &str) 
    {
        let config_dir = "./test";
        let subdeck_limit = 2;
        let deck_names: Vec<&str> = deck_name.split("::").collect();

        if deck_names.len() > subdeck_limit 
        {
            eprintln!("Error: Cannot add deck `{}`. Current subdeck limit is {}.",
                      deck_names[2], subdeck_limit);
        }

        let deck_one_path = format!("{config_dir}/decks/{}.deck", 
                                    deck_names[0]).to_pathbuf();
        let mut deck_two_path = PathBuf::new();

        if deck_names.len() == 2 
        {
            deck_two_path.push(
                format!("{config_dir}/decks/{}-{}.deck", 
                        deck_names[0],
                        deck_names[1]));
        }

        let storage = DbHandler::new(config_dir);
        if !deck_one_path.exists() 
        {
            let new_deck = common::deck::Deck::new(&deck_one_path);
            DeckHandler::add_deck(&new_deck, config_dir);
            storage.add_deck(&new_deck);
        }
        if !deck_two_path.exists() 
        {
            let new_deck = common::deck::Deck::new(&deck_two_path);
            DeckHandler::add_deck(&new_deck, config_dir);
            storage.add_deck(&new_deck);
            return;
        }

        return eprintln!("Error: Deck `{}` already exists!", deck_names[1]);
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
    pub fn remove_subdeck(deck_name: String) { todo!() }
    pub fn rename_subdeck(deck_name: String) { todo!() }
}
