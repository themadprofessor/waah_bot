use serenity::model::channel::Message;

use std::iter::{Iterator, IntoIterator};

pub fn ping(msg: &Message) {
    msg.reply("pong").expect("Failed to send response");
}

pub fn wah<'a, T, I>(msg: &Message, args: T) where T: IntoIterator<Item=&'a str, IntoIter=I>, I: Iterator<Item=&'a str> {
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
