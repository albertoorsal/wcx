use clap::Parser;
use wcx::CounterState;

/// Count lines, words, bytes and chars in a file.
/// With no count flags, all counts are shown.
#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// File to read
    pub file: String,

    /// Print the byte count
    #[arg(short = 'c', long)]
    pub bytes: bool,

    /// Print the character count
    #[arg(short = 'm', long)]
    pub chars: bool,

    /// Print the word count
    #[arg(short, long)]
    pub words: bool,

    /// Print the line count
    #[arg(short, long)]
    pub lines: bool,
}

impl Cli {
    /// Parses args; if no count flag is given, enables all of them.
    pub fn parse_args() -> Self {
        let mut cli = Self::parse();
        if !(cli.bytes || cli.chars || cli.words || cli.lines) {
            cli.bytes = true;
            cli.chars = true;
            cli.words = true;
            cli.lines = true;
        }
        cli
    }

    /// Renders the selected counts in order: lines, words, bytes, chars.
    pub fn render(&self, counts: &CounterState) -> String {
        [
            (self.lines, counts.lines),
            (self.words, counts.words as usize),
            (self.bytes, counts.bytes),
            (self.chars, counts.chars),
        ]
        .iter()
        .filter(|(on, _)| *on)
        .map(|(_, n)| n.to_string())
        .collect::<Vec<_>>()
        .join(" ")
    }
}
