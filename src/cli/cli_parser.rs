/* cli::cli_parser 
 * Used as the CLI interface for all commands.
 *
 * E.g. Cli Parser -> CLI Functions -> Common Functions
 *
 * This will be helpful in the future as the TUI will use the same commands as 
 * the CLI. However, new commands will not have to be written, just their 
 * implementaion.
 *
 * USEFUL FILES
 *  src/cli/cli_parser
 *      This file defines how the user inputs information
 *  src/cli/commands 
 *      This file transforms the user input into a common format.
 *  src/commands 
 *      This is a set of common commands that CLI and TUI will share.
*/

use std::path::PathBuf;

use clap::{Parser, Subcommand};
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
    Edit(Object),
    Rename(Object)
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
    Card(Card),
    Deck(Deck),
}

#[derive(Parser, Debug)]
pub struct Decks { }

#[derive(Parser, Debug)]
pub struct Deck 
{
    pub deck_name: String,
    pub new_name: Option<String>
}

#[derive(Parser, Debug)]
pub struct Card 
{
    pub deck_name: String,
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
                cli::commands::add(object)
            },
            Actions::Remove(object) => 
            {
                cli::commands::remove(object)
            },
            Actions::List(object) =>
            {
                let decks_path = PathBuf::from("./decks");
                cli::commands::list(object, &decks_path)
            },
            Actions::Edit(object) =>
            {
                cli::commands::edit(object)
            },
            Actions::Rename(object) => 
            {
                cli::commands::rename(object)
            }
        }
    }
}
