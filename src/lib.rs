#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::Error;
use solang_parser::lexer::{Lexer, Token};
use std::result::Result;

enum State {
    TopLevel,
    PragmaFound,
    PragmaSolidityFound,
    IgnoringStatement,
    CurlyBracesOpened(usize),
}

#[napi(object)]
pub struct AnalysisResult {
    pub version_pragmas: Vec<String>,
    pub imports: Vec<String>,
}

#[napi]
pub fn analyze(input: String) -> Result<AnalysisResult, Error> {
    // Create the lexer
    let mut comments = Vec::new();
    let lexer = Lexer::new(&input, 0, &mut comments);

    let mut version_pragmas = Vec::new();
    let mut imports = Vec::new();

    let mut state = State::TopLevel;

    for item in lexer {
        if item.is_err() {
            continue;
        }

        let (_, token, _) = item.unwrap();

        match state {
            State::TopLevel => match token {
                Token::Pragma => {
                    state = State::PragmaFound;
                }
                Token::OpenCurlyBrace => {
                    state = State::CurlyBracesOpened(1);
                }
                Token::Semicolon => {
                    state = State::TopLevel;
                }
                _ => {
                    state = State::IgnoringStatement;
                }
            },
            State::PragmaFound => match token {
                Token::Identifier(id) => {
                    if id == "solidity" {
                        state = State::PragmaSolidityFound;
                    } else {
                        state = State::IgnoringStatement;
                    }
                }
                Token::OpenCurlyBrace => {
                    state = State::CurlyBracesOpened(1);
                }
                Token::Semicolon => {
                    state = State::TopLevel;
                }
                _ => {
                    state = State::IgnoringStatement;
                }
            },
            State::PragmaSolidityFound => match token {
                Token::StringLiteral(literal) => {
                    version_pragmas.push(literal.to_string());
                    state = State::IgnoringStatement;
                }
                Token::OpenCurlyBrace => {
                    state = State::CurlyBracesOpened(1);
                }
                Token::Semicolon => {
                    state = State::TopLevel;
                }
                _ => {
                    state = State::IgnoringStatement;
                }
            },
            State::IgnoringStatement => match token {
                Token::OpenCurlyBrace => {
                    state = State::CurlyBracesOpened(1);
                }
                Token::Semicolon => {
                    state = State::TopLevel;
                }
                _ => {}
            },
            State::CurlyBracesOpened(braces) => match token {
                Token::OpenCurlyBrace => {
                    state = State::CurlyBracesOpened(braces + 1);
                }
                Token::CloseCurlyBrace => {
                    if braces == 1 {
                        state = State::TopLevel;
                    } else {
                        state = State::CurlyBracesOpened(braces - 1);
                    }
                }
                _ => {}
            },
        }
    }

    let res = AnalysisResult {
        version_pragmas,
        imports,
    };

    Ok(res)
}
