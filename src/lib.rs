// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod ast;
pub mod error;
pub mod grammar;
pub mod parser;
pub mod ptr_visitor;
pub mod slice_file;
pub mod util;
pub mod visitor;

use crate::ast::Ast;

// TODO comments
pub fn borrow_ast() -> &'static Ast {
    unsafe { &*global_state::AST.get().unwrap().get() }
}

// TODO comments
pub unsafe fn borrow_mut_ast() -> &'static mut Ast {
    &mut *global_state::AST.get().unwrap().get()
}

pub fn report_error() {
    unimplemented!() //TODO
}

pub fn main() {
    // Initialize the global instances of the `Ast` and the `ErrorHandler`.
    global_state::initialize();

    // TODO
}

mod global_state {
    use crate::ast::Ast;
    use crate::error::ErrorReporter;
    use crate::util::ThreadSafe;
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
