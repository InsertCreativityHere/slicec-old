// Copyright (c) ZeroC, Inc.

pub mod ast;
pub mod code_block;
pub mod command_line;
pub mod compilation_state;
pub mod diagnostics;
pub mod grammar;
pub mod parsers;
pub mod slice_file;
pub mod supported_encodings;
pub mod test_helpers;
pub mod utils;
pub mod validators;
pub mod visitor;

// Re-export the `clap`, `convert_case`, and `in_definite` dependencies.
pub extern crate clap;
pub extern crate convert_case;
pub extern crate in_definite;

use command_line::SliceOptions;
use compilation_state::CompilationState;
use slice_file::SliceFile;
use std::collections::HashSet;
use utils::file_util;

pub fn compile_from_options(options: &SliceOptions) -> CompilationState {
    // Create an instance of `CompilationState` for holding all the compiler's state.
    let mut state = CompilationState::create(options);

    // Recursively resolve any Slice files contained in the paths specified by the user.
    let files = file_util::resolve_files_from(options, &mut state.diagnostic_reporter);

    // If any files were unreadable, return without parsing. Otherwise, parse the files normally.
    if !state.diagnostic_reporter.has_errors() {
        compile_files(files, &mut state, options);
    }
    state
}

pub fn compile_from_strings(inputs: &[&str], options: Option<SliceOptions>) -> CompilationState {
    let slice_options = options.unwrap_or_default();

    // Create an instance of `CompilationState` for holding all the compiler's state.
    let mut state = CompilationState::create(&slice_options);

    // Create a Slice file from each of the strings.
    let mut files = Vec::new();
    for (i, &input) in inputs.iter().enumerate() {
        files.push(SliceFile::new(format!("string-{i}"), input.to_owned(), false))
    }

    compile_files(files, &mut state, &slice_options);
    state
}

fn compile_files(files: Vec<SliceFile>, state: &mut CompilationState, options: &SliceOptions) {
    // Convert the `Vec<SliceFile>` into a `HashMap<absolute_path, SliceFile>` for easier lookup, and store it.
    state.files = files.into_iter().map(|f| (f.relative_path.clone(), f)).collect();

    // Retrieve any preprocessor symbols defined by the compiler itself, or by the user on the command line.
    let defined_symbols = HashSet::from_iter(options.definitions.clone());

    // There are 3 phases of compilation handled by `slicec`:
    // 1) Parse the files passed in by the user.
    // 2) Patch the abstract syntax tree generated by the parser.
    // 3) Check the user's Slice definitions for language-mapping agnostic errors.
    parsers::parse_files(state, &defined_symbols);
    unsafe { state.then_apply_unsafe(ast::patch_ast) };
    state.then_apply(validators::validate_ast);
}
