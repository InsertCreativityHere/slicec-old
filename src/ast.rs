// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::downgrade_as;
use crate::upcast_owned_as;

use crate::grammar::*;
use crate::ptr_visitor::PtrVisitor;
use crate::util::{OwnedPtr, WeakPtr};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Ast {
    pub(crate) ast: Vec<OwnedPtr<Module>>,
    pub(crate) anonymous_types: Vec<OwnedPtr<dyn Type>>,
    pub(crate) primitive_cache: HashMap<&'static str, OwnedPtr<Primitive>>,
    pub(crate) type_lookup_table: HashMap<String, WeakPtr<dyn Type>>,
    pub(crate) entity_lookup_table: HashMap<String, WeakPtr<dyn Entity>>,
}

impl Ast {
    pub fn new() -> Ast {
        let mut new_ast = Ast {
            ast: Vec::new(),
            anonymous_types: Vec::new(),
            primitive_cache: HashMap::new(),
            type_lookup_table: HashMap::new(),
            entity_lookup_table: HashMap::new(),
        };

        // Create an instance of each primitive and add them directly into the AST.
        // Primitive types are built in to the compiler. Since they aren't defined in Slice,
        // we 'define' them here when the AST is first created.
        new_ast.add_cached_primitive("bool", Primitive::Bool);
        new_ast.add_cached_primitive("byte", Primitive::Byte);
        new_ast.add_cached_primitive("short", Primitive::Short);
        new_ast.add_cached_primitive("ushort", Primitive::UShort);
        new_ast.add_cached_primitive("int", Primitive::Int);
        new_ast.add_cached_primitive("uint", Primitive::UInt);
        new_ast.add_cached_primitive("varint", Primitive::VarInt);
        new_ast.add_cached_primitive("varuint", Primitive::VarUInt);
        new_ast.add_cached_primitive("long", Primitive::Long);
        new_ast.add_cached_primitive("ulong", Primitive::ULong);
        new_ast.add_cached_primitive("varlong", Primitive::VarLong);
        new_ast.add_cached_primitive("varulong", Primitive::VarULong);
        new_ast.add_cached_primitive("float", Primitive::Float);
        new_ast.add_cached_primitive("double", Primitive::Double);
        new_ast.add_cached_primitive("string", Primitive::String);
        new_ast
    }

    pub fn add_module(&mut self, module_def: Module) {
        // Move the module onto the heap so it can be referenced via pointer.
        let mut module_ptr = OwnedPtr::new(module_def);
        // Add the module into the AST's entity lookup table.
        let weak_ptr = downgrade_as!(module_ptr, dyn Entity);
        self.entity_lookup_table.insert(module_ptr.borrow().parser_scoped_identifier(), weak_ptr);

        // Recursively visit it's contents and add them into the lookup table too.
        let mut visitor = LookupTableBuilder {
            type_lookup_table: &mut self.type_lookup_table,
            entity_lookup_table: &mut self.entity_lookup_table,
        };
        // This is always safe; no other references to the module can exist because we own it,
        // and haven't dereferenced any of the pointers to it we constructed.
        unsafe { module_ptr.visit_ptr_with(&mut visitor); }

        // Store the module in the AST.
        self.ast.push(module_ptr);
    }

    pub fn add_anonymous_type(&mut self, ty: impl Type + 'static) -> &OwnedPtr<dyn Type> {
        let type_ptr = upcast_owned_as!(OwnedPtr::new(ty), dyn Type);
        self.anonymous_types.push(type_ptr);
        self.anonymous_types.last().unwrap()
    }

    fn add_cached_primitive(&mut self, identifier: &'static str, primitive: Primitive) {
        // Move the primitive onto the heap, so it can referenced via pointer.
        let primitive_ptr = OwnedPtr::new(primitive);

        // Insert an entry in the lookup table for the type, and cache the primitive's instance.
        let weak_ptr = downgrade_as!(primitive_ptr, dyn Type);
        self.type_lookup_table.insert(identifier.to_owned(), weak_ptr);
        self.primitive_cache.insert(identifier, primitive_ptr);
    }

    pub fn lookup_primitive(&self, name: &str) -> Option<&OwnedPtr<Primitive>> {
        self.primitive_cache.get(name)
    }

    pub fn lookup_type(&self, name: &str, scope: &Scope) -> Option<&WeakPtr<dyn Type>> {
        // Paths starting with '::' are absolute paths, which can be directly looked up.
        if let Some(unprefixed) = name.strip_prefix("::") {
            return self.type_lookup_table.get(unprefixed);
        }

        // Types are looked up by module scope, since types can only be defined inside modules.
        let mut parents: &[String] = &scope.module_scope;

        // For relative paths, we check each enclosing scope, starting from the bottom
        // (most specified scope), and working our way up to global scope.
        while !parents.is_empty() {
            let candidate = parents.join("::") + "::" + name;
            if let Some(result) = self.type_lookup_table.get(&candidate) {
                return Some(result);
            }
            // Remove the last parent's scope before trying again.
            // It's safe to unwrap here, since we know that `parents` is not empty.
            parents = parents.split_last().unwrap().1;
        }

        // We couldn't find the type in any enclosing scope.
        None
    }

