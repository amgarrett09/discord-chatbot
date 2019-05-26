use cursive::views::{Button, Dialog, DummyView, LinearLayout, TextArea, TextView};

// Function that returns the front-end config view
pub fn init_view() -> Dialog {
    let on_off = LinearLayout::vertical()
        .child(TextView::new("Enable or disable module"))
        .child(Button::new("Turn on", |_| ()))
        .child(Button::new("Turn off", |_| ()));

    let warn_words = LinearLayout::vertical()
        .child(TextView::new("Warn words:"))
        .child(TextView::new("(message is deleted and user is warned)"))
        .child(TextArea::new());

    let kick_words = LinearLayout::vertical()
        .child(TextView::new("Ban words:"))
        .child(TextView::new("(message is deleted and user is kicked)"))
        .child(TextArea::new());

    let words_layout = LinearLayout::horizontal()
        .child(warn_words)
        .child(DummyView)
        .child(kick_words);

    let main_layout = LinearLayout::vertical()
        .child(on_off)
        .child(DummyView)
        .child(words_layout);

    Dialog::around(main_layout).title("Profanity filter configuration")
}
