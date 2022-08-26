use clap::Parser;
use dinovelc::cli::{Cli, Commands};

#[macro_use]
extern crate log;

fn main() {
  env_logger::init();

  info!("Starting dinovelc");
  let args = Cli::parse();

  match &args.command {
    Commands::Hello(hello) => {
      println!("Hello, {}!", hello.name);
    }
  }
}
