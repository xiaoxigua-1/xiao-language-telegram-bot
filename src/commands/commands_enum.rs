use teloxide::{utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Commands {
    #[command(description = "help list.")]
    Help,
    #[command(description = "quick start.")]
    Start,
    #[command(description = "xiao language line of code.")]
    Cloc,
    #[command(description = "send xiao language repo info.")]
    Github,
    #[command(description = "send xiao language author info.")]
    Author,
}
