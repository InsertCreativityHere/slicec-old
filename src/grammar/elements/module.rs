// Copyright (c) ZeroC, Inc.

use super::super::*;
use crate::slice_file::Span;
use crate::utils::ptr_util::WeakPtr;

#[derive(Debug)]
pub struct Module {
    pub identifier: Identifier,
    pub scope: Scope,
    pub attributes: Vec<WeakPtr<Attribute>>,
    pub span: Span,
}

implement_Element_for!(Module, "module");
implement_Attributable_for!(Module);
implement_Entity_for!(Module);
