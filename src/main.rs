use crossterm::style::{Color, Print, SetForegroundColor};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, QueueableCommand};
use ctrlc::set_handler;
use std::fs;
use std::io::{self, stdout, Stdout, Write};
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

const HIGHLIGHT_OFFSET: usize = 5;
const DO_NEWLINE: bool = false;

fn main_uwu() -> io::Result<()> {
    let source = fs::read_to_string("source_text.txt")?;
    let mut stdout = stdout();
    stdout.queue(cursor::Hide)?.queue(Print('\n'))?;
    stdout.flush()?;
    for word in source.split_whitespace() {
        let (start, hl, end, offset) = split_word(word);
        let start_col = u16::try_from(HIGHLIGHT_OFFSET - offset).unwrap();
        stdout
            .queue(cursor::MoveToColumn(start_col))?
            .queue(Clear(ClearType::CurrentLine))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print(start))?
            .queue(SetForegroundColor(Color::White))?
            .queue(Print(hl))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print(end))?;
        if DO_NEWLINE {
            stdout.queue(Print('\n'))?;
        }
        stdout.flush()?;
        sleep(Duration::from_millis(200));
    }
    wrap_up(stdout)
}

fn wrap_up(mut stdout: Stdout) -> io::Result<()> {
    stdout
        .queue(cursor::Show)?
        .queue(SetForegroundColor(Color::Reset))?
        .queue(Print('\n'))?;
    stdout.flush()
}

fn main() {
    set_handler(|| {
        let err_code = if wrap_up(stdout()).is_ok() { 0 } else { 1 };
        exit(err_code);
    })
    .expect("could not set Ctrl-C handler");
    main_uwu().expect("oopsie daisy!");
}

fn split_word(s: &str) -> (&str, &str, &str, usize) {
    let indices: Vec<_> = s.char_indices().map(|(i, _)| i).collect();
    //println!("{:?}", indices);
    let len = indices.len();
    let i = (len.saturating_sub(1) / 2).min(HIGHLIGHT_OFFSET);
    let midpoint = indices.get(i).copied().unwrap_or_default();
    let after = indices.get(i + 1).copied().unwrap_or(len);
    (&s[..midpoint], &s[midpoint..after], &s[after..], i)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn split_basic() {
        assert_eq!(split_word("cat"), ("c", "a", "t", 1));
    }

    #[test]
    fn split_4() {
        assert_eq!(split_word("meow"), ("m", "e", "ow", 1));
    }

    #[test]
    fn split_5() {
        assert_eq!(split_word("Hazel"), ("Ha", "z", "el", 2));
    }

    #[test]
    fn split_one_char() {
        assert_eq!(split_word("a"), ("", "a", "", 0));
    }

    #[test]
    fn split_empty() {
        assert_eq!(split_word(""), ("", "", "", 0));
    }

    #[test]
    fn split_long() {
        assert_eq!(
            split_word("antidisestablishmentarianism"),
            ("antid", "i", "sestablishmentarianism", 5)
        );
    }

    #[test]
    fn split_utf8() {
        assert_eq!(split_word("I’m"), ("I", "’", "m", 1));
    }
}
