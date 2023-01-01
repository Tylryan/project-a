use cursive::align::Align;
use cursive::event::{Key, EventResult, Callback};
use cursive::theme::{Style, Effect};
use cursive::{traits::*, Cursive};
use cursive::views::{Button, LinearLayout, Panel, Dialog, EditView, DummyView, TextView, ResizedView, SelectView, ListView, TextArea};
use cursive::views::OnEventView;
use cursive::event;

type StudyCard = ResizedView<ResizedView<Panel<LinearLayout>>>;


use crate::common::decks::Decks;
use crate::storage::db_handler::DbHandler;
// Grab decks from storage
//

pub struct App 
{
    pub mode: Mode,
    pub screen: Screen,
    pub decks: Decks,
    pub db: DbHandler,
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
            db: DbHandler::new("./test")
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

        home_page(&mut siv);

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


fn home_page(s: &mut Cursive)
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
        .on_submit(|s,item: &str| { 
            let study_page = get_study_card(s, item.to_string());
            s.add_layer(study_page);
            s.screen_mut().add_transparent_layer(DummyView);
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

    let home_page = Panel::new(
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
        .with_name("home");
    s.add_layer(home_page);
}

fn get_study_card(siv: &mut Cursive, front: String) -> StudyCard
{
    siv.with_user_data(| app: &mut App| app.screen = Screen::Study);
    let answer_box = EditView::new()
        .on_submit(|s, answer| {
            let difficulty_screen = get_difficulty(s);
            s.add_layer(difficulty_screen);
        });
    let back_door = Dialog::new()
                           // .title_position(cursive::align::HAlign::Left)
                           .content(answer_box)
                           .padding_top(1)
                           // .title("Russian")
                           .min_height(5);
                           // .button("Quit", Cursive::quit))
    let front_door = Dialog::new()
                    // .title("English")
                    .content(TextView::new(front).align(Align::center()))
                    .min_height(5);

    let study_card = Panel::new(LinearLayout::vertical()
            .child(DummyView)
            .child(TextView::new("English")
                   .h_align(cursive::align::HAlign::Center)
                   )
            .child(front_door)
            .child(DummyView)
            .child(TextView::new("Russian")
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
        .align(Align::bot_center())
        .on_submit(|s, difficulty:&str| {
            s.pop_layer();
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
