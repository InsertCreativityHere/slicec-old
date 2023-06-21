// Copyright (c) ZeroC, Inc.

use crate::diagnostics::{Diagnostic, DiagnosticReporter, Warning};
use crate::grammar::*;

pub fn validate_common_doc_comments(commentable: &dyn Commentable, diagnostic_reporter: &mut DiagnosticReporter) {
    only_operations_can_throw(commentable, diagnostic_reporter);
}

fn only_operations_can_throw(commentable: &dyn Commentable, diagnostic_reporter: &mut DiagnosticReporter) {
    let supported_on = ["operation"];
    if let Some(comment) = commentable.comment() {
        if !supported_on.contains(&commentable.kind()) && !comment.throws.is_empty() {
            for throws_tag in &comment.throws {
                let kind = commentable.kind();
                let note = format!(
                    "'{}' is {} {}",
                    commentable.identifier(),
                    crate::utils::string_util::indefinite_article(kind),
                    kind,
                );

                Diagnostic::new(Warning::IncorrectDocComment {
                    message: "comment has a 'throws' tag, but only operations can throw".to_owned(),
                })
                .set_span(throws_tag.span())
                .set_scope(commentable.parser_scoped_identifier())
                .add_note(note, Some(commentable.span()))
                .report(diagnostic_reporter);
            }
        }
    }
}
