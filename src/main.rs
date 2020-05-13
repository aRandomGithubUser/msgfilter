use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use serenity::model::user::OnlineStatus;
use serenity::model::gateway::Activity;
mod lib;

struct Handler;


impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let config = lib::read_config();
        let prefix = config["prefix"].to_string();
        if msg.content == format!("{}ping", &prefix) {
            // Ping Command

            if let Err(why) = msg.channel_id.say(&ctx.http, format!("Pong!")) {
                println!("Error sending message: {:?}", why);
            }
        }
        lib::check_for_profanity(&ctx, &msg)
    }

    fn ready(&self, ctx: Context, ready: Ready) {
        let activity = Activity::playing("with messages");
        ctx.set_presence(Some(activity), OnlineStatus::DoNotDisturb);
        println!("[INFO] {}#{} is connected! [Shard: {}/{}]", ready.user.name, ready.user.discriminator, ready.shard.unwrap()[0] + 1, ready.shard.unwrap()[1]);
    }
}


fn main() {
    let config = lib::read_config();
    let token: String = config["token"].to_string();
    let shards_count: u64 = config["shards"].to_string().parse::<u64>().unwrap();
    println!("[INFO] Connecting {} shard(s)", shards_count);

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    

    if let Err(why) = client.start_shards(shards_count) {
        println!("Client error: {:?}", why);
    }
}