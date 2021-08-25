//! Turn GitHub and GitLab codesnippet URLs into Markdown codeblocks

pub mod error;
pub use error::Error;

mod providers;
use crate::providers::determine_provider;

/// Extract a codeblock from a URL
///
/// # Example
/// ```
/// # tokio_test::block_on(async {
/// let url = "https://gitlab.com/ewpratten/DeepSpace/-/blob/master/CODEOWNERS#L2";
/// let codeblock = git2codeblock::extract_codeblock(url).await.unwrap();
/// assert_eq!(codeblock, "```\n*       @ewpratten @slownie @johnlownie @LordMichaelmort @awpratten\n```");
/// # })
/// ```
pub async fn extract_codeblock(git_url: &str) -> Result<String, Error> {
    // Build the URL
    let git_url = url::Url::parse(git_url)?;

    // Determine the provider
    let provider = determine_provider(&git_url)?;

    // Get the code snippet
    let snippet = match provider {
        providers::GitProvider::GitHub => providers::github::fetch_snippet(&git_url).await?,
        providers::GitProvider::GitLab => providers::gitlab::fetch_snippet(&git_url).await?,
    };

    // Get the code block header from the ext
    // TODO
    let header = "";

    // Return the code block
    Ok(format!("```{}\n{}\n```", header, snippet.code))
}
