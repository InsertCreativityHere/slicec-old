// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod grammar;
pub mod lexer;
pub mod parser;
pub mod tokens;

use crate::diagnostics;
use crate::slice_file::{Location, Span};

type ParseError<'a> = lalrpop_util::ParseError<Location, tokens::TokenKind<'a>, tokens::Error>;

// TODO add more specific error messages for common cases.

/// Converts an [error](tokens::Error) that was emitted from the parser/lexer into an [error](diagnostics::Error) that
/// can be handled by the [`DiagnosticReporter`](diagnostics::DiagnosticReporter).
fn construct_error_from(parse_error: ParseError, file_name: &str) -> diagnostics::Error {
    match parse_error {
        // A custom error we emitted; See `tokens::ErrorKind`.
        ParseError::User {
            error: (start, parse_error_kind, end),
        } => {
            let error_kind = match parse_error_kind {
                tokens::ErrorKind::MissingDirective => diagnostics::ErrorKind::Syntax {
                    message: "missing preprocessor directive".to_owned(),
                },
                tokens::ErrorKind::UnknownDirective { keyword } => diagnostics::ErrorKind::Syntax {
                    message: format!("unknown preprocessor directive: '{keyword}'"),
                },
                tokens::ErrorKind::UnknownSymbol { symbol, suggestion } => diagnostics::ErrorKind::Syntax {
                    message: match suggestion {
                        Some(s) => format!("unknown symbol '{symbol}', try using '{s}' instead"),
                        None => format!("unknown symbol '{symbol}'"),
                    },
                },
                tokens::ErrorKind::IllegalBlockComment => diagnostics::ErrorKind::Syntax {
                    message: "block comments cannot start on the same line as a preprocessor directive; try moving this comment to another line, or converting it to a line comment".to_owned(),
                },
            };
            diagnostics::Error::new(error_kind).set_span(&Span::new(start, end, file_name))
        }

        // The parser encountered a token that didn't fit any grammar rule.
        ParseError::UnrecognizedToken {
            token: (start, token_kind, end),
            expected,
        } => {
            let message = format!(
                "expected one of {}, but found '{token_kind:?}'", // TODO: should use Display like in Slice parser.
                clean_message(&expected)
            );
            diagnostics::Error::new(diagnostics::ErrorKind::Syntax { message })
                .set_span(&Span::new(start, end, file_name))
        }

        // The parser hit EOF in the middle of a grammar rule.
        ParseError::UnrecognizedEOF { location, expected } => {
            let message = format!("expected one of {}, but found 'EOF'", clean_message(&expected));
            diagnostics::Error::new(diagnostics::ErrorKind::Syntax { message })
                .set_span(&Span::new(location, location, file_name))
        }

        _ => unreachable!("impossible error encounted in preprocessor: {parse_error:?}"),
    }
}

// TODO: we should convert the LALRpop keywords to human words like we do for the Slice parser.
// TODO: this is identical to the bottom of parsers/slice/mod.rs, we should roll them into a helper function.
fn clean_message(expected: &[String]) -> String {
    match expected {
        [first] => first.to_owned(),
        [first, second] => format!("{first} or {second}"),
        many => {
            let (last, others) = many.split_last().unwrap();
            format!("{}, or {last}", others.join(", "))
        }
    }
}
