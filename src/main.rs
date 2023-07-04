use std::env;
use reqwest::{self, header::AUTHORIZATION};
use serenity::{prelude::{EventHandler, Context, GatewayIntents}, model::prelude::{Message, Ready}, async_trait, Client};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected.", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let prefix = "coc::";
        let args: Vec<_> = msg.content.split(" ").collect();

        if msg.content.to_lowercase().starts_with(&format!("{prefix}stats").to_lowercase()) {
            if args.len() == 3 {
                match args[1] {
                    "user" => {
                        let client = reqwest::Client::new();

                        let response = client.get(format!("https://api.clashofclans.com/v1/players/{}", args[2].replace("#", "%23")))
                            .header(AUTHORIZATION, format!("Bearer {}", env::var("AUTHORIZATION_TOKEN").expect("Failed to get authorization token.")))
                            .send()
                            .await
                            .unwrap()
                            .json::<serde_json::Value>()
                            .await
                            .unwrap();

                        let name = response.get("name").unwrap().to_string().replace("\"", "");
                        let trophies = response.get("trophies").unwrap().to_string().replace("\"", "");
                        let best_trophies = response.get("bestTrophies").unwrap().to_string().replace("\"", "");
                        let attack_wins = response.get("attackWins").unwrap().to_string().replace("\"", "");
                        let defense_wins = response.get("defenseWins").unwrap().to_string().replace("\"", "");

                        if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title(format!("{}'s stats", args[1]))
                                    .color(0x6708C7)
                                    .thumbnail("https://yt3.googleusercontent.com/sKnSDOlMiUodnrAS1mBy61M7WqvgplLAzlmAox29S_fw9r6f04VAH-GF7jO8x_PDAx0WkDKD=s900-c-k-c0x00ffffff-no-rj")
                                    .fields([
                                        ("Name", name, false),
                                        ("Trophies", trophies, true),
                                        ("Most Trophies Held", best_trophies, true),
                                        ("", "".to_string(), false),
                                        ("Attack Wins", attack_wins, true),
                                        ("Defense Wins", defense_wins, true)
                                    ])
                                    .footer(|f| {
                                        f.text("Clash of Clans Updates");
                                        f.icon_url("https://yt3.googleusercontent.com/sKnSDOlMiUodnrAS1mBy61M7WqvgplLAzlmAox29S_fw9r6f04VAH-GF7jO8x_PDAx0WkDKD=s900-c-k-c0x00ffffff-no-rj")
                                    })
                                    .author(|f| {
                                        f.icon_url(msg.author.avatar_url().unwrap())
                                    })
                            })
                        }).await {
                            println!("Error sending message: {:?}", e);
                        }
                    },
                    "clan" => {
                        let client = reqwest::Client::new();

                        let response = client.get(format!("https://api.clashofclans.com/v1/players/{}", args[1].replace("#", "%23")))
                            .header(AUTHORIZATION, format!("Bearer {}", env::var("AUTHORIZATION_TOKEN").expect("Failed to get authorization token.")))
                            .send()
                            .await
                            .unwrap()
                            .json::<serde_json::Value>()
                            .await
                            .unwrap();

                        let name = response.get("name").unwrap().to_string().replace("\"", "");
                        let trophies = response.get("trophies").unwrap().to_string().replace("\"", "");
                        let best_trophies = response.get("bestTrophies").unwrap().to_string().replace("\"", "");
                        let attack_wins = response.get("attackWins").unwrap().to_string().replace("\"", "");
                        let defense_wins = response.get("defenseWins").unwrap().to_string().replace("\"", "");

                        if let Err(e) = msg.channel_id.send_message(&ctx.http, |m| {
                            m.embed(|e| {
                                e.title(format!("{}'s stats", args[1]))
                                    .color(0x6708C7)
                                    .thumbnail("https://yt3.googleusercontent.com/sKnSDOlMiUodnrAS1mBy61M7WqvgplLAzlmAox29S_fw9r6f04VAH-GF7jO8x_PDAx0WkDKD=s900-c-k-c0x00ffffff-no-rj")
                                    .fields([
                                        ("Name", name, false),
                                        ("Trophies", trophies, false),
                                        ("Most Trophies Held", best_trophies, false),
                                        ("Attack Wins", attack_wins, false),
                                        ("Defense Wins", defense_wins, false)
                                    ])
                                    .footer(|f| {
                                        f.text("Clash of Clans Updates");
                                        f.icon_url("https://yt3.googleusercontent.com/sKnSDOlMiUodnrAS1mBy61M7WqvgplLAzlmAox29S_fw9r6f04VAH-GF7jO8x_PDAx0WkDKD=s900-c-k-c0x00ffffff-no-rj")
                                    })
                                    .author(|f| {
                                        f.icon_url(msg.author.avatar_url().unwrap())
                                    })
                            })
                        }).await {
                            println!("Error sending message: {:?}", e);
                        }
                    }
                    _ => println!("not finished")
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Failed to get Discord token.");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.expect("Failed to create client.");

    if let Err(e) = client.start().await {
        println!("Client error: {:?}", e);
    }
}