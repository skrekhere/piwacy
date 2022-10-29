use serenity::builder::{CreateApplicationCommand};
use serenity::model::prelude::interaction::application_command::{CommandDataOption, CommandDataOptionValue};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::*;
use crate::Deluge;

pub async fn run(_options: &[CommandDataOption], command: &serenity::model::application::interaction::application_command::ApplicationCommandInteraction, ctx: Context) -> String {
    let option = _options.get(0).unwrap().resolved.as_ref().unwrap();
    println!("yo");
    if let CommandDataOptionValue::String(value) = option {
        let big = torrentfind::query(value, Some(1 as u32), 5).unwrap_or_else(|e| {
            return torrentfind::models::Results::Results(Vec::new());
        });
        if big == torrentfind::models::Results::Results(Vec::new()) {
            if let Err(big) = command.create_interaction_response(&ctx.http, |response| {
                response.
                kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("").embed(|e| {
                    e.title("No results!")
                }))
            }).await {
                println!("{}", big);
            };
            return "".to_string();
        }
        let mut s: String = String::from("");
        let mut i: u32 = 0;
        if let torrentfind::models::Results::Results(vector) = big {
            for torrent in vector.clone() {
                i += 1;
                s.push_str(format!("{}. {} - {} \n", i, torrent.name.as_str(), torrent.size.as_str()).as_str());
            }
            if let Err(big) = command.create_interaction_response(&ctx.http, |response| {
                response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("").embed(|e| {
                    e.title("Search Results")
                    .description(format!("```\n{}\n```", s))
                }).components(|c| {
                    c.create_action_row(|row| {
                        row.create_button(|button| {
                            button.label(">")
                            .custom_id(format!("{{\"query\": \"{}\", \"page\": 1, \"action\": \"f\"}}", value.as_str()))
                        })
                    }).create_action_row(|row| {
                        let mut j: u32 = 0;
                        let mut poggies = vector.clone();
                        for torrent in poggies {
                            j += 1;
                            row.create_button(|button| {
                                button.label(format!("{}", j))
                                .custom_id(format!("{{\"action\":\"g\", \"query\": \"{}\", \"page\": 1, \"number\": {}}}", value.as_str(), j))
                            });
                        }
                        row
                    })
                }))
            }).await{
                println!("{}", big)
            };
            "".to_string()
        }else{
            if let Err(big) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| {
                    message.embed(|e| {
                        e.title("Error! YOU SUCK!!")
                    })
                })
            }).await{

            };
            "".to_string()
        }
    } else {
        if let Err(big) = command.create_interaction_response(&ctx.http, |response| {
            response.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.embed(|e| {
                    e.title("Error! YOU SUCK!!")
                })
            })
        }).await {

        };
        "".to_string()
    }
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