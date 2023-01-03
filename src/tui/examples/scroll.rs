use cursive::traits::Resizable;
use cursive::view::Scrollable;
use cursive::views::{Button, Canvas, Dialog, LinearLayout};
use cursive::Printer;

fn main() {
    let mut siv = cursive::default();

    let a = Dialog::around(
            LinearLayout::vertical()
                // .child(Button::new("Foo", |s| s.add_layer(Dialog::info("Ah"))))
                .child(Canvas::new(()).with_draw(draw).fixed_size((120, 40)))
                // .child(Button::new("Bar", |s| s.add_layer(Dialog::info("Uh"))))
                .scrollable()
                .show_scrollbars(false)
                .scroll_x(true)
        )
        .fixed_size((60, 30));

    siv.add_layer( a );

    siv.add_global_callback('q', |s| s.quit());

    siv.run();
}

fn draw(_: &(), p: &Printer) {
    for x in 0..p.size.x {
        for y in 0..p.size.y {
            let c = (x + 6 * y) % 10;
            p.print((x, y), &format!("{}", c));
        }
    }
}
