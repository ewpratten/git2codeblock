//! Utils for determining if a URL is coming from GitHub or GitLab.

use crate::Error;

pub mod gitlab;
pub mod github;

/// Git Providers
#[derive(Debug, PartialEq)]
pub enum GitProvider {
    GitHub,
    GitLab,
}

// Common snippet type used across providers
#[derive(Debug)]
pub struct CodeSnippet {
    /// Snippet file extension
    pub extension: String,
    /// Snippet content
    pub code: String,
}

/// Determine the git provider from a URL.
pub fn determine_provider(url: &url::Url) -> Result<GitProvider, Error> {
    match url.host_str() {
        Some(host) => match host {
            "github.com" => Ok(GitProvider::GitHub),
            "gitlab.com" => Ok(GitProvider::GitLab),
            _ => Err(Error::UnknownGitProvider),
        },
        None => Err(Error::UnknownGitProvider),
    }
}

/// Extract line numbers from a Url
pub fn extract_line_numbers(url: &url::Url) -> Result<(usize, usize), Error> {
    match url.fragment() {
        Some(fragment) => {
            // Set up a REGEX to pull the line numbers
            let re = regex::Regex::new(r"L(\d+)(?:-L?(\d+))?").unwrap();
            let captures = re.captures(fragment);

            // Pull the matches from the re
            match captures {
                Some(captures) => {
                    let start = captures.get(1).unwrap().as_str().parse::<usize>()?;
                    let end = match captures.get(2) {
                        Some(value) => value.as_str().parse::<usize>()?,
                        None => start,
                    };
                    Ok((start, end))
                }
                None => Err(Error::MissingLineNumbers),
            }
        }
        None => Err(Error::MissingLineNumbers),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;
    #[test]
    fn test_determine_provider() {
        let provider = determine_provider(&Url::parse("https://github.com/ewpratten").unwrap());
        assert!(provider.is_ok());
        assert_eq!(provider.unwrap(), GitProvider::GitHub);
    }
}
