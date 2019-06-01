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
