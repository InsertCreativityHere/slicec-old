// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::grammar::*;
use crate::ast::{Ast, Node};
use crate::util::SliceFile;

/// Base trait for all visitors. It provides default empty implementations for all visitor functions.
///
/// These functions should never be called directly by code outside of this module.
/// Instead, visiting should be done by calling `visit_with` on grammar symbols, which will automatically call the
/// symbol's `x_start` and `x_end` visit functions, and visit through it's contents (if any).
// We keep the parameter names for doc generation, even if they're unused in the default implementations.
#[allow(unused_variables)]
pub trait Visitor {
    fn visit_file_start(&mut self, file: &SliceFile, ast: &Ast) {}
    fn visit_file_end(&mut self, file: &SliceFile, ast: &Ast) {}

    fn visit_module_start(&mut self, module_def: &Module, index: usize, ast: &Ast) {}
    fn visit_module_end(&mut self, module_def: &Module, index: usize, ast: &Ast) {}
    fn visit_struct_start(&mut self, struct_def: &Struct, index: usize, ast: &Ast) {}
    fn visit_struct_end(&mut self, struct_def: &Struct, index: usize, ast: &Ast) {}
    fn visit_interface_start(&mut self, interface_def: &Interface, index: usize, ast: &Ast) {}
    fn visit_interface_end(&mut self, interface_def: &Interface, index: usize, ast: &Ast) {}

    fn visit_data_member(&mut self, data_member: &DataMember, index: usize, ast: &Ast) {}

    fn visit_identifier(&mut self, identifier: &Identifier, ast: &Ast) {}
    fn visit_type_use(&mut self, type_use: &TypeRef, ast: &Ast) {}
}

impl Node {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast) {
        // Forward the `visit` call to the underlying element.
        match self {
            Self::Module(index, module_def)       => { module_def.visit_with(visitor, ast, *index) },
            Self::Struct(index, struct_def)       => { struct_def.visit_with(visitor, ast, *index) },
            Self::Interface(index, interface_def) => { interface_def.visit_with(visitor, ast, *index) },
            Self::DataMember(index, data_member)  => { data_member.visit_with(visitor, ast, *index) },
            _ => { panic!("Node cannot be visited!\n{:?}", self) }
        }
    }
}

impl SliceFile {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast) {
        visitor.visit_file_start(self, ast);
        for id in self.contents.iter() {
            ast.resolve_index(*id).visit_with(visitor, ast);
        }
        visitor.visit_file_end(self, ast);
    }
}

impl Module {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast, index: usize) {
        visitor.visit_module_start(self, index, ast);
        for id in self.contents.iter() {
            ast.resolve_index(*id).visit_with(visitor, ast);
        }
        visitor.visit_module_end(self, index, ast);
    }
}

impl Struct {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast, index: usize) {
        visitor.visit_struct_start(self, index, ast);
        for id in self.contents.iter() {
            ast.resolve_index(*id).visit_with(visitor, ast);
        }
        visitor.visit_struct_end(self, index, ast);
    }
}

impl Interface {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast, index: usize) {
        visitor.visit_interface_start(self, index, ast);
        visitor.visit_interface_end(self, index, ast);
    }
}

impl DataMember {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast, index: usize) {
        visitor.visit_data_member(self, index, ast);
    }
}

impl Identifier {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast) {
        visitor.visit_identifier(self, ast);
    }
}

impl TypeRef {
    pub fn visit_with(&self, visitor: &mut dyn Visitor, ast: &Ast) {
        visitor.visit_type_use(self, ast);
    }
}