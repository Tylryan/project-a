use cursive::event::{Event, Key};
use cursive::traits::*;
use cursive::views::{Dialog, EditView, OnEventView, TextArea};
use cursive::Cursive;

fn main() {
    let mut siv = cursive::default();

    // The main dialog will just have a textarea.
    // Its size expand automatically with the content.
    siv.add_layer(
        Dialog::new()
            .title("Describe your issue")
            .padding_lrtb(1, 1, 1, 0)
            .content(TextArea::new().with_name("Russian"))
            .button("Ok", Cursive::quit),

    );

    siv.run();
}
