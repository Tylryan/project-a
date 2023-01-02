use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;

use cursive::align::Align;
use cursive::event::{Key, EventResult, Callback};
use cursive::theme::{Style, Effect};
use cursive::{traits::*, Cursive};
use cursive::views::{Button, LinearLayout, Panel, Dialog, EditView, DummyView, TextView, ResizedView, SelectView, ListView, TextArea};
use cursive::views::OnEventView;
use cursive::event;

type StudyCard = ResizedView<ResizedView<Panel<LinearLayout>>>;


use crate::common::card::Card;
use crate::common::deck::Deck;
use crate::common::decks::Decks;
use crate::common::study::ReviewSystem;
use crate::storage::db_handler::DbHandler;
// Grab decks from storage
//

pub struct App 
{
    pub mode: Mode,
    pub screen: Screen,
    pub decks: Decks,
    pub db: DbHandler,
    pub review_system: Option<ReviewSystem>,
    pub current_card: RefCell<Box<Option<Card>>>
    // Decks
}

impl App 
{
    pub fn new() -> Self 
    {
        let db = DbHandler::new("./test");

        Self {
            mode: Mode::Normal,
            screen: Screen::Home,
            decks: db.get_decks(),
            db: DbHandler::new("./test"),
            review_system: None,
            current_card: RefCell::new(Box::new(None))
        }
    }
    pub fn run() {
        let mut siv = cursive::default();
        let app     = App::new();
        siv.set_user_data(app);

        siv.add_global_callback('q', |s| s.quit());
        siv.add_global_callback(Key::Backspace, |s| {
            let screen = s.with_user_data(|app: &mut App| app.screen.to_owned()).unwrap();
            let mode = s.with_user_data(|app: &mut App| app.mode.to_owned()).unwrap();
            match screen 
            {
                Screen::Home => return,
                Screen::Study =>{
                    s.pop_layer().unwrap();
                    s.pop_layer().unwrap();
                    return;
                },
                _ => {
                    s.pop_layer().unwrap();
                    return;
                }
            }
        });
        siv.add_global_callback(event::Key::Esc, |s| {
            let screen  = s.with_user_data(|app: &mut App |{return app.screen.to_owned();});
            let mode = s.with_user_data(|app: &mut App| {
                return app.mode.to_owned();
            });
            match screen.unwrap() 
            {

                Screen::Home => return,
                _ => {}
            }
            match mode.unwrap()
            {
                Mode::Insert => {
                    s.screen_mut().add_transparent_layer(DummyView);
                    s.with_user_data(|app: &mut App| app.mode = Mode::Normal);
                },
                Mode::Normal => {}
            }
        });
        siv.add_global_callback('i', |s| {
            let screen  = s.with_user_data(|app: &mut App |{return app.screen.to_owned();});
            let mode = s.with_user_data(|app: &mut App| {
                return app.mode.to_owned();
            });
            match screen.unwrap() 
            {

                Screen::Home => return,
                _ => {}
            }
            match mode.unwrap()
            {
                Mode::Normal => {
                    s.pop_layer();
                    s.with_user_data(|app: &mut App| app.mode = Mode::Insert);
                },
                Mode::Insert => {}
            }
        });

        siv.add_global_callback('H', |s| {
            s.pop_layer().unwrap();
            home_page(s);
        });

        let home_page = home_page(&mut siv);
        siv.add_layer(home_page);

        siv.run();
    }
}
#[derive(Debug, Clone)]
pub enum Mode 
{
    Insert,
    Normal
}

#[derive(Debug, Clone)]
pub enum Screen 
{
    Home,
    Study,
    Difficulty,
}


