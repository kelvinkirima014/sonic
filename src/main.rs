use clap::Parser;

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


fn main() {

    let cli = Cli::parse();

    println!("hello: {}", cli.name);
    println!("count: {}", cli.count);

    if let Some(arg) = cli.third_arg {
        println!("third_arg: {:?}", arg);
    }
 
}
