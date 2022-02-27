use reqwest;
use regex::Regex;

fn main() {
    //get args
    let args: Vec<String> = std::env::args().collect();

    //check to make sure args exist
    if args.len() < 2 {
        println!("Requires a channel link as an argument.\n\nExample:\tyt-rss \"paste-link-to-main-channel-page-here\"");
        std::process::exit(1);
    }

    //make sure arg is a link, assign if is
    let channel_page_html = reqwest::blocking::get(args[1].as_str())
        .expect("Arg must be a link to a channel.")
        .text()
        .expect("Couldn't parse arg as string.");

    let re_rss = Regex::new(r"rssUrl")
        .expect("Couldn't compile Regex");

    let range = re_rss.find(channel_page_html.as_str());
    match range {
        Some(rss_indexes) => {
            let start = rss_indexes.start() + 9;
            let end = rss_indexes.end() + 79;
            println!("RSS link for channel: {}", &channel_page_html[start..end]);
        }
        None => {println!("Can't find any youtube RSS information in the link provided.")}
    }
}

