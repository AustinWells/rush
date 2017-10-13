use std::fs::File;
use std::str::Chars;

use parse::Token::*;

#[derive(Debug, Copy, Clone)]
pub enum ConType {
    Sequence,
    And,
    Or
}

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Identifier,
    FileIn,
    FileOut,
    FileOutAppend,
    FileOutDouble,
    /* From here on it should be a new
     * ParsedLine rather than a new PipeLine
     */
    Semicolon,
    Ampersand,
    Pipe,
    TwoAmpersands,
    TwoPipes,
    DoublePipe,
    EOL
}

#[derive(Debug)]
pub struct PipeLine {
    argv: Vec<String>,
    is_double_redirect: bool,
    next: Option<Box<PipeLine>>
}

#[derive(Debug)]
pub struct ParsedLine {
    con_type: ConType,
    input: File,
    output: File,
    backgroud: bool,
    pipeline: PipeLine,
    next: Option<Box<ParsedLine>>
}

pub fn get_token(line: &mut Chars) -> Token {
    let mut chr: Option<char> = line.next();
    while chr.is_some() && chr.unwrap().is_whitespace() {
        chr = line.next();
    }

    match chr {
        Some(c) =>
            match c {
                '<' => {
                    line.next();
                    FileIn
                }
                '>' => {
                    chr = line.next();
                    if chr.is_some() && chr.unwrap() == '&' {
                        line.next();
                        return FileOutDouble;
                    }
                    if chr.is_some() && chr.unwrap() == '>' {
                        line.next();
                        return FileOutAppend;
                    }
                    FileOut
                }
                ';' => {
                    line.next();
                    Semicolon
                }
                '|' => {
                    chr = line.next();
                    if chr.is_some() {
                        if chr.unwrap() == '|' {
                            line.next();
                            return TwoPipes;
                        }
                        if chr.unwrap() == '&' {
                            line.next();
                            return DoublePipe;
                        }
                    }
                    Pipe
                }
                '&' => {
                    chr = line.next();
                    if chr.is_some() && chr.unwrap() == '&' {
                        line.next();
                        return TwoAmpersands;
                    }
                    Ampersand
                }
                _ => {
                    while chr.is_some() && !chr.unwrap().is_whitespace() && "<>;&|".find(chr.unwrap()).is_none() {
                        chr = line.next();
                    }
                    Identifier
                }
            }
        None => return EOL,
    }
}
