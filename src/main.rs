use clap::Parser;
use hyper::{Client, Uri, Request, Body};
use std::time::Instant;


#[derive(Parser, Default, Debug)]
#[command(version = "1.0")]
#[command(author = "Kirima")]
#[command(about = "CLI tool to measure the speed of your network")]
struct Cli {
    ///name 
    #[arg(short, long)]
    name: String,
    #[arg(short, long)]
    second_arg: Option<String>,
    #[arg(short, long)]
    third_arg: Option<String>,
    #[arg(short, long)]
    count: u8,
}

#[tokio::main]
async fn main() {

    let client = Client::new();

    let req = Request::builder()
        .method(hyper::Method::GET)
        .uri(Uri::from_static("http://www.example.org/"))
        .body(Body::empty())
        .expect("failed to build request body");
    let start = Instant::now();
    let response = client.request(req).await.expect("failed to send response");
    let duration = start.elapsed();
    println!("ping time: {:?}", duration);
    dbg!("{:?}", response);


    let cli = Cli::parse();

    println!("hello: {}", cli.name);
    println!("count: {}", cli.count);

    if let Some(arg) = cli.third_arg {
        println!("third_arg: {:?}", arg);
    }
 
}
