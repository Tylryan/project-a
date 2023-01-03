use crossterm::event::{read, Event, KeyCode};

fn main() 
{
    let key  = match read().unwrap() 
    {
        Event::Key(event) => Some(event.code),
        _ => None
    };
    match key.unwrap()
    {
        KeyCode::Char('w') => println!("Yup"),
        _ => println!("Nope")
    }



}
