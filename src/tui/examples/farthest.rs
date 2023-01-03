use cursive::{
    traits::*,
    views::{
        Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView,
        TextArea, TextView, Button, self, ScrollView, ResizedView, Panel,
    }, CursiveRunnable, event::{Event, EventResult, Key}, reexports::crossbeam_channel::Select, Cursive, CursiveExt,
};

use crossterm::event::{self, KeyCode, KeyEvent};
// This example uses a ListView.
//
// ListView can be used to build forms, with a list of inputs.

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback(Event::Char('a'), |s| s.add_layer(add_card()));
    siv.add_global_callback(Event::Char('e'), |s| s.add_layer(edit_card()));
    siv.add_global_callback(Event::Char('r'), |s| s.add_layer(remove_card()));
    siv.add_global_callback(Event::Key(Key::Backspace), |s| {s.pop_layer().unwrap();});
    siv.set_global_callback('q', |s| s.quit());


    siv.run();
}

fn add_card() -> Dialog
{
    Dialog::new()
        .title("Add Card")
        .padding_top(2)
        .button("Ok", |s| s.quit())
        .content(
            ListView::new()
                .child("Deck Name: ", TextView::new("English").center())
                .delimiter()
                .child("Front", EditView::new().min_width(4))
                .child("Back", EditView::new().min_width(4))
                // Delimiter currently are just a blank line
                .delimiter()
                .min_width(40)
                // .child(
                //     "Age",
                //     // Popup-mode SelectView are small enough to fit here
                //     SelectView::new()
                //         .popup()
                //         .item_str("0-18")
                //         .item_str("19-30")
                //         .item_str("31-40")
                //         .item_str("41+"),
                // )
                // .scrollable(),
        )
}

fn remove_card() -> ResizedView<ScrollView<Dialog>>
{
    let mut select_view: SelectView<String> = SelectView::new();
    select_view.set_on_submit(print_something);

    fn print_something(siv: &mut Cursive, item: &str ) 
    {
        let msg = format!("Deleted: {item}");
        let msg_view = TextView::new(msg);

        // siv.pop_layer();
        siv.add_layer(
            LinearLayout::vertical()
                .child(msg_view)
                .child(Button::new("Ok", |siv| {siv.pop_layer().unwrap();}))
            );
        // siv.on_event(Event::Char('w')).with(|s| {siv.pop_layer().unwrap();});


        // println!("This is the key code: {:#?}", a.unwrap());
    }
    let cards = vec!["Card1", "Card2", "Card3", "Card4","Card5"];

    for i in cards 
    {
        select_view.add_item(i,i.into());
    }
    // list_view.add_child("", select_view)

    Dialog::around(
        LinearLayout::vertical()
        .child( select_view)
        .min_width(50)
        )
        .title("Remove Card")
        .padding_top(2)
        // .button("Ok", |s| s.quit())
        .scrollable()
        .max_height(8)
}
fn edit_card() -> Dialog 
{
    Dialog::new()
        .title("Edit Card")
        .padding_top(2)
        .button("Ok", |s| s.quit())
        .content(
            ListView::new()
                .child("Deck Name: ", TextView::new("English").center())
                .delimiter()
                .child("Front", EditView::new().min_width(4))
                .child("Back", EditView::new().min_width(4))
                // Delimiter currently are just a blank line
                .delimiter()
                .min_width(40)
        )
}
