use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli
{
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands
{
    /// Adds a Deck, or adds Card to Deck
    Add { arg_one: String, arg_two: Option<String> },
    /// Removes a Deck, or removes a Card from Deck
    Remove { arg_one: String, arg_two: Option<String> },
    /// Renames a Deck, or Renames a Card in Deck
    Rename { arg_one: String, arg_two: Option<String> },
    /// Edit a deck of cards
    Edit { deck: String},
}

impl Cli 
{
    pub fn run_clap() 
    {
        let cli = Cli::parse();
        match &cli.commands {
            Commands::Add { arg_one, arg_two } => 
            {
                match arg_two 
                {
                    Some(s) => println!("Add {} to {}", arg_one, s),
                    None => println!("Add {}", arg_one)
                }
            },
            Commands::Remove { arg_one, arg_two} => 
            {
                match arg_two 
                {
                    Some(s) => println!("Remove {} to {}", arg_one, s),
                    None => println!("Remove {}", arg_one)
                }
            }
            Commands::Rename { arg_one, arg_two } => 
            {
                match arg_two 
                {
                    Some(s) => println!("Rename {} to {}", arg_one, s),
                    None => println!("Rename {}", arg_one)
                }
            },
            Commands::Edit { deck } => 
            {
                println!("Edit {}", deck)
            },
        }
    }
}
