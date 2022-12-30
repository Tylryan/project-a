use serde::{Serialize, Deserialize};

use crate::common::deck::Deck;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Decks 
{
    decks: Vec<Deck>,
    count: usize
}

impl Decks 
{
    pub fn new() -> Self
    {
        Self { decks: Vec::new(), count: 0 }
    }
    pub fn add_deck(&mut self, deck: Deck, syncing: bool) -> Result<(), String>
    {
        if !syncing
        {
            // Check if deck exists
            let deck_exists = self.decks.iter().find(|d| *d == &deck);
            if deck_exists.is_some() 
            {
                let err_msg = format!("Error: Deck `{}` already exists", deck.get_name());
                return Err(err_msg);
            }

            self.count +=1;
        }

        self.decks.push(deck);
        return Ok(());
    }

    pub fn update_deck(&mut self, deck: &Deck) 
    {
        let deck_exists = self.get_deck(&deck.get_name());
        
        if deck_exists.is_some() 
        {
            self.remove_deck(deck);
        }

        self.add_deck(deck.to_owned(), false).unwrap();
    }

    pub fn remove_deck(&mut self, deck: &Deck) 
    {
        let mut new_decks: Vec<Deck> = Vec::new();

        for d in self.decks.iter()
        {
            if d != deck 
            {
                new_decks.push(d.to_owned());
            } 
            else if self.count == 0 { }
            else { self.count -=1;}
        }
        self.decks = new_decks;
    }

    pub fn get_deck(&self, deck_name: &str) -> Option<Deck>
    {
        let deck = self.decks.iter().find(|x| x.get_name() == deck_name);
        match deck 
        {
            Some(d) => Some(d.to_owned()),
            None => None 
        }
    }

    pub fn rename_deck(&mut self, deck: Deck) { todo!()}
    pub fn edit_deck(&mut self, deck: Deck) { todo!()}
    pub fn list_decks(&self)  -> Vec<Deck>
    {
        return self.decks.clone();
    }
}
