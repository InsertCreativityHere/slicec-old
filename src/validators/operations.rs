// Copyright (c) ZeroC, Inc.

use crate::diagnostics::{Diagnostic, DiagnosticReporter, Warning};
use crate::grammar::*;

pub fn validate_operation(operation: &Operation, diagnostic_reporter: &mut DiagnosticReporter) {
    non_empty_return_comment(operation, diagnostic_reporter);
    missing_parameter_comment(operation, diagnostic_reporter);
    operation_missing_throws(operation, diagnostic_reporter);
}

fn non_empty_return_comment(operation: &Operation, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(comment) = operation.comment() {
        // Return doc comment exists but operation has no return members.
        // `DocComment.return_members` contains a list of descriptions of the return members.
        // example: "@returns: A description of the return value."
        if !comment.returns.is_empty() && operation.return_members().is_empty() {
            for returns_tag in &comment.returns {
                Diagnostic::new(Warning::IncorrectDocComment {
                    message: "comment has a 'returns' tag, but its operation does not return anything".to_owned(),
                })
                .set_span(returns_tag.span())
                .set_scope(operation.parser_scoped_identifier())
                .add_note(
                    format!("operation '{}' does not have a return type", operation.identifier()),
                    Some(operation.span()),
                )
                .report(diagnostic_reporter);
            }
        }
    }
}

fn missing_parameter_comment(operation: &Operation, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(comment) = operation.comment() {
        for param_tag in &comment.params {
            if !operation
                .parameters()
                .iter()
                .any(|param_def| param_def.identifier() == param_tag.identifier.value)
            {
                let param_identifier = &param_tag.identifier.value;
                Diagnostic::new(Warning::IncorrectDocComment {
                    message: format!(
                        "comment has a 'param' tag for '{}', but its operation has no parameter with that name",
                        param_identifier,
                    ),
                })
                .set_span(param_tag.span())
                .set_scope(operation.parser_scoped_identifier())
                .add_note(
                    format!("operation '{}' has no parameter named '{}'", operation.identifier(), param_identifier),
                    Some(operation.span()),
                )
                .report(diagnostic_reporter);
            }
        }
    }
}

fn operation_missing_throws(operation: &Operation, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(comment) = operation.comment() {
        if !comment.throws.is_empty() && matches!(operation.throws, Throws::None) {
            for throws_tag in &comment.throws {
                Diagnostic::new(Warning::IncorrectDocComment {
                    message: "comment has a 'throws' tag, but its operation does not throw anything".to_owned(),
                })
                .set_span(throws_tag.span())
                .set_scope(operation.parser_scoped_identifier())
                .add_note(
                    format!("operation '{}' does not throw", operation.identifier()),
                    Some(operation.span()),
                )
                .report(diagnostic_reporter);
            }
        }
    }
}
