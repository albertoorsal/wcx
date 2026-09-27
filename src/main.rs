mod cli;

use cli::Cli;
use wcx::{count_str, files::read_file};

fn main() {
    let cli = Cli::parse_args();
    let text = read_file(&cli.file).unwrap_or_else(|e| {
        eprintln!("wcx: {}: {e}", cli.file);
        std::process::exit(1);
    });
    println!("{}", cli.render(&count_str(&text)));
}
