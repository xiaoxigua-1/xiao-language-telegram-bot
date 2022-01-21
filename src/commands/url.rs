use crate::{Cxt};
use teloxide::{RequestError};

pub async fn github_url(cx: &Cxt) -> Result<(), RequestError> {
    cx.reply_to("https://github.com/xiaoxigua-1/Xiao-Language").await?;

    Ok(())
}

pub async fn author_info(cx: &Cxt) -> Result<(), RequestError> {
    cx.reply_to("Website: https://www.xiaoxigua.art/\n\
    GitHub: https://github.com/xiaoxigua-1\n\
    Twitter: https://twitter.com/XiguaXiao").await?;

    Ok(())
}
