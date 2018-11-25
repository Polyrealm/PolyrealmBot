#[allow(non_snake_case)]

#[macro_use] extern crate serenity;
extern crate typemap;

extern crate json;

use json::object;
use serenity::client::{Client, Context};
use serenity::prelude::EventHandler;
use serenity::framework::standard::{StandardFramework, Args, CommandError};
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::fs::File;
use std::io::prelude::*;

use typemap::Key;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

struct JsonData;

impl Key for JsonData {
    type Value = String;
}

fn readCommandFile() -> String {
    let mut file = File::open("commands.json").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents);
    return contents;
}

fn addJsonCommand(cmd: &mut json::JsonValue, cmd_array: &mut json::object::Object) -> String {
    //cmd_array.push();
    //TODO: Figure out if we actually need the name lmao
    let name: String = cmd["name"].take_string().expect("Should be a string").to_string(); //No point in checking if a command has this, because it's required lol. Yes this isn't great code, but it works and that's good enough **FOR NOW**
    let call: String = cmd["call"].take_string().expect("Should be a string").to_string(); //Same here, leave me alone. I could've made this a &str but then I can't return it easily. I know, shitty code yet again.
    let t = cmd["type"].take_string().expect("Should be a string"); //Same here, you should really leave me alone. Also, it's called t because type is a reserved keyword lol.
    if t == "custom" {
        let script = cmd["script"].clone(); //We can just take this, as it's an array
        let o = object!{
            "type" => t,
            "script" => script
        };
        cmd_array[&*call] = o;
        println!("Added command '{}'!", name);
    }

    return call;
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
        .expect("Error creating client");
    let mut framework = StandardFramework::new()
        .configure(|c| c.prefix("p!")) // set the bot's prefix to "~"
        .cmd("ping", ping) // load the default ping command
        .unrecognised_command(unknown_command);

    let json_content = readCommandFile();
    let mut json_commands = json::parse(&(json_content.to_owned())[..]).unwrap();
    println!("{}", json_commands);

    let mut cmd_array = json::object::Object::new();

    for i in 0..json_commands["command_list"].len() {
        let call = addJsonCommand(&mut json_commands["command_list"][i], &mut cmd_array);
        //framework = framework.cmd(&call[..], jsonCommand);
        framework = framework.command(&call[..], |c| c.exec(jsonCommand));
    }

    client.with_framework(framework);

    {
        let mut data = client.data.lock();
        data.insert::<JsonData>(json::stringify(cmd_array));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}

command!(ping(_context, message) {
    let _ = message.reply("Pong!");
});

fn processJsonScript(scr: &json::JsonValue) -> String {
    let mut response = String::from("");

    let mut script = scr.clone();

    for i in 0..script.len() {
        let cmd = script[i]["cmd"].take_string().expect("109:Should be a string");
        if cmd == "echo" {
            let value = script[i]["value"].take_string().expect("110:Should be a string"); //TODO: Make it so that you can do $variable to print out variables
            response.push_str(&value);
        }
    }

    return response;
}

fn jsonCommand(context: &mut Context, message: &Message, args: Args) -> Result<(), CommandError> {
    //message.channel_id.say(&contents) where contents is a String
    //println!("Context: {:?}", context);
    //println!("Message: {:?}", message);
    println!("Args: {:?}", args);
    let content = message.content.clone();
    let mut data = context.data.lock();
    let jsondata_str = data.get_mut::<JsonData>().unwrap();
    let mut jsondata = json::parse(&(jsondata_str.to_owned())[..]).unwrap();

    let mut cmd: String;

    if args.is_empty() {
        cmd = content.to_string();
    } else {
        let mut msg_split = content.split(" ");
        let msg_vec: Vec<&str> = msg_split.collect();
        cmd = msg_vec[1].to_string();
    }

    cmd = (&cmd[2..]).to_string();

    println!("{}", cmd);

    if jsondata.has_key(&(cmd.clone())[..]) {
        println!("yayeet");
        let t = jsondata[cmd.clone()]["type"].take_string().expect("138:Should be a string");
        if t == "custom" {
            let response = processJsonScript(&jsondata[cmd.clone()]["script"]);
            println!("{}", response);
            if let Err(why) = message.channel_id.say(&response) {
                println!("ERROR: {}", why);
            }
        }
    }

    Ok(())
}

fn unknown_command(_context: &mut serenity::prelude::Context, message: &serenity::model::channel::Message, _str: &str) {
    let _ = message.reply("Unknown command!");
}
