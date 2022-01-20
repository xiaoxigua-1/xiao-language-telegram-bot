use crate::{Cxt, reply};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
struct ClocData {
    language: String,
    files: i32,
    lines: i32,
    blanks: i32,
    comments: i32,
    linesOfCode: i32,
}

pub async fn cloc(cx: &Cxt) -> Result<(), reqwest::Error> {
    let url = "https://api.codetabs.com/v1/loc/?github=xiaoxigua-1/Xiao-Language";
    let res = reqwest::get(url).await?
        .json::<Vec<ClocData>>()
        .await?;
    for language_data in res {
        if language_data.language == "Kotlin" {
            let output_text = format!("language: ${}\n\
        files: {}\n\
        lines: {}\n\
        blanks: {}\n\
        comments: {}\n\
        lines of code: {}\n\n",
                                      language_data.language.to_string(),
                                      language_data.files,
                                      language_data.lines,
                                      language_data.blanks,
                                      language_data.comments,
                                      language_data.linesOfCode
            );

            reply(&cx, output_text).await;
            break;
        }
    }

    Ok(())
}