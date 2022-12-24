use std::path::PathBuf;

use clap::{Parser, Subcommand};
// use crate::commands::Commands;
use crate::cli;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli
{
    #[command(subcommand)]
    action: Actions,
}

#[derive(Subcommand, Debug)]
pub enum Actions 
{
    Add(Object),
    Remove(Object),
    List(ListObject),
    Edit(Object)
}

#[derive(Parser, Debug)]
pub struct ListObject
{ 
    #[command(subcommand)]
    pub list_object: ListObjects 
}

#[derive(Subcommand, Debug)]
pub enum ListObjects 
{
    Decks
}

#[derive(Parser, Debug)]
pub struct Object
{ 
    #[command(subcommand)]
    pub object: Objects 
}

#[derive(Subcommand, Debug)]
pub enum Objects 
{
    // Card(Card),
    Deck(Deck),
}


#[derive(Parser, Debug)]
pub struct Decks 
{

}
#[derive(Parser, Debug)]
pub struct Deck 
{
    pub deck_name: String,
    pub new_name: Option<String>
}

#[derive(Parser, Debug)]
pub struct Card 
{
    pub card_name: String,
    pub new_name: Option<String>
}

impl Cli 
{
    pub fn run_clap() 
    {
        let cli = Cli::parse();
        match &cli.action 
        {
            Actions::Add(object) => 
            {
                cli::commands::add_deck(object)
            },
            Actions::Remove(object) => 
            {
                cli::commands::remove_deck(object)
            },
            Actions::List(object) =>
            {
                let decks_path = PathBuf::from("./decks");
                cli::commands::list_decks(object, &decks_path)
            },
            Actions::Edit(object) =>
            {
                cli::commands::edit_deck(object)
            }
        }
    }
}
