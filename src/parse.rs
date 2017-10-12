use std::fs::File;

#[derive(Debug)]
pub enum ConType {
    Sequence,
    And,
    Or
}

#[derive(Debug)]
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
    DoublePipe
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
