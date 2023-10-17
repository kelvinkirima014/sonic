use clap::{Parser, Subcommand};

#[derive(Parser, Default, Debug)]
#[command(author, version, about)]
struct Cli {
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
