//refered https://github.com/serenity-rs/serenity/blob/current/examples/e01_basic_ping_bot/

use std::io;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
extern crate regex;
use regex::Regex;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
       if msg.content == "/ping" {
            println!("Shard {}", ctx.shard_id);

            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        let re = Regex::new(r"/([^']+)-([^']+)\s+(\d+),(\d+)").unwrap();
        let caps = re.captures(&msg.content).unwrap();

        if &caps[1] == "bit_calc"{
            let a =  &caps[3].trim().parse::<u32>().unwrap();
            let b = &caps[4].trim().parse::<u32>().unwrap();
            if &caps[2] == "or"{
            let calc_c = a | b;
            if let Err(why) = msg.channel_id.say(&ctx.http, calc_c.to_string()).await {
                println!("Error sending message: {:?}", why);
            }
        }else if &caps[2] == "and"{
            let calc_c = a & b;
            if let Err(why) = msg.channel_id.say(&ctx.http, calc_c.to_string()).await {
                println!("Error sending message: {:?}", why);
            }
        }
      }
    }



    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
  }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the commandline
  println!("Input token");
  let mut token =String::new();
   io::stdin().read_line(&mut token)
       .expect("Failed to get token");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
