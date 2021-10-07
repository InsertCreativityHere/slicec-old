// Copyright (c) ZeroC, Inc. All rights reserved.

use super::slice::*;
use super::traits::*;
use crate::util::OwnedPtr;

macro_rules! generate_definition_wrapper {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        pub enum Definition {
            $($variant(OwnedPtr<$variant>),)*
        }

        impl Definition {
            pub fn borrow(&self) -> &dyn Entity {
                match self {
                    $(Self::$variant(x) => x.borrow(),)*
                }
            }

            pub unsafe fn borrow_mut(&mut self) -> &mut dyn Entity {
                match self {
                    $(Self::$variant(x) => x.borrow_mut(),)*
                }
            }
        }
    };
}

macro_rules! generate_elements_wrapper {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        pub enum Elements<'a> {
            $($variant(&'a $variant),)*
        }

        impl<'a> Element for Elements<'a> {
            fn kind(&self) -> &'static str {
                match self {
                    $(Self::$variant(x) => x.kind(),)*
                }
            }
        }

        impl<'a> AsElements for Elements<'a> {
            fn concrete_element(&self) -> Elements {
                panic!("Cannot re-wrap elements wrapper") //TODO write a better message here
            }

            fn concrete_element_mut(&mut self) -> ElementsMut {
                panic!("Cannot re-wrap elements wrapper") //TODO write a better message here
            }
        }

        #[derive(Debug)]
        pub enum ElementsMut<'a> {
            $($variant(&'a mut $variant),)*
        }

        impl<'a> Element for ElementsMut<'a> {
            fn kind(&self) -> &'static str {
                match self {
                    $(Self::$variant(x) => x.kind(),)*
                }
            }
        }

        impl<'a> AsElements for ElementsMut<'a> {
            fn concrete_element(&self) -> Elements {
                panic!("Cannot re-wrap elements wrapper") //TODO write a better message here
            }

            fn concrete_element_mut(&mut self) -> ElementsMut {
                panic!("Cannot re-wrap elements wrapper") //TODO write a better message here
            }
        }
    };
}

macro_rules! generate_types_wrapper {
    ($($variant:ident),*) => {
        #[derive(Debug)]
        pub enum Types<'a> {
            $($variant(&'a $variant),)*
        }

        impl<'a> Element for Types<'a> {
            fn kind(&self) -> &'static str {
                match self {
                    $(Self::$variant(x) => x.kind(),)*
                }
            }
        }

        impl<'a> Type for Types<'a> {
            fn is_fixed_size(&self) -> bool {
                match self {
                    $(Self::$variant(x) => x.is_fixed_size(),)*
                }
            }

            fn min_wire_size(&self) -> u32 {
                match self {
                    $(Self::$variant(x) => x.min_wire_size(),)*
                }
            }
        }

        impl<'a> AsElements for Types<'a> {
            fn concrete_element(&self) -> Elements {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }

            fn concrete_element_mut(&mut self) -> ElementsMut {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }
        }

        impl<'a> AsTypes for Types<'a> {
            fn concrete_type(&self) -> Types {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }

            fn concrete_type_mut(&mut self) -> TypesMut {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }
        }

        #[derive(Debug)]
        pub enum TypesMut<'a> {
            $($variant(&'a mut $variant),)*
        }

        impl<'a> Element for TypesMut<'a> {
            fn kind(&self) -> &'static str {
                match self {
                    $(Self::$variant(x) => x.kind(),)*
                }
            }
        }

        impl<'a> Type for TypesMut<'a> {
            fn is_fixed_size(&self) -> bool {
                match self {
                    $(Self::$variant(x) => x.is_fixed_size(),)*
                }
            }

            fn min_wire_size(&self) -> u32 {
                match self {
                    $(Self::$variant(x) => x.min_wire_size(),)*
                }
            }
        }

        impl<'a> AsElements for TypesMut<'a> {
            fn concrete_element(&self) -> Elements {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }

            fn concrete_element_mut(&mut self) -> ElementsMut {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }
        }

        impl<'a> AsTypes for TypesMut<'a> {
            fn concrete_type(&self) -> Types {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }

            fn concrete_type_mut(&mut self) -> TypesMut {
                panic!("Cannot re-wrap types wrapper") //TODO write a better message here
            }
        }
    };
}

macro_rules! implement_as_elements {
    ($type:ident) => {
        impl AsElements for $type {
            fn concrete_element(&self) -> Elements {
                Elements::$type(self)
            }

            fn concrete_element_mut(&mut self) -> ElementsMut {
                ElementsMut::$type(self)
            }
        }
    };
}

macro_rules! implement_as_types {
    ($type:ident) => {
        implement_as_elements!($type);

        impl AsTypes for $type {
            fn concrete_type(&self) -> Types {
                Types::$type(self)
            }

            fn concrete_type_mut(&mut self) -> TypesMut {
                TypesMut::$type(self)
            }
        }
    };
}

pub trait AsElements {
    fn concrete_element(&self) -> Elements;
    fn concrete_element_mut(&mut self) -> ElementsMut;
}

pub trait AsTypes {
    fn concrete_type(&self) -> Types;
    fn concrete_type_mut(&mut self) -> TypesMut;
}

generate_definition_wrapper!(
    Module, Struct, Class, Exception, Interface, Enum, TypeAlias
);

generate_elements_wrapper!(
    Module, Struct, Class, Exception, DataMember, Interface, Operation, Parameter, Enum, Enumerator,
    TypeAlias, TypeRef, Sequence, Dictionary, Primitive, Identifier, Attribute
);

generate_types_wrapper!(
    Struct, Class, Interface, Enum, TypeAlias, Sequence, Dictionary, Primitive
);

implement_as_elements!(Module);
implement_as_types!(Struct);
implement_as_types!(Class);
implement_as_elements!(Exception);
implement_as_elements!(DataMember);
implement_as_types!(Interface);
implement_as_elements!(Operation);
implement_as_elements!(Parameter);
implement_as_types!(Enum);
implement_as_elements!(Enumerator);
implement_as_types!(TypeAlias);
implement_as_types!(Sequence);
implement_as_types!(Dictionary);
implement_as_types!(Primitive);
implement_as_elements!(Identifier);
implement_as_elements!(Attribute);

// Since `TypeRef` has a generic type parameter, we implement the as_wrapper methods by hand.
impl<T: Element + ?Sized> AsElements for TypeRef<T> {
    fn concrete_element(&self) -> Elements {
        self.definition().concrete_element()
    }

    fn concrete_element_mut(&mut self) -> ElementsMut {
        panic!() // TODO write a message here!
    }
}

impl<T: Type + ?Sized> AsTypes for TypeRef<T> {
    fn concrete_type(&self) -> Types {
        self.definition().concrete_type()
    }

    fn concrete_type_mut(&mut self) -> TypesMut {
        panic!() // TODO write a message here!
    }
}
