// Copyright (c) ZeroC, Inc.

use crate::grammar::Encoding;
use crate::implement_diagnostic_functions;
use crate::utils::string_util::{indefinite_article, pluralized_kind};

#[derive(Debug)]
pub enum Error {
    // ---------------- General Errors ---------------- //
    IO {
        action: &'static str,
        path: String,
        error: std::io::Error,
    },

    DoesNotExist {
        /// The identifier that was not found.
        identifier: String,
    },

    Redefinition {
        /// The identifier that was redefined.
        identifier: String,
    },

    ShadowedMember {
        /// The identifier that is being shadowed.
        identifier: String,

        /// The kind of element that is being shadowed.
        kind: &'static str,

        /// The identifier of the parent where the shadowed element is defined.
        parent: String,
    },

    CannotBeUsedAsType {
        /// The kind of element the user tries to use as a type.
        kind: &'static str,
    },

    IllegalInheritance {
        /// What kind of element we expected to inherit from.
        kind: &'static str,
    },

    NotSupportedWithEncoding {
        /// The kind of thing that isn't supported. This is only set for user-defined elements, not built-in elements.
        kind: Option<&'static str>,

        /// A string representation of the type that isn't supported. This includes modifiers like '?' and 'stream'.
        type_string: String,

        /// The encoding that is being used (that doesn't support the above construct).
        encoding: Encoding,
    },

    InfiniteSizeCycle {
        /// The identifier of the type with the infinite cycle.
        identifier: String,
    },

    // ---------------- Syntax Errors ---------------- //
    Syntax {
        /// The message to display to the user.
        message: String,
    },

    // TODO add more more specific syntax errors















    // ---------------- Dictionary Errors ---------------- //
    /// Dictionaries cannot use optional types as keys.
    KeyMustBeNonOptional,

    /// An unsupported type was used as a dictionary key type.
    KeyTypeNotSupported {
        /// The type and/or identifier of the type that was used as a dictionary key type.
        kind: String,
    },

    /// Struct contains a field that cannot be used as a dictionary key type.
    StructKeyContainsDisallowedType {
        /// The identifier of the struct.
        struct_identifier: String,
    },

    /// Structs must be compact to be used as a dictionary key type.
    StructKeyMustBeCompact,

    // ----------------  Encoding Errors ---------------- //
    /// The user specified an encoding multiple times in a single Slice file.
    MultipleEncodingVersions,

    // ----------------  Enum Errors ---------------- //
    /// Enumerator values must be unique.
    DuplicateEnumeratorValue {
        /// The value of the enumerator that was already used.
        enumerator_value: i128,
    },

    /// Enums cannot have optional underlying types.
    CannotUseOptionalUnderlyingType {
        /// The identifier of the enum.
        enum_identifier: String,
    },

    /// An enumerator was found that was out of bounds of the underlying type of the parent enum.
    EnumeratorValueOutOfBounds {
        /// The identifier of the enumerator.
        enumerator_identifier: String,
        /// The value of the out of bounds enumerator.
        value: i128,
        /// The minimum value of the underlying type of the enum.
        min: i128,
        /// The maximum value of the underlying type of the enum.
        max: i128,
    },

    /// Enums must be contain at least one enumerator.
    MustContainEnumerators {
        /// The identifier of the enum.
        enum_identifier: String,
    },

    /// Enum underlying types must be integral types.
    EnumUnderlyingTypeNotSupported {
        /// The identifier of the enum.
        enum_identifier: String,
        /// The name of the non-integral type that was used as the underlying type of the enum.
        kind: Option<String>,
    },

    // ----------------  Operation Errors ---------------- //
    /// A streamed parameter was not the last parameter in the operation.
    StreamedMembersMustBeLast {
        /// The identifier of the parameter that caused the error.
        parameter_identifier: String,
    },

    /// Return tuples for an operation must contain at least two element.
    ReturnTuplesMustContainAtLeastTwoElements,

    /// Multiple streamed parameters were used as parameters for an operation.
    MultipleStreamedMembers,

    // ----------------  Struct Errors ---------------- //
    /// Compact structs cannot be empty.
    CompactStructCannotBeEmpty,

    /// Compact structs cannot contain tagged fields.
    CompactStructCannotContainTaggedFields,

    // ----------------  Tag Errors ---------------- //
    /// A duplicate tag value was found.
    CannotHaveDuplicateTag {
        /// The identifier of the tagged member.
        identifier: String,
    },

