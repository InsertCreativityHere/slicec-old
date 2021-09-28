// Copyright (c) ZeroC, Inc. All rights reserved.

use std::cell::UnsafeCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Location {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub file: String,
}

#[derive(Debug)]
pub struct OwnedPtr<T: ?Sized> {
    // `UnsafeCell` is a magic type, and the ONLY way to signal to the Rust compiler that this data
    // has interior mutability semantics. No other type can work.
    data: Rc<UnsafeCell<T>>,
}

impl<T: Sized> OwnedPtr<T> {
    pub fn new(value: T) -> Self {
        OwnedPtr { data: Rc::new(UnsafeCell::new(value)) }
    }
}

impl<T: ?Sized> OwnedPtr<T> {
    pub fn from_inner(ptr: Rc<UnsafeCell<T>>) -> Self {
        OwnedPtr { data: ptr }
    }

    pub fn into_inner(self) -> Rc<UnsafeCell<T>> {
        self.data
    }

    pub fn downgrade(&self) -> WeakPtr<T> {
        WeakPtr { data: Some(&*self.data) }
    }

    // TODO explaine why this isn't marked unsafe!
    pub fn borrow(&self) -> &T {
        unsafe { &*self.data.get() }
    }

    pub unsafe fn borrow_mut(&self) -> &mut T {
        &mut *self.data.get()
    }
}

impl<T: ?Sized> Clone for OwnedPtr<T> {
    fn clone(&self) -> Self {
        OwnedPtr { data: self.data.clone() }
    }
}

#[derive(Debug)]
pub struct WeakPtr<T: ?Sized> {
    data: Option<*const UnsafeCell<T>>,
}

// TODO It's only safe to call methods on this thing if it's initialized!

impl<T: ?Sized> WeakPtr<T> {
    pub fn uninitialized() -> Self {
        WeakPtr { data: None }
    }

    pub fn from_inner(ptr: *const UnsafeCell<T>) -> Self {
        WeakPtr { data: Some(ptr) }
    }

    pub fn into_inner(self) -> *const UnsafeCell<T> {
        self.data.unwrap()
    }

    // TODO
    // This isn't marked as unsafe because it's assumed all WeakPtr live inside the AST, alongside
    // the OwnedPtr. Since the entire AST goes out of scope at the same time when the program ends,
    // it's impossible to have a dangling pointer here, and so, this function is safe.

    // TODO explaine why this isn't marked unsafe!
    pub fn borrow(&self) -> &T {
        unsafe { &*(*self.data.unwrap()).get() }
    }
}

impl<T: ?Sized> Clone for WeakPtr<T> {
    fn clone(&self) -> Self {
        WeakPtr { data: self.data }
    }
}

// TODO
// These are only temporary because CoerceUnsized is still marked as unstable. When they are FINALLY
// stabilized, we should delete them and let the compiler do the coercing for us.
//
//impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<OwnedPtr<U>> for OwnedPtr<T> {}
//impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<WeakPtr<U>> for WeakPtr<T> {}

#[macro_export]
macro_rules! cast_owned_as {
    ($owned:expr, $new_type:ty) => {{
        let inner: Rc<UnsafeCell<$new_type>> = $owned.into_inner();
        OwnedPtr::from_inner(inner)
    }};
}

#[macro_export]
macro_rules! cast_weak_as {
    ($weak:expr, $new_type:ty) => {{
        let inner: *const std::cell::UnsafeCell<$new_type> = $weak.into_inner();
        WeakPtr::from_inner(inner)
    }};
}

#[macro_export]
macro_rules! downgrade_as {
    ($owned:expr, $new_type:ty) => {
        crate::cast_weak_as!($owned.downgrade(), $new_type)
    };
}

// TODO

// TODO maybe use a Rc<RefCell<T>> as the backing type instead of a Box<T>... and then have
// WeakPtr just store a *RefCell<T>. This still lets us borrow things, but also enforces the borrow
// checking rules at runtime so we can avoid Undefined Behavior... Not sure how important it'll be.
// But the downside here is that the API will have to return and deal with `Ref` instead of `&`.
/*

// This can be derefenced when dangling. It will always be aligned, if the pointeee exists.
// But we just raw dereference this thing. It could totally break if the OwnedPtr isn't around.
// This is unlikely because the AST is effectively static, but you never know.

*/