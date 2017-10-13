use std::fs::File;
use std::str::Chars;

use parse::Token::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ConType {
    Sequence,
    And,
    Or
}

impl Default for ConType {
    fn default() -> ConType {
        ConType::Sequence
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
    arg: Vec<String>,
    is_double_redirect: bool,
    next: Option<Box<PipeLine>>
}

#[derive(Debug, Default)]
pub struct ParsedLine {
    con_type: ConType,
    input: Option<File>,
    output: Option<File>,
    is_doubled: bool,
    backgroud: bool,
    pipeline: Option<PipeLine>,
    next: Option<Box<ParsedLine>>
}

impl ParsedLine {
    pub fn new() -> Self {
        Default::default()
    }
}

pub fn parsed_line(line: String) -> ParsedLine {
    let mut curline = ParsedLine::new();
    let mut tok: Token;
    let s = &mut line.chars();
    let arg = &mut String::new();

    curline.con_type = ConType::Sequence;
    curline.input = None;
    curline.output = None;
    curline.is_doubled = false;
    curline.backgroud = false;
    curline.pipeline = None;
    curline.next = None;

    tok = get_token(s, arg);
    while tok != EOL {
        while tok < Semicolon {
            println!("{:?}: {:?}", tok, arg);
            tok = get_token(s, arg);
        }
        println!("new pipline");
        tok = get_token(s, arg);
    }

    return curline;
}

pub fn get_token(line: &mut Chars, arg: &mut String) -> Token {
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
                    let mut id = String::new();
                    while chr.is_some() && !chr.unwrap().is_whitespace() && "<>;&|".find(chr.unwrap()).is_none() {
                        id.push(chr.unwrap());
                        chr = line.next();
                    }
                    *arg = id;
                    Identifier
                }
            }
        None => return EOL,
    }
}
