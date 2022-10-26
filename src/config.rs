use serde_derive::{Serialize, Deserialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Config{
    pub endpoint: String,
    pub password: String,
    pub token: String,
    pub guildid: u64
}