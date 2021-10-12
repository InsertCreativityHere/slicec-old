// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::slice_file::Location;

pub struct ErrorReporter {
    // TODO
}

impl ErrorReporter {
    pub fn new() -> Self {
        ErrorReporter {}
    }

    pub fn report_error(&mut self,
        message: String,
        location: Option<Location>,
        severity: ErrorLevel)
    {
        // Critical should print everything and immediately terminate,
        // error should stop at the next convenient location,
        // warning and note are okay.
    }
}

pub struct Error {
    message: String,
    location: Location,
    severity: ErrorLevel,
}

pub enum ErrorLevel {
    Critical,
    Error,
    Warning,
    Note,
}
