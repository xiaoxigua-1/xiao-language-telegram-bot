use teloxide::{utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Commands {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "quick start.")]
    Start,
    #[command(description = "xiao language line of code.")]
    Cloc
}
