// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod comments;
mod cycle_detection;

use crate::ast::Ast;
use crate::command_line::{DiagnosticFormat, SliceOptions};
use crate::diagnostics::{DiagnosticReporter, Error, ErrorKind};
use crate::grammar::attributes;
use crate::parse_result::{ParsedData, ParserResult};
use crate::slice_file::SliceFile;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::{fs, io};

// NOTE! it is NOT safe to call any methods on any of the slice entities during parsing.
// Slice entities are NOT considered fully constructed until AFTER parsing is finished (including
// patching). Accessing ANY data, or calling ANY methods before this point may result in panics or
// undefined behavior.

pub fn parse_files(options: &SliceOptions) -> ParserResult {
    let mut data = ParsedData {
        ast: Ast::create(),
        diagnostic_reporter: DiagnosticReporter::new(options),
        files: HashMap::new(),
    };

    let source_files = find_slice_files(&options.sources);
    let mut reference_files = find_slice_files(&options.references);
    // Remove duplicate reference files, or files that are already being parsed as source.
    // This ensures that a file isn't parsed twice, which would cause redefinition errors.
    reference_files.retain(|file| !source_files.contains(file));
    reference_files.sort();
    reference_files.dedup();

    for path in source_files {
        if let Some(slice_file) = try_parse_file(&path, true, &mut data) {
            data.files.insert(path.to_owned(), slice_file);
        }
    }

    for path in reference_files {
        if let Some(slice_file) = try_parse_file(&path, false, &mut data) {
            data.files.insert(path.to_owned(), slice_file);
        }
    }

    // Update the diagnostic reporter with the slice files that contain the file level ignoreWarnings attribute.
    data.diagnostic_reporter.file_level_ignored_warnings = file_ignored_warnings_map(&data.files);

    patch_ast(data)
}

fn try_parse_file(file: &str, is_source: bool, data: &mut ParsedData) -> Option<SliceFile> {
    match fs::read_to_string(file) {
        Ok(raw_text) => {
            // The parser emits errors through `DiagnosticReporter` on it's own, so we don't need to handle them.
            try_parse_string(file, &raw_text, is_source, data).ok()
        }
        Err(err) => {
            data.diagnostic_reporter
                .report_error(Error::new(ErrorKind::IO(err), None));
            None
        }
    }
}

pub fn parse_strings(inputs: &[&str], options: Option<SliceOptions>) -> ParserResult {
    let slice_options = options.unwrap_or(SliceOptions {
        sources: vec![],
        references: vec![],
        warn_as_error: true,
        disable_color: false,
        diagnostic_format: DiagnosticFormat::Human,
        validate: false,
        output_dir: None,
    });

    let mut data = ParsedData {
        ast: Ast::create(),
        diagnostic_reporter: DiagnosticReporter::new(&slice_options),
        files: HashMap::new(),
    };

    for (i, input) in inputs.iter().enumerate() {
        let name = format!("string-{}", i);
        if let Ok(slice_file) = try_parse_string(&name, input, false, &mut data) {
            data.files.insert(slice_file.filename.clone(), slice_file);
        }
    }

    // Update the diagnostic reporter with the slice files that contain the file level ignoreWarnings attribute.
    data.diagnostic_reporter.file_level_ignored_warnings = file_ignored_warnings_map(&data.files);

    patch_ast(data)
}

fn try_parse_string(file: &str, raw_text: &str, is_source: bool, data: &mut ParsedData) -> Result<SliceFile, ()> {
    let ast = &mut data.ast;
    let diagnostic_reporter = &mut data.diagnostic_reporter;

    // Run the raw text through the preprocessor.
    let mut defined_symbols = HashSet::new();
    let mut preprocessor = crate::parsers::Preprocessor::new(file, &mut defined_symbols, diagnostic_reporter);
    let preprocessed_text = preprocessor.parse_slice_file(raw_text)?;

    // Run the preprocessed text through the parser.
    let mut parser = crate::parsers::Parser::new(file, ast, diagnostic_reporter);
    let (file_encoding, file_attributes, modules) = parser.parse_slice_file(preprocessed_text)?;

    // Add the top-level-modules into the AST, but keep `WeakPtr`s to them.
    let top_level_modules = modules
        .into_iter()
        .map(|module| ast.add_named_element(module))
        .collect::<Vec<_>>();

    Ok(SliceFile::new(
        file.to_owned(),
        raw_text.to_owned(),
        top_level_modules,
        file_attributes,
        file_encoding,
        is_source,
    ))
}

fn patch_ast(mut parsed_data: ParsedData) -> ParserResult {
    // TODO integrate this better with ParsedData in the future.
    if !parsed_data.has_errors() {
        unsafe {
            parsed_data = crate::ast::patch_ast(parsed_data)?;
        }
    }

    // TODO move this to a validator now that the patchers can handle traversing cycles on their own.
    if !parsed_data.has_errors() {
        cycle_detection::detect_cycles(&parsed_data.files, &mut parsed_data.diagnostic_reporter);
    }

    parsed_data.into()
}

fn find_slice_files(paths: &[String]) -> Vec<String> {
    let mut slice_paths = Vec::new();
    for path in paths {
        match find_slice_files_in_path(PathBuf::from(path)) {
            Ok(child_paths) => slice_paths.extend(child_paths),
            Err(err) => eprintln!("failed to read file '{}': {}", path, err),
        }
    }

    let mut string_paths = slice_paths
        .into_iter()
        .map(|path| path.to_str().unwrap().to_owned())
        .collect::<Vec<_>>();

    string_paths.sort();
    string_paths.dedup();
    string_paths
}

// Returns a HashMap where the keys are the relative paths of the .slice files that have the file level
// `ignoreWarnings` attribute and the values are the arguments of the attribute.
fn file_ignored_warnings_map(files: &HashMap<String, SliceFile>) -> HashMap<String, Vec<String>> {
    files
        .iter()
        .filter_map(|(path, file)| {
            file.attributes
                .iter()
                .find(|attr| attr.directive == attributes::IGNORE_WARNINGS)
                .map(|attr| attr.arguments.clone())
                .map(|ignored_warnings| (path.to_owned(), ignored_warnings))
        })
        .collect()
}

fn find_slice_files_in_path(path: PathBuf) -> io::Result<Vec<PathBuf>> {
    // If the path is a directory, recursively search it for more slice files.
    if fs::metadata(&path)?.is_dir() {
        find_slice_files_in_directory(path.read_dir()?)
    }
    // If the path is not a directory, check if it ends with 'slice'.
    else if path.extension().filter(|ext| ext.to_str() == Some("slice")).is_some() {
        Ok(vec![path])
    } else {
        Ok(vec![])
    }
}

fn find_slice_files_in_directory(dir: fs::ReadDir) -> io::Result<Vec<PathBuf>> {
    let mut paths = Vec::new();
    for child in dir {
        let child_path = child?.path();
        match find_slice_files_in_path(child_path.clone()) {
            Ok(child_paths) => paths.extend(child_paths),
            Err(err) => eprintln!("failed to read file '{}': {}", child_path.display(), err),
        }
    }
    Ok(paths)
}
