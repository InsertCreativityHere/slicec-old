// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::ast::{Ast, Node};
use crate::error::ErrorHandler;
use crate::grammar::*;
use crate::util::SliceFile;
use crate::visitor::Visitor;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct TableBuilder {
    /// This stack holds the identifiers of any enclosing scopes the builder is currently visiting within.
    ///
    /// Except for the 1st element, which is always the empty string.
    /// This represents the global scope, and ensures we always have a leading '::' when joining identifiers together.
    current_scope: Vec<String>,
    /// 
    scope_patches: Vec<(usize, String)>,
    ///
    type_table: 
}







    /// The patcher immutably visits elements and stores any needed patches in this vector. The patches are directly
    /// applied to the AST nodes in-place after visiting.
    /// Patches are stored as a tuple of a node's index in the AST, and the scope that should be patched into it.
    patches: Vec<(usize, String)>,
    /// TODO
    constructed_table: HashMap<String, usize>,
    /// Reference to the compiler's error handler so the patcher can output errors.
    error_handler: &'a mut ErrorHandler,
}

impl<'a> ScopePatcher<'a> {
    /// Creates a new `ScopePatcher` with an empty lookup table, and starting at global scope ("::").
    pub(crate) fn new(error_handler: &'a mut ErrorHandler) -> Self {
        ScopePatcher {
            // We add an empty string so when we join the vector with '::' separators, we'll get a leading "::".
            current_scope: vec!["".to_owned()],
            patches: Vec::new(),
            constructed_table: HashMap::new(),
            error_handler,
        }
    }

    pub(crate) fn patch_scopes(&mut self, slice_files: &HashMap<String, SliceFile>, ast: &mut Ast) -> HashMap<String, usize> {

    }



    /// Consumes the scope patcher and returns a lookup table that maps a symbol's fully scoped identifier to it's index
    /// in the AST, which allows nodes to be resolved via their fully scoped identifiers.
    pub(crate) fn into_lookup_table(self, ast: &Ast) -> HashMap<String, usize> {
        let mut table = HashMap::<String, usize>::new();
        for (index, scoped_identifier) in self.patches.into_iter() {
            // Issue an error if the table already contains an entry for this fully scoped identifier.
            if let Some(original_index) = table.get(&scoped_identifier) {
                let original = ast.resolve_index(*original_index).as_named_symbol().unwrap();
                let redefinition = ast.resolve_index(index).as_named_symbol().unwrap();

                self.error_handler.report_error((
                    format!("cannot reuse identifier `{}` in this scope", redefinition.identifier()),
                    redefinition.location().clone(),
                ).into());
                self.error_handler.report_note((
                    format!("{} `{}` was originally defined here", original.kind(), original.identifier()),
                    original.location().clone(),
                ).into());
            } else {
                // Otherwise insert the identifier and it's definition's index into the lookup table.
                table.insert(scoped_identifier, index);
            }
        }
        table
    }

    pub(crate) fn patch_scopes(&mut self, slice_files: &HashMap<String, SliceFile>, ast: &mut Ast) {
        // First immutably visit over the slice files to build a list of all the needed patches
        for slice_file in slice_files.values() {
            slice_file.visit_with(self, ast);
        }

        // Second, iterate over the patches and apply them to the AST nodes in-place.
        for (index, scope) in self.patches.into_iter() {
            if scope.is_empty() {
                scope = "::".to_owned();
            }

            let node = ast.resolve_index_mut(index);
            match node {
                Node::Module(_, module_def) => {
                    module_def.scope = Some(scope);
                }
                Node::Struct(_, struct_def) => {
                    struct_def.scope = Some(scope);
                }
                Node::Interface(_, interface_def) => {
                    interface_def.scope = Some(scope);
                }
                Node::DataMember(_, data_member) => {
                    data_member.scope = Some(scope);
                }
                Node::Sequence(_, sequence) => {
                    sequence.scope = Some(scope);
                }
                Node::Dictionary(_, dictionary) => {
                    dictionary.scope = Some(scope);
                }
                _ => {
                    // There are no other other symbols that can appear in the lookup table.
                    panic!("Grammar element does not need scope patching!\n{:?}", node);
                }
            }
        }
    }

    /// Computes the scope of the element currently being visited and adds an entry for it to the patch vector.
    /// Any elements added to the patch vector will have their 'scope' field patched when 'patch_scopes' is called.
    fn add_patch(&mut self, index: usize) {
        self.patches.push((index, self.current_scope.join("::")))
    }

