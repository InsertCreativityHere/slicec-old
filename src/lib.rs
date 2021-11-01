// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod ast;
pub mod command_line;
pub mod code_gen_util;
pub mod error;
pub mod grammar;
pub mod parser;
pub mod ptr_visitor;
pub mod slice_file;
pub mod ptr_util;
pub mod validator;
pub mod visitor;

use crate::ast::Ast;
use crate::error::ErrorLevel;
use crate::slice_file::Location;

// TODO rename this!
pub fn main() {
    // Initialize the global instances of the `Ast` and the `ErrorHandler`.
    global_state::initialize();

    // TODO
}

// TODO comments
pub fn borrow_ast() -> &'static Ast {
    unsafe { &*global_state::AST.get().unwrap().get() }
}

// TODO comments
pub unsafe fn borrow_mut_ast() -> &'static mut Ast {
    &mut *global_state::AST.get().unwrap().get()
}

pub fn report_note(message: String, location: Option<Location>) {
    report_error_impl(message, location, ErrorLevel::Note);
}

pub fn report_warning(message: String, location: Option<Location>) {
    report_error_impl(message, location, ErrorLevel::Warning);
}

pub fn report_error(message: String, location: Option<Location>) {
    report_error_impl(message, location, ErrorLevel::Error);
}

pub fn report_critical(message: String, location: Option<Location>) {
    report_error_impl(message, location, ErrorLevel::Critical);
}

fn report_error_impl(message: String, location: Option<Location>, severity: ErrorLevel) {
    let error_reporter = unsafe { &mut *global_state::ERROR_REPORTER.get().unwrap().get() };
    error_reporter.report_error(message, location, severity);
}

mod global_state {
    use crate::ast::Ast;
    use crate::error::ErrorReporter;
    use crate::ptr_util::ThreadSafe;
    use once_cell::unsync::OnceCell;
    use std::cell::UnsafeCell;

    type ThreadSafeCell<T> = ThreadSafe<OnceCell<UnsafeCell<T>>>;

    pub(super) static AST: ThreadSafeCell<Ast> = ThreadSafe(OnceCell::new());
    pub(super) static ERROR_REPORTER: ThreadSafeCell<ErrorReporter> = ThreadSafe(OnceCell::new());

    pub(super) fn initialize() {
        let _ = AST.set(UnsafeCell::new(Ast::new()));
        let _ = ERROR_REPORTER.set(UnsafeCell::new(ErrorReporter::new()));
    }
}
