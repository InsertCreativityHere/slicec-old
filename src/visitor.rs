// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::grammar::{
    Class, DataMember, Definition, Enum, Enumerator, Exception, Interface, Module, Operation,
    Parameter, Struct, TypeAlias,
};
use crate::slice_file::SliceFile;
use crate::util::Ptr;

pub trait Visitor {
    fn visit_file_start(&mut self, slice_file: &SliceFile) {}
    fn visit_file_end(&mut self, slice_file: &SliceFile) {}

    fn visit_module_start(&mut self, module_def: &Module) {}
    fn visit_module_end(&mut self, module_def: &Module) {}
    fn visit_struct_start(&mut self, struct_def: &Struct) {}
    fn visit_struct_end(&mut self, struct_def: &Struct) {}
    fn visit_class_start(&mut self, class_def: &Class) {}
    fn visit_class_end(&mut self, class_def: &Class) {}
    fn visit_exception_start(&mut self, exception_def: &Exception) {}
    fn visit_exception_end(&mut self, exception_def: &Exception) {}
    fn visit_interface_start(&mut self, interface_def: &Interface) {}
    fn visit_interface_end(&mut self, interface_def: &Interface) {}
    fn visit_enum_start(&mut self, enum_def: &Enum) {}
    fn visit_enum_end(&mut self, enum_def: &Enum) {}

    fn visit_operation_start(&mut self, operation: &Operation) {}
    fn visit_operation_end(&mut self, operation: &Operation) {}

    fn visit_type_alias(&mut self, type_alias: &TypeAlias) {}

    fn visit_data_member(&mut self, data_member: &DataMember) {}
    fn visit_parameter(&mut self, parameter: &Parameter) {}
    fn visit_return_member(&mut self, parameter: &Parameter) {}
    fn visit_enumerator(&mut self, enumerator: &Enumerator) {}
}

impl SliceFile {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_file_start(self);
        for module_def in self.contents {
            module_def.borrow().visit_with(visitor);
        }
        visitor.visit_file_end(self);
    }
}

impl Module {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_module_start(self);
        for definition in self.contents {
            match definition.borrow() {
                Definition::Struct(struct_def) => struct_def.visit_with(visitor),
                Definition::Class(class_def) => class_def.visit_with(visitor),
                Definition::Exception(exception_def) => exception_def.visit_with(visitor),
                Definition::Interface(interface_def) => interface_def.visit_with(visitor),
                Definition::Enum(enum_def) => enum_def.visit_with(visitor),
                Definition::TypeAlias(type_alias) => type_alias.visit_with(visitor),
            }
        }
        visitor.visit_module_end(self);
    }
}

impl Struct {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_struct_start(self);
        for data_member in self.members {
            data_member.borrow().visit_with(visitor);
        }
        visitor.visit_struct_end(self);
    }
}

impl Class {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_class_start(self);
        for data_member in self.members {
            data_member.borrow().visit_with(visitor);
        }
        visitor.visit_class_end(self);
    }
}

impl Exception {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_exception_start(self);
        for data_member in self.members {
            data_member.borrow().visit_with(visitor);
        }
        visitor.visit_exception_end(self);
    }
}

impl Interface {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_interface_start(self);
        for operation in self.operations {
            operation.borrow().visit_with(visitor);
        }
        visitor.visit_interface_end(self);
    }
}

impl Enum {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_enum_start(self);
        for enumerators in self.enumerators {
            enumerators.borrow().visit_with(visitor);
        }
        visitor.visit_enum_end(self);
    }
}

impl Operation {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_operation_start(self);
        for parameter in self.parameters {
            parameter.borrow().visit_with(visitor, true);
        }
        for return_members in self.return_type {
            return_members.borrow().visit_with(visitor, false);
        }
        visitor.visit_operation_end(self);
    }
}

impl TypeAlias {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_type_alias(self);
    }
}

impl DataMember {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_data_member(self);
    }
}

impl Parameter {
    pub fn visit_with(&self, visitor: &mut impl Visitor, is_parameter: bool) {
        if is_parameter {
            visitor.visit_parameter(self);
        } else {
            visitor.visit_return_member(self);
        }
    }
}

impl Enumerator {
    pub fn visit_with(&self, visitor: &mut impl Visitor) {
        visitor.visit_enumerator(self);
    }
}
