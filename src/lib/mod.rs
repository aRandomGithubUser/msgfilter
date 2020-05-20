use serenity::model::channel::Message;
use serenity::prelude::Context;
use censor::*;
use std::fs::read_to_string;
use json::parse;
use json::JsonValue;
use regex::Regex;


pub fn read_config() -> JsonValue {
    let contents = read_to_string("config.json").expect("Make sure you have a config.json at the root of this app");
    let config = parse(&contents.to_string()).unwrap();
    return config;
}

pub fn check_for_profanity(ctx: &Context, msg: &Message) {
    // A simple function that checks for profanity
    let censor = censor_filter();
    
    let regex_extra: Regex = Regex::new(r"(?:^|\W)(fuck|ass|shit|fucker|motherfucker|fk|fuk|fuc|dong|dick|penis|sex|vulva|vagina|dick|cock)(?:$|\W)").unwrap(); 
    if censor.check(&msg.content)|| regex_extra.is_match(&msg.content) {
        if let Err(why) = msg.delete(&ctx.http) {
            println!("Error: {:?}", why);
            msg.channel_id.say(&ctx.http, format!("An error occurred: `{:?}`", why)).unwrap();
        }
        else {
            msg.channel_id.say(&ctx.http, format!("Profane content from `{}#{}` (`{}`) removed.", &msg.author.name, &msg.author.discriminator, &msg.author.id)).unwrap();
        }
    }
}

fn censor_filter() -> Censor {
    return Censor::Standard + Censor::Sex - "ass" - "fetish" + "dong";
}

