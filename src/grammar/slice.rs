// Copyright (c) ZeroC, Inc. All rights reserved.

use super::comments::DocComment;
use super::traits::*;
use super::util::Scope;
use super::wrappers::*;
use crate::slice_file::Location;
use crate::util::*;

#[derive(Debug)]
pub struct Module {
    pub identifier: Identifier,
    pub contents: Vec<Definition>,
    pub parent: Option<WeakPtr<Module>>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

implement_Element_for!(Module, "module");
implement_Entity_for!(Module);
implement_Container_for!(Module, Definition, contents);

#[derive(Debug)]
pub struct Struct {
    pub identifier: Identifier,
    pub members: Vec<OwnedPtr<DataMember>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

impl Type for Struct {
    fn get_concrete_type(&self) -> Types {
        Types::Struct(self)
    }

    fn is_fixed_size(&self) -> bool {
        // A struct is fixed size if and only if all it's members are fixed size.
        self.members.iter().all(
            |member| member.borrow().data_type.definition().is_fixed_size()
        )
    }

    fn min_wire_size(&self) -> u32 {
        // The min-wire-size of a struct is the min-wire-size of all its members added together.
        self.members.iter().map(
            |member| member.borrow().data_type.definition().min_wire_size()
        ).sum()
    }
}

implement_Element_for!(Struct, "struct");
implement_Entity_for!(Struct);
implement_Container_for!(Struct, OwnedPtr<DataMember>, members);
implement_Contained_for!(Struct, Module);

#[derive(Debug)]
pub struct Class {
    pub identifier: Identifier,
    pub members: Vec<OwnedPtr<DataMember>>,
    pub base: Option<TypeRef<Class>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

impl Type for Class {
    fn get_concrete_type(&self) -> Types {
        Types::Class(self)
    }

    fn is_fixed_size(&self) -> bool {
        // A class is fixed size if and only if all it's members are fixed size.
        self.members.iter().all(
            |member| member.borrow().data_type.definition().is_fixed_size()
        )
    }

