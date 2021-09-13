// Copyright (c) ZeroC, Inc. All rights reserved.

use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::cell::{Ref, RefMut, RefCell};

pub struct SharedPtr<T: ?Sized>(Rc<RefCell<T>>);

impl<T> SharedPtr<T> {
    //pub fn new(value: T) -> Self {
    //    Rc::new(RefCell::new(value))
    //}
//
    //pub fn downgrade(&self) -> WeakPtr<T> {
    //    Rc::downgrade(self.0)
    //}

    pub fn borrow(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }
}

pub struct WeakPtr<T: ?Sized>(Weak<RefCell<T>>);

impl <T> WeakPtr<T> {
    //pub fn borrow(&self) -> Ref<'_, T> {
    //    self.0.upgrade().unwrap().borrow()
    //}
//
    //pub fn borrow_mut(&self) -> RefMut<'_, T> {
    //    self.0.upgrade().unwrap().borrow_mut()
    //}
}
