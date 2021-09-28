// Copyright (c) ZeroC, Inc. All rights reserved.

#[derive(Clone, Debug)]
pub struct Scope {
    pub raw_module_scope: String,
    pub module_scope: Vec<String>,
    pub raw_parser_scope: String,
    pub parser_scope: Vec<String>,
}

impl Scope {
    pub fn new(name: &str, is_module: bool) -> Scope {
        let mut module_scope = Vec::new();
        if is_module {
            module_scope.push(name.to_owned());
        }
        let parser_scope = vec![name.to_owned()];

        Scope {
            raw_module_scope: module_scope.join("::"),
            module_scope,
            raw_parser_scope: parser_scope.join("::"),
            parser_scope,
        }
    }

    pub fn create_child_scope(parent: &Scope, name: &str, is_module: bool) -> Scope {
        let mut module_scope = parent.module_scope.clone();
        if is_module {
            module_scope.push(name.to_owned());
        }
        let mut parser_scope = parent.parser_scope.clone();
        parser_scope.push(name.to_owned());

        Scope {
            raw_module_scope: module_scope.join("::"),
            module_scope,
            raw_parser_scope: parser_scope.join("::"),
            parser_scope,
        }
    }
}
