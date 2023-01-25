// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::ast::Ast;
use crate::diagnostics::{DiagnosticReporter, Warning, WarningKind};
use crate::grammar::*;
use crate::validators::{ValidationChain, Validator};

pub fn comments_validators() -> ValidationChain {
    vec![
        Validator::Entities(only_operations_can_throw),
        Validator::Operations(non_empty_return_comment),
        Validator::Operations(missing_parameter_comment),
        Validator::DocComments(linked_identifiers_exist),
    ]
}

fn non_empty_return_comment(operation: &Operation, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(comment) = operation.comment() {
        // Return doc comment exists but operation has no return members.
        // `DocComment.return_members` contains a list of descriptions of the return members.
        // example: @returns: A description of the return value.
        if !comment.returns.is_empty() && operation.return_members().is_empty() {
            Warning::new(WarningKind::ExtraReturnValueInDocComment)
                .set_span(comment.span())
                .report(diagnostic_reporter, Some(operation));
        }
    }
}

fn missing_parameter_comment(operation: &Operation, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(comment) = operation.comment() {
        comment.params.iter().for_each(|param| {
            if !operation
                .parameters()
                .iter()
                .map(|p| p.identifier.value.clone())
                .any(|identifier| identifier == param.identifier.value.clone())
            {
                Warning::new(WarningKind::ExtraParameterInDocComment {
                    identifier: param.identifier.value.clone(),
                })
                .set_span(comment.span())
                .report(diagnostic_reporter, Some(operation));
            }
        });
    }
}

fn only_operations_can_throw(entity: &dyn Entity, diagnostic_reporter: &mut DiagnosticReporter) {
    let supported_on = ["operation"];
    if let Some(comment) = entity.comment() {
        if !supported_on.contains(&entity.kind()) && !comment.throws.is_empty() {
            let warning_kind = WarningKind::ExtraThrowInDocComment {
                kind: entity.kind().to_owned(),
                identifier: entity.identifier().to_owned(),
            };
            Warning::new(warning_kind)
                .set_span(comment.span())
                .report(diagnostic_reporter, Some(entity))
        };
    }
}

fn linked_identifiers_exist(entity: &dyn Entity, ast: &Ast, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(comment) = entity.comment() {
        if let Some(overview) = comment.overview.as_ref() {
            for component in &overview.message {
                if let MessageComponent::Link(link_tag) = component {
                    // Todo: We should issue different errors if the thing doesn't exist vs isn't the right type.
                    if ast
                        .find_element_with_scope::<dyn Entity>(&link_tag.link.value, entity.module_scope())
                        .is_err()
                    {
                        Warning::new(WarningKind::InvalidDocCommentLinkIdentifier {
                            identifier: link_tag.link.value.to_owned(),
                        })
                        .set_span(comment.span())
                        .report(diagnostic_reporter, Some(entity));
                    }
                }
            }
        }
    }
}
