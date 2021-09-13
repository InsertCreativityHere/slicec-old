// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::util::{SharedPtr, WeakPtr};

use super::comments::DocComment;
use super::traits::*;
use super::util::*;

pub struct Module {
    pub identifier: Identifier,
    pub contents: Vec<SharedPtr<Definition>>,
    pub parent: Option<WeakPtr<Module>>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub enum Definition {
    Struct(Struct),
    Class(Class),
    Exception(Exception),
    Interface(Interface),
    Enum(Enum),
    TypeAlias(TypeAlias),
}

pub struct Struct {
    pub identifier: Identifier,
    pub members: Vec<SharedPtr<DataMember>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Class {
    pub identifier: Identifier,
    pub members: Vec<SharedPtr<DataMember>>,
    pub base: Option<TypeRef<Class>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Exception {
    pub identifier: Identifier,
    pub members: Vec<SharedPtr<DataMember>>,
    pub base: Option<TypeRef<Exception>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct DataMember {
    pub identifier: Identifier,
    pub data_type: TypeRef,
    pub tag: Option<u32>,
    pub parent: WeakPtr<dyn Container<DataMember>>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Interface {
    pub identifier: Identifier,
    pub operations: Vec<SharedPtr<Operation>>,
    pub bases: Vec<TypeRef<Interface>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Operation {
    pub identifier: Identifier,
    pub return_type: Vec<SharedPtr<Parameter>>,
    pub parameters: Vec<SharedPtr<Parameter>>,
    pub parent: WeakPtr<Interface>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Parameter {
    pub identifier: Identifier,
    pub data_type: TypeRef,
    pub tag: Option<u32>,
    pub is_streamed: bool,
    pub is_returned: bool,
    pub parent: WeakPtr<Operation>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Enum {
    pub identifier: Identifier,
    pub enumerators: Vec<SharedPtr<Enumerator>>,
    pub underlying: Option<TypeRef>,
    pub is_unchecked: bool,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct Enumerator {
    pub identifier: Identifier,
    pub value: i64,
    pub parent: WeakPtr<Enum>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

pub struct TypeAlias {
    pub identifier: Identifier,
    pub underlying: TypeRef,
    pub comment: Option<DocComment>,
}

pub struct TypeRef<T: Type + ?Sized = dyn Type> {
    pub type_string: String,
    pub definition: WeakPtr<T>,
    pub is_optional: bool,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub location: Location,
}

pub struct Identifier {
    pub value: String,
    pub location: Location,
}

pub struct Sequence {
    pub element_type: TypeRef,
}

pub struct Dictionary {
    pub key_type: TypeRef,
    pub value_type: TypeRef,
}

pub enum Primitive {
    Bool,
    Byte,
    Short,
    UShort,
    Int,
    UInt,
    VarInt,
    VarUInt,
    Long,
    ULong,
    VarLong,
    VarULong,
    Float,
    Double,
    String,
}

pub struct Attribute {
    pub prefix: Option<String>,
    pub directive: String,
    pub arguments: Vec<String>,
    pub raw_directive: String,
    pub location: Location,
}
