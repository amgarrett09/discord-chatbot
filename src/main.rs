use cursive::Cursive;
use cursive::views::{Dialog, LinearLayout, DummyView, TextView, EditView};
use cursive::traits::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut app = Cursive::default();

    main_menu(&mut app);
    check_for_key(&mut app);

    app.run();
}

fn check_for_key(app: &mut Cursive) {
    let path = Path::new("key.cfg");

    let mut created = false;

    // Get or create a config file to store the application key
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(_) => match File::create(&path) {
            Ok(file) => {
                created = true;
                file
            }
            Err(why) => panic!("Couldn't create a file to store application key:\
                {:?}", why)
        }
    };

    let mut s = String::new();

    // If we already had a config file, try to read from it
    if !created {
        if let Err(why) = file.read_to_string(&mut s) {
            panic!("Couldn't read from key config file: {:?}", why);
        }
    }

    // Close the key file for now
    drop(file);

    // If there's a not a key in the file already
    if !s.contains(":") {
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

        app.pop_layer();

        app.add_layer(Dialog::text("Saved application key!")
            .button("Ok", |a| {
                a.pop_layer();
            }));
    }
}

fn main_menu(app: &mut Cursive) {
    let dialog = Dialog::text("When you're ready to launch, hit the button...")
        .title("Welcome")
        .button("Launch bot", |_| ())
        .button("Quit", Cursive::quit);

    app.add_layer(dialog);
}
