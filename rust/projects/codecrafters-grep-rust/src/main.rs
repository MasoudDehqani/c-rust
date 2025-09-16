use std::env;
use std::io;
use std::process;

#[derive(Debug)]
enum Patterns {
    LiteralCharacter(char),
    Digit,
    Word,
    PositiveCharacterGroups(Vec<char>),
    NegativeCharacterGroups(Vec<char>),
    StartOfStringAnchor(String),
    EndOfStringAnchor(String),
    Exact(String),
}

impl Patterns {
    fn is_match(&self, target: &str) -> bool {
        match self {
            Self::LiteralCharacter(ch) => target.chars().all(|c| c == *ch),
            Self::Digit => target.chars().all(|c| c.is_digit(10)),
            Self::Word => target.chars().all(|c| c.is_alphanumeric() || c == '_'),
            Self::PositiveCharacterGroups(chars) => chars.iter().any(|c| target.contains(*c)),
            Self::NegativeCharacterGroups(chars) => target.chars().any(|c| !chars.contains(&c)),
            Self::StartOfStringAnchor(s) => target.starts_with(s),
            Self::EndOfStringAnchor(s) => target.ends_with(s),
            Self::Exact(s) => s == target,
        }
    }

    fn from(pattern_string: &str, patterns: &mut Vec<Patterns>) {
        let mut is_adding_character_class = false;
        let mut is_adding_positive_character_groups = false;
        let mut is_adding_negative_character_groups = false;
        let mut positive_character_groups = vec![];
        let mut negative_character_groups = vec![];
        let mut is_adding_start_of_string = false;
        let mut start_of_string = String::new();

        let tst = pattern_string.rsplit(['$']);
        let tst2 = pattern_string.split(['^']);

        if pattern_string.contains("^") && pattern_string.contains("$") {
            patterns.push(Patterns::Exact(
                pattern_string.trim_matches(['^', '$']).to_string(),
            ));

            return;
        }

        tst2.filter(|c| !c.is_empty() && *c != pattern_string && !c.contains(['[', ']']))
            .map(|c| c.trim_matches('$'))
            .for_each(|c| patterns.push(Patterns::StartOfStringAnchor(c.to_string())));

        tst.filter(|c| !c.is_empty() && *c != pattern_string && !c.contains(['[', ']']))
            .map(|c| c.trim_matches('^'))
            .for_each(|c| patterns.push(Patterns::EndOfStringAnchor(c.to_string())));

        if patterns.len() > 0 {
            return;
        }

        pattern_string.chars().for_each(|ch| {
            if is_adding_character_class {
                match ch {
                    'd' => patterns.push(Patterns::Digit),
                    'w' => patterns.push(Patterns::Word),
                    _ => (),
                }
                is_adding_character_class = false;
                return;
            }

            if is_adding_positive_character_groups {
                if ch == ']' {
                    is_adding_positive_character_groups = false;
                    return;
                } else if ch != '^' {
                    positive_character_groups.push(ch);
                }
            }

            if is_adding_negative_character_groups {
                if ch == ']' {
                    is_adding_negative_character_groups = false;
                    return;
                } else {
                    negative_character_groups.push(ch);
                }
            }

            if is_adding_start_of_string {
                start_of_string.push(ch);
            }

            match ch {
                '\\' => is_adding_character_class = true,
                '[' => is_adding_positive_character_groups = true,
                '^' => {
                    if is_adding_positive_character_groups {
                        is_adding_negative_character_groups = true;
                        is_adding_positive_character_groups = false
                    } else {
                        is_adding_start_of_string = true
                    }
                }
                _ => {
                    if !is_adding_character_class
                        && !is_adding_positive_character_groups
                        && !is_adding_negative_character_groups
                        && ch != '^'
                        && !is_adding_start_of_string
                        && ch != '$'
                    {
                        patterns.push(Self::LiteralCharacter(ch))
                    }
                }
            }
        });

        if !positive_character_groups.is_empty() {
            patterns.push(Self::PositiveCharacterGroups(positive_character_groups));
        }

        if !negative_character_groups.is_empty() {
            patterns.push(Self::NegativeCharacterGroups(negative_character_groups));
        }

        if !start_of_string.is_empty() {
            patterns.push(Self::StartOfStringAnchor(start_of_string))
        }
    }
}

fn match_pattern(input_line: &str, pattern: &str) -> bool {
    let mut patterns = vec![];
    Patterns::from(pattern, &mut patterns);

    println!("{patterns:?}");

    let input_line_vec = input_line.chars().collect::<Vec<_>>();

    for i in 0..input_line_vec.len() {
        let slice = input_line_vec[i..].iter().collect::<Vec<_>>();
        let mut next_index = 0_usize;

        let res = patterns.iter().fold(vec![], |acc, p| {
            next_index += 1;

            match p {
                Patterns::StartOfStringAnchor(s) => {
                    // let target_range = slice
                    //     .get((next_index - 1)..s.len())
                    //     .unwrap_or_default()
                    //     .iter()
                    //     .fold(String::new(), |acc, curr| acc + &curr.to_string());

                    let res = p.is_match(input_line);

                    next_index += s.len() + 1;

                    vec![acc, vec![res]].concat()
                }
                Patterns::EndOfStringAnchor(_s) => vec![acc, vec![p.is_match(input_line)]].concat(),
                Patterns::Exact(s) => return vec![acc, vec![input_line == s]].concat(),
                _ => match slice.get(next_index - 1) {
                    Some(ch) => vec![acc, vec![p.is_match(&ch.to_string())]].concat(),
                    None => return vec![false],
                },
            }
        });

        println!("{res:?}");

        if !res.is_empty() && res.iter().all(|b| *b) {
            return true;
        }
    }

    false
}

// Usage: echo <input_text> | your_program.sh -E <pattern>
fn main() {
    eprintln!("Logs from your program will appear here!");

    if env::args().nth(1).unwrap() != "-E" {
        println!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        println!("0");
        process::exit(0)
    } else {
        println!("1");
        process::exit(1)
    }
}
