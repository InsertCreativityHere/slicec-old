// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::ast::Ast;
use crate::code_gen_util::SupportedEncodings;
use crate::grammar::*;
use crate::ptr_util::OwnedPtr;
use crate::ptr_visitor::PtrVisitor;
use crate::slice_file::SliceFile;
use std::collections::HashMap;

pub(super) fn patch_encodings(slice_files: &HashMap<String, SliceFile>, ast: &mut Ast) {
    let mut patcher = EncodingPatcher { slice_files, has_unpatched_encodings: true };

    while patcher.has_unpatched_encodings {
        patcher.has_unpatched_encodings = false;
        for module in &mut ast.ast {
            unsafe { module.visit_ptr_with(&mut patcher); }
        }
    }
}

struct EncodingPatcher<'files> {
    slice_files: &'files HashMap<String, SliceFile>,
    has_unpatched_encodings: bool,
}

impl<'files> EncodingPatcher<'files> {

}

impl<'files> PtrVisitor for EncodingPatcher<'files> {

}





// First make sure that all the composite types are patched, then do the actual patching, if not. Skip.