mod deluge_rpc;
mod config;
mod commands;

use crate::deluge_rpc::Deluge;
use config::Config;

use serde_json::{Value};
use serenity::async_trait;
use serenity::model::application::interaction::{Interaction};
use serenity::model::application::interaction::message_component::MessageComponentInteraction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

use once_cell::sync::Lazy;

pub static cfg: Lazy<Config> = Lazy::new(|| { confy::load("piwacy", None).unwrap()});
struct Handler {
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let _content = match command.data.name.as_str() {
                "search" => commands::search::run(&command.data.options, &command, ctx).await,
                "list" => commands::list::run(ctx, &command).await,
                _ => "not implemented :(".to_string(),
            };


        } else if let Interaction::MessageComponent(component) = interaction {
            println!("{}", component.data.custom_id);
            let json: Value = serde_json::from_str(component.data.custom_id.as_str()).unwrap_or_else(|e| {
                println!("{}", e);
                return None;
            }).unwrap();
            match json.get("action").unwrap().as_str().unwrap() {
                "f" => {
                    newpage(component, json, ctx, 1).await;
                }
                "b" => {
                    newpage(component, json, ctx, -1).await;
                }
                "g" => {
                    let big = torrentfind::query(json.get("query").unwrap().as_str().unwrap(), Some(json.get("page").unwrap().as_u64().unwrap() as u32), 5).unwrap_or_else(|e| {
                        return torrentfind::models::Results::Results(Vec::new());
                    });
                    if big == torrentfind::models::Results::Results(Vec::new()) {
                        if let Err(big) = component.get_interaction_response(&ctx.http).await.unwrap().edit(&ctx.http, |response| {
                            response.embed(|e| {
                                e.title("No results!")
                            }).set_components(serenity::builder::CreateComponents(Vec::new()))
                        }).await {
                            println!("{}", big);
                        };
                        return;
                    }
                    if let torrentfind::models::Results::Results(vector) = big {
                        component.defer(&ctx.http).await.unwrap();
                        let torrent: torrentfind::models::Torrent = vector.get((json.get("number").unwrap().as_u64().unwrap() - 1) as usize).unwrap().clone();
                        let magnet = torrentfind::querymagnet(torrent.name.as_str()).unwrap();
                        let big = reqwest::get(format!("https://1337x.to/torrent/{}/{}/", torrentfind::getid(torrent.name.as_str()).unwrap(), torrent.name.as_str())).await.unwrap();
                        let regex = regex::Regex::new("Category</strong> <span>[^<]*").unwrap();
                        let html = big.text().await.unwrap();
                        let captured = &regex.captures(&html).unwrap().get(0);
                        let mut category = captured.unwrap().as_str().split_once("<span>").unwrap().1;
                        if category == "Movies" {
                            category = "MV";
                        }
                        if category == "Anime" {
                            category = "TV";
                        }
                        println!("yo");
                        let mut deluge = Deluge::new(String::from(&cfg.endpoint)).unwrap();
                        deluge.login((&cfg.password).to_string()).await.unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                        deluge.connect_to_first_available_host().await.unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                        deluge.add_magnet(magnet.link, format!("/downloads/{}", category)).await.unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                        println!("yo");
                        component.create_followup_message(&ctx.http, |response| {
                            response.embed(|e| {
                                e.title("Success!")
                                .description(format!("Downloading `{}`", torrent.name.as_str()))
                            })
                        }).await.unwrap();
                    }
                }
                _ => panic!("bad!")
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
                cfg.guildid
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
            .create_application_command(|command| commands::search::register(command))
            .create_application_command(|command| commands::list::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);
    }
}

async fn newpage(component: MessageComponentInteraction, json: Value, ctx: Context, offset: i64){
    component.defer(&ctx.http).await.unwrap();
    let newpage = json.get("page").unwrap().as_i64().unwrap() + offset;
    println!("{}", newpage);
    let big = torrentfind::query(json.get("query").unwrap().as_str().unwrap(), Some(newpage as u32), 5).unwrap_or_else(|_| {
        return torrentfind::models::Results::Results(Vec::new());
    });
    if big == torrentfind::models::Results::Results(Vec::new()) {
        if let Err(big) = component.get_interaction_response(&ctx.http).await.unwrap().edit(&ctx.http, |response| {
            response.embed(|e| {
                e.title("No results!")
            }).set_components(serenity::builder::CreateComponents(Vec::new()).create_action_row(|row| {
                if newpage != 1 {
                    row.create_button(|button| {
                        button.label("<")
                        .custom_id(format!("{{\"query\": {}, \"page\": {}, \"action\": \"b\"}}", json.get("query").unwrap(), newpage))
                    })
                } else {
                    row
                }
            }).clone())
        }).await {
            println!("{}", big);
        };
        return;
    }
    let mut s: String = String::from("");
    let mut i: i64 = 0;
    println!("yo");
    if let torrentfind::models::Results::Results(vector) = big {
        println!("yo");
        for torrent in vector.clone() {
            i += 1;
            s.push_str(format!("{}. {} - {} \n", i + ((newpage - 1) * 5), torrent.name.as_str(), torrent.size.as_str()).as_str());
        }
        println!("yo");
        if let Err(big) = component.get_interaction_response(&ctx.http).await.unwrap().edit(&ctx.http,|response| {
            println!("yo");
            response
            .embed(|e| {
                e.title("Search Results")
                .description(format!("```\n{}\n```", s))
            }).set_components(serenity::builder::CreateComponents(Vec::new()).create_action_row(|row| {
                if newpage - 1 != 0 {
                    row.create_button(|button| {
                        button.label("<")
                        .custom_id(format!("{{\"query\": \"{}\", \"page\": {}, \"action\": \"b\"}}", json.get("query").unwrap().as_str().unwrap(), newpage))
                    });
                }
                row.create_button(|button| {
                    button.label(">")
                    .custom_id(format!("{{\"query\": \"{}\", \"page\": {}, \"action\": \"f\"}}", json.get("query").unwrap().as_str().unwrap(), newpage))
                })
            }).create_action_row(|row| {
                let mut j: i64 = 0;
                let poggies = vector.clone();
                for _ in poggies {
                    j += 1;
                    row.create_button(|button| {
                        button.label(format!("{}", j + ((newpage - 1) * 5)))
                        .custom_id(format!("{{\"action\":\"g\", \"query\": \"{}\", \"page\": {}, \"number\": {}}}", json.get("query").unwrap().as_str().unwrap(), newpage, j))
                    });
                }
                row
            }).clone())

        }).await{
            println!("{}", big)
        };
        println!("yo");
    }
}

// localclient:3dc823dd1aacdee5068cc57037192a79971a4e4a
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut client = Client::builder(&cfg.token, GatewayIntents::empty())
    .event_handler(Handler {
    })
    .application_id(1034141949222998086)
    .await
    .expect("Error creating client");
    let mut deluge = Deluge::new((&cfg.endpoint).clone())?;
    deluge.login((&cfg.password).clone()).await?;
    deluge.connect_to_first_available_host().await?;

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
