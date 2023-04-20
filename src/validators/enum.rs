// Copyright (c) ZeroC, Inc.

use crate::diagnostics::{Diagnostic, DiagnosticReporter, Error};
use crate::grammar::*;
use std::collections::HashMap;

// TODO change the order of these to be more logical once the logic has been changed!
pub fn validate(enum_def: &Enum, diagnostic_reporter: &mut DiagnosticReporter) {
    enumerator_values_fit_within_backing_type(enum_def, diagnostic_reporter);
    has_allowed_underlying_type(enum_def, diagnostic_reporter);
    enumerator_values_are_unique(enum_def, diagnostic_reporter);
    underlying_type_cannot_be_optional(enum_def, diagnostic_reporter);
    checked_enums_are_not_empty(enum_def, diagnostic_reporter);
}

/// Validate that all of the enum's enumerators  fit within the bounds of it's underlying type.
fn enumerator_values_fit_within_backing_type(enum_def: &Enum, diagnostic_reporter: &mut DiagnosticReporter) {
    // Determine the range of allowed values for enumerators in this enum.
    let min_and_max = if enum_def.supported_encodings().supports(&Encoding::Slice1) {
        // The enum was defined in a Slice1 file, so it's underlying type is int32 and its enumerators must be positive.
        Some((0, i32::MAX as i128))
    } else if let Some(underlying_type) = &enum_def.underlying {
        // The enum wasn't defined in a Slice1 file, and has an explicit underlying type. Use the bounds of that type.
        underlying_type.numeric_bounds()
    } else {
        // The enum wasn't defined in a Slice1 file, and has an implicit underlying type of varint32.
        Primitive::VarInt32.numeric_bounds()
    };

    // Iterate through the enumerators and report an error for any with values outside the allowed range.
    if let Some((min, max)) = min_and_max {
        for enumerator in enum_def.enumerators() {
            let value = enumerator.value();
            if value < min || value > max {
                let identifier = enumerator.identifier().to_owned();
                Diagnostic::new(Error::EnumeratorValueOutOfBounds { identifier, value, min, max })
                    .set_span(enumerator.span())
                    .report(diagnostic_reporter);
            }
        }
    }
}

/// Validate that if an explicit underlying type was specified, that type is integral (int8, varuint32, etc.).
fn has_allowed_underlying_type(enum_def: &Enum, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(underlying_type) = &enum_def.underlying {
        if !underlying_type.is_integral() {
            Diagnostic::new(Error::UnderlyingTypeMustBeIntegral {
                identifier: enum_def.identifier().to_owned(),
                kind: underlying_type.definition().kind().to_owned(),
            })
            .set_span(enum_def.span())
            .report(diagnostic_reporter);
        }
    }
}

/// Validate that enumerator values aren't re-used within an enum.
fn enumerator_values_are_unique(enum_def: &Enum, diagnostic_reporter: &mut DiagnosticReporter) {
    let mut value_to_enumerator_map: HashMap<i128, &Enumerator> = HashMap::new();
    for enumerator in enum_def.enumerators() {
        // If the value is already in the map, another enumerator already used it. Get that enumerator from the map
        // and emit an error. Otherwise add the enumerator and its value to the map.
        if let Some(alt_enum) = value_to_enumerator_map.get(&enumerator.value()) {
            Diagnostic::new(Error::DuplicateEnumeratorValue {
                value: enumerator.value(),
            })
            .set_span(enumerator.span())
            .add_note(
                format!("the value was previously used by '{}' here:", alt_enum.identifier()),
                Some(alt_enum.span()),
            )
            .report(diagnostic_reporter);
        } else {
            value_to_enumerator_map.insert(enumerator.value(), enumerator);
        }
    }
}

/// Validate the the underlying type of an enum is not optional.
fn underlying_type_cannot_be_optional(enum_def: &Enum, diagnostic_reporter: &mut DiagnosticReporter) {
    if let Some(underlying_type) = &enum_def.underlying {
        if underlying_type.is_optional {
            Diagnostic::new(Error::CannotUseOptionalUnderlyingType {
                identifier: enum_def.identifier().to_owned(),
            })
            .set_span(enum_def.span())
            .report(diagnostic_reporter);
        }
    }
}

/// Validate that checked enums are not empty.
fn checked_enums_are_not_empty(enum_def: &Enum, diagnostic_reporter: &mut DiagnosticReporter) {
    if !enum_def.is_unchecked && enum_def.enumerators.is_empty() {
        Diagnostic::new(Error::MustContainEnumerators {
            identifier: enum_def.identifier().to_owned(),
        })
        .set_span(enum_def.span())
        .report(diagnostic_reporter);
    }
}
