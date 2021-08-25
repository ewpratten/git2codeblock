use std::path::PathBuf;

use crate::Error;

use super::{extract_line_numbers, CodeSnippet};

/// Fetch a snippet from a GitLab URL
pub async fn fetch_snippet(git_url: &url::Url) -> Result<CodeSnippet, Error> {
    // Grab the line numbers
    let line_numbers = extract_line_numbers(git_url)?;

    // Get the raw URL for the referenced file
    let raw_url = git_url.to_string().replace("/-/blob/", "/-/raw/");

    // Fetch the raw file
    let raw_file = reqwest::get(&raw_url).await?.text().await?;

    // Split to lines
    let lines = raw_file.lines();
    let content = lines
        .skip(line_numbers.0 - 1)
        .take(line_numbers.1 - line_numbers.0 + 1)
        .collect::<Vec<&str>>()
        .join("\n");

    Ok(CodeSnippet {
        extension: git_url
            .path()
            .split("/")
            .last()
            .unwrap_or_default()
            .split(".")
            .last()
            .unwrap_or_default()
            .to_string(),
        code: content,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gitlab_snippet() {
        tokio_test::block_on(async {
            let url = "https://gitlab.com/ewpratten/DeepSpace/-/blob/master/CODEOWNERS#L1-2";
            let snippet = fetch_snippet(&url::Url::parse(url).unwrap()).await.unwrap();
            assert_eq!(snippet.extension, "CODEOWNERS");
            assert_eq!(snippet.code, "# These owners will be the default owners for everything in the repo.\n*       @ewpratten @slownie @johnlownie @LordMichaelmort @awpratten");
        })
    }
}
