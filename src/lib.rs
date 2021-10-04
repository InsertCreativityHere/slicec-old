// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod ast;
pub mod error;
pub mod grammar;
pub mod ptr_visitor;
pub mod slice_file;
pub mod util;
pub mod visitor;

use crate::ast::Ast;

pub fn borrow_ast() -> &'static Ast {
    global_state::AST.get().unwrap().borrow()
}

pub unsafe fn borrow_mut_ast() -> &'static mut Ast {
    global_state::AST.get().unwrap().borrow_mut()
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
    use crate::util::OwnedPtr;
    use once_cell::unsync::OnceCell;

    pub(super) static AST: ThreadSafe<Ast> = ThreadSafe(OnceCell::new());
    pub(super) static ERROR_REPORTER: ThreadSafe<ErrorReporter> = ThreadSafe(OnceCell::new());

    pub(super) fn initialize() {
        let _ = AST.set(OwnedPtr::new(Ast::new()));
        let _ = ERROR_REPORTER.set(OwnedPtr::new(ErrorReporter::new()));
    }

    // `ThreadSafe` is a transparent wrapper for marking data as thread-safe, even if it isn't.
    // The Rust compiler automatically infers thread-safety at compile time, but data can be
    // explicitily marked as thread-safe by implementing the `Sync` trait on it, like here.
    //
    // We use this as a hack to satisfy the Rust compiler. Only thread-safe data can be stored in
    // static variables, since it MIGHT be accessed from other threads. But since the slice
    // compiler is single threaded, this isn't a concern.
    //
    // If we ever make the slice compiler multi-threaded we'd have to make the data thread-safe
    // anyways, and then could drop this hack.
    pub(super) struct ThreadSafe<T>(OnceCell<OwnedPtr<T>>);

    unsafe impl<T> Sync for ThreadSafe<T> {}

    impl<T> std::ops::Deref for ThreadSafe<T> {
        type Target = OnceCell<OwnedPtr<T>>;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
}
