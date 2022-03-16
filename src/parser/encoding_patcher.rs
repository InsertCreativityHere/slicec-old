// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::ast::Ast;
use crate::code_gen_util::SupportedEncodings;
use crate::grammar::*;
use crate::ptr_util::OwnedPtr;
use crate::ptr_visitor::PtrVisitor;
use crate::slice_file::SliceFile;
use std::collections::HashMap;

pub(super) fn patch_supported_encodings(slice_files: &HashMap<String, SliceFile>, ast: &mut Ast) {
    let mut patcher = EncodingPatcher { slice_files };

    for module in &mut ast.ast {
        unsafe { module.visit_ptr_with(&mut patcher); }
    }
}

struct EncodingPatcher<'files> {
    slice_files: &'files HashMap<String, SliceFile>
}

impl<'files> EncodingPatcher<'files> {
    /// Returns the encodings that are possible for types declared within the specified file.
    /// For a file with an encoding of 'V', the contents of the file can only be supported by
    /// encodings equal to, or less than 'V'.
    fn get_possible_encodings(&self, file_name: &str) -> SupportedEncodings {
        let slice_file = self.slice_files.get(file_name).unwrap();

        let encodings = match slice_file.file_encoding() {
            SliceEncoding::Slice11 => vec![SliceEncoding::Slice11, SliceEncoding::Slice2],
            SliceEncoding::Slice2 => vec![SliceEncoding::Slice2],
        };
        SupportedEncodings::new(&encodings)
    }
}

impl<'files> PtrVisitor for EncodingPatcher<'files> {
    unsafe fn visit_struct_start(&mut self, struct_ptr: &mut OwnedPtr<Struct>) {
        let struct_def = struct_ptr.borrow_mut();
        let mut encodings = self.get_possible_encodings(&struct_def.location().file);

        // Non-compact structs are not supported with the 1.1 encoding.
        if !struct_def.is_compact { encodings.disable_11(); }
        for member in struct_def.members() {
            let member_encodings = member.data_type.definition().supported_encodings();
            let is_optional = member.data_type.is_optional && !member.tag.is_some();
    
            if !member_encodings.supports_11() || is_optional { encodings.disable_11(); }
            if !member_encodings.supports_2() { encodings.disable_2(); }
        }

        struct_def.supported_encodings = Some(encodings);
    }

    unsafe fn visit_class_start(&mut self, class_ptr: &mut OwnedPtr<Class>) {
        let class_def = class_ptr.borrow_mut();
        let mut encodings = self.get_possible_encodings(&class_def.location().file);

        // Classes are only supported with the 1.1 encoding.
        encodings.disable_2();
        for member in class_def.members() {
            let member_encodings = member.data_type.definition().supported_encodings();
            let is_optional = member.data_type.is_optional && !member.tag.is_some();
    
            if !member_encodings.supports_11() || is_optional { encodings.disable_11(); }
        }

        class_def.supported_encodings = Some(encodings);
    }

    unsafe fn visit_exception_start(&mut self, exception_ptr: &mut OwnedPtr<Exception>) {
        let exception_def = exception_ptr.borrow_mut();
        let mut encodings = self.get_possible_encodings(&exception_def.location().file);

        // Exception inheritance is only supported with the 1.1 encoding.
        if exception_def.base.is_some() { encodings.disable_2(); }
        for member in exception_def.members() {
            let member_encodings = member.data_type.definition().supported_encodings();
            let is_optional = member.data_type.is_optional && !member.tag.is_some();
    
            if !member_encodings.supports_11() || is_optional { encodings.disable_11(); }
            if !member_encodings.supports_2() { encodings.disable_2(); }
        }

        exception_def.supported_encodings = Some(encodings);
    }

    unsafe fn visit_interface_start(&mut self, interface_ptr: &mut OwnedPtr<Interface>) {
        let interface_def = interface_ptr.borrow_mut();
        let encodings = self.get_possible_encodings(&interface_def.location().file);

        interface_def.supported_encodings = Some(encodings);
    }

    unsafe fn visit_enum_start(&mut self, enum_ptr: &mut OwnedPtr<Enum>) {
        let enum_def = enum_ptr.borrow_mut();
        let mut encodings = self.get_possible_encodings(&enum_def.location().file);

        // Enums with underlying types are not supported with the 1.1 encoding.
        if enum_def.underlying.is_some() { encodings.disable_11(); }

        enum_def.supported_encodings = Some(encodings);
    }

    unsafe fn visit_trait(&mut self, trait_ptr: &mut OwnedPtr<Trait>) {
        let trait_def = trait_ptr.borrow_mut();
        let mut encodings = self.get_possible_encodings(&trait_def.location().file);

        // Traits are not supported with the 1.1 encoding.
        encodings.disable_11();

        trait_def.supported_encodings = Some(encodings);
    }
}
