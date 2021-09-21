// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::util::{OwnedPtr, Ptr, WeakPtr};
use crate::grammar::*;
use crate::visitor::Visitor;
use std::collections::HashMap;

pub struct Ast {
    ast: Vec<OwnedPtr<Module>>,
    type_lookup_table: HashMap<String, WeakPtr<dyn Type>>,
    entity_lookup_table: HashMap<String, WeakPtr<dyn Entity>>,
    primitive_cache: HashMap<&'static str, OwnedPtr<Primitive>>,
}

impl Ast {
    pub fn new() -> Ast {
        let mut new_ast = Ast {
            ast: Vec::new(),
            type_lookup_table: HashMap::new(),
            entity_lookup_table: HashMap::new(),
            primitive_cache: HashMap::new(),
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

    fn add_cached_primitive(&mut self, identifier: &'static str, primitive: Primitive) {
        // Move the primitive onto the heap, so it can referenced via pointer.
        let primitive_ptr = OwnedPtr::new(primitive);

        // Insert an entry in the lookup table for the type, and cache the primitive's instance.
        self.type_lookup_table.insert(identifier.to_owned(), primitive_ptr.downgrade());
        self.primitive_cache.insert(identifier, primitive_ptr);
    }

    pub fn add_module(&mut self, module_def: Module) {
        // Move the module onto the heap so it can be referenced via pointer.
        let module_ptr = OwnedPtr::new(module_def);
        // Add the module into the AST's entity lookup table.
        self.entity_lookup_table.insert(
            module_ptr.borrow().parser_scoped_identifier(), module_ptr.downgrade(),
        );

        // Recursively visit it's contents and add them into the lookup table too.
        let mut visitor = LookupTableBuilder {
            type_lookup_table: &mut self.type_lookup_table,
            entity_lookup_table: &mut self.entity_lookup_table
        };
        module_def.visit_with(&mut visitor);

        // Store the module in the AST.
        self.ast.push(module_ptr);
    }

    pub fn lookup_type(&self, name: &str, scope: &Scope) -> Option<&WeakPtr<dyn Type>> {
        // Paths starting with '::' are absolute paths, which can be directly looked up.
        if name.starts_with("::") {
            return self.type_lookup_table.get(&name[2..]);
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
        if name.starts_with("::") {
            return self.entity_lookup_table.get(&name[2..]);
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

impl<'ast> Visitor for LookupTableBuilder<'ast> {
    fn visit_module_start(&mut self, module_def: &Module) {
        for definition_ptr in module_def.contents {
            let definition = definition_ptr.borrow();

            let is_type = match definition {
                Definition::Module(module_def)       => false,
                Definition::Struct(struct_def)       => true,
                Definition::Class(class_def)         => true,
                Definition::Exception(exception_def) => false,
                Definition::Interface(interface_def) => true,
                Definition::Enum(enum_def)           => true,
                Definition::TypeAlias(type_alias)    => true,
            };

            if is_type {
                self.type_lookup_table.insert(
                    definition.module_scoped_identifier(), definition_ptr.downgrade(),
                );
            }
            self.entity_lookup_table.insert(
                definition.parser_scoped_identifier(), definition_ptr.downgrade(),
            );
        }
    }

    fn visit_struct_start(&mut self, struct_def: &Struct) {
        for member in struct_def.members {
            self.entity_lookup_table.insert(
                member.borrow().parser_scoped_identifier(), member.downgrade(),
            );
        }
    }

    fn visit_class_start(&mut self, class_def: &Class) {
        for member in class_def.members {
            self.entity_lookup_table.insert(
                member.borrow().parser_scoped_identifier(), member.downgrade(),
            );
        }
    }

    fn visit_exception_start(&mut self, exception_def: &Exception) {
        for member in exception_def.members {
            self.entity_lookup_table.insert(
                member.borrow().parser_scoped_identifier(), member.downgrade(),
            );
        }
    }

    fn visit_interface_start(&mut self, interface_def: &Interface) {
        for operation in interface_def.operations {
            self.entity_lookup_table.insert(
                operation.borrow().parser_scoped_identifier(), operation.downgrade(),
            );
        }
    }

    fn visit_enum_start(&mut self, enum_def: &Enum) {
        for enumerator in enum_def.enumerators {
            self.entity_lookup_table.insert(
                enumerator.borrow().parser_scoped_identifier(), enumerator.downgrade(),
            );
        }
    }

    fn visit_operation_start(&mut self, operation: &Operation) {
        for parameter in operation.parameters {
            self.entity_lookup_table.insert(
                parameter.borrow().parser_scoped_identifier(), parameter.downgrade(),
            );
        }
        for return_member in operation.return_type {
            self.entity_lookup_table.insert(
                return_member.borrow().parser_scoped_identifier(), return_member.downgrade(),
            );
        }
    }
}
