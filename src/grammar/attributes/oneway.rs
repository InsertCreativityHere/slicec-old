// Copyright (c) ZeroC, Inc.

use super::super::Throws;
use super::*;

#[derive(Debug)]
pub struct Oneway {}

impl Oneway {
    pub fn parse_from(Unparsed { directive, args }: &Unparsed, span: &Span, reporter: &mut DiagnosticReporter) -> Self {
        debug_assert_eq!(directive, Self::directive());

        check_that_no_arguments_were_provided(args, Self::directive(), span, reporter);

        Oneway {}
    }

    pub fn validate_on(&self, applied_on: Attributables, span: &Span, reporter: &mut DiagnosticReporter) {
        if let Attributables::Operation(operation) = applied_on {
            // If the operation can return or throw data, it can't be marked oneway.
            if !operation.return_type.is_empty() || !matches!(operation.throws, Throws::None) {
                let note = "operations that return or throw data cannot be marked oneway";
                report_unexpected_attribute(self, span, Some(note), reporter);
            }
        } else {
            let note = "the oneway attribute can only be applied to operations";
            report_unexpected_attribute(self, span, Some(note), reporter);
        }
    }
}

implement_attribute_kind_for!(Oneway, "oneway", false);
