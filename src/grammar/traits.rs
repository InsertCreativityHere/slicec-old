
use crate::util::{SharedPtr, WeakPtr};

use super::comments::DocComment;
use super::slice::{Attribute, Identifier};
use super::util::*;

pub trait Element {
    fn kind(&self) -> &'static str;
}

pub trait Symbol: Element {
    fn location(&self) -> &Location;
}

pub trait NamedSymbol: Symbol {
    fn identifier(&self) -> &String;
    fn raw_identifier(&self) -> &Identifier;
}

pub trait ScopedSymbol: Symbol {
    fn scope(&self) -> &str;
    fn parser_scope(&self) -> &str;
    fn raw_scope(&self) -> &Scope;
}

pub trait Attributable: Symbol {
    fn attributes(&self) -> &Vec<Attribute>;
    fn has_attribute(&self, directive: &str) -> bool;
    fn get_attribute(&self, directive: &str) -> Option<&Vec<String>>;
    fn get_raw_attribute(&self, directive: &str) -> Option<Attribute>;
}

pub trait Commentable: Symbol {
    fn comment(&self) -> &DocComment;
}

pub trait Entity: NamedSymbol + ScopedSymbol + Attributable + Commentable {}

pub trait Container<T: Entity + ?Sized = dyn Entity>: Entity {
    fn contents(&self) -> &Vec<SharedPtr<T>>;
}

pub trait Contained<T: Entity + ?Sized = dyn Entity>: Entity {
    fn parent(&self) -> &WeakPtr<T>;
}

pub trait Type: Element {
    fn is_fixed_size(&self) -> bool;
    fn min_wire_size(&self) -> u32;
}
