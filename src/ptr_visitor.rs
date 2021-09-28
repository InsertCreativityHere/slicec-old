// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::grammar::*;
use crate::util::OwnedPtr;

// Keep parameter names for doc generation, even if they're unused in the default implementations.
#[allow(unused_variables)]
pub trait PtrVisitor {
    fn visit_module_start(&mut self, module_ptr: &OwnedPtr<Module>) {}
    fn visit_module_end(&mut self, module_ptr: &OwnedPtr<Module>) {}
    fn visit_struct_start(&mut self, struct_ptr: &OwnedPtr<Struct>) {}
    fn visit_struct_end(&mut self, struct_ptr: &OwnedPtr<Struct>) {}
    fn visit_class_start(&mut self, class_ptr: &OwnedPtr<Class>) {}
    fn visit_class_end(&mut self, class_ptr: &OwnedPtr<Class>) {}
    fn visit_exception_start(&mut self, exception_ptr: &OwnedPtr<Exception>) {}
    fn visit_exception_end(&mut self, exception_ptr: &OwnedPtr<Exception>) {}
    fn visit_interface_start(&mut self, interface_ptr: &OwnedPtr<Interface>) {}
    fn visit_interface_end(&mut self, interface_ptr: &OwnedPtr<Interface>) {}
    fn visit_enum_start(&mut self, enum_ptr: &OwnedPtr<Enum>) {}
    fn visit_enum_end(&mut self, enum_ptr: &OwnedPtr<Enum>) {}

    fn visit_operation_start(&mut self, operation_ptr: &OwnedPtr<Operation>) {}
    fn visit_operation_end(&mut self, operation_ptr: &OwnedPtr<Operation>) {}

    fn visit_type_alias(&mut self, type_alias_ptr: &OwnedPtr<TypeAlias>) {}

    fn visit_data_member(&mut self, data_member_ptr: &OwnedPtr<DataMember>) {}
    fn visit_parameter(&mut self, parameter_ptr: &OwnedPtr<Parameter>) {}
    fn visit_return_member(&mut self, parameter_ptr: &OwnedPtr<Parameter>) {}
    fn visit_enumerator(&mut self, enumerator_ptr: &OwnedPtr<Enumerator>) {}
}

impl OwnedPtr<Module> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_module_start(self);
        for definition in &self.borrow().contents {
            match definition {
                Definition::Module(module_ptr)        => module_ptr.visit_ptr_with(visitor),
                Definition::Struct(struct_ptr)        => struct_ptr.visit_ptr_with(visitor),
                Definition::Class(class_ptr)          => class_ptr.visit_ptr_with(visitor),
                Definition::Exception(exception_ptr)  => exception_ptr.visit_ptr_with(visitor),
                Definition::Interface(interface_ptr)  => interface_ptr.visit_ptr_with(visitor),
                Definition::Enum(enum_ptr)            => enum_ptr.visit_ptr_with(visitor),
                Definition::TypeAlias(type_alias_ptr) => type_alias_ptr.visit_ptr_with(visitor),
            }
        }
        visitor.visit_module_end(self);
    }
}

impl OwnedPtr<Struct> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_struct_start(self);
        for data_member in &self.borrow().members {
            data_member.visit_ptr_with(visitor);
        }
        visitor.visit_struct_end(self);
    }
}

impl OwnedPtr<Class> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_class_start(self);
        for data_member in &self.borrow().members {
            data_member.visit_ptr_with(visitor);
        }
        visitor.visit_class_end(self);
    }
}

impl OwnedPtr<Exception> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_exception_start(self);
        for data_member in &self.borrow().members {
            data_member.visit_ptr_with(visitor);
        }
        visitor.visit_exception_end(self);
    }
}

impl OwnedPtr<Interface> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_interface_start(self);
        for operation in &self.borrow().operations {
            operation.visit_ptr_with(visitor);
        }
        visitor.visit_interface_end(self);
    }
}

impl OwnedPtr<Enum> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_enum_start(self);
        for enumerators in &self.borrow().enumerators {
            enumerators.visit_ptr_with(visitor);
        }
        visitor.visit_enum_end(self);
    }
}

impl OwnedPtr<Operation> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_operation_start(self);
        for parameter in &self.borrow().parameters {
            parameter.visit_ptr_with(visitor, true);
        }
        for return_members in &self.borrow().return_type {
            return_members.visit_ptr_with(visitor, false);
        }
        visitor.visit_operation_end(self);
    }
}

impl OwnedPtr<TypeAlias> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_type_alias(self);
    }
}

impl OwnedPtr<DataMember> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_data_member(self);
    }
}

impl OwnedPtr<Parameter> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor, is_parameter: bool) {
        if is_parameter {
            visitor.visit_parameter(self);
        } else {
            visitor.visit_return_member(self);
        }
    }
}

impl OwnedPtr<Enumerator> {
    pub fn visit_ptr_with(&self, visitor: &mut impl PtrVisitor) {
        visitor.visit_enumerator(self);
    }
}