    /// Cannot tag a class.
    CannotTagClass {
        /// The identifier of the tagged member.
        identifier: String,
    },

    /// Cannot tag a member that contains a class.
    CannotTagContainingClass {
        /// The identifier of the tagged member.
        identifier: String,
    },

    /// A tag value was not in the expected range, 0 .. i32::MAX.
    TagValueOutOfBounds,

    /// A tagged member was not set to optional.
    TaggedMemberMustBeOptional {
        /// The identifier of the tagged member.
        identifier: String,
    },

    // ----------------  General Errors ---------------- //
    /// A compact ID was not in the expected range, 0 .. i32::MAX.
    CompactIdOutOfBounds,

    /// A self-referential type alias has no concrete type.
    SelfReferentialTypeAliasNeedsConcreteType {
        /// The name of the type alias.
        identifier: String,
    },

    /// An integer literal was outside the parsable range of 0..i128::MAX.
    IntegerLiteralOverflows,

    /// An integer literal contained illegal characters for its base.
    InvalidIntegerLiteral {
        /// The base of the integer literal; Ex: 16 (hex), 10 (dec).
        base: u32,
    },

    /// An invalid Slice encoding was used.
    InvalidEncodingVersion {
        /// The encoding version that was used.
        encoding: String,
    },

    // ----------------  Attribute Errors ---------------- //
    /// An invalid argument was provided to an attribute directive.
    ArgumentNotSupported {
        /// The argument that was provided.
        argument: String,
        /// The directive it was provided to.
        directive: String,
    },

    // The following are errors that are needed to report cs attribute errors.
    MissingRequiredArgument {
        argument: String,
    },

    MissingRequiredAttribute {
        attribute: String,
    },

    TooManyArguments {
        expected: String,
    },

    UnexpectedAttribute {
        attribute: String,
    },

    AttributeIsNotRepeatable {
        attribute: String,
    },

    // ----------------  Type Alias Errors ---------------- //
    /// A type alias had an optional underlying type.
    TypeAliasOfOptional,
}

