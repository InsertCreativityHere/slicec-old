// Copyright (c) ZeroC, Inc. All rights reserved.

use super::comments::DocComment;
use super::slice::{Attribute, Identifier};
use super::util::{Scope, TagFormat};
use super::wrappers::{AsElements, AsTypes};
use crate::slice_file::Location;

pub trait Element: std::fmt::Debug + AsElements {
    fn kind(&self) -> &'static str;
}

pub trait Symbol: Element {
    fn location(&self) -> &Location;
}

pub trait ScopedSymbol: Symbol {
    fn module_scope(&self) -> &str;
    fn parser_scope(&self) -> &str;
    fn raw_scope(&self) -> &Scope;
}

pub trait NamedSymbol: ScopedSymbol {
    fn identifier(&self) -> &str;
    fn raw_identifier(&self) -> &Identifier;

    fn module_scoped_identifier(&self) -> String {
        self.module_scope().to_owned() + "::" + self.identifier()
    }

    fn parser_scoped_identifier(&self) -> String {
        self.parser_scope().to_owned() + "::" + self.identifier()
    }
}

pub trait Attributable: Symbol {
    fn attributes(&self) -> &Vec<Attribute>;
    fn has_attribute(&self, directive: &str) -> bool;
    fn get_attribute(&self, directive: &str) -> Option<&Vec<String>>;
    fn get_raw_attribute(&self, directive: &str) -> Option<&Attribute>;
}

pub trait Commentable: Symbol {
    fn comment(&self) -> Option<&DocComment>;
}

pub trait Entity: NamedSymbol + ScopedSymbol + Attributable + Commentable {}

pub trait Container<T>: Entity {
    fn contents(&self) -> &Vec<T>;
}

pub trait Contained<T: Entity + ?Sized>: Entity {
    fn parent(&self) -> &T;
}

pub trait Type: Element + AsTypes {
    fn is_fixed_size(&self) -> bool;
    fn min_wire_size(&self) -> u32;
    fn uses_classes(&self) -> bool;
    fn tag_format(&self) -> TagFormat;
}

macro_rules! implement_Element_for {
    ($type:ty, $kind_string:literal$(, $($bounds:tt)+)?) => {
        impl$(<T: $($bounds)+>)? Element for $type {
            fn kind(&self) -> &'static str {
                $kind_string
            }
        }
    };
}

macro_rules! implement_Symbol_for {
    ($type:ty$(, $($bounds:tt)+)?) => {
        impl$(<T: $($bounds)+>)? Symbol for $type {
            fn location(&self) -> &Location {
                &self.location
            }
        }
    };
}

macro_rules! implement_Scoped_Symbol_for {
    ($type:ty$(, $($bounds:tt)+)?) => {
        impl$(<T: $($bounds)+>)? ScopedSymbol for $type {
            fn module_scope(&self) -> &str {
                &self.scope.raw_module_scope
            }

            fn parser_scope(&self) -> &str {
                &self.scope.raw_parser_scope
            }

            fn raw_scope(&self) -> &Scope {
                &self.scope
            }
        }
    };
}

macro_rules! implement_Named_Symbol_for {
    ($type:ty) => {
        impl NamedSymbol for $type {
            fn identifier(&self) -> &str {
                &self.identifier.value
            }

            fn raw_identifier(&self) -> &Identifier {
                &self.identifier
            }
        }
    };
}

macro_rules! implement_Attributable_for {
    ($type:ty$(, $($bounds:tt)+)?) => {
        impl$(<T: $($bounds)+>)? Attributable for $type {
            fn attributes(&self) -> &Vec<Attribute> {
                &self.attributes
            }

            fn has_attribute(&self, directive: &str) -> bool {
                self.get_raw_attribute(directive).is_some()
            }

            fn get_attribute(&self, directive: &str) -> Option<&Vec<String>> {
                self.get_raw_attribute(directive).and_then(
                    |attribute| Some(&attribute.arguments)
                )
            }

            fn get_raw_attribute(&self, directive: &str) -> Option<&Attribute> {
                for attribute in &self.attributes {
                    if attribute.prefixed_directive == directive {
                        return Some(attribute);
                    }
                }
                None
            }
        }
    };
}

macro_rules! implement_Commentable_for {
    ($type:ty) => {
        impl Commentable for $type {
            fn comment(&self) -> Option<&DocComment> {
                self.comment.as_ref()
            }
        }
    };
}

macro_rules! implement_Entity_for {
    ($type:ty) => {
        implement_Symbol_for!($type);
        implement_Named_Symbol_for!($type);
        implement_Scoped_Symbol_for!($type);
        implement_Attributable_for!($type);
        implement_Commentable_for!($type);

        impl Entity for $type {}
    };
}

macro_rules! implement_Container_for {
    ($type:ty, $contained_type:ty, $field_name:ident) => {
        impl Container<$contained_type> for $type {
            fn contents(&self) -> &Vec<$contained_type> {
                &self.$field_name
            }
        }
    };
}

macro_rules! implement_Contained_for {
    ($type:ty, $container_type:ty) => {
        impl Contained<$container_type> for $type {
            fn parent(&self) -> &$container_type {
                self.parent.borrow()
            }
        }
    };
}

pub(crate) use implement_Element_for;
pub(crate) use implement_Symbol_for;
pub(crate) use implement_Named_Symbol_for;
pub(crate) use implement_Scoped_Symbol_for;
pub(crate) use implement_Attributable_for;
pub(crate) use implement_Commentable_for;
pub(crate) use implement_Entity_for;
pub(crate) use implement_Container_for;
pub(crate) use implement_Contained_for;
