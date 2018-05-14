use serenity::model::channel::{Message, Channel};

use imgur::Imgur;

use std::iter::{Iterator, IntoIterator};

pub fn ping(msg: &Message) {
    println!("Ping command received");
    msg.reply("pong").expect("Failed to send response");
}

pub fn wah<'a, T, I>(msg: &Message, args: T) where T: IntoIterator<Item=&'a str, IntoIter=I>, I: Iterator<Item=&'a str> {
    println!("Wah command received");
    match args.into_iter().next().unwrap_or("").parse::<usize>() {
        Ok(count) => {
            let mut s = String::with_capacity(2 + count);
            s += "W";
            for _ in 0..count {
                s += "A"
            }
            s += "H";
            msg.reply(&s).expect("Failed to send response");
        },
        Err(_) => {
            msg.reply("Wat").expect("Failed to send response");
        }
    }
}

pub fn img(msg: &Message, imgur: &Imgur) {
    println!("Img command received");
    match imgur.get_rand("waluigi", &mut ::rand::thread_rng()) {
        Ok(url) => {
            if let Err(e) = msg.channel_id.send_message(|create_msg| create_msg.embed(|create_embed| create_embed.image(url))) {
                eprintln!("Failed to send message! {}", e);
            }
        },
        Err(e) => {
            eprintln!("Failed to get Waluigi img! {}", e)
        }
    }
}
