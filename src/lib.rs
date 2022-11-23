// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod ast;
pub mod code_block;
pub mod command_line;
pub mod compilation_result;
pub mod diagnostics;
pub mod grammar;
pub mod parser;
pub mod parsers;
pub mod slice_file;
pub mod supported_encodings;
pub mod utils;
pub mod validators;
pub mod visitor;

// Re-export the `clap` and `convert_case` dependencies.
pub extern crate clap;
pub extern crate convert_case;

use crate::command_line::SliceOptions;
use compilation_result::{CompilationData, CompilationResult};
use slice_file::SliceFile;
use utils::file_util;

use std::collections::{HashMap, HashSet};

/// TODO write a module comment about early exit and phases of compilation!

/// TODO
pub fn compile_files_from_options(options: &SliceOptions) -> CompilationResult {
    // Create an instance of `CompilationData` for holding all the compiler's state.
    let mut data = CompilationData::create(options);

    // TODO
    let files = file_util::get_files_from_command_line(options, &mut data.diagnostic_reporter);

    // TODO
    match data.diagnostic_reporter.has_errors() {
        true => data.into(),
        false => compile_files(files, data, options),
    }
}

/// TODO
fn compile_files(files: Vec<SliceFile>, mut data: CompilationData, options: &SliceOptions) -> CompilationResult {
    // Convert the vector of files into a hashmap of (filename, file) for easier lookup and store it.
    data.files = files
        .into_iter()
        .map(|file| (file.filename.clone(), file))
        .collect::<HashMap<_, _>>();

    // Retrieve any preprocessor symbols defined by the compiler itself or by the user (on the command line).
    let defined_symbols = HashSet::from_iter(options.definitions.iter().cloned());

    // There are 3 phases of compilation handled by `slicec`:
    // 1) Parse the files passed in by the user.
    // 2) Patch the abstract syntax tree generated by the parser.
    // 3) Check the user's Slice definitions for language-mapping-agnostic errors.
    parsers::parse_files(data, &defined_symbols)
        .and_then(|data| unsafe { ast::patch_ast(data) })
        .and_then(validators::validate_compilation_data)
}

/// TODO
#[cfg(test)]
pub fn compile_files_from_strings(strings: &[&str], options: Option<SliceOptions>) -> CompilationResult {
    // Use a set of default options if none were provided.
    let slice_options = options.unwrap_or(SliceOptions {
        sources: vec![],
        references: vec![],
        warn_as_error: true,
        disable_color: false,
        diagnostic_format: DiagnosticFormat::Human,
        validate: false,
        output_dir: None,
        definitions: vec![],
    });

    // Create an instance of `CompilationData` for holding all the compiler's state.
    let data = CompilationData::create(&slice_options);

    // Create a SliceFile from each of the strings.
    let string_files = strings
        .iter()
        .enumerate()
        .map(|(i, s)| SliceFile::new_unparsed(format!("string-{i}"), s.to_owned(), false))
        .collect();

    // Compile the strings as if they were files.
    compile_files(string_files, data, &slice_options)
}
