use clap::Parser;

mod client;
mod account;
mod bucket;
mod hash;

#[derive(Parser, Debug)]
#[command()]
pub struct Cli {
    pub name: Option<String>,
}

impl Cli {
    pub fn new(name: Option<String>) -> Self {
        Self { name }
    }
}