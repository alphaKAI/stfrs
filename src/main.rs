use aho_corasick::{AhoCorasick, MatchKind};
use colored::{control, Color, ColoredString, Colorize};
use std::io::{self, Write};
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
struct ColorSpec {
    pub pat_to_color: HashMap<String, Color>,
}

impl ColorSpec {
    pub fn new(pat_to_color: HashMap<String, Color>) -> Self {
        Self { pat_to_color }
    }

    pub fn patterns(&self) -> Vec<&String> {
        self.pat_to_color.keys().collect::<Vec<_>>()
    }

    pub fn values(&self) -> Vec<&Color> {
        self.pat_to_color.values().collect::<Vec<_>>()
    }
}

fn parse_args(args: &[String]) -> Option<ColorSpec> {
    let mut pat_to_color = HashMap::new();

    for arg in args {
        let splitted: Vec<_> = arg.split('=').collect();
        if splitted.len() != 2 {
            return None;
        }

        let pat = splitted[0];
        match Color::from_str(splitted[1]).ok() {
            Some(color) => {
                pat_to_color.insert(pat.to_string(), color);
            }
            None => return None,
        }
    }

    Some(ColorSpec::new(pat_to_color))
}

trait ApplyColor {
    fn apply_color(&self, c: &Color) -> ColoredString;
}
impl ApplyColor for &str {
    fn apply_color(&self, c: &Color) -> ColoredString {
        match c {
            Color::Black => self.black(),
            Color::Red => self.red(),
            Color::Green => self.green(),
            Color::Yellow => self.yellow(),
            Color::Blue => self.blue(),
            Color::Magenta => self.magenta(),
            Color::Cyan => self.cyan(),
            Color::White => self.white(),
            Color::BrightBlack => self.bright_black(),
            Color::BrightRed => self.bright_red(),
            Color::BrightGreen => self.bright_green(),
            Color::BrightYellow => self.bright_yellow(),
            Color::BrightBlue => self.bright_blue(),
            Color::BrightMagenta => self.bright_magenta(),
            Color::BrightCyan => self.bright_cyan(),
            Color::BrightWhite => self.bright_white(),
            Color::TrueColor { r: _, g: _, b: _ } => unimplemented!(),
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>()[1..].to_vec();
    control::SHOULD_COLORIZE.set_override(true);

    if let Some(spec) = parse_args(&args) {
        let ac = AhoCorasick::builder()
            .ascii_case_insensitive(true)
            .match_kind(MatchKind::LeftmostFirst)
            .build(spec.patterns())
            .unwrap();

        for line in io::stdin().lines() {
            if let Ok(line) = line {
                let line = line.trim_end();

                let mut matches = vec![];
                for mat in ac.find_iter(line) {
                    matches.push((mat.pattern(), mat.start(), mat.end()));
                }

                let mut cursor = 0;
                let mut chunks = vec![];

                for (pat, s_idx, e_idx) in matches.iter() {
                    if cursor < *s_idx {
                        chunks.push(line[cursor..*s_idx].to_string().normal());
                    }

                    let pat_idx = pat.as_usize();
                    let pat_str = line[*s_idx..*e_idx].to_string();
                    let color = spec.values()[pat_idx];
                    chunks.push(pat_str.as_str().apply_color(color));

                    cursor = *e_idx;
                }

                if cursor != line.len() {
                    chunks.push(line[cursor..].to_string().normal());
                }

                for chunk in chunks {
                    if write!(&mut io::stdout(), "{chunk}").is_err() {
                        break;
                    } else {
                    }
                }
                if writeln!(&mut io::stdout()).is_err() {
                    break;
                } else {
                }
            } else {
            }
        }
    } else {
        println!("Invalid Argument.");
    }
}
