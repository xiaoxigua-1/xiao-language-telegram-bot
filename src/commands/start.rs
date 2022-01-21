use crate::{Cxt, reply};
use teloxide::{RequestError};

pub async fn start(cx: &Cxt) -> Result<(), RequestError> {
    cx.reply_to("None.").await?;

    Ok(())
}