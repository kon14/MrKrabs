use clap::Parser;
use std::process;
use crate::constants;

/// Mr.Krabs is a simple price tracking bot for that big e-shopping platform we do not name
#[derive(Parser, Debug)]
#[clap(about, version)]
pub struct Args {
    /// Target query url
    #[clap(short='u', long="url")]
    pub url: String,

    /// Minimum product price
    #[clap(short='m', long="min-price")]
    pub min_price: f64,

    /// Maximum product price
    #[clap(short='M', long="max-price")]
    pub max_price: f64,

    /// How often should this run (in mins)
    #[clap(short='p', long="period", default_value=constants::DEFAULT_PERIOD, conflicts_with="run-once")]
    pub period: u16,

    /// How often should this run (in mins)
    #[clap(short='o', long="run-once", conflicts_with="period")]
    pub run_once: bool,

    /// Path to store sqlite db file
    #[clap(long="sqlite-file", default_value=constants::DEFAULT_SQLITE_FILE_PATH)]
    pub sqlite_path: String, // TODO: Use xdg or wtvr

    /// Enable desktop notifications
    #[cfg(feature = "notifications")]
    #[clap(long="notify", takes_value=false)]
    pub notifications: bool,
}

pub fn get_args() -> Args {
  let args = Args::parse();
  validate(&args);
  args
}

fn validate(args: &Args) {
  if !args.url.starts_with(constants::DOMAIN_NAME) {
    println!("Invalid Argument: Url must begin with {}", constants::DOMAIN_NAME);
    process::exit(1);
  }

  if args.min_price > args.max_price {
    println!("Invalid Argument: Minim price ({}) can't exceed maximum price ({})", args.min_price, args.max_price);
    process::exit(2);
  }
}