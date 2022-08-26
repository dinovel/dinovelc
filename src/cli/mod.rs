use clap::{Parser, Subcommand, Args};

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
#[clap(propagate_version = true)]
pub struct Cli {
  #[clap(flatten)]
  pub verbose: clap_verbosity_flag::Verbosity,

  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  Hello(Hello),
}

#[derive(Debug, Args)]
pub struct Hello {
  #[clap(value_parser)]
  pub name: String,
}
