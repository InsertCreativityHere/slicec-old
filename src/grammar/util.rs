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

    pub fn push_scope(&mut self, name: &str, is_module: bool) {
        if is_module {
            self.raw_module_scope += &("::".to_owned() + name);
            self.module_scope.push(name.to_owned());
        }
        self.raw_parser_scope += &("::".to_owned() + name);
        self.parser_scope.push(name.to_owned());
    }

    pub fn pop_scope(&mut self) {
        // If the last parser scope is also a module scope, pop off a module scope as well.
        if self.parser_scope.last() == self.module_scope.last() {
            self.module_scope.pop();
            // If there are multiple scope segments, truncate off the last one.
            // Otherwise, if there's only one scope, just clear the string.
            if let Some(index) = self.raw_module_scope.rfind("::") {
                self.raw_module_scope.truncate(index);
            } else {
                self.raw_module_scope.clear();
            }
        }

        self.parser_scope.pop();
        if let Some(index) = self.raw_parser_scope.rfind("::") {
            self.raw_parser_scope.truncate(index);
        } else {
            self.raw_parser_scope.clear();
        }
    }
}
