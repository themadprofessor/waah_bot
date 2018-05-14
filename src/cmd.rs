use serenity::model::channel::{Message, Channel};

use imgur::Imgur;

use std::iter::{Iterator, IntoIterator};

fn handle_err(res: ::serenity::Result<Message>) {
    if let Err(e) = res {
        eprintln!("Failed to send response! {}", e);
    }
}

pub fn help(msg: &Message) {
    println!("Help command received");
    handle_err(msg.reply("~ping, ~wah num, ~img"));
}

pub fn ping(msg: &Message) {
    println!("Ping command received");
    handle_err(msg.reply("pong"));
}

pub fn wah<'a, T, I>(msg: &Message, args: T) where T: IntoIterator<Item=&'a str, IntoIter=I>, I: Iterator<Item=&'a str> {
    println!("Wah command received");
    match args.into_iter().next().unwrap_or("").parse::<usize>() {
        Ok(count) => {
            if 5 + msg.author.name.len() + count >= 2000 {
                handle_err(msg.reply("Wat"));
                return;
            }
            let mut s = String::with_capacity(2 + count);
            s += "W";
            for _ in 0..count {
                s += "A"
            }
            s += "H";
            handle_err(msg.reply(&s));
        },
        Err(_) => {
            handle_err(msg.reply("Wat"));
        }
    }
}

pub fn img(msg: &Message, imgur: &Imgur) {
    println!("Img command received");
    match imgur.get_rand("waluigi", &mut ::rand::thread_rng()) {
        Ok(url) => {
            handle_err(msg.channel_id.send_message(|create_msg| create_msg.embed(|create_embed| create_embed.image(url))));
        },
        Err(e) => {
            eprintln!("Failed to get Waluigi img! {}", e)
        }
    }
}
