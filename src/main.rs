mod bot;

use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, DummyView, TextView, EditView};
use cursive::traits::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::sync::mpsc;

fn main() {
    let mut app = Cursive::default();

    app.set_user_data(Data { key: String::new() });

    // Sender goes to backend thread. Messages travel backend -> frontend.
    let (txb, _): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // Receiver goes to backend thread. Messages travel frontend -> backend.
    let (txf, rxb): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    thread::spawn(move || {
        bot::run_bot(&txb, &rxb);
    });

    main_menu(&mut app, txf);
    check_for_key(&mut app);

    app.run();
}

struct Data {
    key: String
}

fn check_for_key(app: &mut Cursive) {
    let path = Path::new("key.cfg");

    // Buffer for key config file contents
    let mut buffer = String::new();

    // Try to open key config file and read from it, if it exists
    let file = File::open(&path);
    if let Ok(f) = file {
        let mut file = f;

        if let Err(why) = file.read_to_string(&mut buffer) {
            panic!("Couldn't read from key config file: {:?}", why);
        }
    }

    // If we didn't find a key
    if !buffer.contains(":") {
        let edit_name = EditView::new()
            .with_id("app_name");

        let edit_key = EditView::new()
            .with_id("key");

        let text1 = TextView::new("We couldn't find an existing application key, \
            so you'll need to enter a name for your bot and its application key now.\
            \n\n\
            Enter your bot's name:")
            .fixed_width(25);

        let layout = LinearLayout::vertical()
            .child(text1)
            .child(DummyView)
            .child(edit_name)
            .child(DummyView)
            .child(TextView::new("Enter the application key:"))
            .child(edit_key);

        let dialog = Dialog::around(layout)
            .title("Enter bot's name")
            .button("Ok", |a| {
                let name = a.call_on_id("app_name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();

                let key = a.call_on_id("key", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();

                ok(a, &name, &key);
            })
            .button("Quit", Cursive::quit);

        app.add_layer(dialog);
    } else {
        // If we do find a key, put it in user storage
        let fields: Vec<&str>  = buffer.split(":").collect();

        let key_slice = fields[fields.len() - 1];

        let key = format!{"{}", key_slice.trim()};

        app.with_user_data(|data: &mut Data| {
            data.key = key;
        });
    }

    // Private function which runs when we hit "Okay" on the dialog above
    fn ok(app: &mut Cursive, name: &str, key: &str) {
        let s: String = format!("{}: {}\n", name, key);

        let path = Path::new("key.cfg");
        let mut file = match File::create(&path) {
            Ok(file) => file,
            Err(why) => panic!("Couldn't open key config file: {:?}", why)
        };

        if let Err(why) = file.write_all(s.as_bytes()) {
            panic!("Couldn't write key to file: {:?}", why);
        }

        // Save key in user data
        app.with_user_data(|data: &mut Data| {
            data.key = format!{"{}", key};
        });

        app.pop_layer();

        app.add_layer(Dialog::text("Saved application key!")
            .button("Ok", |a| {
                a.pop_layer();
            }));
    }
}

fn main_menu(app: &mut Cursive, tx: mpsc::Sender<String>) {
    let dialog = Dialog::text("When you're ready to launch, hit the button...")
        .title("Welcome")
        .button("Launch bot", move |a| {
            let key = &a.user_data::<Data>().unwrap().key;
            tx.send(key.to_string()).unwrap();
        })
        .button("Quit", Cursive::quit);

    app.add_layer(dialog);
}
