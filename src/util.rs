// Copyright (c) ZeroC, Inc. All rights reserved.

use std::cell::UnsafeCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Location {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub file: String,
}

#[derive(Clone, Debug)]
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
    pub fn downgrade(&self) -> WeakPtr<T> {
        WeakPtr { data: &*self.data }
    }

    pub fn into_inner(self) -> Rc<UnsafeCell<T>> {
        self.data
    }

    pub fn borrow<'ptr>(&'ptr self) -> &'ptr T {
        &*self.data.get()
    }

    pub unsafe fn borrow_mut<'ptr>(&'ptr self) -> &'ptr mut T {
        self.data.get_mut()
    }
}

#[derive(Clone, Debug)]
pub struct WeakPtr<T: ?Sized> {
    data: *const UnsafeCell<T>,
}

impl<T: ?Sized> WeakPtr<T> {
    // TODO
    // This isn't marked as unsafe because it's assumed all WeakPtr live inside the AST, alongside
    // the OwnedPtr. Since the entire AST goes out of scope at the same time when the program ends,
    // it's impossible to have a dangling pointer here, and so, this function is safe.
    pub fn borrow(&self) -> &T {
        &*(*self.data).get()
    }

    pub fn into_inner(self) -> *const UnsafeCell<T> {
        self.data
    }
}

// TODO
// These are only temporary because CoerceUnsized is still marked as unstable. When they are FINALLY
// stabilized, we should delete them and let the compiler do the coercing for us.

#[macro_export]
macro_rules! cast_owned_as {
    ($owned:expr, $new_type:ty) => {{
        let inner: Rc<UnsafeCell<$new_type>> = $owned.data;
        OwnedPtr {data: inner}
    }};
}

#[macro_export]
macro_rules! cast_weak_as {
    ($weak:expr, $new_type:ty) => {{
        let inner: *const std::cell::UnsafeCell<$new_type> = $weak.into_inner();
        WeakPtr {data: inner}
    }};
}

#[macro_export]
macro_rules! downgrade_as {
    ($owned:expr, $new_type:ty) => {
        crate::cast_weak_as!($owned.downgrade(), $new_type)
    };
}











// TODO maybe use a Rc<RefCell<T>> as the backing type instead of a Box<T>... and then have
// WeakPtr just store a *RefCell<T>. This still lets us borrow things, but also enforces the borrow
// checking rules at runtime so we can avoid Undefined Behavior... Not sure how important it'll be.
// But the downside here is that the API will have to return and deal with `Ref` instead of `&`.
/*

pub trait Ptr<T: ?Sized> {
    fn borrow(&self) -> &T;
    unsafe fn borrow_mut(&mut self) -> &mut T;
}

impl<T> OwnedPtr<T> {
    pub fn new(value: T) -> OwnedPtr<T> {
        OwnedPtr {
            data: Box::new(value),
        }
    }

    pub fn downgrade(&self) -> WeakPtr<T> {
        WeakPtr {
            data: (&*self.data as *const T) as *mut T,
        }
    }

    // TODO THIS IS REALLY REALLY BAD.
    // WE NEED TO BE VERY CAREFUL WITH USING THIS UNTIL COERCEUNSIZED IS STABILIZED!
    // IT BLINDLY CASTS A `*T` TO A `*U`
   //pub fn downgrade_as<U>(&self) -> WeakPtr<U> {
   //    WeakPtr {
   //        data: (&*self.data as *const T) as *mut U,
   //    }
   //}
}

// TODO this is stupid and only required because CoerceUniszed is unstable! Sucks.
macro_rules! downgrade_as {
    ($owned:expr, $new_type:ty) => {
        WeakPtr {
            data: (&*owned.data as *const $new_type) as *mut $new_type,
        }
    };
}

pub(crate) use downgrade_as;


impl<T> Ptr<T> for OwnedPtr<T> {
    fn borrow(&self) -> &T {
        &self.data
    }

    unsafe fn borrow_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

// This can be derefenced when dangling. It will always be aligned, if the pointeee exists.
// But we just raw dereference this thing. It could totally break if the OwnedPtr isn't around.
// This is unlikely because the AST is effectively static, but you never know.


// TODO: add this CoerceUnsized is stabalized
// Also, maybe add it to OwnedPtr if we make that Unsized as well...
//impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<WeakPtr<U>> for WeakPtr<T> {}

impl<T: ?Sized> Ptr<T> for WeakPtr<T> {
    fn borrow(&self) -> &T {
        unsafe {
            self.data.as_ref().unwrap()
        }
    }

    unsafe fn borrow_mut(&mut self) -> &mut T {
        self.data.as_mut().unwrap()
    }
}

*/