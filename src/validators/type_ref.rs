// Copyright (c) ZeroC, Inc.

use super::{attributes, dictionary, sequence};
use crate::diagnostics::DiagnosticReporter;
use crate::grammar::{TypeRef, Types, Attributable};

pub fn validate(type_ref: &TypeRef, diagnostic_reporter: &mut DiagnosticReporter) {
    attributes::validate(type_ref.attributes(false), type_ref, diagnostic_reporter);
    match type_ref.concrete_type() {
        Types::Dictionary(dictionary) => dictionary::validate(dictionary, diagnostic_reporter),
        Types::Sequence(sequence) => sequence::validate(sequence, diagnostic_reporter),
        _ => {}
    }
}
