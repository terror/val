use {clap::Parser, std::process, val::arguments::Arguments};

fn main() {
  if let Err(error) = Arguments::parse().run() {
    eprintln!("error: {error}");
    process::exit(1);
  }
}
