// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::command_line::SliceOptions;
use crate::diagnostics::{DiagnosticReporter, Error, ErrorKind};
use crate::slice_file::SliceFile;

use std::path::PathBuf;
use std::{fs, io};

pub fn get_files_from_command_line(
    options: &SliceOptions,
    diagnostic_reporter: &mut DiagnosticReporter,
) -> Vec<SliceFile> {
    // TODO simplify and fix this by keeping a `HashMap<path, is_source>` instead of 2 vecs!
    // This means we can drop the `retain` and `dedup` since those are for free with a HashMap.
    // See: www.github.com/zeroc-ice/icerpc/issues/298

    let source_files = find_slice_files(&options.sources);
    let mut reference_files = find_slice_files(&options.references);
    // Remove duplicate reference files, or files that are already being parsed as source.
    // This ensures that a file isn't parsed twice, which would cause redefinition errors.
    reference_files.retain(|file| !source_files.contains(file));
    reference_files.sort();
    reference_files.dedup();

    let mut files = Vec::new();
    for path in source_files {
        match fs::read_to_string(&path) {
            Ok(raw_text) => files.push(SliceFile::new_unparsed(path, raw_text, true)),
            Err(err) => diagnostic_reporter.report_error(Error::new(ErrorKind::IO(err), None)),
        }
    }
    for path in reference_files {
        match fs::read_to_string(&path) {
            Ok(raw_text) => files.push(SliceFile::new_unparsed(path, raw_text, false)),
            Err(err) => diagnostic_reporter.report_error(Error::new(ErrorKind::IO(err), None)),
        }
    }
    files
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
