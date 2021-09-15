// Copyright (c) ZeroC, Inc. All rights reserved.

#[derive(Clone, Debug)]
pub struct Location {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub file: String,
}

pub trait Ptr<T: ?Sized> {
    fn borrow(&self) -> &T;
    unsafe fn borrow_mut(&mut self) -> &mut T;
}

#[derive(Debug)]
pub struct OwnedPtr<T> {
    data: Box<T>,
}

// TODO: explicitely de-implement these traits when negative traits bounds are stable.
// Prevent OwnedPtr from being sent, or shared between threads, since it isn't thread safe.
//impl<T> !Send for OwnedPtr<T> {}
//impl<T> !Sync for OwnedPtr<T> {}

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
}

impl<T> Ptr<T> for OwnedPtr<T> {
    fn borrow(&self) -> &T {
        &self.data
    }

    unsafe fn borrow_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

#[derive(Clone, Debug)]
pub struct WeakPtr<T: ?Sized> {
    data: *mut T,
}

// TODO: explicitely de-implement these traits when negative traits bounds are stable.
// Prevent WeakPtr from being sent, or shared between threads, since it isn't thread safe.
//impl<T> !Send for WeakPtr<T> {}
//impl<T> !Sync for WeakPtr<T> {}

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