    /// Computes the fully scoped identifier for the provided element, and stores an entry for it in the lookup table.
    fn add_table_entry(&mut self, element: &impl NamedSymbol, index: usize) {
        let scoped_identifier = self.current_scope.join("::") + "::" + element.identifier();
        self.constructed_table.insert(scoped_identifier, index);
    }
}

impl<'a> Visitor for ScopePatcher<'a> {
    fn visit_module_start(&mut self, module_def: &Module, index: usize, _: &Ast) {
        self.add_patch(index);
        self.add_table_entry(module_def, index);
        self.current_scope.push(module_def.identifier().to_owned());
    }

    fn visit_module_end(&mut self, _: &Module, _: usize, _: &Ast) {
        self.current_scope.pop();
    }

    fn visit_struct_start(&mut self, struct_def: &Struct, index: usize, _: &Ast) {
        self.add_patch(index);
        self.add_table_entry(struct_def, index);
        self.current_scope.push(struct_def.identifier().to_owned());
    }

    fn visit_struct_end(&mut self, _: &Struct, _: usize, _: &Ast) {
        self.current_scope.pop();
    }

    fn visit_interface_start(&mut self, interface_def: &Interface, index: usize, _: &Ast) {
        self.add_patch(index);
        self.add_table_entry(interface_def, index);
        self.current_scope.push(interface_def.identifier().to_owned());
    }

    fn visit_interface_end(&mut self, _: &Interface, _: usize, _: &Ast) {
        self.current_scope.pop();
    }

    fn visit_data_member(&mut self, data_member: &DataMember, index: usize, _: &Ast) {
        self.add_patch(index);
        self.add_table_entry(data_member, index);
    }

    fn visit_sequence(&mut self, _: &Sequence, index: usize, _: &Ast) {
        self.add_patch(index);
    }

    fn visit_dictionary(&mut self, _: &Dictionary, index: usize, _: &Ast) {
        self.add_patch(index);
    }
}

#[derive(Debug)]
pub(crate) struct TypePatcher<'a> {
    /// Reference to the compiler's error handler so the patcher can output errors.
    error_handler: &'a mut ErrorHandler,
}

impl<'a> TypePatcher<'a> {
    pub(crate) fn new(error_handler: &'a mut ErrorHandler) -> Self {
        TypePatcher { error_handler }
    }

    pub(crate) fn patch_types(&mut self, ast: &mut Ast, lookup_table: &HashMap<String, usize>) {
        for node in ast.iter_mut() {
            // Get the fully qualified scope for the element, and a reference to it's data type field.
            match node {
                Node::DataMember(_, data_member) => {
                    let scope = data_member.scope.as_ref().unwrap();
                    self.patch_type(scope, lookup_table, &mut data_member.data_type);
                },
                Node::Sequence(_, sequence) => {
                    let scope = sequence.scope.as_ref().unwrap();
                    self.patch_type(scope, lookup_table, &mut sequence.element_type);
                },
                Node::Dictionary(_, dictionary) => {
                    let scope = dictionary.scope.as_ref().unwrap();
                    self.patch_type(scope, lookup_table, &mut dictionary.key_type);
                    self.patch_type(scope, lookup_table, &mut dictionary.value_type);
                },
                _ => {}
            };
        }
    }

    fn patch_type(&mut self, scope: &str, lookup_table: &HashMap<String, usize>, type_use: &mut TypeRef) {
        // Skip if the type doesn't need patching. This is the case for builtin types that don't need resolving.
        if type_use.definition.is_some() {
            return;
        }

        // Attempt to resolve the type, and report an error if it fails.
        match Self::find_type(scope, &type_use.type_name, lookup_table) {
            Some(index) => {
                type_use.definition = Some(index);
            },
            None => {
                self.error_handler.report_error((
                    format!("failed to resolve type `{}` in scope `{}`", &type_use.type_name, scope),
                    type_use.location.clone(),
                ).into());
            },
        }
    }

    fn find_type(scope: &str, typename: &str, lookup_table: &HashMap<String, usize>) -> Option<usize> {
        // If the typename starts with '::' it's an absolute path, and we can directly look it up.
        if typename.starts_with("::") {
            return lookup_table.get(typename).copied();
        }

        // Search each enclosing scope for the type, from the bottom up.
        let parents: Vec<&str> = scope.split("::").collect();
        for i in (0..parents.len()).rev() {
            let test_scope = parents[..i].join("::") + "::" + typename;
            if let Some(result) = lookup_table.get(&test_scope) {
                return Some(*result);
            }
        }
        None
    }
}
