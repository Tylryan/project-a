use chrono::{prelude::*, Duration};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Difficulty 
{
    Easy,
    Normal,
    Hard,
    Wrong,
    None
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum CardStatus 
{
    Unseen,
    Seen,
    New,
    Review,
    Mature,
}

#[derive(Debug, Clone)]
pub struct Card 
{
    front: String,
    back: String,
    difficulty: Difficulty,
    date_created: DateTime<Local>,
    last_show_date: DateTime<Local>,
    next_show_date: DateTime<Local>,
    times_correct: usize
}

impl PartialEq for Card 
{
    fn eq(&self, other: &Self) -> bool
    {
        let fronts_equal = self.front == other.front;
        let backs_equal  = self.back == other.back;

        return fronts_equal && backs_equal;
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
