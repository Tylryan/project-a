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

use std::{io::Write, process::exit};

use chrono::{Duration, Local};

// This might not be useful to do it this way
use crate::{common::{
    deck::Deck,
    card::{Card, Difficulty, CardStatus},
    decks::Decks,
}, storage::db_handler::DbHandler};

// Rename to something else
pub enum Message 
{
    QuitNoMessage,
    QuitWithMessage(String),
    Continue,
}

#[derive(Debug, Clone)]
pub struct ReviewSystem
{
    deck: Deck,
    decks: Decks,
    pub study_cards: Vec<Card>,
    pub current_card: Option<Card>,
    storage: DbHandler,
    pub cards_reviewed: usize,
}

impl ReviewSystem
{
    pub fn new(deck: &Deck, storage: &DbHandler) -> Self
    {
        Self { 
            deck: deck.to_owned(),
            decks: storage.get_decks(),
            study_cards: Vec::new(),  
            current_card: None,
            storage: storage.to_owned(),
            cards_reviewed: 0,
        }
    }

    pub fn study_tui(&mut self) 
    {
        self.storage.sync_decks();
        self.generate_study_deck();

    }
    pub fn study_cli(&mut self)
    {
        // Get the deck
        // ---- Normally wouldn't do this way
        // let deck: Deck   = self.storage.get_decks()
        //     .get_deck(&self.deck.get_name())
        //     .unwrap();
        self.storage.sync_decks();
        // let rs = Self::new(&deck, &storage);
        // ------------------
        match self.study() 
        {
            Message::QuitNoMessage => return,
            Message::QuitWithMessage(message) => 
            {
                println!("{message}");
                exit(0);
            },
            Message::Continue => {}
        }
    }
    // Eventually these will be KeyBindings
    pub fn study(&mut self) -> Message
    {
        self.generate_study_deck();
        let current_card = self.get_current_card();

        if current_card.is_none() 
        {
            return Message::QuitWithMessage("No Cards to study!".into());
        }
        let front = current_card.clone().unwrap().get_front();
        let back  = current_card.clone().unwrap().get_back();
        print!("{front}: ");
        std::io::stdout().flush().unwrap();
        let input = Self::input();
        if input == "q" { return Message::QuitNoMessage;}
        else if input == back
        {
            self.card_correct(&current_card.unwrap());
        }
        else if input != back 
        {
            self.card_incorrect(&current_card.unwrap());
        }

        return Message::Continue;
    }

    pub fn card_correct(&mut self, current_card: &Card)
    {
            println!("CORRECT!");

            // rs.mark_correct(&current_card.as_ref().unwrap());
            let mut new_card   = current_card.to_owned();
            new_card.set_status(CardStatus::Review);
            new_card.set_last_show_date(Local::now());
            let times_correct = new_card.get_times_correct() + 1;
            new_card.set_times_correct(times_correct);
            let mut multiplier = times_correct.clone() as i64;
            println!("Next Show: {}", new_card.get_next_show_date());
            println!("Times correct: {}", new_card.get_times_correct());
            print!("Difficulty from 1-3?: ");
            std::io::stdout().flush().unwrap();
            match Self::input().parse().unwrap()
            {
                1 => {
                    new_card.set_difficulty(Difficulty::Easy);
                    multiplier *= 3;
                },
                2 => { new_card.set_difficulty(Difficulty::Normal); },
                3 => {
                    new_card.set_difficulty(Difficulty::Hard);
                    if multiplier == 1 {}
                    else {multiplier -=1}
                },
                _ => { new_card.set_difficulty(Difficulty::Normal); }
            };
            println!("Multiplier: {multiplier}");
            if new_card.get_times_correct() > 3 
            {
                // For the sake of readability
                multiplier = (times_correct - 3) as i64;
                println!("Multiplier {multiplier}");
                let next_show_date = new_card.get_last_show_date() + ( Duration::days(multiplier));
                new_card.set_next_show_date(next_show_date);
            } 
            else 
            {
                let next_show_date = Local::now() + ( Duration::minutes(2));
                new_card.set_next_show_date(next_show_date);
            }

            self.deck.update_card(&new_card).unwrap(); // Works
            self.deck.set_review();
            self.deck.set_unseen();
            self.decks.update_deck(&self.deck);             // Does works
            self.storage.save(&self.decks);

    }
    pub fn card_incorrect(&mut self, current_card: &Card)
    {
        println!("INCORRECT!");
        println!("Correct Answer: {}", current_card.get_back());
        let mut new_card   = current_card.to_owned();
        new_card.set_status(CardStatus::Review);
        let next_show_date = Local::now() + Duration::minutes(1);
        new_card.set_next_show_date(next_show_date);
        new_card.set_last_show_date(Local::now());
        new_card.set_times_correct(0);
        new_card.set_difficulty(Difficulty::Wrong);
        // Regardless of current card status, set all the way back to review
        self.deck.update_card(&new_card).unwrap(); // Works
        self.deck.set_review();
        self.deck.set_unseen();
        self.decks.update_deck(&self.deck);             // Does works
        self.storage.save(&self.decks);
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
        let naive_today = Local::now().naive_local() + Duration::hours(6);
        let todays_cards: Vec<Card> = self.deck.get_cards().iter().filter(|c| c.get_next_show_date().naive_local() <= naive_today)
            .map(|c| c.to_owned())
            .collect();
        self.study_cards = todays_cards;
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
    fn input() -> String
    {
        let mut buffer = String::new();
        std::io::stdin().read_line(&mut buffer).unwrap();
        std::io::stdout().flush().unwrap();

        return buffer.trim().to_string();
    }
}
