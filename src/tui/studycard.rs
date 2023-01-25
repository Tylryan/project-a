use std::fs::write;

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
    }, 
    CursiveExt                                ,
};

use crate::{
    storage::db_handler::DbHandler          ,
    common::{
        card::{Card, CardStatus, Difficulty},
        deck::Deck                          ,
        decks::Decks                        ,
        study::ReviewSystem                 ,
    }, tui::{global_callbacks, homepage}                                       ,
};

use super::{tui::{Screen, StudyCard, App, Mode}, homepage::{get_current_card, home_page}};

// fn get_study_card_window(siv: &mut Cursive, card: Card) -> StudyCard
pub fn get_study_card_window(siv: &mut Cursive, card: Card) -> StudyCard
{
    siv.with_user_data(| app: &mut App| app.screen = Screen::Study);
    siv.with_user_data(| app: &mut App| app.mode = Mode::Insert);

    let answer_box = EditView::new()
        .on_submit(submit_answer)
        .with_name("answer") ;

    let back_door = Dialog::new()
                           .content(answer_box)
                           .padding_top(1)
                           .min_height(5);

    let front_door = Dialog::new()
                    .content(
                        TextView::new(card.get_front())
                            .align(Align::center())
                            .with_name("question"))
                    .min_height(5);

    let study_card = Panel::new(LinearLayout::vertical()
            .child( DummyView )
            .child( front_door)
            .child( DummyView )
            .child( back_door )
            )
            .title("Study")
            .min_width(50)
            .min_height(15);
    return study_card;
}

pub fn get_difficulty_window(s: &mut Cursive) -> StudyCard
{
    s.with_user_data(|app: &mut App| {app.screen = Screen::Difficulty});
    let select_view= SelectView::new() 
        .item("Easy", Difficulty::Easy)
        .item("Ok", Difficulty::Normal)
        .item("Hard", Difficulty::Hard)
        .selected(1)
        .align(Align::bot_center())
        .with_name("difficulty");

    let select_view = OnEventView::new(select_view)
        .on_pre_event_inner('k', |d,_| 
                        {
                            let cb = d.get_mut().select_up(1);
                            Some(EventResult::Consumed(Some(cb)))
                        })
        .on_pre_event_inner('j', |d,_|
                        {
                            let cb = d.get_mut().select_down(1);
                            Some(EventResult::Consumed(Some(cb)))
                        }) ;

    let select_view = LinearLayout::vertical()
        .child(TextView::new("Difficulty").align(Align::bot_center()))
        .child(DummyView)
        .child(select_view) ;

    let difficulty_screen = Panel::new(select_view)
        .min_width(30)
        .max_height(20) ;

    return difficulty_screen;

}



pub fn submit_answer(s: &mut Cursive, answer: &str) 
{
    let current_card = s.with_user_data(|app: &mut App| app.current_card.to_owned()).unwrap().unwrap();

    // If answer is correct
    if answer == current_card.get_back() 
    {
        let difficulty_window = get_difficulty_window(s);
        s.add_layer(difficulty_window);

        s.call_on_name("difficulty", |view: &mut SelectView<Difficulty>| {
            view.set_on_submit(submit_correct_answer);
        });

    } 
    // If answer is incorrect
    else { submit_incorrect_answer(s); }

    // Prep next card
    s.with_user_data(|app: &mut App| app.review_system.to_owned().unwrap().generate_study_deck());
    let current_deck = s.with_user_data(|app: &mut App| app.current_deck.to_owned()).unwrap().unwrap();
    let study_cards = s.with_user_data(|app: &mut App| app.review_system.to_owned().unwrap().study_cards).unwrap();

    // Wrong here
    // does not increment to next card
    // get_current_card does appear to work fine though
    let next_card    = get_current_card(s, &current_deck);


    s.call_on_name("question", |view: &mut TextView| {
        view.set_content(next_card.to_owned().unwrap().get_front());
    });
    s.call_on_name("answer", |view: &mut EditView| {
        view.set_content("");
    });
}

