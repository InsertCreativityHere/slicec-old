// Copyright (c) ZeroC, Inc. All rights reserved.

pub mod helpers;

mod comments {

    use crate::assert_errors;
    use crate::helpers::parsing_helpers::{parse_for_ast, parse_for_diagnostics};
    use slice::diagnostics::WarningKind;
    use slice::grammar::*;
    use test_case::test_case;

    #[test_case("/// This is a doc comment.", "This is a doc comment."; "doc comment")]
    #[test_case("/// This is a\n/// multiline doc comment.", "This is a\nmultiline doc comment."; "multiline doc comment")]
    fn doc_comments_added_to_comment_overview(doc_comment: &str, expected: &str) {
        // Arrange
        let slice = format!(
            "
                module tests;

                {doc_comment}
                interface MyInterface
                {{
                }}
            "
        );

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let interface_def = ast.find_element::<Interface>("tests::MyInterface").unwrap();
        let interface_overview = interface_def.comment().unwrap().overview.as_ref().unwrap();

        assert_eq!(interface_overview.message.len(), 1);

        let MessageComponent::Text(text) = &interface_overview.message[0] else { panic!() };
        assert_eq!(text, expected);
    }

    #[test]
    fn doc_comments_params() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @param testParam: My test param
                testOp(testParam: string);
            }
        ";

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let operation = ast.find_element::<Operation>("tests::TestInterface::testOp").unwrap();
        let op_doc_params = &operation.comment().unwrap().params;

        assert_eq!(op_doc_params.len(), 1);

        let op_doc_param = &op_doc_params[0];
        assert_eq!(op_doc_param.identifier.value, "testParam");

        assert_eq!(op_doc_param.message.len(), 2);
        let MessageComponent::Text(text) = &op_doc_param.message[0] else { panic!() };
        assert_eq!(text, "My test param");
        let MessageComponent::Text(text) = &op_doc_param.message[0] else { panic!() };
        assert_eq!(text, "\n");
    }

    #[test]
    fn doc_comments_returns() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @returns bool
                testOp(testParam: string) -> bool;
            }
        ";

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let operation = ast.find_element::<Operation>("tests::TestInterface::testOp").unwrap();
        let op_doc_returns = &operation.comment().unwrap().returns;

        assert_eq!(op_doc_returns.len(), 1);

        let op_doc_return = &op_doc_returns[0];

        assert_eq!(op_doc_return.message.len(), 1);
        let MessageComponent::Text(text) = &op_doc_return.message[0] else { panic!() };
        assert_eq!(text, "\n");

        let Some(identifier) = &op_doc_return.identifier else { panic!() };
        assert_eq!(identifier.value, "bool");
    }

    #[test]
    fn operation_with_no_return_but_doc_comment_contains_return_fails() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @returns: This operation will return a bool.
                testOp(testParam: string);
            }
        ";

        // Act
        let diagnostic_reporter = parse_for_diagnostics(slice);

        // Assert
        assert_errors!(diagnostic_reporter, [
            "void operation must not contain doc comment return tag"
        ]);
    }

    #[test]
    fn operation_with_doc_comment_for_param_but_no_param_fails() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @param testParam1: A string param
                /// @param testParam2: A bool param
                testOp(testParam1: string);
            }
        ";

        // Act
        let diagnostic_reporter = parse_for_diagnostics(slice);

        // Assert
        assert_errors!(diagnostic_reporter, [
            "doc comment has a param tag for 'testParam2', but there is no parameter by that name",
        ]);
    }

    #[test]
    fn operation_with_correct_doc_comments() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @param testParam1: A string param
                /// @returns: bool
                /// @throws MyException: Some message about why testOp throws
                testOp(testParam1: string) -> bool;
            }
        ";

        // Act
        let diagnostic_reporter = parse_for_diagnostics(slice);

        // Assert
        assert_errors!(diagnostic_reporter);
    }

    #[test]
    fn doc_comments_throws() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @throws MyThrownThing: Message about my thrown thing.
                testOp(testParam: string) -> bool;
            }
        ";

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let operation = ast.find_element::<Operation>("tests::TestInterface::testOp").unwrap();
        let op_doc_throws = &operation.comment().unwrap().throws;

        assert_eq!(op_doc_throws.len(), 1);

        let op_doc_throw = &op_doc_throws[0];
        
        assert_eq!(op_doc_throw.message.len(), 2);
        let MessageComponent::Text(text) = &op_doc_throw.message[0] else { panic!() };
        assert_eq!(text, "Message about my thrown thing.");
        let MessageComponent::Text(text) = &op_doc_throw.message[1] else { panic!() };
        assert_eq!(text, "\n");

        let Some(identifier) = &op_doc_throw.identifier else { panic!() };
        assert_eq!(identifier.value, "MyThrownThing")
    }

    #[test]
    fn doc_comments_non_operations_cannot_throw() {
        // Arrange
        let slice = "
            module tests;

            /// @throws MyThrownThing: Message about my thrown thing.
            struct S
            {
            }
        ";

        // Act
        let diagnostic_reporter = parse_for_diagnostics(slice);

        // Assert
        assert_errors!(diagnostic_reporter, [
            "doc comment indicates that struct 'S' throws, however, only operations can throw",
        ]);
    }

    #[test]
    fn doc_comments_see_also() {
        // Arrange
        let slice = "
            module tests;

            interface TestInterface
            {
                /// @see MySee
                testOp(testParam: string) -> bool;
            }
        ";

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let operation = ast.find_element::<Operation>("tests::TestInterface::testOp").unwrap();
        let op_doc_sees = &operation.comment().unwrap().see;

        assert_eq!(op_doc_sees.len(), 1);

        let op_doc_see = &op_doc_sees[0];
        assert_eq!(op_doc_see.link.value, "MySee");
    }

    #[test_case("/// This is a doc comment.", (4, 16), (4, 39); "doc comment")]
    fn doc_comments_span(comment: &str, expected_start: (usize, usize), expected_end: (usize, usize)) {
        // Arrange
        let slice = format!(
            "
            module tests;

            {comment}
            interface MyInterface
            {{
            }}
            "
        );

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let interface_def = ast.find_element::<Interface>("tests::MyInterface").unwrap();
        let interface_doc = interface_def.comment().unwrap();

        assert_eq!(interface_doc.span().start, expected_start.into());
        assert_eq!(interface_doc.span().end, expected_end.into());
    }

    #[test_case("/* This is a block comment. */"; "block comment")]
    #[test_case("/*\n* This is a multiline block comment.\n */"; "multi-line block comment")]
    #[test_case("// This is a comment."; "comment")]
    fn non_doc_comments_are_ignored(comment: &str) {
        // Arrange
        let slice = format!(
            "
                module tests;

                {comment}
                interface MyInterface
                {{
                }}
            "
        );

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let interface_def = ast.find_element::<Interface>("tests::MyInterface").unwrap();
        let interface_doc = interface_def.comment();

        assert!(interface_doc.is_none());
    }

    #[test]
    fn doc_comment_linked_identifiers() {
        let slice = "
            module tests;

            /// This comment is for {@link TestStruct}
            struct TestStruct
            {
            }
            ";

        // Act
        let ast = parse_for_ast(slice);

        // Assert
        let struct_def = ast.find_element::<Struct>("tests::TestStruct").unwrap();
        let overview_message = &struct_def.comment().unwrap().overview.as_ref().unwrap().message;

        assert_eq!(overview_message.len(), 3);
        let MessageComponent::Text(text) = &overview_message[0] else { panic!() };
        assert_eq!(text, "This comment is for ");
        let MessageComponent::Link(link_tag) = &overview_message[1] else { panic!() };
        assert_eq!(link_tag.link.value, "TestStruct");
        let MessageComponent::Text(text) = &overview_message[2] else { panic!() };
        assert_eq!(text, "\n");
    }

    #[test]
    fn missing_doc_comment_linked_identifiers() {
        let slice = "
            module tests;

            /// A test struct. Similar to {@link OtherStruct}.
            struct TestStruct
            {
            }
            ";

        // Act
        let diagnostic_reporter = parse_for_diagnostics(slice);

        // Assert
        let expected = &crate::helpers::new_warning(WarningKind::InvalidDocCommentLinkIdentifier {
            identifier: "OtherStruct".to_owned(),
        });
        assert_errors!(diagnostic_reporter, [&expected]);
    }

    #[test]
    fn invalid_doc_comment_tag() {
        let slice = "
            module tests;

            /// A test struct. Similar to {@linked OtherStruct}{}.
            struct TestStruct
            {
            }
            ";

        // Act
        let diagnostic_reporter = parse_for_diagnostics(slice);

        // Assert
        let expected = &crate::helpers::new_warning(WarningKind::UnknownDocCommentTag {
            tag: "linked".to_owned(),
        });
        assert_errors!(diagnostic_reporter, [&expected]);
    }
}
