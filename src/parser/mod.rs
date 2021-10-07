// Copyright (c) ZeroC, Inc. All rights reserved.
// TODO most of this module was just copy & pasted from the original implementation so that people
// can start using the newer implementation sooner. I really need to revisit all the code in this
// module and properly rewrite to take full advantage of the newer implementation.

mod comments;
mod parent_patcher;
mod preprocessor;
mod slice;
mod type_patcher;

// NOTE! it is NOT safe to call any methods on any of the slice entitites during parsing.
// Slice entities are NOT considered fully constructed until AFTER parsing is finished (including patching).
// Accessing ANY data, or calling ANY methods before this point may result in panics or undefined behavior.

// TODO write a real function signature here
pub fn parse_files(ast: &mut crate::ast::Ast) {
    // TODO parse stuff here!

    parent_patcher::patch_parents(ast);
    type_patcher::patch_types(ast);
}
