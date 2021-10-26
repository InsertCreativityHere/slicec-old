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
        let parser_scope = name
            .split("::")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();

        let module_scope = if is_module {
            parser_scope.clone()
        } else {
            Vec::new()
        };

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

/// This tag format describes how the data is encoded and how it can be skipped by the decoding
/// code if the tagged parameter is present in the buffer but is not known to the receiver.
#[derive(Clone, Debug)]
pub enum TagFormat {
    /// A fixed size numeric encoded on 1 byte such as bool or byte.
    F1,

    /// A fixed size numeric encoded on 2 bytes such as short.
    F2,

    /// A fixed size numeric encoded on 4 bytes such as int or float.
    F4,

    /// A fixed size numeric encoded on 8 bytes such as long or double.
    F8,

    /// A variable-length size encoded on 1 or 5 bytes.
    Size,

    /// A variable-length size followed by size bytes.
    VSize,

    /// A fixed length size (encoded on 4 bytes) followed by size bytes.
    FSize,

    /// Represents a class, but is no longer encoded or decoded.
    Class,

    /// Pseudo non-encoded format that means one of F1, F2, F4 or F8.
    VInt,

    /// Pseudo non-encoded format: like VSize but the size is optimized out.
    OVSize,
}
