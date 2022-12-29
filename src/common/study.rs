// - Grab X Unseen cards (new)
// - Grab X Review cards (review)
// - Combine both and sort by self.show_next

// Get question wrong
//     - self.times_correct = 0
//         - reset times correct
//     - self.next_show += Duration::minutes(1);
//         - Show again in 1 minute
//     - self.status = CardStatus::Review
// Get question right
//     - if >= 3 times in a row; 
//         - self.next_show += Duration::day(self.times_correct - 2);
//     - else
//         - self.next_show += Duration::minutes(self.times_correct);
//     - self.times_correct +=1
//     - if self.show_next - today > 1 month;
//         - self.status = CardStatus::Mature

use std::borrow::BorrowMut;

use chrono::Duration;

// This might not be useful to do it this way
use crate::common::{
    deck::Deck,
    card::Card,
};

pub struct ReviewSystem
{
    deck: Deck,
    pub study_cards: Vec<Card>,
    current_card: Option<Card>
}

impl ReviewSystem
{
    pub fn new(deck: &Deck) -> Self
    {
        Self { 
            deck: deck.to_owned(), 
            study_cards: Vec::new(),  
            current_card: None,
        }
    }

    pub fn get_current_card(&self) -> Option<Card>
    {
        if let Some(current_card) = self.study_cards.get(0) 
        {
            return Some(current_card.to_owned());
        }
        return None;
    }

    pub fn generate_study_deck(&mut self)
    {
        self.set_study_cards();
        self.deck.sort();
    }

    pub fn set_study_cards(&mut self) 
    {
        let max_new_cards    = self.deck.get_max_daily_new() + 1;
        let max_review_cards = self.deck.get_max_daily_review() +1;
        self.study_cards     = self.get_review_cards(max_review_cards);
        let new_cards        = self.get_new_cards(max_new_cards);

        self.study_cards.extend(new_cards);
    }

    pub fn get_review_cards(&mut self, mut max_review_cards: usize) -> Vec<Card>
    {
        // self.deck.get_cards().sort_by_key(|c| c.get_next_show_date());
        let review_cards = self.deck.get_review();

        if review_cards.len() == 0 { max_review_cards = 0 }
        else if max_review_cards >= review_cards.len() 
        {
            max_review_cards = review_cards.len();
        }

        let review_cards = review_cards[..max_review_cards].to_owned();

        return review_cards;
    }

    pub fn mark_correct(&mut self, card: &Card) 
    {
        // let cards = self.deck.get_cards();
        // cards.iter_mut().find(|c| *c == card).unwrap() = card.borrow_mut();
    }
    pub fn get_new_cards(&mut self, mut max_new_cards: usize) -> Vec<Card>
    {
        let new_cards = self.deck.get_unseen();

        if new_cards.len() == 0 { max_new_cards = 0 }
        else if max_new_cards >= new_cards.len() 
        {
            max_new_cards = new_cards.len();
        }
        let new_cards = new_cards[..max_new_cards].to_owned();

        return new_cards;
    }
}
