#[allow(non_snake_case)]

#[macro_use] extern crate serenity;

extern crate json;

use serenity::client::Client;
use serenity::prelude::EventHandler;
use serenity::framework::standard::StandardFramework;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::fs::File;
use std::io::prelude::*;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn readCommandFile() -> String {
    let mut file = File::open("commands.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return contents;
}

fn addJsonCommand(client: &serenity::Client, cmd: &json::JsonValue, cmd_array: &json::JsonValue) {

}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("p!")) // set the bot's prefix to "~"
        .cmd("ping", ping) // load the default ping command
        .unrecognised_command(unknown_command));

    let json_content = readCommandFile();
    let json_commands = json::parse(&(json_content.to_owned())[..]).unwrap();

    let mut cmd_array = json_commands["temp_cmd_list"];

    for i in 0..json_commands["command_list"].len() {
        addJsonCommand(&client, &json_commands["command_list"][i]);
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

command!(jsonCommand(_context, message) {
    //message.channel_id.say(&contents) where contents is a String
});

fn unknown_command(_context: &mut serenity::prelude::Context, message: &serenity::model::channel::Message, _str: &str) {
    let _ = message.reply("Unknown command!");
}
