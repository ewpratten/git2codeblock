use std::path::PathBuf;

use regex::Regex;

use crate::Error;

use super::{extract_line_numbers, CodeSnippet};

/// Fetch a snippet from a GitHub URL
pub async fn fetch_snippet(git_url: &url::Url) -> Result<CodeSnippet, Error> {
    // Grab the line numbers
    let line_numbers = extract_line_numbers(git_url)?;

    // Get the raw URL for the referenced file
    let url_re = Regex::new(r"https://github.com/([A-Za-z\d-]+)/([A-Za-z\d-]+)/blob/(.*)").unwrap();
    let captures = url_re.captures(git_url.as_str()).unwrap();
    let raw_url = format!(
        "https://raw.githubusercontent.com/{}/{}/{}",
        captures.get(1).unwrap().as_str(),
        captures.get(2).unwrap().as_str(),
        captures.get(3).unwrap().as_str()
    );

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
    fn test_github_snippet() {
        tokio_test::block_on(async {
            let url = "https://github.com/Ewpratten/pihole-api/blob/master/upload.sh#L1-L2";
            let snippet = fetch_snippet(&url::Url::parse(url).unwrap()).await.unwrap();
            assert_eq!(snippet.extension, "sh");
            assert_eq!(snippet.code, "#!/bin/bash\nrm -rf dist/*");
        })
    }
}