pub fn submit_correct_answer(siv: &mut Cursive, difficulty: &Difficulty)
{
        siv.pop_layer();
        let current_card   = siv.with_user_data(|app: &mut App| app.current_card.to_owned()).unwrap().unwrap();
        let mut new_card   = current_card.to_owned();
        let times_correct  = new_card.get_times_correct() +1;
        let mut multiplier = times_correct.clone() as i64;

        // TODO: This 2 should be dynamic an come from a config file
        if new_card.get_times_correct() >= 2
        {
            multiplier         = (times_correct - 2) as i64;
            let next_show_date = new_card.get_last_show_date() + Duration::days(multiplier);
            new_card.set_next_show_date(next_show_date);
        }
        else 
        {
            let next_show_date = Local::now() + (Duration::minutes(2));
            new_card.set_next_show_date(next_show_date);
        }

        new_card.set_difficulty(difficulty.to_owned());
        new_card.set_status(CardStatus::Review);
        new_card.set_last_show_date(Local::now());
        new_card.set_times_correct(times_correct);
        new_card.set_status(CardStatus::Review);

        // Error
        // Not updating DB
        // let storage          = DbHandler::new("./test");
        let mut current_deck = siv.with_user_data(|app: &mut App| app.current_deck.to_owned()).unwrap().unwrap();
        let storage   = siv.with_user_data(|app: &mut App| app.db.to_owned()).unwrap();
        let mut decks = siv.with_user_data(|app: &mut App| app.decks.to_owned()).unwrap();

        current_deck.update_card(&new_card).unwrap();
        // Updates the review cards in deck
        current_deck.set_review();
        // current_deck.set_unseen();

        decks.update_deck(&current_deck);
        storage.save(&decks);


        siv.with_user_data(|app: &mut App| app.current_deck = Some(current_deck.to_owned()));
        // s.with_user_data(|app: &mut App| app.db = DbHandler::new("./test"));
        // siv.with_user_data(|app: &mut App| app.db = storage);
        siv.with_user_data(|app: &mut App| app.decks = decks);

        // Not sure what this is doing
        siv.with_user_data(|app: &mut App| app.review_system.to_owned().unwrap().generate_study_deck());

        let next_card = get_current_card(siv, &current_deck);
        if next_card.is_none()
        {
            let no_cards = Panel::new(LinearLayout::vertical()
                .child(Dialog::around(TextView::new("No cards left!").center()))
                .child(TextView::new("Press <Enter> to continue")))
                .title("Finished!");
            let no_cards = OnEventView::new(no_cards)
                .on_event( Key::Esc  , pop_layer )
                .on_event( Key::Enter, pop_layer );

            fn pop_layer(siv: &mut Cursive) 
            { 
                siv.pop_layer();
                siv.pop_layer();
                // siv.pop_layer();
            }

            siv.add_layer(no_cards);
            siv.with_user_data(|app: &mut App| app.screen = Screen::Home);

            return;
        }

}
pub fn submit_incorrect_answer(siv: &mut Cursive) 
{
    let current_card = siv.with_user_data(|app: &mut App| app.current_card.to_owned()).unwrap().unwrap();
    let mut new_card   = current_card.to_owned();
    let next_show_date = Local::now() + Duration::minutes(1);

    new_card.set_status(CardStatus::Review);
    new_card.set_next_show_date(next_show_date);
    new_card.set_last_show_date(Local::now());
    new_card.set_times_correct(0);
    new_card.set_difficulty(Difficulty::Wrong);

    let mut current_deck = siv.with_user_data(|app: &mut App| app.current_deck.to_owned()).unwrap().unwrap();
    let storage          = DbHandler::new("./test");
    let mut decks        = siv.with_user_data(|app: &mut App| app.decks.to_owned()).unwrap();

    current_deck.update_card(&new_card).unwrap();
    // Updates review cards in deck
    current_deck.set_review();
    current_deck.set_unseen();
    decks.update_deck(&current_deck);
    storage.save(&decks);

    siv.with_user_data(|app: &mut App| app.current_card = Some(new_card));
    siv.with_user_data(|app: &mut App| app.current_deck = Some(current_deck));
    // s.with_user_data(|app: &mut App| app.db = DbHandler::new("./test"));
    siv.with_user_data(|app: &mut App| app.decks = decks);
    siv.with_user_data(|app: &mut App| app.review_system.to_owned().unwrap().generate_study_deck());

}