fn home_page(s: &mut Cursive) -> LinearLayout
{
    s.with_user_data(|app: &mut App| app.screen = Screen::Home);
    match s.pop_layer() 
    {
        Some(_) => {},
        None    => {}
    }


    let bold = Style::from(Effect::Bold);
    let underline = Style::from(Effect::Underline);
    let bold_underline = Style::merge(&[bold,underline]);
    let decks = s.with_user_data(|app : &mut App| app.decks.list_decks()).unwrap();
    let deck_names = decks.iter().map(|d| d.get_name());
    // let deck_names = vec!["hello","there", "bob", "how", "is", "today","treating","you", "today", "oh","you know"];
    let mut decks: SelectView<String> = SelectView::new()
        .align(Align::center_left())
        // .set_on_submit(study_page);
        .on_submit(|s,deck_name: &str| { 
            //TODO
            let deck: Deck = s.with_user_data(|app: &mut App| {
                return app.decks.get_deck(&deck_name).unwrap();
            }).unwrap();

            let storage = s.with_user_data(|app: &mut App| app.db.to_owned()).unwrap();
            // {
                let mut rs = ReviewSystem::new(&deck, &storage);
                rs.generate_study_deck();
                s.with_user_data(|app: &mut App| app.review_system = Some(rs));
            // }
            let rs = s.with_user_data(|app: &mut App| app.review_system.to_owned());

            let current_card = Rc::new(rs.unwrap().unwrap().get_current_card());
            match current_card.as_ref()
            {
                Some(card) => {
                    let study_page = get_study_card(s, card.to_owned());
                    s.add_layer(study_page);
                    s.screen_mut().add_transparent_layer(DummyView);
                },
                // Go home
                None => 
                {
                    let no_cards = Dialog::new()
                        .content(TextView::new("No cards to study!"))
                        .title("Whoop!");
                    let no_cards = OnEventView::new(no_cards)
                        .on_event(Key::Esc, |s| {s.pop_layer();})
                        .on_event(Key::Enter, |s| {s.pop_layer();})
                        ;
                    s.add_layer(no_cards);
                }
            }
            // }
        });

    for value in deck_names
    {
        decks.add_item(value.to_string(), value.to_string());
    }

    let decks = OnEventView::new(decks)
        .on_pre_event_inner('k', |d,_| 
                            {
                                let cb = d.select_up(1);
                                Some(EventResult::Consumed(Some(cb)))
                            })
        .on_pre_event_inner('j', |d,_|
                            {
                                let cb = d.select_down(1);
                                Some(EventResult::Consumed(Some(cb)))
                            });

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


fn setup_session(siv: &mut Cursive,deck_name: String) 
{
    let deck: Deck = siv.with_user_data(|app: &mut App| {
        return app.decks.get_deck(&deck_name).unwrap();
    }).unwrap();

    let storage = siv.with_user_data(|app: &mut App| app.db.to_owned()).unwrap();
    let mut rs = ReviewSystem::new(&deck, &storage);
    rs.generate_study_deck();
}

fn get_study_card(siv: &mut Cursive, card: Card) -> StudyCard
{
    siv.with_user_data(| app: &mut App| app.screen = Screen::Study);

    let answer_box = EditView::new()
        .on_submit(|s, answer| {
            // s.pop_layer().unwrap();
            let difficulty_screen = get_difficulty(s);
            s.add_layer(difficulty_screen);
        });
    let back_door = Dialog::new()
                           .content(answer_box)
                           .padding_top(1)
                           .min_height(5);

    let front_door = Dialog::new()
                    .content(TextView::new(card.get_front()).align(Align::center()))
                    .min_height(5);

    let study_card = Panel::new(LinearLayout::vertical()
            .child(DummyView)
            .child(TextView::new("Front")
                   .h_align(cursive::align::HAlign::Center)
                   )
            .child(front_door)
            .child(DummyView)
            .child(TextView::new("Back")
                   .h_align(cursive::align::HAlign::Center)
                   )
            .child(back_door)

            )
            .title("Study")
            .min_width(50)
            .min_height(20);
    return study_card;
}

fn get_difficulty(s: &mut Cursive) -> StudyCard
{
    s.with_user_data(|app: &mut App| {app.screen = Screen::Difficulty});
    let select_view = SelectView::new() 
        .item_str("Easy")
        .item_str("Ok")
        .item_str("Hard")
        .selected(1)
        .align(Align::bot_center())
        .on_submit(|s, difficulty:&str| {
            s.pop_layer();
            s.pop_layer();
            // get_study_card(s, card);
            // let msg = format!("You picked {difficulty} difficulty");
            // // let next_card = get_study_card(siv, deck_name, first_card)
            // s.add_layer(Dialog::new()
            //             .content(TextView::new(msg)))
        });
    // select_view.align(Align::bot_center());

    let select_view = OnEventView::new(select_view)
        .on_pre_event_inner('k', |d,_| 
                            {
                                let cb = d.select_up(1);
                                Some(EventResult::Consumed(Some(cb)))
                            })
        .on_pre_event_inner('j', |d,_|
                            {
                                let cb = d.select_down(1);
                                Some(EventResult::Consumed(Some(cb)))
                            });

    let select_view = LinearLayout::vertical()
        .child(TextView::new("Difficulty")
               .align(Align::bot_center()))
        .child(DummyView)
        .child(select_view);

    let difficulty_screen = Panel::new(select_view)
        .min_width(30)
        .max_height(20) ;

    return difficulty_screen;

}
