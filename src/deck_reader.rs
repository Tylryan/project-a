use std::path::PathBuf;

use crate::card::Card;

pub struct Reader { }

impl Reader 
{
    pub fn read_to_vec(path: &PathBuf) -> Result<Vec<String>, std::io::Error>
    {
        let file_contents: Vec<String> = std::fs::read_to_string(path)?
            .trim()
            .split('\n')
            .map(|x| x.to_string())
            .collect();
        let file_contents = Reader::clean_comments(file_contents);
        println!("{file_contents:?}");
        
        return Ok(file_contents);
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
}
