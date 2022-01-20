use crate::{Cxt, Commands};
use teloxide::{RequestError, utils::command::BotCommand};

pub async fn help(cx: &Cxt) -> Result<(), RequestError> {
    cx.reply_to(Commands::descriptions()).await?;
    Ok(())
}