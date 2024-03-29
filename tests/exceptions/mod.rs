// Copyright (c) ZeroC, Inc.

mod container;
mod inheritance;
mod mode_compatibility;
mod tags;

use crate::test_helpers::*;
use slicec::grammar::{Exception, NamedSymbol, Operation, Throws};

#[test]
fn throws_specific_exception() {
    let slice = "
        module Test

        exception E {}

        interface I {
            op() throws E
        }
    ";

    let ast = parse_for_ast(slice);
    let op = ast.find_element::<Operation>("Test::I::op").unwrap();

    let Throws::Specific(exception) = &op.throws else { panic!("Expected throws to be specific") };

    assert_eq!(
        exception.parser_scoped_identifier(),
        ast.find_element::<Exception>("Test::E")
            .unwrap()
            .parser_scoped_identifier()
    )
}

#[test]
fn throws_nothing() {
    let slice = "
        module Test

        interface I {
            op()
        }
    ";

    let ast = parse_for_ast(slice);
    let op = ast.find_element::<Operation>("Test::I::op").unwrap();

    assert!(matches!(op.throws, Throws::None));
}

#[test]
fn throws_any_exception() {
    let slice = "
        mode = Slice1
        module Test

        interface I {
            op() throws AnyException
        }
    ";

    let ast = parse_for_ast(slice);
    let op = ast.find_element::<Operation>("Test::I::op").unwrap();

    assert!(matches!(op.throws, Throws::AnyException));
}
