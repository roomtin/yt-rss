# yt-rss
A super small utility for extracting the RSS link of a Youtube channel given the link to its main page.

## Building:
Requires rust's cargo tool to be installed. After cloning, `cd yt-rss` and run `cargo build --release` inside the package folder. The binary produced should be in `target/release` after building. Copy it to `/usr/bin` or `/bin` to install it so that it can be run anywhere, or run it directly with `./yt-rss`.

## Usage:
$ yt-rss \<channel-link\>

### Note:
In some shells you might need to surround the link with double or single quotations.
