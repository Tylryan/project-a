use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::common::card::Card;
use crate::user::deck_handler::DeckHandler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck 
{
    name: String,
    cards: Vec<Card>,
    path: PathBuf,
    unseen_count: usize,
    max_daily_review: usize,
    max_daily_new: usize
}

impl PartialEq for Deck 
{
    fn eq(&self, other: &Deck) -> bool 
    {
        return self.name.to_lowercase() == other.name.to_lowercase();
    }
}

impl Deck 
{
    pub fn new(path: &PathBuf) -> Self
    { 
        let path: PathBuf     = path.clone();
        let deck_name: String = path.file_stem()
            // .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Self {
            name: deck_name,
            cards: Vec::new(),
            path,
            unseen_count: 0,
            max_daily_review: 5,
            max_daily_new: 5,
        }
    }

    pub fn read_from_file(&mut self, path: &PathBuf)
    {
        let deck_as_vec: Vec<String> = DeckHandler::read_to_vec(path).unwrap();
        for row in deck_as_vec
        {
            let new_card: Card = DeckHandler::row_to_card(&row);
            self.add_card(&new_card, false).unwrap();
        }
    }

    pub fn list_cards(&self) -> Result<Vec<String>, std::io::Error>
    {
        return Ok(DeckHandler::read_to_vec(&self.path)?);
    }
    pub fn list_db_cards(&self) -> Vec<Card> 
    {
        return self.cards.to_owned();
    }
    pub fn add_card(&mut self, new_card: &Card, syncing: bool) -> Result<(), String>
    {
        let card = self.cards.iter().find(|c| *c == new_card);
        if card.is_some() && !syncing 
        {
            let error_msg = "This card is already in the deck";
            return Err(error_msg.into());
        }

        self.cards.push(new_card.to_owned());
        self.unseen_count +=1;

        return Ok(());
    }

    pub fn remove_card(&mut self, card: Card){}
    pub fn edit_deck(&mut self)
    {
        // let terminal: String = std::env::var("TERM").unwrap_or("xterm".into());
        let editor: String  = std::env::var("EDITOR").unwrap_or("vim".into());
        let path: &str      = self.path.to_str().unwrap();
        let command: String = format!("{editor} {path}");
        std::process::Command::new("bash")
            .args(["-c", &command])
            .spawn().unwrap()
            .wait().unwrap();
    }

    pub fn get_name(&self) -> String 
    {
        return self.name.clone();

    }

    pub fn set_name(&mut self, name: &str) 
    {
        self.name = String::from(name);
    }

    pub fn get_path(&self) -> PathBuf 
    {
        return self.path.to_owned();
    }
}
