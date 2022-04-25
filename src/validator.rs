// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::grammar::*;
use crate::error::ErrorReporter;
use crate::visitor::Visitor;

#[derive(Debug)]
pub(crate) struct Validator<'a> {
    pub error_reporter: &'a mut ErrorReporter,
}

// TODO add additional validation logic here.
impl<'a> Visitor for Validator<'a> {
    fn visit_struct_start(&mut self, struct_def: &Struct) {
        if struct_def.is_compact {
            // Compact structs cannot be empty.
            if struct_def.members().is_empty() {
                self.error_reporter.report_error(
                    "compact structs cannot be empty",
                    Some(&struct_def.location),
                )
            } else {
                // Compact structs cannot have tagged data members.
                let mut has_tags = false;
                for member in struct_def.members() {
                    if member.tag.is_some() {
                    self.error_reporter.report_error(
                            "tagged data members are not supported in compact structs\n\
                            consider removing the tag, or making the struct non-compact".to_owned(),
                            Some(&member.location),
                        );
                        has_tags = true;
                    }
                }

                if has_tags {
                self.error_reporter.report_note(
                        format!("struct '{}' is declared compact here", struct_def.identifier()),
                        Some(&struct_def.location),
                    );
                }
            }
        }
    }
}
