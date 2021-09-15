// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::util::Location;

use super::comments::*;
use super::slice::*;
use super::traits::*;
use super::util::*;

#[derive(Debug)]
pub enum Definition {
    Struct(Struct),
    Class(Class),
    Exception(Exception),
    Interface(Interface),
    Enum(Enum),
    TypeAlias(TypeAlias),
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

macro_rules! forward_trait_for_Definition {
    ($type:ty) => {
        forward_trait_for_Definition!($type, );
    };
    ($type:ty, $(($function:ident, $return:ty$(, $param:ident;$param_type:ty)?)),*) => {
        impl $type for Definition {
            $(fn $function(&self$(, $param: $param_type)?) -> $return {
                match self {
                    Self::Struct(x) => x.$function($($param)?),
                    Self::Class(x) => x.$function($($param)?),
                    Self::Exception(x) => x.$function($($param)?),
                    Self::Interface(x) => x.$function($($param)?),
                    Self::Enum(x) => x.$function($($param)?),
                    Self::TypeAlias(x) => x.$function($($param)?),
                }
            })*
        }
    };
}

forward_trait_for_Definition!(Element,
    (kind, &'static str)
);

forward_trait_for_Definition!(Symbol,
    (location, &Location)
);

forward_trait_for_Definition!(NamedSymbol,
    (identifier, &String),
    (raw_identifier, &Identifier)
);

forward_trait_for_Definition!(ScopedSymbol,
    (scope, &String),
    (parser_scope, &String),
    (raw_scope, &Scope)
);

forward_trait_for_Definition!(Commentable,
    (comment, Option<&DocComment>)
);

forward_trait_for_Definition!(Attributable,
    (attributes, &Vec<Attribute>),
    (has_attribute, bool, directive;&str),
    (get_attribute, Option<&Vec<String>>, directive;&str),
    (get_raw_attribute, Option<&Attribute>, directive;&str)
);

forward_trait_for_Definition!(Entity);

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
