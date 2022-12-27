use std::path::PathBuf;

use glob::glob;

use crate::common;

// When dealing with a deck's contents, use the deck reader.
pub struct DeckHandler { }

impl DeckHandler 
{
    // Read deck to vec
    pub fn read_to_vec(path: &PathBuf) -> Result<Vec<String>, std::io::Error>
    {
        let file_contents: Vec<String> = std::fs::read_to_string(path)?
            .trim()
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        let file_contents = DeckHandler::clean_comments(file_contents);
        
        return Ok(file_contents);
    }

    // NOTE: This should work for TUI as well
    pub fn add_card(card: &common::card::Card, deck_name: &str, config_path: &str) 
    {
        let deck_path_str = format!("./test/decks/{deck_name}.deck");
        let deck_path     = PathBuf::from(&deck_path_str);
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
        let front = card.get_front();
        let back  = card.get_back();

        let mut og_cards: Vec<String> = DeckHandler::read_to_vec(&deck_path).unwrap();
        // If the card already exists in the deck
        if let Some(_) = DeckHandler::find_index(&front, &og_cards) 
        {
            return eprintln!("Error: Card `{}` already in deck!", front);
        }

        let mut new_cards = String::new();
        let empty_deck    = og_cards.len() == 1 && og_cards[0] == "";
        let new_card      = format!("{front} = {back}");

        if empty_deck 
        { 
            std::fs::write(deck_path, new_card).unwrap();
            return;
        }

        og_cards.push(new_card);
        new_cards = og_cards.join("\n");
        std::fs::write(deck_path, new_cards).unwrap();

    }

    pub fn row_to_card(row: &str) -> common::card::Card 
    {
        let front_back: Vec<String>  = row.split('=')
            .map(|x| x.trim().to_string())
            .collect();

        let front = front_back[0].clone();
        let back  = front_back[1].clone();

        return common::card::Card::new(front,back);
    }
    pub fn find_index(front: &str, deck_vec: &Vec<String>) -> Option<usize> 
    {
        for (index,row) in deck_vec.iter().enumerate() 
        {
            let card_front: String = row.split('=')
                .map(|x| x.to_string())
                .collect::<Vec<String>>()[0]
                .trim()
                .to_lowercase();

            if card_front == front.to_lowercase() { return Some(index); }
        }

        return None;
    }

    pub fn clean_comments(hector: Vec<String>) -> Vec<String>
    {
        let mut new_hector: Vec<String> = Vec::new();
        for line in hector.iter()
        {
            if line.starts_with("#") { continue }
            else if line.contains("#") 
            { 
                let cleaned_line = line.split("#")
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .get(0)
                    .unwrap()
                    .to_owned();
                new_hector.push(cleaned_line);
            } 
            else { new_hector.push(line.clone()); }
        }
        return new_hector;
    }

    pub fn list_user_decks(config_path: &str) -> Vec<String> 
    {
        let mut deck_names: Vec<String> = Vec::new();
        let deck_path  = format!("{config_path}/decks/*.deck");
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

    pub fn remove_deck(deck_name: &str, config_path: &str) 
    {
        let deck_path   = PathBuf::from(format!("{config_path}/decks/{deck_name}.deck"));

        if deck_path.exists() 
        { 
            std::fs::remove_file(&deck_path).unwrap();
        }
        else 
        {
            return eprintln!("Error: Deck `{deck_name}` was not found!");
        }
    }

    pub fn edit_deck(deck_name: &str, config_path: &str) 
    {
        let decks_path = format!(".test/decks/{deck_name}.deck");
        if !PathBuf::from(&decks_path).exists() 
        {
            return eprintln!("Error: Deck `{deck_name}` not found!");
        }
        let deck_path: String = format!("{config_path}/decks/{deck_name}.deck");
        let editor: String    = std::env::var("EDITOR").unwrap();
        let command: String   = format!("{editor} {deck_path}");
        std::process::Command::new("bash")
            .args(["-c", &command])
            .spawn()
            .unwrap()
            .wait().unwrap();
    }
    pub fn rename_deck(deck_name: &str, new_name: &str, config_path: &str) 
    {
        let deck_path: String = format!("{config_path}/decks/{deck_name}.deck");
        let new_name: String  = format!("{config_path}/decks/{new_name}.deck");

        std::fs::rename(deck_path, new_name).unwrap();

    }

    pub fn add_deck(deck: &common::deck::Deck, config_path: &str) 
    {
        let deck_name = deck.get_name();
        let decks_dir   = PathBuf::from(format!("{config_path}/decks"));

        if !decks_dir.exists() 
        { 
            std::fs::create_dir(&decks_dir).unwrap()
        }

        let deck_buffer = deck.get_path();

        if deck_buffer.exists()
        {
            return eprintln!("Error: Deck `{deck_name}` already exists!");
        }

        // Create it for the user
        std::fs::File::create(&deck_buffer).unwrap();
    }
}
