use clap::Parser;
use hyper::{Client, Uri, Request, Body};
use std::time::Instant;


#[derive(Parser, Default, Debug)]
#[command(version = "1.0")]
#[command(author = "Kirima")]
#[command(about = "CLI tool to measure the speed of your network")]
struct Cli {
    ///checks the speed of your internet connection
    #[arg(short, long)]
    check: Option<String>,
    #[arg(short, long)]
    upload: Option<bool>,
    #[arg(short, long)]
    download: Option<bool>,
}

#[tokio::main]
async fn main() {

    

    let client = Client::new();

    let req = Request::builder()
        .method(hyper::Method::GET)
        .uri(Uri::from_static("http://127.0.0.1:8080/download"))
        .body(Body::empty())
        .expect("failed to build request body");
    let start = Instant::now();
    let response = client.request(req).await.expect("failed to send response");
    dbg!("Response: {:?}", response);


    let cli= Cli::parse();

    if let  Some(arg) = cli.download {
        println!("{:?}", arg);
    }

   // dbg!("Response: {:?}", response);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);

  


}
