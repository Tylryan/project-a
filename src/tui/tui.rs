use cursive::{
    views::{LinearLayout, Panel, ResizedView}, Cursive};

use crate::{
    storage::db_handler::DbHandler,
    common::{
        card::Card                ,
        deck::Deck                ,
        decks::Decks              ,
        study::ReviewSystem       ,
    }                             ,
    tui::{
        global_callbacks::set_initial_callbacks,
        homepage::home_page                    ,
    }
};

pub type StudyCard = ResizedView<ResizedView<Panel<LinearLayout>>>;

#[derive(Debug, Clone)]
pub enum Mode 
{
    Insert,
    Normal,
}

#[derive(Debug, Clone)]
pub enum Screen 
{
    Home      ,
    Study     ,
    Difficulty,
}

pub struct App
{
    pub mode:          Mode                ,
    pub screen:        Screen              ,
    pub decks:         Decks               ,
    pub db:            DbHandler           ,
    pub review_system: Option<ReviewSystem>,
    // The should probably just come from review_system
    pub current_card:  Option<Card>        ,
    pub current_deck:  Option<Deck>        ,
}
// Create functions that pull from the review system and make life easier.

impl App 
{
    pub fn new() -> Self 
    {
        let db = DbHandler::new("./test");

        Self {
            mode:          Mode::Normal  ,
            screen:        Screen::Home  ,
            decks:         db.get_decks(),
            db:            db            ,
            review_system: None          ,
            current_card:  None          ,
            current_deck:  None          ,
        }
    }
    pub fn run() {
        let mut siv = cursive::default();
        let app     = App::new();
        // set_theme(&mut siv);

        siv.set_user_data(app);
        set_initial_callbacks(&mut siv);

        let home_page = home_page(&mut siv);
        siv.add_layer(home_page);

        siv.run();
    }
}

fn set_theme(siv: &mut Cursive) 
{
    let mut palette = cursive::theme::Palette::default();
    // Eventually this will go in a config that the user can change
    use cursive::theme::PaletteColor::*;
    use cursive::theme::Color;
    let dark_black  = Color::parse("dark black").unwrap();
    let light_green = Color::parse("light green").unwrap();
    let dark_white  = Color::parse("dark white").unwrap();
    let cyan        = Color::parse("dark cyan").unwrap();
    let none        = Color::TerminalDefault;
    palette[Background]        = none;
    palette[Shadow]            = none;
    palette[View]              = none;
    palette[Primary]           = none;
    palette[Secondary]         = none;
    palette[Tertiary]          = none;
    palette[TitlePrimary]      = none;
    palette[TitleSecondary]    = none;
    palette[Highlight]         = none;
    palette[HighlightInactive] = none;
    palette[HighlightText]     = cyan;

    let theme = cursive::theme::Theme{
        palette,
        shadow: true,
        borders: cursive::theme::BorderStyle::Simple,
    };
    siv.set_theme(theme);

}
