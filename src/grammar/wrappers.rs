// Copyright (c) ZeroC, Inc. All rights reserved.

use super::slice::*;
use super::traits::*;
use crate::util::{OwnedPtr, Ptr};

#[derive(Debug)]
pub enum Definition {
    Module(OwnedPtr<Module>),
    Struct(OwnedPtr<Struct>),
    Class(OwnedPtr<Class>),
    Exception(OwnedPtr<Exception>),
    Interface(OwnedPtr<Interface>),
    Enum(OwnedPtr<Enum>),
    TypeAlias(OwnedPtr<TypeAlias>),
}

// We implement wrapper versions of `borrow` and `borrow_mut` on `Definition` for convenience.
impl Definition {
    pub fn borrow(&self) -> &dyn Entity {
        match self {
            Self::Module(module_ptr) => module_ptr.borrow(),
            Self::Struct(struct_ptr) => struct_ptr.borrow(),
            Self::Class(class_ptr) => class_ptr.borrow(),
            Self::Exception(exception_ptr) => exception_ptr.borrow(),
            Self::Interface(interface_ptr) => interface_ptr.borrow(),
            Self::Enum(enum_ptr) => enum_ptr.borrow(),
            Self::TypeAlias(type_alias_ptr) => type_alias_ptr.borrow(),
        }
    }

    pub unsafe fn borrow_mut(&mut self) -> &mut dyn Entity {
        match self {
            Self::Module(module_ptr) => module_ptr.borrow_mut(),
            Self::Struct(struct_ptr) => struct_ptr.borrow_mut(),
            Self::Class(class_ptr) => class_ptr.borrow_mut(),
            Self::Exception(exception_ptr) => exception_ptr.borrow_mut(),
            Self::Interface(interface_ptr) => interface_ptr.borrow_mut(),
            Self::Enum(enum_ptr) => enum_ptr.borrow_mut(),
            Self::TypeAlias(type_alias_ptr) => type_alias_ptr.borrow_mut(),
        }
    }
}

#[derive(Debug)]
pub enum Types<'a> {
    Struct(&'a Struct),
    Class(&'a Class),
    Interface(&'a Interface),
    Enum(&'a Enum),
    TypeAlias(&'a TypeAlias),
    Sequence(&'a Sequence),
    Dictionary(&'a Dictionary),
    Primitive(&'a Primitive),
}

macro_rules! forward_trait_for_Types {
    ($type:ty, $(($function:ident, $return:ty$(, $param:ident;$param_type:ty)?)),*) => {
        impl<'a> $type for Types<'a> {
            $(fn $function(&self$(, $param: $param_type)?) -> $return {
                match self {
                    Self::Struct(x) => x.$function($($param)?),
                    Self::Class(x) => x.$function($($param)?),
                    Self::Interface(x) => x.$function($($param)?),
                    Self::Enum(x) => x.$function($($param)?),
                    Self::TypeAlias(x) => x.$function($($param)?),
                    Self::Sequence(x) => x.$function($($param)?),
                    Self::Dictionary(x) => x.$function($($param)?),
                    Self::Primitive(x) => x.$function($($param)?),
                }
            })*
        }
    };
}

forward_trait_for_Types!(Element,
    (kind, &'static str)
);

forward_trait_for_Types!(Type,
    (get_concrete_type, Types),
    (is_fixed_size, bool),
    (min_wire_size, u32)
);

macro_rules! implement_from_type_to_types {
    ($type:ty, $variant:path) => {
        impl<'a> From<&'a $type> for Types<'a> {
            fn from(def: &'a $type) -> Types<'a> {
                $variant(def)
            }
        }
    };
}

implement_from_type_to_types!(Struct, Types::Struct);
implement_from_type_to_types!(Class, Types::Class);
implement_from_type_to_types!(Interface, Types::Interface);
implement_from_type_to_types!(Enum, Types::Enum);
implement_from_type_to_types!(TypeAlias, Types::TypeAlias);
implement_from_type_to_types!(Sequence, Types::Sequence);
implement_from_type_to_types!(Dictionary, Types::Dictionary);
implement_from_type_to_types!(Primitive, Types::Primitive);
