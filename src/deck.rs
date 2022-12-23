use std::path::PathBuf;

use crate::card::Card;
use crate::deck_reader::Reader;

#[derive(Debug, Clone)]
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
        return self.name == other.name;
    }
}

impl Deck 
{
    pub fn new(path: &PathBuf) -> Self
    { 
        let path: PathBuf     = path.clone();
        let deck_name: String = path.into_iter().last()
            .unwrap()
            .to_string_lossy()
            .split(".")
            .collect::<Vec<&str>>()[0]
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
        let deck_as_vec: Vec<String> = Reader::read_to_vec(path).unwrap();
        for row in deck_as_vec
        {
            let new_card: Card = Reader::row_to_card(row);
            self.add_card(new_card).unwrap();
        }
    }

    pub fn add_card(&mut self, new_card: Card) -> Result<(), String>
    {
        match self.cards.iter().find(|card| *card == &new_card)
        {
            Some(_) => 
            {
                let error_msg = "This card is already in the deck";
                return Err(error_msg.into());
            },
            None => self.cards.push(new_card)
        }
        return Ok(());
    }
    pub fn remove_card(&mut self, front: String, back: String){}
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
}
