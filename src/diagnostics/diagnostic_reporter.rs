// Copyright (c) ZeroC, Inc.

use crate::ast::Ast;
use crate::command_line::{DiagnosticFormat, SliceOptions};
use crate::diagnostics::{Diagnostic, DiagnosticKind, Diagnostics, Warning};
use crate::grammar::{validate_allow_arguments, Attributable, Attribute, Entity};
use crate::slice_file::SliceFile;
use std::collections::HashMap;

#[derive(Debug)]
pub struct DiagnosticReporter {
    /// Stores all the diagnostics that have been reported, in the order they were reported.
    diagnostics: Diagnostics,
    /// If true, compilation will fail on warnings in addition to errors.
    treat_warnings_as_errors: bool,
    /// Lists all the warnings that should be suppressed by this reporter.
    pub allowed_warnings: Vec<String>,
    /// Can specify json to serialize errors as JSON or console to output errors to console.
    pub diagnostic_format: DiagnosticFormat,
    /// If true, diagnostic output will not be styled with colors.
    pub disable_color: bool,
}

impl DiagnosticReporter {
    pub fn new(slice_options: &SliceOptions) -> Self {
        let mut diagnostic_reporter = DiagnosticReporter {
            diagnostics: Diagnostics::default(),
            treat_warnings_as_errors: slice_options.warn_as_error,
            diagnostic_format: slice_options.diagnostic_format,
            disable_color: slice_options.disable_color,
            allowed_warnings: slice_options.allowed_warnings.clone(),
        };

        // Validate any arguments passed to `--allow` on the command line.
        validate_allow_arguments(&slice_options.allowed_warnings, None, &mut diagnostic_reporter);

        diagnostic_reporter
    }

    /// Checks if any errors have been reported during compilation.
    pub fn has_errors(&self) -> bool {
        self.diagnostics.error_count != 0
    }

    /// Checks if any diagnostics (warnings or errors) have been reported during compilation.
    pub fn has_diagnostics(&self) -> bool {
        self.diagnostics.error_count + self.diagnostics.warning_count != 0
    }

    /// Returns the total number of errors and warnings reported through the diagnostic reporter.
    pub fn get_totals(&self) -> (usize, usize) {
        (self.diagnostics.error_count, self.diagnostics.warning_count)
    }

    /// Returns 1 if any errors were reported and 0 if no errors were reported.
    /// If `treat_warnings_as_errors` is true, warnings well be counted as errors by this function.
    pub fn get_exit_code(&self) -> i32 {
        i32::from(self.has_errors() || (self.treat_warnings_as_errors && self.has_diagnostics()))
    }

    /// Consumes the diagnostic reporter and returns an iterator over its diagnostics, with any suppressed warnings
    /// filtered out. (ie: any warnings covered by an `allow` attribute or a `--allow` command line flag).
    pub fn into_diagnostics<'a>(
        mut self,
        ast: &'a Ast,
        files: &'a HashMap<String, SliceFile>,
    ) -> Diagnostics {
        // Helper function that checks whether a warning should be suppressed according to the provided identifiers.
        fn is_warning_suppressed_by<'b>(mut identifiers: impl Iterator<Item = &'b String>, warning: &Warning) -> bool {
            identifiers.any(|identifier| identifier == "All" || identifier == warning.error_code())
        }

        // Helper function that checks whether a warning should be suppressed according to the provided attributes.
        fn is_warning_suppressed_by_attributes(attributes: Vec<&Attribute>, warning: &Warning) -> bool {
            let mut allowed_warnings = attributes.into_iter().filter_map(Attribute::match_allow_warnings);
            allowed_warnings.any(|allowed| is_warning_suppressed_by(allowed.iter(), warning))
        }

        // Filter out any diagnostics that should be suppressed.
        self.diagnostics.inner.retain(|diagnostic| {
            let mut is_suppressed = false;

            if let DiagnosticKind::Warning(warning) = &diagnostic.kind {
                // Check if the warning is suppressed by an `--allow` flag passed on the command line.
                is_suppressed |= is_warning_suppressed_by(self.allowed_warnings.iter(), warning);

                // If the warning has a span, check if it's suppressed by an `allow` attribute on its file.
                if let Some(span) = diagnostic.span() {
                    let file = files.get(&span.file).expect("slice file didn't exist");
                    is_suppressed |= is_warning_suppressed_by_attributes(file.attributes(false), warning);
                }

                // If the warning has a scope, check if it's suppressed by an `allow` attribute in that scope.
                if let Some(scope) = diagnostic.scope() {
                    let entity = ast.find_element::<dyn Entity>(scope).expect("entity didn't exist");
                    is_suppressed |= is_warning_suppressed_by_attributes(entity.attributes(true), warning);
                }
            }
            !is_suppressed
        });

        self.diagnostics
    }

    pub(super) fn report(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}
