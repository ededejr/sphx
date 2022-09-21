use clap::Parser;
use rand::Rng;
use std::error::Error;

#[derive(Parser)]
struct Args {
    #[clap(short = 'l', long = "length", default_value_t = 8)]
    length: usize,
    #[clap(short = 'p', long = "pattern", default_value = "Aa0!")]
    pattern: String
}

pub fn run() -> Result<String, Box<dyn Error>> {
  let args = Args::parse();
  let chars = build_chars_from_pattern(&args);
  let mut rng = rand::thread_rng();

  let value: String = (0..(args.length))
  .map(|_| {
    let index = rng.gen_range(0..chars.len());
    chars.as_bytes()[index] as char
  })
  .collect();

  Ok(value)
}

fn build_chars_from_pattern(args: &Args) -> String {
  let mut chars = String::new();
  if args.pattern.contains('A') {
    chars.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
  }
  if args.pattern.contains('a') {
    chars.push_str("abcdefghijklmnopqrstuvwxyz");
  }
  if args.pattern.contains('0') {
    chars.push_str("0123456789");
  }
  if args.pattern.contains('!') {
    chars.push_str("!@#$%^&*()_+-=[]{};':,./<>?");
  }
  chars
}