use clap::{Args, Parser, Subcommand};
use reqwest::{Client, Response};

#[derive(Parser, Debug)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    Get(Get),
    Post(Post),
}

#[derive(Args, Debug)]
pub struct Get {
    #[arg(short, long)]
    pub url: String,
}

#[derive(Args, Debug)]
pub struct Post {
    #[arg(short, long)]
    pub url: String,
    #[arg(short, long)]
    pub body: String,
}

pub async fn get(arg: &Get) -> anyhow::Result<Response> {
    let client = Client::new();
    let response = client.get(&arg.url).send().await?;
    Ok(response)
}

fn main() {
    let args = Cli::parse();
    println!("{:?}", args);
}