implement_diagnostic_functions!(
    Error,
    (
        "E001",
        IO,
        format!("unable to {action} '{path}': {}", io_error_message(error)),
        action,
        path,
        error
    ),
    (
        "E002",
        Redefinition,
        format!("'{identifier}' is already defined in this scope"),
        identifier
    ),
    (
        "E003",
        DoesNotExist,
        format!("no element with identifier '{identifier}' exists"),
        identifier
    ),
    (
        "E004",
        ShadowedMember,
        format!(
            "'{identifier}' shadows {a} {kind} inherited from '{parent}'",
            a = indefinite_article(&kind),
        ),
        identifier,
        kind,
        parent
    ),
    (
        "E005",
        CannotBeUsedAsType,
        format!(
            "{kinds} cannot be used as a type",
            kinds = pluralized_kind(kind),
        ),
        kind
    ),
    (
        "E006",
        IllegalInheritance,
        format!(
            "{kinds} can only inherit from other {kinds}",
            kinds = pluralized_kind(kind),
        ),
        kind
    ),
    (
        "E007",
        NotSupportedWithEncoding,
        format!(
            "{}{}'{type_string}' is not supported by the {encoding} encoding",
            kind.unwrap_or(""),
            if kind.is_some() { " " } else { "" },
        ),
        kind,
        type_string,
        encoding
    ),
    (
        "E008",
        InfiniteSizeCycle,
        format!("self-referential type '{identifier}' has infinite size"),
        identifier
    ),
    (
        "E009",
        Syntax,
        format!("invalid syntax: {message}"),
        message
    ),






    







    (
        "E004",
        ArgumentNotSupported,
        format!("'{argument}' is not a legal argument for the '{directive}' attribute"),
        argument,
        directive
    ),
    (
        "E005",
        KeyMustBeNonOptional,
        "optional types are not valid dictionary key types"
    ),
    (
        "E006",
        StructKeyMustBeCompact,
        "structs must be compact to be used as a dictionary key type"
    ),
    (
        "E007",
        KeyTypeNotSupported,
        format!("invalid dictionary key type: {kind}"),
        kind
    ),
    (
        "E008",
        StructKeyContainsDisallowedType,
        format!("struct '{struct_identifier}' contains fields that are not a valid dictionary key types"),
        struct_identifier
    ),
    (
        "E009",
        CannotUseOptionalUnderlyingType,
        format!("invalid enum '{enum_identifier}': enums cannot have optional underlying types"),
        enum_identifier
    ),
    (
        "E010",
        MustContainEnumerators,
        format!("invalid enum '{enum_identifier}': enums must contain at least one enumerator"),
        enum_identifier
    ),
    (
        "E011",
        EnumUnderlyingTypeNotSupported,
        {
            if let Some(kind) = kind {
                format!("invalid enum '{enum_identifier}': underlying type '{kind}' is not supported", )
            } else {
                format!("invalid enum '{enum_identifier}': missing required underlying type")
            }
        },
        enum_identifier,
        kind
    ),
    (
        "E014",
        CannotHaveDuplicateTag,
        format!("invalid tag on member '{identifier}': tags must be unique"),
        identifier
    ),
    (
        "E016",
        StreamedMembersMustBeLast,
        format!("invalid parameter '{parameter_identifier}': only the last parameter in an operation can use the stream modifier"),
        parameter_identifier
    ),
    (
        "E017",
        ReturnTuplesMustContainAtLeastTwoElements,
        "return tuples must have at least 2 elements"
    ),
    (
        "E018",
        CompactStructCannotContainTaggedFields,
        "tagged fields are not supported in compact structs\nconsider removing the tag, or making the struct non-compact"
    ),
    (
        "E019",
        TaggedMemberMustBeOptional,
        format!("invalid tag on member '{identifier}': tagged members must be optional"),
        identifier
    ),
    (
        "E020",
        CannotTagClass,
        format!("invalid tag on member '{identifier}': tagged members cannot be classes"),
        identifier
    ),
    (
        "E021",
        CannotTagContainingClass,
        format!("invalid tag on member '{identifier}': tagged members cannot contain classes"),
        identifier
    ),
    (
        "E024",
        CompactStructCannotBeEmpty,
        "compact structs must be non-empty"
    ),
    (
        "E025",
        SelfReferentialTypeAliasNeedsConcreteType,
        format!("self-referential type alias '{identifier}' has no concrete type"),
        identifier
    ),
    (
        "E026",
        EnumeratorValueOutOfBounds,
        format!(
            "invalid enumerator '{enumerator_identifier}': enumerator value '{value}' is out of bounds. The value must be between '{min}..{max}', inclusive",
        ),
        enumerator_identifier, value, min, max
    ),
    (
        "E027",
        TagValueOutOfBounds,
        "tag values must be within the range 0 <= value <= 2147483647"
    ),
    (
        "E028",
        DuplicateEnumeratorValue,
        format!("enumerator values must be unique; the value '{enumerator_value}' is already in use"),
        enumerator_value
    ),
    (
        "E034",
        UnexpectedAttribute,
        format!("unexpected attribute '{attribute}'"),
        attribute
    ),
    (
        "E035",
        MissingRequiredArgument,
        format!("missing required argument '{argument}'"),
        argument
    ),
    (
        "E036",
        TooManyArguments,
        format!("too many arguments, expected '{expected}'"),
        expected
    ),
    (
        "E037",
        MissingRequiredAttribute,
        format!("missing required attribute '{attribute}'"),
        attribute
    ),
    (
        "E038",
        MultipleStreamedMembers,
        "cannot have multiple streamed members"
    ),
    (
        "E039",
        CompactIdOutOfBounds,
        "compact IDs must be within the range 0 <= ID <= 2147483647"
    ),
    (
        "E040",
        IntegerLiteralOverflows,
        "integer literal is outside the parsable range of -2^127 <= i <= 2^127 - 1"
    ),
    (
        "E041",
        InvalidIntegerLiteral,
        format!("integer literal contains illegal characters for base-{base}"),
        base
    ),
    (
        "E042",
        InvalidEncodingVersion,
        format!("'{encoding}' is not a valid Slice encoding version"),
        encoding
    ),
    (
        "E043",
        MultipleEncodingVersions,
        "only a single encoding can be specified per file".to_owned()
    ),
    (
        "E050",
        AttributeIsNotRepeatable,
        format!("duplicate attribute '{attribute}'"),
        attribute
    ),
    (
        "E051",
        TypeAliasOfOptional,
        "optional types cannot be aliased"
    )
);

fn io_error_message(error: &std::io::Error) -> String {
    match error.kind() {
        std::io::ErrorKind::NotFound => "No such file or directory".to_owned(),
        _ => error.to_string(),
    }
}
