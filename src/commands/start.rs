use crate::{Cxt, reply};
use teloxide::{RequestError};

pub async fn start(cx: &Cxt) -> Result<(), RequestError> {
    reply(&cx, format!("")).await;
    Ok(())
}