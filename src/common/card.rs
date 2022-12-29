use std::cmp::Ordering;

use chrono::{prelude::*, Duration};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Difficulty 
{
    Easy,
    Normal,
    Hard,
    Wrong,
    None
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum CardStatus 
{
    Unseen,
    Seen,
    New,
    Review,
    Mature,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card 
{
    front: String,
    back: String,
    difficulty: Difficulty,
    status: CardStatus,
    date_created: DateTime<Local>,
    last_show_date: DateTime<Local>,
    next_show_date: DateTime<Local>,
    times_correct: usize
}

impl PartialEq for Card 
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.front == other.front;
    }

    fn ne(&self, other: &Self) -> bool 
    {
        self.front != other.front
    }
}

impl Card 
{
    pub fn new(front: String, back: String) -> Self 
    {
        let current_date   = Local::now();
        let next_show_date = current_date + Duration::minutes(1);
        Self {
            front,
            back,
            difficulty: Difficulty::None,
            status: CardStatus::Unseen,
            date_created: current_date,
            last_show_date: current_date,
            next_show_date,
            times_correct: 0
        }
    }

    pub fn set_front(&mut self, front: String) { self.front = front; }
    pub fn get_front(&self) -> String          { self.front.clone()  }
    pub fn set_back(&mut self, back: String)   { self.back = back;   }
    pub fn get_back(&self) -> String           { self.back.clone()   }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) 
    {
        self.difficulty = difficulty;
    }

    pub fn get_difficulty(&self) -> Difficulty 
    { 
        self.difficulty.clone() 
    }
    pub fn get_status(&self) -> CardStatus 
    {
        self.status.to_owned()
    }
    pub fn set_status(&mut self, status: CardStatus) 
    {
        self.status = status
    }

    pub fn set_last_show_date(&mut self, last_show_date: DateTime<Local>) 
    { 
        self.last_show_date = last_show_date; 
    }
    pub fn get_last_show_date(&self) -> DateTime<Local> 
    { 
        self.last_show_date.clone() 
    }

    pub fn set_next_show_date(&mut self, next_show_date: DateTime<Local>) 
    { 
        self.next_show_date = next_show_date; 
    }
    pub fn get_next_show_date(&self) -> DateTime<Local> 
    { 
        self.next_show_date.clone() 
    }

    pub fn get_times_correct(&self) -> usize 
    { 
        self.times_correct.clone() 
    }
    pub fn set_times_correct(&mut self, times_correct: usize)
    { 
        self.times_correct = times_correct; 
    }
}
