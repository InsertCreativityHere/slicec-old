// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::grammar::*;
use crate::util::OwnedPtr;

// Keep parameter names for doc generation, even if they're unused in the default implementations.
#[allow(unused_variables)]
pub trait PtrVisitor {
    unsafe fn visit_module_start(&mut self, module_ptr: &mut OwnedPtr<Module>) {}
    unsafe fn visit_module_end(&mut self, module_ptr: &mut OwnedPtr<Module>) {}
    unsafe fn visit_struct_start(&mut self, struct_ptr: &mut OwnedPtr<Struct>) {}
    unsafe fn visit_struct_end(&mut self, struct_ptr: &mut OwnedPtr<Struct>) {}
    unsafe fn visit_class_start(&mut self, class_ptr: &mut OwnedPtr<Class>) {}
    unsafe fn visit_class_end(&mut self, class_ptr: &mut OwnedPtr<Class>) {}
    unsafe fn visit_exception_start(&mut self, exception_ptr: &mut OwnedPtr<Exception>) {}
    unsafe fn visit_exception_end(&mut self, exception_ptr: &mut OwnedPtr<Exception>) {}
    unsafe fn visit_interface_start(&mut self, interface_ptr: &mut OwnedPtr<Interface>) {}
    unsafe fn visit_interface_end(&mut self, interface_ptr: &mut OwnedPtr<Interface>) {}
    unsafe fn visit_enum_start(&mut self, enum_ptr: &mut OwnedPtr<Enum>) {}
    unsafe fn visit_enum_end(&mut self, enum_ptr: &mut OwnedPtr<Enum>) {}

    unsafe fn visit_operation_start(&mut self, operation_ptr: &mut OwnedPtr<Operation>) {}
    unsafe fn visit_operation_end(&mut self, operation_ptr: &mut OwnedPtr<Operation>) {}

    unsafe fn visit_type_alias(&mut self, type_alias_ptr: &mut OwnedPtr<TypeAlias>) {}

    unsafe fn visit_data_member(&mut self, data_member_ptr: &mut OwnedPtr<DataMember>) {}
    unsafe fn visit_parameter(&mut self, parameter_ptr: &mut OwnedPtr<Parameter>) {}
    unsafe fn visit_return_member(&mut self, parameter_ptr: &mut OwnedPtr<Parameter>) {}
    unsafe fn visit_enumerator(&mut self, enumerator_ptr: &mut OwnedPtr<Enumerator>) {}
}

impl OwnedPtr<Module> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_module_start(self);
        for definition in &mut self.borrow_mut().contents {
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
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_struct_start(self);
        for data_member in &mut self.borrow_mut().members {
            data_member.visit_ptr_with(visitor);
        }
        visitor.visit_struct_end(self);
    }
}

impl OwnedPtr<Class> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_class_start(self);
        for data_member in &mut self.borrow_mut().members {
            data_member.visit_ptr_with(visitor);
        }
        visitor.visit_class_end(self);
    }
}

impl OwnedPtr<Exception> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_exception_start(self);
        for data_member in &mut self.borrow_mut().members {
            data_member.visit_ptr_with(visitor);
        }
        visitor.visit_exception_end(self);
    }
}

impl OwnedPtr<Interface> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_interface_start(self);
        for operation in &mut self.borrow_mut().operations {
            operation.visit_ptr_with(visitor);
        }
        visitor.visit_interface_end(self);
    }
}

impl OwnedPtr<Enum> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_enum_start(self);
        for enumerators in &mut self.borrow_mut().enumerators {
            enumerators.visit_ptr_with(visitor);
        }
        visitor.visit_enum_end(self);
    }
}

impl OwnedPtr<Operation> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_operation_start(self);
        for parameter in &mut self.borrow_mut().parameters {
            parameter.visit_ptr_with(visitor, true);
        }
        for return_members in &mut self.borrow_mut().return_type {
            return_members.visit_ptr_with(visitor, false);
        }
        visitor.visit_operation_end(self);
    }
}

impl OwnedPtr<TypeAlias> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_type_alias(self);
    }
}

impl OwnedPtr<DataMember> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_data_member(self);
    }
}

impl OwnedPtr<Parameter> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor, is_parameter: bool) {
        if is_parameter {
            visitor.visit_parameter(self);
        } else {
            visitor.visit_return_member(self);
        }
    }
}

impl OwnedPtr<Enumerator> {
    pub unsafe fn visit_ptr_with(&mut self, visitor: &mut impl PtrVisitor) {
        visitor.visit_enumerator(self);
    }
}
