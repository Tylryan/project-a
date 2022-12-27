use std::path::{Path, PathBuf};
use glob::glob;

use crate::{common::{commands, decks::Decks, deck::Deck}, user::deck_handler::DeckHandler};
#[derive(Debug, Clone)]
pub struct DbHandler 
{
    decks: Decks,
    file_path: PathBuf,
    config_path: String,
}

impl DbHandler 
{
    pub fn new(config_path: &str) -> Self 
    {
        let config_path_buf = PathBuf::from(&config_path);
        if !config_path_buf.exists()
        {
            let db_file = format!("{config_path}/db-file.json");
            DbHandler::new_file(&db_file);
        }

        let file_path = PathBuf::from(format!("{config_path}/db-file.json"));
        let decks     = DbHandler::read(config_path);

        Self { decks, file_path, config_path: config_path.into() }
    }
    pub fn get_decks(&self) -> Decks
    {
        return self.decks.clone();
    }
    // Before reading, update it with the user's verison
    pub fn sync(&mut self)
    {
        // let user_decks = DeckHandler::list_user_decks(&self.config_path);
        // // If no decks/ or decks/ is empty and db is empty, create empty Decks
        // let mut decks_path = PathBuf::from("./test/decks");
        // let has_decks_dir = decks_path.exists();
        // if !has_decks_dir 
        // {
        //     std::fs::create_dir(&decks_path).unwrap();
        // }

        // decks_path.push("/*");
    }

    pub fn sync_decks(&mut self) 
    {
        let user_deck_names = DeckHandler::list_user_decks(&self.config_path);
        println!("{user_deck_names:#?}");
        {
            let db_deck_names = self.decks.list_decks().iter().map(|x| x.get_name());
            println!("{db_deck_names:#?}");
        }

        // for deck in user_deck_names
        // {
        //     let deck_path = format!("{}/decks/{deck}.deck", self.config_path);

        // }
    }

    pub fn sync_cards(&mut self, deck: &Deck)
    {
        let deck_path = format!("{}/decks/{}.deck", self.config_path, deck.get_name());
        let deck_buffer = PathBuf::from(&deck_path);
        let user_cards = DeckHandler::read_to_vec(&deck_buffer).unwrap();



    }

    pub fn list_decks(&mut self) 
    {
        let a = self.decks.list_decks();


    }

    pub fn read(config_path: &str) -> Decks
    { 
        let storage_path     = format!("{config_path}/db-file.json");
        if !PathBuf::from(&storage_path).exists() 
        {
            DbHandler::new_file(&storage_path);
        }

        let storage_contents = std::fs::read_to_string(storage_path).unwrap();
        let decks: Decks     = serde_json::from_str(&storage_contents).unwrap();

        return decks;
    }

    pub fn new_file(path: &str) 
    {
        std::fs::File::create(path).unwrap();
        let new_deck = Decks::new();
        DbHandler::first_save(&new_deck, path);

    }
    // Save json to file
    fn first_save(decks: &Decks, path: &str)
    {
        let json_decks = serde_json::to_string(decks).unwrap();
        std::fs::write(path, json_decks).unwrap()
    }
    pub fn save(&self, decks: &Decks)
    {
        let json_decks = serde_json::to_string(decks).unwrap();
        std::fs::write(self.file_path.clone(), json_decks).unwrap()
    }
    // Pull from User's decks and update db
    pub fn add_deck(&self, deck: &Deck)
    {
        let deck_buffer = PathBuf::from(format!("{}/{}",self.config_path, deck.get_name()));
        let storage   = DbHandler::new(&self.config_path);
        let mut decks = storage.get_decks();
        let deck = match decks.get_deck(deck.get_name())
        {
            Some(d) => d,
            None    => Deck::new(&deck_buffer)
        };
        decks.update_deck(&deck);
        storage.save(&decks);
    }
}
