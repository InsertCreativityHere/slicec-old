// Copyright (c) ZeroC, Inc. All rights reserved.

//! This module pulls in the parsing code generated by LALRPOP and contains private helper functions used by it.
//!
//! While many of these functions could be written directly into the parser rules, we implement them here instead, to
//! keep the rules focused on grammar instead of implementation details, making the grammar easier to read and modify.

use crate::grammar::{DocComment, Message, MessageComponent, Overview};
use crate::slice_file::{Location, Span};

use lalrpop_util::lalrpop_mod;

// Place the code generated by LALRPOP into a submodule named 'lalrpop'.
lalrpop_mod!(
    #[allow(unused, clippy::all)] // LALRPOP generates stuff we don't use, and isn't worth linting.
    pub lalrpop,
    "/parsers/comments/grammar.rs"
);

// Helper macro for storing parsed tags inside the correct field of a doc comment,
// and extending the doc comment's span to the end of the new tag.
macro_rules! append_tag_to_comment {
    ($comment:ident, $field:ident, $block:expr) => {{
        $comment.span.end = $block.span.end;
        $comment.$field.push($block);
        $comment
    }};
}

pub(self) use append_tag_to_comment; // To let LALRPOP use the macro.

/// Creates a new doc comment with the specified overview and everything else empty.
/// Because of how parsing works, we always get an `Overview` token, so here we check if
/// the overview is actually empty, and if so, set it to `None` instead.
fn create_doc_comment(overview: Option<Overview>, start: Location, file: &str) -> DocComment {
    // We subtract 3 from the start of the comment to account for the leading "///" that is always present.
    // This span is automatically extended as more constructs are parsed.
    let mut span = Span::new(start, start, file);
    span.start.col -= 3;

    // If an overview is present, extend the comment's span to include the overview.
    if let Some(overview_field) = &overview {
        span.end = overview_field.span.end;
    }

    DocComment {
        overview,
        params: Vec::new(),
        returns: Vec::new(),
        throws: Vec::new(),
        see: Vec::new(),
        span,
    }
}

/// Creates a string representing a Slice identifier that can be relatively or globally scoped.
fn get_scoped_identifier_string<'a>(first: &'a str, mut others: Vec<&'a str>, is_globally_scoped: bool) -> String {
    others.insert(0, first);
    if is_globally_scoped {
        others.insert(0, ""); // Gives a leading "::" when we `join`.
    }
    others.join("::")
}

/// Removes any leading whitespace from the inline part of the message, then combines it with any following lines.
fn construct_section_message(inline_message: Option<Message>, message_lines: Option<Message>) -> Message {
    let mut message_lines = message_lines.unwrap_or_default();

    if let Some(mut message) = inline_message {
        // Remove any leading whitespace from the inline portion of the message.
        if let Some(MessageComponent::Text(text)) = message.first_mut() {
            *text = text.trim_start().to_owned();
        }

        // Add a newline to the end of the inline message.
        message.push(MessageComponent::Text("\n".to_owned()));

        // Combine the 2 messages together, with the inline portion at the front.
        message.append(&mut message_lines);
        message_lines = message;
    }

    message_lines
}

/// Removes any common leading whitespace from the provided message lines and returns the result.
/// Each element in the vector represents one line of the message.
/// `None` means the line existed but was empty, `Some(message)` means the line had a message.
fn sanitize_message_lines(lines: Vec<Option<Message>>) -> Message {
    // First compute the amount of leading whitespace that is common to every line.
    let mut common_leading_whitespace = usize::MAX;
    for line in &lines {
        // We only check lines that have a message on them (eg: they're non-empty).
        if let Some(message) = &line {
            // To check the start of the line, we check the first message component.
            // It's safe to unwrap because the parser will have returned `None` for an empty message.
            match message.first().unwrap() {
                MessageComponent::Text(text) => {
                    // Determine how many whitespace characters are at the beginning of this line,
                    // then take the minimum of this and the amount of whitespace on all the other lines so far.
                    let whitespace_index = text.find(|c: char| !c.is_whitespace()).unwrap_or_default();
                    common_leading_whitespace = std::cmp::min(whitespace_index, common_leading_whitespace);
                }
                MessageComponent::Link(_) => {
                    // If a line starts with a link, the common leading whitespace must be 0.
                    // We set this, then exit the loop, since we can't find less than 0 whitespace characters.
                    common_leading_whitespace = 0;
                    break;
                }
            }
        }
    }

    // Now that we know the common leading whitespace, we iterate through the lines again and remove the whitespace.
    lines
        .into_iter()
        .flat_map(|line| match line {
            // If the message had text, we remove the common leading whitespace and append a newline at the end.
            Some(mut message) => {
                if let MessageComponent::Text(text) = message.first_mut().unwrap() {
                    text.replace_range(..common_leading_whitespace, "");
                }
                message.push(MessageComponent::Text("\n".to_owned()));
                message
            }

            // If the line was empty, we create a new message that only contains a newline character.
            None => vec![MessageComponent::Text("\n".to_owned())],
        })
        .collect()
}
