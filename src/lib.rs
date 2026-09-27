use core::fmt;

pub mod files;

#[cfg(test)]
mod test;

// Define structs
// Counter State (lines, words, bytes, chars)
#[derive(Debug)]
pub struct CounterState {
    pub lines: usize,
    pub words: u16,
    pub bytes: usize,
    pub chars: usize
}


impl fmt::Display for CounterState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.lines,
            self.words,
            self.bytes,
            self.chars
        )
    }
}


pub fn count_str(text : &str) -> CounterState{
    let mut words = 0;
    let mut chars = 0;
    let mut lines = 0;
    let mut in_word = false;
    let bytes = text.len();

    for c in text.chars() {
        chars += 1;
        if c.is_whitespace() {
            in_word = false;
        } else if !in_word {
            words += 1;
            in_word = true;
        }
        if c == '\n' {
            lines+=1;
        }
    }

    let counter_state = CounterState {
        lines : lines,
        words: words,
        bytes: bytes,
        chars: chars,
    };

    return counter_state;
}
