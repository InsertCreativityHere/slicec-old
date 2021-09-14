// Copyright (c) ZeroC, Inc. All rights reserved.

use std::cell::{Ref, RefCell, RefMut};

pub trait Ptr<T: ?Sized> {
    fn borrow(&self) -> Ref<T>;
    fn borrow_mut(&self) -> RefMut<T>;
}

#[derive(Debug)]
pub struct OwnedPtr<T> {
    data: Box<RefCell<T>>,
}

// TODO: explicitely de-implement these traits when negative traits bounds are stable.
// Prevent OwnedPtr from being sent, or shared between threads, since it isn't thread safe.
//impl<T> !Send for OwnedPtr<T> {}
//impl<T> !Sync for OwnedPtr<T> {}

impl<T> OwnedPtr<T> {
    pub fn new(value: T) -> OwnedPtr<T> {
        OwnedPtr {
            data: Box::new(RefCell::new(value)),
        }
    }

    pub fn downgrade(&self) -> WeakPtr<T> {
        WeakPtr {
            data: &*self.data
        }
    }
}

impl<T> Ptr<T> for OwnedPtr<T> {
    fn borrow(&self) -> Ref<T> {
        self.data.borrow()
    }

    fn borrow_mut(&self) -> RefMut<T> {
        self.data.borrow_mut()
    }
}

#[derive(Clone, Debug)]
pub struct WeakPtr<T: ?Sized> {
    data: *const RefCell<T>
}

// TODO: explicitely de-implement these traits when negative traits bounds are stable.
// Prevent WeakPtr from being sent, or shared between threads, since it isn't thread safe.
//impl<T> !Send for WeakPtr<T> {}
//impl<T> !Sync for WeakPtr<T> {}

impl<T: ?Sized> Ptr<T> for WeakPtr<T> {
    fn borrow(&self) -> Ref<T> {
        (*self.data).borrow()
    }

    fn borrow_mut(&self) -> RefMut<T> {
        (*self.data).borrow_mut()
    }
}
