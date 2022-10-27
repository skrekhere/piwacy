use serenity::builder::{CreateApplicationCommand};
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;
use serde_json::Value;
use crate::deluge_rpc::Deluge;
use crate::cfg;

pub async fn run(ctx: Context, command: &serenity::model::application::interaction::application_command::ApplicationCommandInteraction) -> String{
    let mut deluge = Deluge::new(String::from(&cfg.endpoint)).unwrap();
    deluge.login((&cfg.password).to_string()).await.unwrap_or_else(|e| {
        println!("{}", e);
    });
    deluge.connect_to_first_available_host().await.unwrap_or_else(|e| {
        println!("{}", e);
    });
    let j = deluge.get_torrents_status().await.unwrap_or_else(|e| {
        println!("{}", e);
        return Value::Null;
    });
    command.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::ChannelMessageWithSource)
        .interaction_response_data(|message| message.content("").embed(|e| {
            e.title("Currently downloading torrents:");
            let mut desc: String = "".to_string();
            for key in j.as_object().unwrap().keys() {
                if j.get(key).unwrap().get("state").unwrap().as_str().unwrap().to_lowercase() == "downloading"{
                    desc.push_str(format!("{} - {:?}%\n\n", j.get(key).unwrap().get("name").unwrap().as_str().unwrap(), ((j.get(key).unwrap().get("progress").unwrap().as_f64().unwrap() * 100.0).round() / 100.0)).as_str());
                }
            }
            if desc == "" {
                e.description("```\nNot currently downloading anything!\n```")
            } else {
                e.description(format!("```\n{}\n```", desc))
            }
        }))
    }).await.unwrap();
    "".to_string()

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("list").description("lists all torrents")
}