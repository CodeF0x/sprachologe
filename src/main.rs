mod util;

use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    model::{
        prelude::Message,
        interactions::application_command::ApplicationCommandInteraction
    },
    prelude::*,
};
use meddl_translate::translate;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.mentions_me(&ctx.http)
            .await
            .unwrap_or(false)
             {
                let copy = msg.clone();
                match msg.referenced_message {
                    Some(ref_msg) => {
                        if let Err(why) = copy.reply(&ctx.http, translate(&ref_msg.content)).await {
                            eprintln!("{:?}", why);
                        }
                    },
                    None => {
                        if let Err(why) = copy.reply(&ctx.http, "Da is kei Text du Kaschber.").await {
                            eprintln!("{:?}", why)
                        }
                    }
                }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("translate").description("Deutsch zu Meddlfrängisch übersetzen.").create_option(|option| {
                        option
                            .name("text")
                            .description("Der Text zum Übersetzen")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                    })
                })
                .create_application_command(|command| {
                    command.name("adressenleak").description("Kontaktmöglichkeiten anzeigen, um die Brügel nauszuschmaßen.")
                })
        })
            .await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "translate" => get_text_from_command_and_translate(&command),
                "adressenleak" => "Kontaktinfos".to_string(),
                _ => "Random Fehlermeldung aus Pool einbauen".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }
}

fn get_text_from_command_and_translate(command: &ApplicationCommandInteraction) -> String {
    let options = command
        .data
        .options
        .get(0)
        .expect("No text given.")
        .resolved
        .as_ref()
        .expect("No text given.");

    if let ApplicationCommandInteractionDataOptionValue::String(text) = options {
        translate(text)
    } else {
        "Scheiße, da is was schiefglaufne verdammde Aggst.".to_string()
    }
}

#[tokio::main]
async fn main() {
    let credentials = util::get_credentials()
        .expect("Could not read credentials from file.");

    let application_id: u64 = credentials.1
        .parse()
        .expect("application id is not a valid id");

    let mut client = Client::builder(credentials.0)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}