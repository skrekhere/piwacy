mod deluge_rpc;
mod config;
mod commands;

use crate::deluge_rpc::Deluge;

use serde_json::{Value};
use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType, InteractionType};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;

struct Handler {
}

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "search" => commands::search::run(&command.data.options, &command, ctx).await,
                _ => "not implemented :(".to_string(),
            };


        } else if let Interaction::MessageComponent(mut component) = interaction {
            println!("{}", component.data.custom_id);
            let json: Value = serde_json::from_str(component.data.custom_id.as_str()).unwrap_or_else(|e| {
                println!("{}", e);
                return None;
            }).unwrap();
            match json.get("action").unwrap().as_str().unwrap() {
                "f" => {
                    component.defer(&ctx.http).await.unwrap();
                    let newpage = json.get("page").unwrap().as_u64().unwrap() + 1;
                    let big = torrentfind::query(json.get("query").unwrap().as_str().unwrap(), Some(newpage as u32), 5).unwrap_or_else(|e| {
                        return torrentfind::models::Results::Results(Vec::new());
                    });
                    if big == torrentfind::models::Results::Results(Vec::new()) {
                        if let Err(big) = component.get_interaction_response(&ctx.http).await.unwrap().edit(&ctx.http, |response| {
                            response.embed(|e| {
                                e.title("No results!")
                            }).set_components(serenity::builder::CreateComponents(Vec::new())
                            .create_action_row(|row| {
                                row.create_button(|button| {
                                    button.label("<")
                                    .custom_id(format!("{{\"query\": {}, \"page\": {}, \"action\": \"b\"}}", json.get("query").unwrap(), newpage))
                                })
                            }).clone())
                        }).await {
                            println!("{}", big);
                        };
                        return;
                    }
                    let mut s: String = String::from("");
                    let mut i: u64 = 0;
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
                            }).set_components(serenity::builder::CreateComponents(Vec::new())
                            .create_action_row(|row| {
                                row.create_button(|button| {
                                    button.label("<")
                                    .custom_id(format!("{{\"query\": {}, \"page\": {}, \"action\": \"b\"}}", json.get("query").unwrap(), newpage))
                                }).create_button(|button| {
                                    button.label(">")
                                    .custom_id(format!("{{\"query\": {}, \"page\": {}, \"action\": \"f\"}}", json.get("query").unwrap(), newpage))
                                })
                            }).create_action_row(|row| {
                                let mut j: u64 = 0;
                                let mut poggies = vector.clone();
                                for torrent in poggies {
                                    j += 1;
                                    row.create_button(|button| {
                                        button.label(format!("{}", j + ((newpage - 1) * 5)))
                                        .custom_id(format!("{{\"action\":\"g\", \"query\": \"{}\", \"page\": {}, \"number\": {}}}", json.get("query").unwrap(), newpage, j))
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
                "b" => {
                    component.defer(&ctx.http).await.unwrap();
                    let newpage = json.get("page").unwrap().as_u64().unwrap() - 1;
                    println!("{}", newpage);
                    let big = torrentfind::query(json.get("query").unwrap().as_str().unwrap(), Some(newpage as u32), 5).unwrap_or_else(|e| {
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
                    let mut s: String = String::from("");
                    let mut i: u64 = 0;
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
                                    if(newpage - 1 != 0){
                                        row.create_button(|button| {
                                            button.label("<")
                                            .custom_id(format!("{{\"query\": {}, \"page\": {}, \"action\": \"b\"}}", json.get("query").unwrap(), newpage))
                                        });
                                    }
                                    row.create_button(|button| {
                                        button.label(">")
                                        .custom_id(format!("{{\"query\": {}, \"page\": {}, \"action\": \"f\"}}", json.get("query").unwrap(), newpage))
                                    })
                                }).create_action_row(|row| {
                                    let mut j: u64 = 0;
                                    let mut poggies = vector.clone();
                                    for torrent in poggies {
                                        j += 1;
                                        row.create_button(|button| {
                                            button.label(format!("{}", j + ((newpage - 1) * 5)))
                                            .custom_id(format!("{{\"action\":\"g\", \"query\": \"{}\", \"page\": {}, \"number\": {}}}", json.get("query").unwrap(), newpage, j))
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
                        let mut cfg: config::Config = confy::load("piwacy", None).unwrap();
                        if !cfg.endpoint.ends_with('/') {
                            cfg.endpoint.push('/');
                        }
                        let mut deluge = Deluge::new(String::from(cfg.endpoint)).unwrap();
                        deluge.login(cfg.password).await.unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                        deluge.connect_to_first_available_host().await.unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                        deluge.add_magnet(magnet.link, format!("/downloads/{}", category)).await.unwrap_or_else(|e| {
                            println!("{}", e);
                        });
                    }
                }
                _ => panic!("bad!")
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let mut cfg: config::Config = confy::load("piwacy", None)?;
        if !cfg.endpoint.ends_with('/') {
            cfg.endpoint.push('/');
        }

        let guild_id = GuildId(
                cfg.guildid
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
            .create_application_command(|command| commands::search::register(command))
        })
        .await;

        println!("I now have the following guild slash commands: {:#?}", commands);
    }
}

// localclient:3dc823dd1aacdee5068cc57037192a79971a4e4a
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut cfg: config::Config = confy::load("piwacy", None)?;
    if !cfg.endpoint.ends_with('/') {
        cfg.endpoint.push('/');
    }



    let mut client = Client::builder(cfg.token, GatewayIntents::empty())
    .event_handler(Handler {
    })
    .application_id(1034141949222998086)
    .await
    .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}
