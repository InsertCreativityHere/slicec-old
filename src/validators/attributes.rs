// Copyright (c) ZeroC, Inc.

use crate::diagnostics::DiagnosticReporter;
use crate::grammar::{Attributable, Attribute};

pub fn validate(
    attributes: Vec<&Attribute>,
    applied_to: &impl Attributable,
    diagnostic_reporter: &mut DiagnosticReporter,
) {
    // TODO
}
