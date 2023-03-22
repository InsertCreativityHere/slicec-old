// Copyright (c) ZeroC, Inc.

use crate::diagnostics::{Diagnostic, DiagnosticReporter, Error};
use crate::grammar::*;
use crate::validators::{ValidationChain, Validator};

pub fn miscellaneous_validators() -> ValidationChain {
    vec![
        Validator::Parameters(stream_parameter_is_last),
        Validator::Parameters(at_most_one_stream_parameter),
        Validator::Struct(validate_compact_struct_not_empty),
        Validator::Module(file_scoped_modules_cannot_contain_sub_modules),
        Validator::TypeAlias(type_aliases_cannot_be_optional),
    ]
}

fn file_scoped_modules_cannot_contain_sub_modules(module_def: &Module, diagnostic_reporter: &mut DiagnosticReporter) {
    if module_def.is_file_scoped {
        module_def.submodules().iter().for_each(|submodule| {
            Diagnostic::new(Error::FileScopedModuleCannotContainSubModules {
                identifier: module_def.identifier().to_owned(),
            })
            .set_span(submodule.span())
            .report(diagnostic_reporter);
        });
    }
}

fn at_most_one_stream_parameter(members: &[&Parameter], diagnostic_reporter: &mut DiagnosticReporter) {
    let streamed_members = members.iter().filter(|member| member.is_streamed).collect::<Vec<_>>();
    if streamed_members.len() > 1 {
        streamed_members
        .split_last() // Split at the last element, which is the one we do not want to report an error for.
        .unwrap().1 // All members before the split. Safe to unwrap since we know there are at least two members.
        .iter()
        .for_each(|m| Diagnostic::new(Error::MultipleStreamedMembers).set_span(m.span()).report(diagnostic_reporter));
    }
}

fn stream_parameter_is_last(members: &[&Parameter], diagnostic_reporter: &mut DiagnosticReporter) {
    members
        .split_last() // Returns None if members is empty.
        .map_or(vec![], |(_, remaining)| remaining.to_vec())
        .into_iter()
        .filter(|m| m.is_streamed)
        .for_each(|m| {
           Diagnostic::new(Error::StreamedMembersMustBeLast { parameter_identifier: m.identifier().to_owned() })
                .set_span(m.span())
                .report(diagnostic_reporter);
        });
}

fn validate_compact_struct_not_empty(struct_def: &Struct, diagnostic_reporter: &mut DiagnosticReporter) {
    // Compact structs must be non-empty.
    if struct_def.is_compact && struct_def.fields().is_empty() {
        Diagnostic::new(Error::CompactStructCannotBeEmpty)
            .set_span(struct_def.span())
            .report(diagnostic_reporter);
    }
}

fn type_aliases_cannot_be_optional(type_alias: &TypeAlias, diagnostic_reporter: &mut DiagnosticReporter) {
    if type_alias.underlying.is_optional {
        Diagnostic::new(Error::TypeAliasOfOptional)
            .set_span(type_alias.span())
            .add_note(
                "try removing the trailing `?` modifier from its definition",
                Some(type_alias.underlying.span()),
            )
            .add_note(
                "instead of aliasing an optional type directly, try making it optional where you use it",
                None,
            )
            .report(diagnostic_reporter)
    }
}
