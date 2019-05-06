extern crate clap;
// extern crate indicatif;
// extern crate console;
extern crate reqwest;

// https://github.com/mattgathu/duma

use clap::{Arg, App};
// use indicatif::{HumanBytes, ProgressBar, ProgressStyle};
// use reqwest::{Client};

fn main() {
    let matches = App::new("rust-get")
        .version("0.1.0")
        .author("blawesom")
        .about("Web tool in Rust")
        .arg(Arg::with_name("URL")
                    .required(true)
                    .takes_value(true)
                    .index(1)
                    .help("url to get"),
            )
        .get_matches();
    
    let url = matches.value_of("URL").unwrap();
    
    if let Err(e) = get_url(&url) {
        eprintln!("Error: {}", e);
    }
}

fn get_url(url: &str) -> reqwest::Result<()> {

    if url
    
    let target_url = 

    let body = reqwest::get(url)?
                    .text()?;
    
    println!("{}", url);
    println!("body = {:?}", body);

    Ok(())
}
