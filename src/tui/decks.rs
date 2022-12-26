use serde::{Serialize, Deserialize};

use crate::tui::deck::Deck;

#[derive(Debug,Clone, Serialize, Deserialize)]
pub struct Decks 
{
    decks: Vec<Deck>
}

impl Decks 
{
    pub fn add_deck(&mut self, deck: Deck) { todo!()}
    pub fn remove_deck(&mut self, deck: Deck) { todo!()}
    pub fn rename_deck(&mut self, deck: Deck) { todo!()}
    pub fn edit_deck(&mut self, deck: Deck) { todo!()}
}
