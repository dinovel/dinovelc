use clap::Parser;
use dinovelc::cli::{Cli, Commands};

fn main() {
  let args = Cli::parse();

  match &args.command {
    Commands::Hello(hello) => {
      println!("Hello, {}!", hello.name);
    }
  }
}
