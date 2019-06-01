// These are used to report errors to the front end of the application
// in the form of Cursive views.
use cursive::views::Dialog;
use cursive::Cursive;

pub fn panic(app: &mut Cursive, err_text: &str) {
    let dialog = Dialog::text(err_text)
        .button("Quit", Cursive::quit)
        .title("Fatal Error!");

    app.add_layer(dialog);
}

pub fn config_write_err(app: &mut Cursive) {
    let text =
        "Failed to write config file.\n\nYou can continue, and your settings will \
         remain for the current session, but they will not be remembered next time you start \
         the app.\n\nContinue?";

    let dialog = Dialog::text(text)
        .button("Yes", |a| {
            a.pop_layer();
            a.pop_layer();
        })
        .button("No", Cursive::quit);

    app.add_layer(dialog);
}
