mod bot;

#[macro_use]
extern crate serenity;

use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::mpsc;
use std::thread;

fn main() {
    let mut app = Cursive::default();

    app.set_user_data(Data {
        token: String::new(),
    });

    // Sender goes to backend thread. Messages travel backend -> frontend.
    let (txb, _): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // Receiver goes to backend thread. Messages travel frontend -> backend.
    let (txf, rxb): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let cb_sink = app.cb_sink().clone();

    thread::spawn(move || {
        bot::run_bot(&txb, &rxb, cb_sink);
    });

    main_menu(&mut app, txf);
    check_for_token(&mut app);

    app.run();
}

struct Data {
    token: String,
}

fn check_for_token(app: &mut Cursive) {
    let path = Path::new("tokens.cfg");

    // Buffer for token config file contents
    let mut buffer = String::new();

    // Try to open token config file and read from it, if it exists
    let file = File::open(&path);
    if let Ok(f) = file {
        let mut file = f;

        if let Err(why) = file.read_to_string(&mut buffer) {
            panic!("Couldn't read from token config file: {:?}", why);
        }
    }

    // If we didn't find a token
    if !buffer.contains(":") {
        let edit_name = EditView::new().with_id("app_name");

        let edit_token = EditView::new().with_id("token");

        let text1 = TextView::new(
            "We couldn't find an existing application token, \
             so you'll need to enter a name for your bot and its application token.\
             \n\n\
             Enter your bot's name:",
        )
        .fixed_width(25);

        let layout = LinearLayout::vertical()
            .child(text1)
            .child(edit_name)
            .child(DummyView)
            .child(TextView::new("Enter the application token:"))
            .child(edit_token);

        let dialog = Dialog::around(layout)
            .title("Enter bot's name")
            .button("Ok", |a| {
                let name = a
                    .call_on_id("app_name", |view: &mut EditView| view.get_content())
                    .unwrap();

                let token = a
                    .call_on_id("token", |view: &mut EditView| view.get_content())
                    .unwrap();

                ok(a, &name, &token);
            })
            .button("Quit", Cursive::quit);

        app.add_layer(dialog);
    } else {
        // If we do find a token, put it in user storage
        let fields: Vec<&str> = buffer.split(":").collect();

        let token_slice = fields[fields.len() - 1];

        let token = format! {"{}", token_slice.trim()};

        app.with_user_data(|data: &mut Data| {
            data.token = token;
        });
    }

    // Private function which runs when we hit "Okay" on the dialog above
    fn ok(app: &mut Cursive, name: &str, token: &str) {
        let s: String = format!("{}: {}\n", name, token);

        let path = Path::new("tokens.cfg");
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open token config file: {:?}", why),
        };

        if let Err(why) = file.write_all(s.as_bytes()) {
            panic!("Couldn't write token to file: {:?}", why);
        }

        // Save token in user data
        app.with_user_data(|data: &mut Data| {
            data.token = format! {"{}", token};
        });

        app.pop_layer();

        app.add_layer(Dialog::text("Saved application token!").button("Ok", |a| {
            a.pop_layer();
        }));
    }
}

fn main_menu(app: &mut Cursive, tx: mpsc::Sender<String>) {
    let launch = Button::new("Launch bot", move |a| {
        let token = &a.user_data::<Data>().unwrap().token;
        tx.send(token.to_string()).unwrap();
    });

    let configure = Button::new("Configure", |_| ());

    let layout = LinearLayout::vertical()
        .child(launch)
        .child(DummyView)
        .child(configure)
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    let dialog = Dialog::around(layout).title("Welcome");

    app.add_layer(dialog);
}
