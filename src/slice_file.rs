// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::grammar::{Attribute, Module};
use crate::util::WeakPtr;

#[derive(Clone, Debug)]
pub struct Location {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub file: String,
}

pub struct SliceFile {
    pub filename: String,
    pub relative_path: String,
    pub raw_text: String,
    pub contents: Vec<WeakPtr<Module>>,
    pub attributes: Vec<Attribute>,
    pub is_source: bool,
    line_positions: Vec<usize>,
}

// TODO
