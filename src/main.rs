mod commands;

use teloxide::{prelude::*};
use std::error::Error;
use commands::commands_enum::Commands;
use crate::commands::*;

pub type Cxt = UpdateWithCx<AutoSend<Bot>, Message>;
pub type TgErr<T> = anyhow::Result<T>;

pub async fn reply(cx: &Cxt, message: String) {
    if let Err(e) = cx.reply_to(message).await {
        println!("reply error {:?}", e);
    }
}

async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Commands,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Commands::Help => help(&cx).await?,
        Commands::Start => start(&cx).await?,
        Commands::Cloc => cloc(&cx).await?,
        Commands::Author => author_info(&cx).await?,
        Commands::Github => github_url(&cx).await?
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");
    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, "xiaoLanguageBot", answer).await;
}