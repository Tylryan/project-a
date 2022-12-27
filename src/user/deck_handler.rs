use std::path::PathBuf;

use glob::glob;

use crate::common::card::Card;

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
    pub fn add_card(front: &str, back: &str, deck_path: &PathBuf) 
    {
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

    pub fn row_to_card(row: String) -> Card 
    {
        let front_back: Vec<String>  = row.split('=')
            .map(|x| x.trim().to_string())
            .collect();

        let front = front_back[0].clone();
        let back  = front_back[1].clone();

        return Card::new(front,back);
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
        let decks_path = format!("{config_path}/decks/*");
        let decks_list: Vec<String> = glob(&decks_path)
            .unwrap()
            .map(|x| x.unwrap().to_str()
                 .to_owned()
                 .unwrap()
                 .to_string())
            .collect();
        return decks_list;
    }
}
