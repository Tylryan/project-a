use cursive::{
    Cursive          ,
    event::{self,Key},
    views::DummyView ,
};

use crate::tui::{
    tui::{Screen, App, Mode},
    homepage::home_page     ,
};

pub fn set_initial_callbacks(siv: &mut Cursive) 
{
    siv.add_global_callback('q', |s| s.quit());
    siv.add_global_callback(Key::Backspace, |s| {
        let screen = s.with_user_data(|app: &mut App| app.screen.to_owned()).unwrap();

        match screen 
        {
            Screen::Home  => return      ,
            Screen::Study =>
            {
                s.pop_layer().unwrap();
                s.pop_layer().unwrap();
                return;
            }                            ,
            other         => 
            {
                s.pop_layer().unwrap();
                return;
            }                            ,
        }
    });
    siv.add_global_callback(event::Key::Esc, |s| {
        let screen  = s.with_user_data(|app: &mut App |{ return app.screen.to_owned() });
        let mode    = s.with_user_data(|app: &mut App| { return app.mode.to_owned()   });
        match screen.unwrap() 
        {
            Screen::Home => return,
            other        => {}    ,
        }
        match mode.unwrap()
        {
            Mode::Insert => 
            {
                s.screen_mut().add_transparent_layer(DummyView);
                s.with_user_data(|app: &mut App| app.mode = Mode::Normal);
            }                                                               ,
            Mode::Normal => {}                                              ,
        }
    });

    siv.add_global_callback('i', |s| {
        let screen  = s.with_user_data(|app: &mut App |{ return app.screen.to_owned() });
        let mode    = s.with_user_data(|app: &mut App| { return app.mode.to_owned()   });

        match screen.unwrap() 
        {
            Screen::Home => return,
            other => {}           ,
        }

        match mode.unwrap()
        {
            Mode::Normal => 
            {
                s.pop_layer();
                s.with_user_data(|app: &mut App| app.mode = Mode::Insert);
            }                                                                ,
            Mode::Insert => {}                                               ,
        }
    });

    siv.add_global_callback('H', |s| {
        s.pop_layer().unwrap();
        home_page(s);
    });
}
