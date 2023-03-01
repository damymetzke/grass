use clap::Parser;
use grass::get_hello;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   count: u8,
}

fn main() {
   let args = Args::parse();

   for _ in 0..args.count {
       println!("{}", get_hello("Worlds"))
   }
}
