use crate::error_views;
use crate::util;
use crate::Data;
use crate::ModuleStatus;
use cursive::traits::*;
use cursive::views::{Button, Dialog, DummyView, LinearLayout, TextArea, TextView};
use cursive::Cursive;
use wordlist::WordList;

const WORD_LIST_PATH: &'static str = "profanity_filter/wordlist.cfg";

// Function that returns the front-end config view
pub fn init_view(app: &mut Cursive, status: ModuleStatus) {
    // Load word list contents
    let buffer = match util::get_file_contents(WORD_LIST_PATH) {
        Ok(s) => s,
        _ => String::new(),
    };

    let word_list: WordList = buffer.parse().expect("Failed to parse word list");

    let status_text = status.to_string();

    let status_indicator = LinearLayout::horizontal()
        .child(TextView::new("Current status: "))
        .child(TextView::new(status_text).with_id("status"));

    let on_off = LinearLayout::vertical()
        .child(status_indicator)
        .child(Button::new("Turn on", |a| {
            a.call_on_id("status", |view: &mut TextView| {
                view.set_content(ModuleStatus::Enabled.to_string());
            });
        }))
        .child(Button::new("Turn off", |a| {
            a.call_on_id("status", |view: &mut TextView| {
                view.set_content(ModuleStatus::Disabled.to_string());
            });
        }));

    let warn_words = TextArea::new()
        .content(word_list.warn_words.join("\n"))
        .with_id("warn_words");

    let warn_words_layout = LinearLayout::vertical()
        .child(TextView::new("Warn words:"))
        .child(TextView::new("(message is deleted and user is warned)"))
        .child(warn_words);

    let kick_words = TextArea::new()
        .content(word_list.kick_words.join("\n"))
        .with_id("kick_words");

    let kick_words_layout = LinearLayout::vertical()
        .child(TextView::new("Kick words:"))
        .child(TextView::new("(message is deleted and user is kicked)"))
        .child(kick_words);

    let words_layout = LinearLayout::horizontal()
        .child(warn_words_layout)
        .child(DummyView)
        .child(kick_words_layout);

    let main_layout = LinearLayout::vertical()
        .child(on_off)
        .child(DummyView)
        .child(words_layout);

    app.add_layer(
        Dialog::around(main_layout)
            .title("Profanity filter configuration")
            .button("Save", |a| {
                save(a);
            })
            .button("Cancel", |a| {
                a.pop_layer();
            }),
    );
}

fn save(app: &mut Cursive) {
    // Grab filtered words and store in wordlist file
    let warn_words_area = match app.find_id::<TextArea>("warn_words") {
        Some(v) => v,
        None => {
            error_views::panic(app, "Expected warn words text area to exist");
            return;
        }
    };

    let kick_words_area = match app.find_id::<TextArea>("kick_words") {
        Some(v) => v,
        None => {
            error_views::panic(app, "Expected kick words text area to exist");
            return;
        }
    };

    let warn_words: Vec<String> = warn_words_area
        .get_content()
        .split("\n")
        .map(|word| word.to_string())
        .collect();

    let kick_words: Vec<String> = kick_words_area
        .get_content()
        .split("\n")
        .map(|word| word.to_string())
        .collect();

    // Although we're converting the words from a string and then back to a string,
    // doing it this way means if we want to change the output formatting, we only
    // have to change that in the WordList trait implementations.
    let word_list = WordList {
        warn_words: warn_words,
        kick_words: kick_words,
    };

    let content = word_list.to_string();
    util::write_to_file(WORD_LIST_PATH, &content).expect("Failed to write word list file");

    // Save module status
    let status_text = match app.find_id::<TextView>("status") {
        Some(v) => v,
        None => {
            error_views::panic(app, "Expected status text to exist");
            return;
        }
    };

    let status: ModuleStatus = match status_text.get_content().source().parse() {
        Ok(s) => s,
        _ => {
            error_views::panic(app, "Failed to parse status text.");
            return;
        }
    };

    app.with_user_data(|data: &mut Data| {
        data.modules.insert("profanity_filter".to_string(), status);
    });

    // Refresh config menu
    app.pop_layer();
    app.pop_layer();
    crate::module_configuration(app);
}

mod wordlist {
    use std::error::Error;
    use std::fmt;
    use std::str::FromStr;

    pub struct WordList {
        pub warn_words: Vec<String>,
        pub kick_words: Vec<String>,
    }

    impl ToString for WordList {
        fn to_string(&self) -> String {
            format!(
                "warn_words: {};\nkick_words: {}",
                self.warn_words.join("\n"),
                self.kick_words.join("\n")
            )
        }
    }

    impl FromStr for WordList {
        type Err = ParseError;

        fn from_str(content: &str) -> Result<Self, Self::Err> {
            if content.trim().is_empty() {
                return Ok(WordList {
                    warn_words: Vec::new(),
                    kick_words: Vec::new(),
                });
            }

            let mut split_file = content.split(";");

            // Only consider first two elements separated by ";"
            let first = match split_file.next() {
                Some(st) => st,
                None => "",
            };
            let second = match split_file.next() {
                Some(st) => st,
                None => "",
            };

            // Split the two elements into key-value pairs
            let first_pair: Vec<&str> = first.split(":").collect();
            let second_pair: Vec<&str> = second.split(":").collect();

            let first_key = first_pair[0].trim();
            let second_key = second_pair[0].trim();

            if first_key == second_key {
                return Err(ParseError);
            }

            // Warn words and kick words could be stored in either the first or second
            // pair. Treat text formatted any other way as nonsense.
            let warn_words = match first_key {
                "warn_words" => match first_pair.get(1) {
                    Some(st) => st.trim(),
                    None => "",
                },
                _ => match second_key {
                    "warn_words" => match second_pair.get(1) {
                        Some(st) => st.trim(),
                        None => "",
                    },
                    _ => "",
                },
            };

            let kick_words = match first_key {
                "kick_words" => match first_pair.get(1) {
                    Some(st) => st.trim(),
                    None => "",
                },
                _ => match second_key {
                    "kick_words" => match second_pair.get(1) {
                        Some(st) => st.trim(),
                        None => "",
                    },
                    _ => "",
                },
            };

            // Collect words in vectors. They should be separated by newlines
            let warn_words_vec: Vec<String> = warn_words
                .split("\n")
                .map(|word| word.trim().to_string())
                .collect();

            let kick_words_vec: Vec<String> = kick_words
                .split("\n")
                .map(|word| word.trim().to_string())
                .collect();

            let out = WordList {
                warn_words: warn_words_vec,
                kick_words: kick_words_vec,
            };

            Ok(out)
        }
    }

    #[derive(Debug, Clone)]
    pub struct ParseError;

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Failed to parse word list content")
        }
    }

    impl Error for ParseError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            None
        }
    }
}
