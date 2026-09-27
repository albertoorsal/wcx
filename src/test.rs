use super::*;

// Empty input: every counter should be 0
#[test]
fn empty_string() {
    let result = count_str("");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 0);
    assert_eq!(result.bytes, 0);
    assert_eq!(result.chars, 0);
}

// "hello"
#[test]
fn single_word() {
    let result = count_str("hello");

    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 1);
    assert_eq!(result.bytes, 5);
    assert_eq!(result.chars, 5);
}

// "hello world"
#[test]
fn two_words() {
    let result = count_str("hello world");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 2);
    assert_eq!(result.bytes, 11);
    assert_eq!(result.chars, 11);
}

// "a   b": repeated spaces should not create extra words
#[test]
fn multiple_spaces_between_words() {
    let result = count_str("a   b");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 2);
    assert_eq!(result.bytes, 5);
    assert_eq!(result.chars, 5);
}

// "  hi  "
#[test]
fn leading_and_trailing_spaces() {
    let result = count_str("  hi  ");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 1);
    assert_eq!(result.bytes, 6);
    assert_eq!(result.chars, 6);
}

// "a\nb\n": lines = number of '\n'
#[test]
fn counts_newlines() {
    let result = count_str("a\nb\n");
    assert_eq!(result.lines, 2);
    assert_eq!(result.words, 2);
    assert_eq!(result.bytes, 4);
    assert_eq!(result.chars, 4);
}

// "a\nb": no trailing newline
#[test]
fn no_trailing_newline() {
    let result = count_str("a\nb");
    assert_eq!(result.lines, 1);
    assert_eq!(result.words, 2);
    assert_eq!(result.bytes, 3);
    assert_eq!(result.chars, 3);
}

// "a\tb\nc": tabs and newlines separate words
#[test]
fn mixed_whitespace() {
    let result = count_str("a\tb\nc");
    assert_eq!(result.lines, 1);
    assert_eq!(result.words, 3);
    assert_eq!(result.bytes, 5);
    assert_eq!(result.chars, 5);
}

// // "is: Alberto": punctuation stays part of the word
#[test]
fn punctuation() {
    let result = count_str("is: Alberto");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 2);
    assert_eq!(result.bytes, 11);
    assert_eq!(result.chars, 11);
}

// // "añ ü": bytes != chars
#[test]
fn non_ascii() {
    let result = count_str("añ ü");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 2);
    assert_eq!(result.bytes, 6);
    assert_eq!(result.chars, 4);
}

#[test]
fn emojis() {
    let result: CounterState = count_str("año ñandú 🦀");
    assert_eq!(result.lines, 0);
    assert_eq!(result.words, 3);
    assert_eq!(result.bytes, 17);
    assert_eq!(result.chars, 11);
}

// // ---- File-based tests (src/testdata) ----
// // Expected values match `wc -l -w -c -m` on each file

// fn assert_counts(text: &str, lines: usize, words: u16, bytes: usize, chars: usize) {
//     let result = count_str(text);
//     assert_eq!(result.lines, lines);
//     assert_eq!(result.words, words);
//     assert_eq!(result.bytes, bytes);
//     assert_eq!(result.chars, chars);
// }

// #[test]
// fn file_empty() {
//     assert_counts(include_str!("testdata/empty_file.txt"), 0, 0, 0, 0);
// }

// #[test]
// fn file_normal_text() {
//     assert_counts(include_str!("testdata/normal_text.txt"), 4, 71, 410, 408);
// }

// #[test]
// fn file_without_jump_line() {
//     assert_counts(include_str!("testdata/without_jump_line.txt"), 0, 71, 409, 407);
// }

// #[test]
// fn file_double_spaces() {
//     assert_counts(include_str!("testdata/double_spaces.txt"), 8, 50, 292, 287);
// }

// #[test]
// fn file_text_with_accents() {
//     assert_counts(include_str!("testdata/text_with_accents.txt"), 3, 52, 343, 306);
// }

// // ---- One fixture per string test (src/testdata/cases) ----

// #[test]
// fn file_cases() {
//     assert_counts(include_str!("testdata/cases/empty_string.txt"), 0, 0, 0, 0);
//     assert_counts(include_str!("testdata/cases/single_word.txt"), 0, 1, 5, 5);
//     assert_counts(include_str!("testdata/cases/two_words.txt"), 0, 2, 11, 11);
//     assert_counts(include_str!("testdata/cases/multiple_spaces_between_words.txt"), 0, 2, 5, 5);
//     assert_counts(include_str!("testdata/cases/leading_and_trailing_spaces.txt"), 0, 1, 6, 6);
//     assert_counts(include_str!("testdata/cases/counts_newlines.txt"), 2, 2, 4, 4);
//     assert_counts(include_str!("testdata/cases/no_trailing_newline.txt"), 1, 2, 3, 3);
//     assert_counts(include_str!("testdata/cases/mixed_whitespace.txt"), 1, 3, 5, 5);
//     assert_counts(include_str!("testdata/cases/punctuation.txt"), 0, 2, 11, 11);
//     assert_counts(include_str!("testdata/cases/non_ascii.txt"), 0, 2, 6, 4);
//     assert_counts(include_str!("testdata/cases/emojis.txt"), 0, 3, 17, 11);
// }
