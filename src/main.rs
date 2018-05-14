#![feature(process_exitcode_placeholder)]

#[macro_use] extern crate serenity;
extern crate config;
extern crate directories;
extern crate failure;

use serenity::prelude::*;
use serenity::{
    Client,
    model::channel::Message
};
use failure::Error;

mod conf;

struct Handler {
    conf: conf::Config,
}

impl Handler {
    pub fn new(conf: conf::Config) -> Handler {
        Handler {conf}
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with(&self.conf.cmd_char) {
            let mut split = msg.content[self.conf.cmd_char.len()..].split(' ');
            match &split.next() {
                Some(cmd) => {
                    match *cmd {
                        "ping" => {msg.reply("pong").expect("Failed to send response");},
                        "wah" => {
                            match split.next().unwrap_or("").parse::<u64>() {
                                Ok(count) => {
                                    let mut s = String::with_capacity(2 + count);
                                    s += "W";
                                    for _ in 0..count {
                                        s += "A"
                                    }
                                    s += "H";
                                    msg.reply(&s).expect("Failed to send response");
                                },
                                Err(e) => {
                                    msg.reply("Wat").expect("Failed to send response");
                                }
                            }
                        }
                        _ => {}
                    }
                },
                None => ()
            }
        }
    }
}

fn run() -> Result<(), Error> {
    let conf = {
        let project_dirs = ::directories::ProjectDirs::from("io", "discord", "discord_bot");
        let mut config = ::config::Config::new();
        let paths = [project_dirs.data_dir(),
            project_dirs.data_local_dir(),
            project_dirs.config_dir()];

        for &path in paths.iter() {
            if path.exists() {
                config.merge(config::File::with_name(path.to_str().unwrap()))?;
            }
        }
        config.merge(::config::Environment::with_prefix("waah"))?;
        #[cfg(debug_assertions)] config.merge(::config::File::with_name("debug.toml"))?; //Only include debug.toml if this a test scenario
        conf::Config::from_conf(config)
    }?;
    println!("Config loaded");

    let mut client = Client::new(
        &conf.discord_token.clone(),
        Handler::new(conf)
    ).map_err(|e| ::failure::err_msg(format!("{}", e)))?;
    println!("Client starting");
    if let Err(e) = client.start() {
        eprintln!("Failed to start client! {}", e)
    }
    Ok(())
}

fn main() -> ::std::process::ExitCode {
    match run() {
        Ok(_) => ::std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{}", e);
            ::std::process::ExitCode::FAILURE
        }
    }
}
