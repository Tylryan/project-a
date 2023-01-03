use std::collections::HashMap;

use cursive::align::Align;
use cursive::event::Key;
use cursive::{traits::*, Cursive};
use cursive::views::{LinearLayout, Dialog, TextView, EditView, SelectView, OnEventView};

fn main() {
    let mut siv = cursive::default();
    // You put the last pages behind and pop of all others until you are where you want to be
    
    siv.add_layer(TextView::new("Finished!"));
    siv.add_global_callback('q', |s| s.quit());

    let cards: HashMap<&str, &str> = HashMap::from([
        ("hello how are things going? Just seeing if this text will wrap or not.", "there"),
        ("how","are"),
    ]);

    for (key,value) in cards.iter() 
    {
        display_card(&mut siv, key, value);
    }

    siv.run();
}

fn show_difficulty(s: &mut Cursive) 
{
    s.add_layer(LinearLayout::horizontal()
            .child( SelectView::new()
                .item("Easy", 0)
                .item("Normal", 1)
                .item("Hard", 2)
                .h_align(cursive::align::HAlign::Center)
                .on_submit(|s, i| {
                    s.pop_layer().unwrap();
                    s.pop_layer().unwrap();
                })
                .min_width(30)
            )
            .min_width(10));
}

fn display_card(s: &mut Cursive,front: &str, back: &str) 
{
    let front_card = Dialog::new()
                           .title("English")
                           .content(TextView::new(front).align(Align::center()))
                           .min_height(4);
    let back_card = Dialog::new()
                           .title("Russian")
                           // .title_position(cursive::align::HAlign::Left)
                           .content(EditView::new().with_name("russian"));
    let card_page = OnEventView::new(back_card).on_event(Key::Enter,  show_difficulty);

    s.add_layer(
        LinearLayout::vertical()
            .child(front_card)
            .child(card_page)
            .min_width(50)
            .max_width(50)
            .min_height(8)
    );
}
