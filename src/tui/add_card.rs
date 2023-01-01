use cursive::{
    traits::*,
    views::{
        Checkbox, Dialog, EditView, LinearLayout, ListView, SelectView,
        TextArea, TextView,
    }, CursiveRunnable, event::{Event, EventResult},
};

// This example uses a ListView.
//
// ListView can be used to build forms, with a list of inputs.

fn main() {
    let mut siv = cursive::default();
    siv.add_global_callback(Event::Char('a'), |s| s.add_layer(add_card()));
    siv.add_global_callback(Event::Char('e'), |s| s.add_layer(edit_card()));
    siv.add_global_callback(Event::Char('r'), |s| s.add_layer(remove_card()));
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

fn remove_card() -> Dialog 
{
    let mut list_view = ListView::new()
        .child("Deck Name: ", TextView::new("English").center())
        .delimiter();
    let cards = vec!["Card1", "Card2", "Card3"];
    for i in cards 
    {
        list_view.add_child("", TextView::new(i.to_owned()) );
    }
    Dialog::new()
        .title("Remove Card")
        .padding_top(2)
        .button("Ok", |s| s.quit())
        .content( list_view)
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
