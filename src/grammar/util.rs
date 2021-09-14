// Copyright (c) ZeroC, Inc. All rights reserved.

#[derive(Clone, Debug)]
pub struct Location {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub file: String,
}

#[derive(Clone, Debug)]
pub struct Scope {
    pub scope: Vec<String>,
    pub raw_scope: String,
    pub parser_scope: Vec<String>,
    pub raw_parser_scope: String,
}
