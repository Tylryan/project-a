use std::{path::PathBuf, io::ErrorKind, process::exit};

use crate::{
    common::{ decks::Decks, deck::Deck, card::Card }, 
    user::deck_handler::DeckHandler
};
#[derive(Debug, Clone)]
pub struct DbHandler 
{
    decks: Decks,
    db_file_path: PathBuf,
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
            DbHandler::new_db_file(&db_file);
        }

        let db_file_path = PathBuf::from(format!("{config_path}/db-file.json"));
        let decks        = DbHandler::read(config_path);

        Self { decks, db_file_path, config_path: config_path.into() }
    }

    pub fn sync_decks(&mut self) 
    {
        let config_path          = self.config_path.clone();
        let user_deck_names      = DeckHandler::list_user_decks(&self.config_path);
        let db_decks: Vec<Deck>  = self.get_decks().list_decks();
        let mut new_decks: Decks = Decks::new();

        for user_deck_name in user_deck_names 
        {
            let db_deck = db_decks.iter().find(|d| d.get_name() == user_deck_name );
            // If user deck has one that db doesn't, update db
            if db_deck.is_none() 
            {
                let deck_path    = format!("{config_path}/decks/{user_deck_name}.deck");
                let deck_buffer  = PathBuf::from(&deck_path);
                let mut new_deck = Deck::new(&deck_buffer);

                self.sync_cards(&mut new_deck);
                new_decks.add_deck(new_deck, true).unwrap();
                continue;
            }

            // Once
            let mut db_deck = db_deck.unwrap().clone();
            self.sync_cards(&mut db_deck);
            new_decks.add_deck(db_deck, true).unwrap();
        }

        let storage = DbHandler::new(&config_path);
        self.decks  = new_decks.to_owned();
        storage.save(&self.decks);
    }

    pub fn sync_cards(&mut self, deck: &mut Deck)
    {
        let deck_path = format!("{}/decks/{}.deck", self.config_path, deck.get_name());
        let deck_buffer = PathBuf::from(&deck_path);
        let user_cards = DeckHandler::read_to_vec(&deck_buffer).unwrap();
        let db_cards = deck.list_card_names();

        // Add cards to DB if needed
        for user_card in user_cards.iter()
        {
            let front_back: Vec<&str> = user_card
                .split('=')
                .collect();

            if front_back.first().unwrap() == &"" { break }


            let front   = front_back[0].trim().to_string();
            let back    = front_back.get(1);
            let db_card = db_cards.iter()
                .find(|c| *c == &front);

            if db_card.is_none() 
            {
                println!("Added card: {}", user_card);
                let back = back.unwrap().trim().to_string();
                let new_card = Card::new(front, back);
                deck.add_card(&new_card, true).unwrap();
            }
        }

        // Remove cards from db if needed
        for db_card in db_cards 
        {
            let user_card = DeckHandler::find_index(&db_card, &user_cards);
            if user_card.is_some() {continue}
            
            let card = deck.get_cards().iter()
                .find(|c| c.get_front() == db_card)
                .unwrap()
                .to_owned();

            deck.remove_card(card);
        }
        // Don't save cards to db here
    }

    pub fn get_decks(&self) -> Decks
    {
        return self.decks.clone();
    }

    pub fn list_cards(&mut self, deck: &Deck) -> Vec<Card>
    {
        return deck.list_db_cards();
    }

    pub fn read(config_path: &str) -> Decks
    { 
        let storage_path     = format!("{config_path}/db-file.json");
        if !PathBuf::from(&storage_path).exists() 
        {
            DbHandler::new_db_file(&storage_path);
        }

        let err_msg = format!("Error: `{storage_path}` does not exist!");
        let storage_contents = std::fs::read_to_string(storage_path)
            .expect("Error: `{storage_path}` does not exist!");
        let decks: Decks     = serde_json::from_str(&storage_contents).unwrap();

        return decks;
    }

    pub fn new_db_file(path: &str) 
    {
        match std::fs::File::create(path)
        {
            Ok(_) => {},
            Err(e) => match e.kind() 
            {
                ErrorKind::NotFound         => 
                {
                    return eprintln!("Error: File `{path}` not found!");
                },
                ErrorKind::PermissionDenied => 
                {
                    return eprintln!("Permission Denied Error: Could not open `{path}`");

                },
                _other_error                => 
                {
                    return eprintln!("Error: problem opening {path}");
                }
            }
        }

        let new_deck = Decks::new();
        DbHandler::first_save(&new_deck, path);

    }
    // Save json to file
    fn first_save(decks: &Decks, path: &str)
    {
        let json_decks = serde_json::to_string_pretty(decks).unwrap();
        std::fs::write(path, json_decks).unwrap()
    }
    pub fn save(&self, decks: &Decks)
    {
        let json_decks = serde_json::to_string_pretty(decks).unwrap();
        std::fs::write(self.db_file_path.clone(), json_decks).unwrap()
    }
    // Pull from User's decks and update db
    pub fn add_deck(&self, deck: &Deck)
    {
        let deck_buffer = PathBuf::from(format!("{}/{}",self.config_path, deck.get_name()));
        let storage     = DbHandler::new(&self.config_path);
        let mut decks   = storage.get_decks();

        let deck = match decks.get_deck(&deck.get_name())
        {
            Some(d) => d,
            None    => Deck::new(&deck_buffer)
        };

        decks.update_deck(&deck);
        storage.save(&decks);
    }

    pub fn add_card(&mut self, card: &Card, deck_name: &str) 
    {
        let deck_buffer = PathBuf::from(format!("{}/{}",self.config_path, deck_name));
        let storage     = DbHandler::new(&self.config_path);
        let mut decks   = storage.get_decks();

        let mut deck = match decks.get_deck(deck_name)
        {
            Some(d) => d,
            None    => Deck::new(&deck_buffer)
        };

        deck.add_card(&card, false).unwrap();
        decks.update_deck(&deck);
        storage.save(&decks);
        self.decks = decks.to_owned();
    }

    pub fn remove_deck(&mut self, deck_name: &str) 
    {
        let storage   = DbHandler::new(&self.config_path);
        let mut decks = storage.get_decks();

        let deck = self.clone().decks.get_deck(deck_name.into());
        if deck.is_none() 
        {
            return eprintln!("Error: `{deck_name}` does not exist!");
        }

        decks.remove_deck(&deck.as_ref().unwrap());
        storage.save(&decks);
        self.decks = decks.to_owned();
    }

    pub fn rename_deck(&mut self, deck_name: &str, new_name: &str) 
    {
        let storage   = DbHandler::new(&self.config_path);
        let mut decks = storage.get_decks();

        let deck = self.clone().decks.get_deck(deck_name.into());

        if deck.is_some()
        {
            return eprintln!("Error: `{new_name}` already exists!");
        }

        let mut deck = deck.unwrap();
        deck.set_name(new_name);
        decks.update_deck(&deck);
        storage.save(&decks);
    }
}