    pub fn lookup_entity(&self, name: &str, scope: &Scope) -> Option<&WeakPtr<dyn Entity>> {
        // Paths starting with '::' are absolute paths, which can be directly looked up.
        if let Some(unprefixed) = name.strip_prefix("::") {
            return self.entity_lookup_table.get(unprefixed);
        }

        // Entites are looked up by parser scope, since entities can be defined anywhere, not
        // just inside modules. Ex: A parameter in an operation.
        let mut parents: &[String] = &scope.parser_scope;

        // For relative paths, we check each enclosing scope, starting from the bottom
        // (most specified scope), and working our way up to global scope.
        while !parents.is_empty() {
            let candidate = parents.join("::") + "::" + name;
            if let Some(result) = self.entity_lookup_table.get(&candidate) {
                return Some(result);
            }
            // Remove the last parent's scope before trying again.
            // It's safe to unwrap here, since we know that `parents` is not empty.
            parents = parents.split_last().unwrap().1;
        }

        // We couldn't find the entity in any enclosing scope.
        None
    }
}

struct LookupTableBuilder<'ast> {
    type_lookup_table: &'ast mut HashMap<String, WeakPtr<dyn Type>>,
    entity_lookup_table: &'ast mut HashMap<String, WeakPtr<dyn Entity>>,
}

impl<'ast> LookupTableBuilder<'ast> {
    fn add_type_entry<T: Type + Entity + 'static>(&mut self, definition: &OwnedPtr<T>) {
        let identifier = definition.borrow().module_scoped_identifier();
        let weak_ptr = downgrade_as!(definition, dyn Type + 'static);
        self.type_lookup_table.insert(identifier, weak_ptr);
    }

    fn add_entity_entry<T: Entity + 'static>(&mut self, definition: &OwnedPtr<T>) {
        let identifier = definition.borrow().parser_scoped_identifier();
        let weak_ptr = downgrade_as!(definition, dyn Entity);
        self.entity_lookup_table.insert(identifier, weak_ptr);
    }
}

impl<'ast> PtrVisitor for LookupTableBuilder<'ast> {
    unsafe fn visit_module_start(&mut self, module_ptr: &mut OwnedPtr<Module>) {
        self.add_entity_entry(module_ptr);
    }

    unsafe fn visit_struct_start(&mut self, struct_ptr: &mut OwnedPtr<Struct>) {
        self.add_type_entry(struct_ptr);
        self.add_entity_entry(struct_ptr);
    }

    unsafe fn visit_class_start(&mut self, class_ptr: &mut OwnedPtr<Class>) {
        self.add_type_entry(class_ptr);
        self.add_entity_entry(class_ptr);
    }

    unsafe fn visit_exception_start(&mut self, exception_ptr: &mut OwnedPtr<Exception>) {
        self.add_entity_entry(exception_ptr);
    }

    unsafe fn visit_interface_start(&mut self, interface_ptr: &mut OwnedPtr<Interface>) {
        self.add_type_entry(interface_ptr);
        self.add_entity_entry(interface_ptr);
    }

    unsafe fn visit_enum_start(&mut self, enum_ptr: &mut OwnedPtr<Enum>) {
        self.add_type_entry(enum_ptr);
        self.add_entity_entry(enum_ptr);
    }

    unsafe fn visit_operation_start(&mut self, operation_ptr: &mut OwnedPtr<Operation>) {
        self.add_entity_entry(operation_ptr);
    }

    unsafe fn visit_type_alias(&mut self, type_alias_ptr: &mut OwnedPtr<TypeAlias>) {
        self.add_type_entry(type_alias_ptr);
        self.add_entity_entry(type_alias_ptr);
    }

    unsafe fn visit_data_member(&mut self, data_member_ptr: &mut OwnedPtr<DataMember>) {
        self.add_entity_entry(data_member_ptr);
    }

    unsafe fn visit_parameter(&mut self, parameter_ptr: &mut OwnedPtr<Parameter>) {
        self.add_entity_entry(parameter_ptr);
    }

    unsafe fn visit_return_member(&mut self, parameter_ptr: &mut OwnedPtr<Parameter>) {
        self.add_entity_entry(parameter_ptr);
    }

    unsafe fn visit_enumerator(&mut self, enumerator_ptr: &mut OwnedPtr<Enumerator>) {
        self.add_entity_entry(enumerator_ptr);
    }
}
