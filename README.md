# git2codeblock
[![Crates.io](https://img.shields.io/crates/v/git2codeblock)](https://crates.io/crates/git2codeblock) 
[![Docs.rs](https://docs.rs/git2codeblock/badge.svg)](https://docs.rs/git2codeblock) 
[![Build](https://github.com/Ewpratten/git2codeblock/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/git2codeblock/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/git2codeblock/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/git2codeblock/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/git2codeblock/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/git2codeblock/actions/workflows/audit.yml)


`git2codeblock` is a crate that converts git URLs into codeblocks. This is mainly for use in one of my Discord bots, but is written to be used anywhere.

## Example

Given the url: [`https://gitlab.com/ewpratten/DeepSpace/-/blob/master/CODEOWNERS#L2`](https://gitlab.com/ewpratten/DeepSpace/-/blob/master/CODEOWNERS#L2), `git2codeblock` will realize that this is GitLab, correctly process the file, and return a Markdown fenced codeblock.

```rust
let url = "https://gitlab.com/ewpratten/DeepSpace/-/blob/master/CODEOWNERS#L2";
let codeblock = git2codeblock::extract_codeblock(url).await.unwrap();
assert_eq!(codeblock, "```\n*       @ewpratten @slownie @johnlownie @LordMichaelmort @awpratten\n```");
```
