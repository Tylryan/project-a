use std::path::PathBuf;
use glob::glob;

use crate::storage::db_handler::DbHandler;
use crate::user::deck_handler::DeckHandler;
use crate::common;

pub struct Commands { }

impl Commands 
{
    pub fn add_deck(deck_name: String)    
    { 
        // Eventually will be able to add subdecks
        let config_path = "./test";
        let decks_dir   = PathBuf::from(format!("{config_path}/decks"));

        if !decks_dir.exists() 
        { 
            std::fs::create_dir(&decks_dir).unwrap()
        }

        let deck_path   = format!("{config_path}/decks/{deck_name}.deck");
        let deck_buffer = PathBuf::from(format!("{deck_path}"));
        let new_deck    = common::deck::Deck::new(&deck_buffer);

        if deck_buffer.exists()
        {
            return eprintln!("Error: Deck `{deck_name}` already exists!");
        }

        // Create it for the user
        std::fs::File::create(&deck_buffer).unwrap();
        // Create it for the database
        let storage = DbHandler::new(config_path);
        storage.add_deck(&new_deck);

    }
    pub fn remove_deck(deck_name: String) 
    { 
        // Eventually will be able to add subdecks
        let config_path = "./test";
        let deck_path  = PathBuf::from(format!("{config_path}/decks/{deck_name}.deck"));

        if deck_path.exists() 
        { 
            std::fs::remove_file(&deck_path).unwrap();
        }
    }

    pub fn list_decks(deck_path: &PathBuf) -> Vec<String>
    {
        let mut deck_names: Vec<String> = Vec::new();
        let deck_path  = deck_path
            .to_string_lossy()
            .to_string() + "/*";

        let deck_paths = glob(&deck_path).unwrap();

        for deck in deck_paths
        {
            let deck_name = deck.unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();
            deck_names.push(deck_name);
        }

        return deck_names;
    }

    pub fn list_cards(deck_path: &PathBuf) -> Vec<String> 
    {
        return DeckHandler::read_to_vec(deck_path).unwrap();
    }

    pub fn edit_deck(deck_name: String) 
    {
        let config_path       = "./test";
        let deck_path: String = format!("{config_path}/decks/{deck_name}.deck");
        let editor: String    = std::env::var("EDITOR").unwrap();
        let command: String   = format!("{editor} {deck_path}");
        std::process::Command::new("bash")
            .args(["-c", &command])
            .spawn()
            .unwrap()
            .wait().unwrap();
    }

    pub fn rename_deck(deck_name: String, new_name: String) 
    {
        let config_path       = "./test";
        let deck_path: String = format!("{config_path}/decks/{deck_name}.deck");
        let new_name: String = format!("{config_path}/decks/{new_name}.deck");

        std::fs::rename(deck_path, new_name).unwrap();
    }
    pub fn add_subdeck(decks_names: String) { todo!()}
    pub fn remove_subdeck(deck_name: String) { todo!() }
    pub fn rename_subdeck(deck_name: String) { todo!() }
    pub fn add_card(card: &common::card::Card, deck_name: &str) 
    { 
        let deck_path_str = format!("./test/decks/{deck_name}.deck");
        let deck_path     = PathBuf::from(&deck_path_str);
        let config_path   = "./test";
        // check if deck exists
        if !deck_path.exists() 
        {
            return eprintln!("Error: Deck `{deck_name}` not found!");
        }

        let cards: Vec<String> = DeckHandler::read_to_vec(&deck_path).unwrap();
        let front              = card.get_front();

        if let Some(_) = DeckHandler::find_index(&front, &cards) 
        {
            return eprintln!("Error: Card `{}` already in deck!", front);
        }

        DeckHandler::add_card(&card, &deck_path);
        let storage = DbHandler::new(config_path);
        storage.add_card(&card, &deck_name);
    }
    pub fn remove_card(front: String, deck_name: String) { todo!() }
    pub fn rename_card(front: String) { todo!() }
}
