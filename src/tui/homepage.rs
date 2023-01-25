use chrono::{Local, Duration};
use cursive::{
    Cursive                                   ,
    traits::*                                 ,
    align::Align                              ,
    event::{self,Key, EventResult, Event}     ,
    theme::{Style, Effect}                    ,
    views::{
        LinearLayout, Panel     , Dialog      , 
        EditView    , DummyView , TextView    ,
        ResizedView , SelectView, OnEventView ,
    }, CursiveExt                                         ,
};

use crate::{
    storage::db_handler::DbHandler,
    common::{
        card::Card                ,
        deck::Deck                ,
        study::ReviewSystem       ,
    }, 
    tui::{
        studycard::get_study_card_window ,
        tui::{Screen, App}        ,
    }
};

pub fn home_page(s: &mut Cursive) -> LinearLayout
{
    s.with_user_data(|app: &mut App| app.screen = Screen::Home);
    match s.pop_layer() 
    {
        Some(_) => {},
        None    => {},
    }

    // Make these traits or something
    let bold           = Style::from(Effect::Bold);
    let underline      = Style::from(Effect::Underline);
    let bold_underline = Style::merge(&[bold,underline]);

    let decks      = s.with_user_data(|app : &mut App| app.decks.list_decks()).unwrap();
    let deck_names = decks.iter().map(|d| d.get_name());

    let mut decks: SelectView<String> = SelectView::new()
        .align(Align::center_left())
        .on_submit(|s,deck_name: &str| { 
            let deck: Deck = s.with_user_data(|app: &mut App| { return app.decks.get_deck(&deck_name).unwrap(); }).unwrap();

            s.with_user_data(|app: &mut App| app.current_deck = Some(deck.to_owned()));
            let storage = s.with_user_data(|app: &mut App| app.db.to_owned()).unwrap();
            let mut rs = ReviewSystem::new(&deck, &storage);
            rs.generate_study_deck();
            s.with_user_data(|app: &mut App| app.review_system = Some(rs.to_owned()));
            // let rs = s.with_user_data(|app: &mut App| app.review_system.to_owned()).unwrap();
            let current_card = get_current_card(s, &deck);

            match current_card.as_ref()
            {
                Some(card) => {
                    // Simple display the current card to the user
                    let study_page = get_study_card_window(s, card.to_owned());
                    // TODO: Need to implement the study logic here in order to get next card?
                    // s.with_user_data(|app: &mut App| app.review_system.unwrap().study_cards.pop()).unwrap();
                    s.add_layer(study_page);
                    // s.screen_mut().add_transparent_layer(DummyView);
                },
                // Go home
                None => 
                {
                    let no_cards = Dialog::new()
                        .content(TextView::new("No cards to study!"))
                        .title("Whoops!");
                    let no_cards = OnEventView::new(no_cards)
                        .on_event( Key::Esc  , pop_layer )
                        .on_event( Key::Enter, pop_layer );

                    fn pop_layer(siv: &mut Cursive) { siv.pop_layer(); }

                    s.add_layer(no_cards);
                }
            }
        });

    for value in deck_names
    {
        decks.add_item(value.to_string(), value.to_string());
    }

    let decks = OnEventView::new(decks)
                    .on_pre_event_inner( 'k', move_up  )
                    .on_pre_event_inner( 'j', move_down);

    fn move_up(select_view: &mut SelectView, _: &Event) -> Option<EventResult>
    {
        let cb = select_view.select_up(1);
        Some(EventResult::Consumed(Some(cb)))
    }
    fn move_down(select_view: &mut SelectView, _: &Event) -> Option<EventResult>
    {
        let cb = select_view.select_down(1);
        Some(EventResult::Consumed(Some(cb)))
    }

    let home_page = LinearLayout::vertical()
        .child(Panel::new(
            Panel::new(LinearLayout::vertical()
                .child(TextView::new("Decks").style(bold_underline).align(Align::center()))
                .child(DummyView)
                .child(decks.scrollable())
        )
        // .title("home")
        .min_width(30)
        .min_height(5)
        .max_height(20)
        .scrollable()
        )
        .title("HOME")
        .with_name("home")
        );

    return home_page;
}

pub fn get_current_card(s: &mut Cursive, deck: &Deck) -> Option<Card>
{
    // s.with_user_data(|app: &mut App| app.db = DbHandler::new("./test")).unwrap();
    let mut storage = s.with_user_data(|app: &mut App| app.db.to_owned()).unwrap();
    storage.sync_decks();
    // let mut rs = ReviewSystem::new(&deck, &storage);

    // rs.generate_study_deck();
    // s.with_user_data(|app: &mut App| app.review_system = Some(rs.to_owned()));
    // let rs = s.with_user_data(|app: &mut App| app.review_system.to_owned()).unwrap();
    let rs = s.with_user_data(|app: &mut App| app.review_system.to_owned()).unwrap();

    let current_card = rs.unwrap().get_current_card();
    s.with_user_data(|app: &mut App| {app.current_card = current_card.to_owned();});
    return current_card;
}
