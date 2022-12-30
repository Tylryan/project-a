use std::path::PathBuf;

use serde::{Serialize, Deserialize};

use crate::common::card::Card;
use crate::user::deck_handler::DeckHandler;

use super::card::CardStatus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deck 
{
    name:             String             ,
    review:           Vec<Card>          ,
    unseen:           Vec<Card>          ,
    cards:            Vec<Card>          ,
    path:             PathBuf            ,
    unseen_count:     usize              ,
    parent_deck:      Option<String>     ,
    children_decks:   Option<Vec<String>>,
    max_daily_review: usize              ,
    max_daily_new:    usize              ,
}

impl PartialEq for Deck 
{
    fn eq(&self, other: &Deck) -> bool 
    {
        self.name.to_lowercase() == other.name.to_lowercase()
    }
}

impl Deck 
{
    pub fn new(path: &PathBuf) -> Self
    { 
        let path: PathBuf     = path.clone();
        let deck_name: String = path.file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        Self {
            path                        ,
            name:             deck_name ,
            cards:            Vec::new(),
            review:           Vec::new(),
            unseen:           Vec::new(),
            parent_deck:      None      ,
            children_decks:   None      ,
            unseen_count:     0         ,
            max_daily_review: 5         ,
            max_daily_new:    5         ,
        }
    }

    pub fn get_review(&mut self) -> Vec<Card> 
    { 
        self.set_review();
        self.review.to_owned() 
    }

    pub fn sort(&mut self) 
    {
        self.cards.sort_by_key(|c| c.get_next_show_date());
    }

    pub fn get_max_daily_review(&self) -> usize { self.max_daily_review }
    pub fn set_max_daily_review(&mut self, max_daily_review: usize) 
    {
        self.max_daily_review = max_daily_review;
    }
    pub fn get_max_daily_new(&self) -> usize { self.max_daily_new }
    pub fn set_max_daily_new(&mut self, max_daily_review: usize) 
    {
        self.max_daily_new = max_daily_review;
    }

    pub fn set_review(&mut self)
    {
        self.review = self.cards.iter().filter(|c| c.get_status() == CardStatus::Review)
            .map(|c| c.to_owned())
            .collect();
    }

    pub fn review_push(&mut self, card: &Card) 
    {
        self.review.push(card.to_owned());
    }

    pub fn get_unseen(&mut self) -> Vec<Card> { 
        self.set_unseen();
        self.unseen.to_owned() 
    }

    pub fn set_unseen(&mut self)
    {
        let unseen_list: Vec<Card> = self.cards.iter()
            .filter(|c| c.get_status() == CardStatus::Unseen)
            .map(|c| c.to_owned())
            .collect();
        self.unseen = unseen_list.to_owned();
        self.unseen_count = unseen_list.len();
    }

    pub fn update_card(&mut self, new_card: &Card) -> Result<(), String>
    {
        // If not exists, return error
        let old_card = self.get_card(&new_card.get_front());
        if old_card.is_none()
        {
            let err_msg = format!("Error: Card `{}` does not exist!",new_card.get_front());
            return Err(err_msg);

        }

        self.remove_card(old_card.unwrap());
        self.add_card(new_card, false).unwrap();
        self.set_unseen();
        // Remove existing card
        // Add new card
        return Ok(());
    }

    pub fn get_card(&self, card_front: &str) -> Option<Card>
    {
        if let Some(c) = self.cards.iter().find(|c| c.get_front() == card_front) 
        {
            return Some(c.to_owned());
        }
        return None;

    }
    pub fn set_parent_deck(&mut self, deck_name: &str) 
    {
        self.parent_deck = Some(String::from(deck_name));
    }

    pub fn get_parent_deck(&self) -> Option<String>
    {
        if let Some(parent) = self.parent_deck.to_owned()
        {
            return Some(parent);
        }

        return None;
    }

    pub fn read_from_file(&mut self, path: &PathBuf)
    {
        let deck_as_vec: Vec<String> = DeckHandler::read_to_vec(path).unwrap();
        for row in deck_as_vec
        {
            let new_card: Card = DeckHandler::row_to_card(&row);
            self.add_card(&new_card, false).unwrap();
        }
        self.set_unseen();
    }

    pub fn list_card_names(&self) -> Vec<String>
    {
        let mut card_names: Vec<String> = Vec::new();
        for card in self.cards.iter() 
        {
            card_names.push(card.get_front());
        }
        return card_names;
    }

    pub fn get_cards(&mut self) -> Vec<Card> 
    {
        return self.cards.to_owned();
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
        if !syncing { self.unseen_count +=1; }
        self.set_unseen();

        return Ok(());
    }

    pub fn remove_card(&mut self, card: Card)
    {
        self.cards = self.cards.iter()
            .filter(|c| *c != &card)
            .map(|c| c.to_owned())
            .collect();
    }
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
