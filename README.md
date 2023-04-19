# clisearch
[![Downloads](https://img.shields.io/crates/d/clisearch.svg?style=flat-square)](https://crates.io/crates/clisearch/)
[![Version](https://img.shields.io/crates/v/clisearch.svg?style=flat-square)](https://crates.io/crates/clisearch/)
[![License](https://img.shields.io/crates/l/clisearch.svg?style=flat-square)](https://crates.io/crates/clisearch/)


## Synopsis
A crate that provides the option of performing google search in the CLI

## Explanation
I was looking for the ability of performing google searches inside of the terminal and decided to build this tool with rust, not for any particular reason other than to learn. If you decide to use this there are a few things you should be aware of. This tool was created to purely meet my bare minimum requirement which is to provide a prompt and receive a list of links. At the time of writing that is all it does with no edge case handling, there are multiple improvement areas that may be adressed in the future.

## Usage
Download with cargo
```bash
cargo install clisearch
```
Example usage
```bash
clisearch "your prompt here"
```
Please note that the quotationmarks are necessary - whatever is within the quotationsmarks is what will be google searched.

## Want something fixed?
If you found a bug, an issue, an improvement, please submit an issue(if it hasn't already been raised) using the issue tab towards the top.
