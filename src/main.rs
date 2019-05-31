mod bot;
mod types;
mod util;

#[macro_use]
extern crate serenity;

use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

use bot::profanity_filter;
use chatbot_macros::module_configuration;

pub use types::module_status::ModuleStatus;

// Enter the names of bot modules as arguments.
// This translates into a constant array called VALID_MODULES and a function called
// "module_configuration" which generates the module configuration view.
module_configuration!(profanity_filter);

struct Data {
    token: String,
    modules: HashMap<String, ModuleStatus>,
}

const TOKENS_FILE_PATH: &'static str = "tokens.cfg";
const MODULE_CONFIG_PATH: &'static str = "modules.cfg";

fn main() {
    let mut app = Cursive::default();

    // Initialize bot module settings
    let mut modules = HashMap::new();
    for md in VALID_MODULES.iter() {
        modules.insert(md.to_string(), ModuleStatus::Disabled);
    }

    app.set_user_data(Data {
        token: String::new(),
        modules: modules,
    });

    // Sender goes to backend thread. Messages travel backend -> frontend.
    let (txb, _): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    // Receiver goes to backend thread. Messages travel frontend -> backend.
    let (txf, rxb): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let cb_sink = app.cb_sink().clone();

    thread::spawn(move || {
        bot::run_bot(&txb, &rxb, cb_sink);
    });

    load_configuration(&mut app);
    main_menu(&mut app, txf);
    check_for_token(&mut app);

    app.run();
}

fn check_for_token(app: &mut Cursive) {
    // Try to open token config file and read from it, if it exists
    let buffer = match util::get_file_contents(TOKENS_FILE_PATH) {
        Ok(s) => s,
        _ => String::new(),
    };

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
                    .expect("Expected edit view for app name to exist.");

                let token = a
                    .call_on_id("token", |view: &mut EditView| view.get_content())
                    .expect("Expected edit view for token to exist.");

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

        if let Err(why) = util::write_to_file(TOKENS_FILE_PATH, &s) {
            panic!("Couldn't write to tokens config file: {:?}", why);
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
        let token = &a
            .user_data::<Data>()
            .expect("Expected user data to exist.")
            .token;
        if let Err(why) = tx.send(token.to_string()) {
            panic!("Couldn't send token to bot thread: {:?}", why);
        }
    });

    let configure = Button::new("Configure", |a| {
        module_configuration(a);
    });

    let layout = LinearLayout::vertical()
        .child(launch)
        .child(DummyView)
        .child(configure)
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    let dialog = Dialog::around(layout).title("Welcome");

    app.add_layer(dialog);
}

fn load_configuration(app: &mut Cursive) {
    let buffer = match util::get_file_contents(MODULE_CONFIG_PATH) {
        Ok(s) => s,
        _ => String::new(),
    };

    // Set default settings if they are missing from modules file or if file is missing
    let mut output: Vec<String> = vec![buffer.clone()];

    for module in VALID_MODULES.iter() {
        let key = format!("{}: ", module);

        if !buffer.contains(&key) {
            output.push(format!("\n{}disabled", key));
        }
    }

    let content = output.join(";");

    // If we had to add settings, write them to file
    if output.len() > 1 {
        if let Err(why) = util::write_to_file(MODULE_CONFIG_PATH, &content) {
            panic!("Couldn't write to module config file: {:?}", why);
        }
    }

    // Read module settings and prepare to store settings in user data
    let user_data = &mut app
        .user_data::<Data>()
        .expect("Expected user data to exist");
    let file_entries = content.split(";");

    // Split each file entry into key-value pair
    let pairs = file_entries.map(|entry| {
        let mut elements = entry.split(":");

        // Making sure each pair has two elements
        let key = match elements.next() {
            Some(key) => key.trim(),
            None => "",
        };

        let value = match elements.next() {
            Some(value) => value.trim(),
            None => "",
        };

        let pair = (key, value);
        pair
    });

    for pair in pairs {
        let (key, value) = pair;
        let status = match value.parse() {
            Ok(s) => s,
            Err(_) => ModuleStatus::Disabled,
        };

        // Update module setting if key is valid
        if user_data.modules.contains_key(key) {
            user_data.modules.insert(key.to_string(), status);
        }
    }
}