    fn min_wire_size(&self) -> u32 {
        // The min-wire-size of a class is the min-wire-size of all its members added together.
        self.members.iter().map(
            |member| member.borrow().data_type.definition().min_wire_size()
        ).sum()
    }
}

implement_Element_for!(Class, "class");
implement_Entity_for!(Class);
implement_Container_for!(Class, OwnedPtr<DataMember>, members);
implement_Contained_for!(Class, Module);

#[derive(Debug)]
pub struct Exception {
    pub identifier: Identifier,
    pub members: Vec<OwnedPtr<DataMember>>,
    pub base: Option<TypeRef<Exception>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

implement_Element_for!(Exception, "exception");
implement_Entity_for!(Exception);
implement_Container_for!(Exception, OwnedPtr<DataMember>, members);
implement_Contained_for!(Exception, Module);

#[derive(Debug)]
pub struct DataMember {
    pub identifier: Identifier,
    pub data_type: TypeRef,
    pub tag: Option<u32>,
    pub parent: WeakPtr<dyn Container<OwnedPtr<DataMember>>>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

implement_Element_for!(DataMember, "data member");
implement_Entity_for!(DataMember);
implement_Contained_for!(DataMember, dyn Container<OwnedPtr<DataMember>> + 'static);

#[derive(Debug)]
pub struct Interface {
    pub identifier: Identifier,
    pub operations: Vec<OwnedPtr<Operation>>,
    pub bases: Vec<TypeRef<Interface>>,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

impl Type for Interface {
    fn get_concrete_type(&self) -> Types {
        Types::Interface(self)
    }

    fn is_fixed_size(&self) -> bool {
        false
    }

    fn min_wire_size(&self) -> u32 {
        // TODO write a comment explaining why this is 3.
        3
    }
}

implement_Element_for!(Interface, "interface");
implement_Entity_for!(Interface);
implement_Container_for!(Interface, OwnedPtr<Operation>, operations);
implement_Contained_for!(Interface, Module);

#[derive(Debug)]
pub struct Operation {
    pub identifier: Identifier,
    pub return_type: Vec<OwnedPtr<Parameter>>,
    pub parameters: Vec<OwnedPtr<Parameter>>,
    pub parent: WeakPtr<Interface>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

impl Operation {
    pub fn has_unstreamed_parameters(&self) -> bool {
        // Operations can have at most 1 streamed parameter. So, if it has more than 1 parameter
        // there must be unstreamed parameters. Otherwise we check if the 1 parameter is streamed.
        match self.parameters.len() {
            0 => false,
            1 => !self.parameters[0].borrow().data_type.is_streamed,
            _ => true,
        }
    }

    pub fn has_unstreamed_return_members(&self) -> bool {
        // Operations can have at most 1 streamed return member. So, if it has more than 1 member
        // there must be unstreamed members. Otherwise we check if the 1 member is streamed.
        match self.return_type.len() {
            0 => false,
            1 => !self.return_type[0].borrow().data_type.is_streamed,
            _ => true,
        }
    }

    pub fn get_unstreamed_parameters(&self) -> &[OwnedPtr<Parameter>] {
        let length = self.parameters.len();
        // Operations can have at most 1 streamed parameter, and it must be the last parameter.
        if length > 0 && self.parameters[length - 1].borrow().data_type.is_streamed {
            // Return a slice of the parameter vector with the last parameter (which is streamed)
            // removed from it. It is safe to unwrap here, because we know that `length > 0`.
            self.parameters.split_last().unwrap().1
        } else {
            &self.parameters
        }
    }
}

implement_Element_for!(Operation, "operation");
implement_Entity_for!(Operation);
implement_Contained_for!(Operation, Interface);

#[derive(Debug)]
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

impl Element for Parameter {
    fn kind(&self) -> &'static str {
        if self.is_returned {
            "return element"
        } else {
            "parameter"
        }
    }
}

implement_Entity_for!(Parameter);
implement_Contained_for!(Parameter, Operation);

#[derive(Debug)]
pub struct Enum {
    pub identifier: Identifier,
    pub enumerators: Vec<OwnedPtr<Enumerator>>,
    pub underlying: Option<TypeRef<Primitive>>,
    pub is_unchecked: bool,
    pub parent: WeakPtr<Module>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

impl Enum {
    pub fn underlying_type<'a>(&'a self) -> &'a Primitive {
        // If the enum has an underlying type, return a reference to it's definition.
        // Otherwise, enums have a backing type of `byte` by default. Since `byte` is a type
        // defined by the compiler, we fetch it's definition directly from the global AST.
        self.underlying.as_ref().map_or(
            crate::borrow_ast().lookup_primitive("byte").unwrap().borrow(),
            |data_type| data_type.definition(),
        )
    }

    pub fn get_min_max_values(&self) -> Option<(i64, i64)> {
        let values = self.enumerators.iter().map(
            |enumerator| enumerator.borrow().value
        );
        let min = values.clone().min();
        let max = values.max();

        // There might not be a minimum value if the enum is empty.
        if min.is_some() {
            // A `min` existing guarantees a `max` does too, so it's safe to unwrap here.
            Some((min.unwrap(), max.unwrap()))
        } else {
            None
        }
    }
}

impl Type for Enum {
    fn get_concrete_type(&self) -> Types {
        Types::Enum(self)
    }

    fn is_fixed_size(&self) -> bool {
        self.underlying_type().is_fixed_size()
    }

    fn min_wire_size(&self) -> u32 {
        self.underlying_type().min_wire_size()
    }
}

implement_Element_for!(Enum, "enum");
implement_Entity_for!(Enum);
implement_Container_for!(Enum, OwnedPtr<Enumerator>, enumerators);
implement_Contained_for!(Enum, Module);

#[derive(Debug)]
pub struct Enumerator {
    pub identifier: Identifier,
    pub value: i64,
    pub parent: WeakPtr<Enum>,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

implement_Element_for!(Enumerator, "enumerator");
implement_Entity_for!(Enumerator);
implement_Contained_for!(Enumerator, Enum);

#[derive(Debug)]
pub struct TypeAlias {
    pub identifier: Identifier,
    pub underlying: TypeRef,
    pub parent: WeakPtr<Module>,
    pub comment: Option<DocComment>,
    pub location: Location,
}

// The `implement_trait_for` macros expect structs to store all it's data as fields on itself,
// but TypeAliases store data in their underlying `TypeRef`, so we need to manually implement
// some traits to forward to the underlying `TypeRef`, instead of being able to use macros.

impl ScopedSymbol for TypeAlias {
    fn module_scope(&self) -> &String {
        self.underlying.module_scope()
    }

    fn parser_scope(&self) -> &String {
        self.underlying.parser_scope()
    }

    fn raw_scope(&self) -> &Scope {
        self.underlying.raw_scope()
    }
}

impl Attributable for TypeAlias {
    fn attributes(&self) -> &Vec<Attribute> {
        self.underlying.attributes()
    }

    fn has_attribute(&self, directive: &str) -> bool {
        self.underlying.has_attribute(directive)
    }

    fn get_attribute(&self, directive: &str) -> Option<&Vec<String>> {
        self.underlying.get_attribute(directive)
    }

    fn get_raw_attribute(&self, directive: &str) -> Option<&Attribute> {
        self.underlying.get_raw_attribute(directive)
    }
}

impl Entity for TypeAlias {}

impl Type for TypeAlias {
    fn get_concrete_type(&self) -> Types {
        Types::TypeAlias(self)
    }

    fn is_fixed_size(&self) -> bool {
        self.underlying.definition().is_fixed_size()
    }

    fn min_wire_size(&self) -> u32 {
        self.underlying.definition().min_wire_size()
    }
}

implement_Element_for!(TypeAlias, "type alias");
implement_Symbol_for!(TypeAlias);
implement_Named_Symbol_for!(TypeAlias);
implement_Commentable_for!(TypeAlias);
implement_Contained_for!(TypeAlias, Module);

#[derive(Debug)]
pub struct TypeRef<T: Element + ?Sized = dyn Type> {
    pub type_string: String,
    pub definition: WeakPtr<T>,
    pub is_optional: bool,
    pub is_streamed: bool,
    pub scope: Scope,
    pub attributes: Vec<Attribute>,
    pub location: Location,
}

impl<T: Element + ?Sized> TypeRef<T> {
    pub fn definition(&self) -> &T {
        self.definition.borrow()
    }
}

impl<T: Element + ?Sized + Type> TypeRef<T> {
    pub fn is_bit_sequence_encodable(&self) -> bool {
        self.is_optional && self.min_wire_size() == 0
    }
}

// Technically, `TypeRef` is NOT a type; It represents somewhere that a type is referenced.
// But, for convenience, we implement type on it, so that users of the API can call methods on
// the underlying type without having to first call `.definition()` all the time.
impl<T: Element + ?Sized + Type> Type for TypeRef<T> {
    fn get_concrete_type(&self) -> Types {
        self.definition().get_concrete_type()
    }

    fn is_fixed_size(&self) -> bool {
        self.definition().is_fixed_size()
    }

    fn min_wire_size(&self) -> u32 {
        let underlying = self.definition();
        if self.is_optional {
            // TODO explain why classes and interfaces still take up 1 byte.
            match underlying.kind() {
                "class" | "interface" => 1,
                _ => 0,
            }
        } else {
            underlying.min_wire_size()
        }
    }
}

implement_Element_for!(TypeRef<T>, "type reference", Element + ?Sized);
implement_Symbol_for!(TypeRef<T>, Element + ?Sized);
implement_Scoped_Symbol_for!(TypeRef<T>, Element + ?Sized);
implement_Attributable_for!(TypeRef<T>, Element + ?Sized);

#[derive(Debug)]
pub struct Sequence {
    pub element_type: TypeRef,
}

impl Sequence {
    pub fn has_fixed_size_numeric_elements(&self) -> bool {
        let mut definition = self.element_type.definition().get_concrete_type();

        // If the elements are enums with an underlying type, check the underlying type instead.
        if let Types::Enum(enum_def) = definition {
            definition = enum_def.underlying_type().get_concrete_type()
        }

        if let Types::Primitive(primitive) = definition {
            primitive.is_numeric_or_bool() && primitive.is_fixed_size()
        } else {
            false
        }
    }
}

impl Type for Sequence {
    fn get_concrete_type(&self) -> Types {
        Types::Sequence(self)
    }

    fn is_fixed_size(&self) -> bool {
        false
    }

    fn min_wire_size(&self) -> u32 {
        1
    }
}

implement_Element_for!(Sequence, "sequence");

#[derive(Debug)]
pub struct Dictionary {
    pub key_type: TypeRef,
    pub value_type: TypeRef,
}

impl Type for Dictionary {
    fn get_concrete_type(&self) -> Types {
        Types::Dictionary(self)
    }

    fn is_fixed_size(&self) -> bool {
        false
    }

    fn min_wire_size(&self) -> u32 {
        1
    }
}

implement_Element_for!(Dictionary, "dictionary");

#[derive(Debug)]
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

impl Primitive {
    fn is_numeric(&self) -> bool {
        matches!(self,
            Self::Byte | Self::Short | Self::UShort | Self::Int | Self::UInt | Self::VarInt |
            Self::VarUInt | Self::Long | Self::ULong | Self::VarLong | Self::VarULong |
            Self::Float | Self::Double
        )
    }

    fn is_numeric_or_bool(&self) -> bool {
        self.is_numeric() || matches!(self, Self::Bool)
    }
}

impl Type for Primitive {
    fn get_concrete_type(&self) -> Types {
        Types::Primitive(self)
    }

    fn is_fixed_size(&self) -> bool {
        matches!(self,
            Self::Bool | Self::Byte | Self::Short | Self::UShort | Self::Int | Self::UInt |
            Self::Long | Self::ULong | Self::Float | Self::Double
        )
    }

    fn min_wire_size(&self) -> u32 {
        match self {
            Self::Bool => 1,
            Self::Byte => 1,
            Self::Short => 2,
            Self::UShort => 2,
            Self::Int => 4,
            Self::UInt => 4,
            Self::VarInt => 1,
            Self::VarUInt => 1,
            Self::Long => 8,
            Self::ULong => 8,
            Self::VarLong => 1,
            Self::VarULong => 1,
            Self::Float => 4,
            Self::Double => 8,
            Self::String => 1,
        }
    }
}

impl Element for Primitive {
    fn kind(&self) -> &'static str {
        match self {
            Self::Bool => "bool",
            Self::Byte => "byte",
            Self::Short => "short",
            Self::UShort => "ushort",
            Self::Int => "int",
            Self::UInt => "uint",
            Self::VarInt => "varint",
            Self::VarUInt => "varuint",
            Self::Long => "long",
            Self::ULong => "ulong",
            Self::VarLong => "varlong",
            Self::VarULong => "varulong",
            Self::Float => "float",
            Self::Double => "double",
            Self::String => "string",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub value: String,
    pub location: Location,
}

implement_Element_for!(Identifier, "identifier");
implement_Symbol_for!(Identifier);

#[derive(Clone, Debug)]
pub struct Attribute {
    pub prefix: Option<String>,
    pub directive: String,
    pub prefixed_directive: String,
    pub arguments: Vec<String>,
    pub location: Location,
}

implement_Element_for!(Attribute, "attribute");
implement_Symbol_for!(Attribute);
