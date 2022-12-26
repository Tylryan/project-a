use std::path::PathBuf;
use glob::glob;

use crate::user::deck_handler::DeckHandler;

pub struct Commands { }

impl Commands 
{
    pub fn add_deck(deck_name: String)    
    { 
        // Eventually will be able to add subdecks
        let config_path = "./test";
        let mut decks_path  = PathBuf::from(format!("{config_path}/decks"));
        if !decks_path.exists() 
        { 
            std::fs::create_dir(&decks_path).unwrap()
        }
        decks_path.push(format!("{deck_name}.deck"));
        std::fs::File::create(&decks_path).unwrap();
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
    pub fn add_card(front: String, back: String, deck_name: String) { todo!() }
    pub fn remove_card(front: String, deck_name: String) { todo!() }
    pub fn rename_card(front: String) { todo!() }
}
