use serenity::builder::{CreateApplicationCommand};
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;

pub async fn run(_options: &[CommandDataOption], ctx: Context){

}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("search").description("Searches 1337x for torrents").create_option(|option| {
        option
        .name("query")
        .description("the text to search for")
        .kind(CommandOptionType::String)
        .required(true)
    })
}