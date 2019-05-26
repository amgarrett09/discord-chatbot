mod commands;
pub mod profanity_filter;

use std::sync::mpsc;

use serenity::client;
use serenity::framework::StandardFramework;
use serenity::prelude::*;

use cursive::views::Dialog;
use cursive::Cursive;

struct Handler;

impl EventHandler for Handler {}

fn send_error_message(msg: &'static str, cb_sink: &cursive::CbSink) {
    let err_msg = String::from(msg);
    let _ = cb_sink.send(Box::new(|a: &mut Cursive| {
        a.add_layer(Dialog::text(err_msg).button("Quit", Cursive::quit));
    }));
}

pub fn run_bot(_tx: &mpsc::Sender<String>, rx: &mpsc::Receiver<String>, cb_sink: cursive::CbSink) {
    // Wait for application token from main thread
    let token = match rx.recv() {
        Ok(st) => st,
        Err(_) => {
            send_error_message("Error: couldn't read application token.", &cb_sink);
            return;
        }
    };

    if client::validate_token(&token).is_err() {
        send_error_message("Error: invalid token", &cb_sink);
        return;
    }

    let mut client = Client::new(&token, Handler).expect("Failed to create client");

    client.with_framework(
        StandardFramework::new()
            .configure(|c| c.prefix("!"))
            .command("ping", |c| c.cmd(commands::standard::ping)),
    );

    if client.start().is_err() {
        send_error_message("Error: failed to start client.", &cb_sink);
    }
}
