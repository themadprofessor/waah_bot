#[macro_use] extern crate serenity;

use serenity::prelude::*;
use serenity::{
    Client
};

use std::env; // To be replaced by config

struct Handler;

impl EventHandler for Handler {}

fn main() -> Result<(), ::serenity::Error> {
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("discord token"),
        Handler)?;
    if let Err(e) = client.start() {
        eprintln!("Failed to start client! {}", e)
    }
    Ok(())
}

command!(ping(_context, msg) {
    let _ = msg.reply("Pong");
});