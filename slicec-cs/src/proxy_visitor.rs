// Copyright (c) ZeroC, Inc. All rights reserved.

use crate::code_block::CodeBlock;
use crate::comments::*;
use crate::cs_util::*;
use crate::decoding::*;
use crate::encoding::*;
use slice::ast::Ast;
use slice::grammar::*;
use slice::util::*;
use slice::visitor::Visitor;
use slice::writer::Writer;

pub struct ProxyVisitor<'a> {
    output: &'a mut Writer,
}

impl<'a> ProxyVisitor<'a> {
    pub fn new(output: &'a mut Writer) -> ProxyVisitor<'a> {
        ProxyVisitor { output }
    }
}

impl Visitor for ProxyVisitor<'_> {
    fn visit_module_start(&mut self, module_def: &Module, _: usize, _: &Ast) {
        // write_comment(&mut self.output, module_def);
        let content = format!("\nnamespace {}\n{{", module_def.identifier());
        self.output.write(&content);
        self.output.indent_by(4);
    }

    fn visit_module_end(&mut self, _: &Module, _: usize, _: &Ast) {
        self.output.clear_line_separator();
        self.output.indent_by(-4);
        self.output.write("\n}");
        self.output.write_line_separator();
    }

    fn visit_interface_start(&mut self, interface_def: &Interface, _: usize, ast: &Ast) {
        let prx_interface = format!("{}Prx", interface_name(interface_def)); // IFooPrx
        let prx_impl: String = prx_interface.chars().skip(1).collect(); // IFooPrx -> FooPrx

        let all_bases: Vec<&Interface> = vec![];
        let bases: Vec<&Interface> = vec![];

        // prx impl bases
        let mut prx_impl_bases: Vec<String> = vec![
            prx_interface.clone(),
            "IceRpc.IPrx".to_owned(),
            format!("global::System.IEquatable<{}>", &prx_impl),
        ];

        if all_bases.iter().any(|b| b.scope() == "::IceRpc::Service")
            && interface_def.scope() != "::IceRpc::Service"
        {
            prx_impl_bases.push("IceRpc.IServicePrx".to_owned());
        }

        // prx bases
        let prx_bases: Vec<String> = bases
            .into_iter()
            .map(|b| escape_scoped_identifier(b, CaseStyle::Pascal, interface_def.scope()))
            .collect();

        // writeProxyDocComment(p, getDeprecateReason(p));
        // emitTypeIdAttribute(p->scoped());
        // emitCustomAttributes(p);
        // TODO: above doc comments and attributes

        // Generate abstract methods and documentation
        write!(
            self.output,
            r#"
{doc_comment}
public partial interface {prx_interface}{prx_bases}
{{
    {operations}
}}

/// <summary>Typed proxy struct. It implements <see cref="{prx_interface}"/> by sending requests to a remote IceRPC service.</summary>
{type_id_attribute}{custom_attributes}
public readonly partial struct {prx_impl} : {prx_impl_bases}
{{
    {request_class}
    {response_class}
}}
"#,
            doc_comment = "///TODO:",
            prx_interface = prx_interface,
            type_id_attribute = "", // TODO: emitTypeIdAttribute(p->scoped()),
            custom_attributes = "", // TODO: emitCustomAttributes(p),
            prx_bases = prx_bases.join(", "),
            prx_impl = prx_impl,
            prx_impl_bases = prx_impl_bases.join(", "),
            request_class = request_class(interface_def, &prx_impl, ast).indent(),
            response_class = response_class(interface_def, ast).indent(),
            operations = prx_operations(interface_def, ast).indent()
        )
    }
}

pub fn interface_name(interface_def: &Interface) -> String {
    let identifier = fix_case(interface_def.identifier(), CaseStyle::Pascal);
    let mut chars = identifier.chars();

    // Check if the interface already follows the 'I' prefix convention.
    if identifier.chars().count() > 2
        && chars.next().unwrap() == 'I'
        && chars.next().unwrap().is_uppercase()
    {
        identifier.to_owned()
    } else {
        format!("I{}", identifier)
    }
}

fn prx_operations(interface_def: &Interface, ast: &Ast) -> CodeBlock {
    let mut code = CodeBlock::new();

    let operations = interface_def.operations(ast);

    for operation in operations {
        let operation_name = escape_identifier(operation, CaseStyle::Pascal);
        let async_name = operation_name + "Async";

        let deprecate_reason = match &operation.comment {
            Some(comment) if comment.deprecate_reason.is_some() => {
                format!(
                    r#"[global::System::Obsolete("{}")]"#,
                    comment.deprecate_reason.as_ref().unwrap()
                )
            }
            _ => "".to_owned(),
        };

        writeln!(
            code,
            "{doc_comment}{deprecate_reason}\n{return} {name}({params});\n",
            doc_comment = operation_doc_comment(operation, false, ast),
            deprecate_reason = deprecate_reason,
            return = operation_return_task(operation, false, ast),
            name = async_name,
            params = get_invocation_params(operation, ast).join(", ")
        )
    }

    code
}

pub fn operation_return_task(operation: &Operation, is_dispatch: bool, ast: &Ast) -> String {
    let return_members = operation.return_members(ast);
    if return_members.is_empty() {
        if is_dispatch {
            "global::System.Threading.Tasks.ValueTask".to_owned()
        } else {
            "global::System.Threading.Tasks.Task".to_owned()
        }
    } else {
        let return_type = operation_return_type(operation, is_dispatch, ast);
        if is_dispatch {
            format!("global::System.Threading.Tasks.ValueTask<{}>", return_type)
        } else {
            format!("global::System.Threading.Tasks.Task<{}>", return_type)
        }
    }
}

pub fn operation_return_type(operation: &Operation, is_dispatch: bool, ast: &Ast) -> String {
    let return_type = &operation.return_type;

    let has_marshaled_result = false; // TODO: do we still want to keep this?

    if is_dispatch && has_marshaled_result {
        return "".to_owned();
    }

    let return_members = operation.return_members(ast);
    match return_members.len() {
        0 => "void".to_owned(),
        1 => param_type_to_string(&return_members[0].data_type, is_dispatch, ast),
        _ => to_tuple_type(&return_members, is_dispatch, ast),
    }
}

pub fn to_tuple_type(members: &[&Member], is_dispatch: bool, ast: &Ast) -> String {
    match members.len() {
        0 => panic!("tuple type with no members"),
        1 => param_type_to_string(&members[0].data_type, is_dispatch, ast),
        _ => format!(
            "({})",
            members
                .into_iter()
                .map(|m| param_type_to_string(&m.data_type, is_dispatch, ast))
                .collect::<Vec<String>>()
                .join(", ")
        ),
    }
}

pub fn to_tuple_return(members: &[&Member], prefix: &str, ast: &Ast) -> String {
    match members.len() {
        0 => panic!("tuple type with no members"),
        1 => member_name(&members[0], prefix, true),
        _ => format!(
            "({})",
            members
                .iter()
                .map(|m| member_name(m, prefix, true))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

// TODO: maybe rename operation_param_to_string
pub fn param_type_to_string(type_ref: &TypeRef, is_dispatch: bool, ast: &Ast) -> String {
    let context = if is_dispatch {
        TypeContext::Incoming
    } else {
        TypeContext::Outgoing
    };

    type_to_string(type_ref, type_ref.scope(), ast, context)
}

pub fn get_invocation_params(operation: &Operation, ast: &Ast) -> Vec<String> {
    let mut params = Vec::new();

    let operation_parameters = operation.parameters(ast);

    for p in operation.parameters(ast) {
        params.push(format!(
            "{attributes}{param_type} {param_name}",
            attributes = "", // TOOD: getParamAttributes(p)
            param_type = type_to_string(&p.data_type, p.scope(), ast, TypeContext::Outgoing),
            param_name = member_name(p, "", true)
        ))
    }

    params.push(format!(
        "IceRpc.Invocation? {} = null",
        escape_member_name(&operation_parameters, "invocation")
    ));
    params.push(format!(
        "global::System.Threading.CancellationToken {} = default",
        escape_member_name(&operation_parameters, "cancel")
    ));

    params
}

fn request_class(interface_def: &Interface, prx_impl: &str, ast: &Ast) -> CodeBlock {
    let operations = interface_def.operations(ast);

    if !operations.iter().any(|o| o.has_non_streamed_params(ast)) {
        return "".into();
    }

    let mut request_operations = CodeBlock::new();

    for operation in operations {
        let params: Vec<&Member> = operation.non_streamed_params(ast);

        if params.len() == 0 {
            continue;
        }

        writeln!(
            request_operations,
            r#"
/// <summary>Creates the request payload for operation {name}.</summary>
/// <param name="prx">Typed proxy to the target service.</param>
/// <param name="arg{s}">The request argument{s}.</param>
/// <returns>The payload.</returns>
public static global::System.ReadOnlyMemory<global::System.ReadOnlyMemory<byte>> {escaped_name}({prx_impl} prx, {_in}{params} arg{s}) =>
    IceRpc.Payload.{create_payload}(
        prx.Payload,
        {_in}arg{s},
        {encode_action},
        {class_format});
"#,
            name = operation.identifier(),
            s = if params.len() == 1 { "" } else { "s" },
            escaped_name = escape_identifier(operation, CaseStyle::Pascal),
            prx_impl = prx_impl,
            params = to_tuple_type(&params, true, ast),
            _in = if params.len() == 1 { "" } else { "in " },
            create_payload = if params.len() == 1 { "FromSingleArg" } else { "FromArgs" },
            encode_action = request_encode_action(operation, ast).indent().indent(),
            class_format = operation_format_type_to_string(operation)
        )
    }

    format!("\
/// <summary>Converts the arguments of each operation that takes arguments into a request payload.</summary>
public static class Request
{{
    {}
}}
", request_operations.indent()).into()
}

fn response_class(interface_def: &Interface, ast: &Ast) -> CodeBlock {
    let operations = interface_def.operations(ast);

    if !operations.iter().any(|o| o.has_non_streamed_return(ast)) {
        return "".into();
    }

    let mut response_operations = CodeBlock::new();

    for operation in operations {
        let members = operation.return_members(ast);

        if members.len() == 0 {
            continue;
        }

        let escaped_name = escape_identifier(operation, CaseStyle::Pascal);
        writeln!(
            response_operations,
            r#"
/// <summary>The <see cref="IceRpc.Gen.ResponseDecodeFunc{{T}}"/> for the return value type of operation {name}.</summary>
public static {return_type} {escaped_name}(global::System.ReadOnlyMemory<byte> payload, IceRpc.StreamParamReceiver? streamParamReceiver, IceRpc.Encoding payloadEncoding, IceRpc.Connection connection, IceRpc.IInvoker? invoker) =>
    IceRpc.Payload.ToReturnValue(
        payload,
        payloadEncoding,
        {response_decode_func},
        connection,
        invoker);
"#,
            name = operation.identifier(),
            return_type = to_tuple_type(&members, false, ast),
            escaped_name = escaped_name,
            response_decode_func = response_decode_func(operation, ast)
        );
    }

    format!(
        r#"
/// <summary>Holds a <see cref="IceRpc.Gen.ResponseDecodeFunc{{T}}"/> for each non-void remote operation defined in <see cref="{interface_name}Prx"/>.</summary>
public static class Response
{{
    {response_operations}
}}
"#,
interface_name = interface_name(interface_def),
response_operations = response_operations.indent()
    )
    .into()
}

fn request_encode_action(operation: &Operation, ast: &Ast) -> CodeBlock {
    // TODO: scope
    let ns = get_namespace(operation);

    // We only want the non-streamed params
    let params: Vec<&Member> = operation.non_streamed_params(ast);

    // When the operation's parameter is a T? where T is an interface or a class, there is a
    // built-in encoder, so defaultEncodeAction is true.
    if params.len() == 1
        && get_bit_sequence_size(&params, ast) == 0
        && params.first().unwrap().tag.is_none()
    {
        encode_action(&params.first().unwrap().data_type, &ns, true, true, ast)
    } else {
        format!(
            "\
(IceRpc.IceEncoder encoder, {_in}{param_type} value) =>
{{
    {encode}
}}",
            _in = if params.len() == 1 { "" } else { "in " },
            param_type = to_tuple_type(&params, true, ast),
            encode = encode_operation(operation, false, ast).indent()
        )
        .into()
    }
}

fn response_decode_func(operation: &Operation, ast: &Ast) -> CodeBlock {
    let ns = get_namespace(operation);
    // vec of members
    let members = operation.return_members(ast);

    assert!(
        !members.is_empty()
            && (members.len() > 1 || !members.last().unwrap().data_type.is_streamed)
    );

    if members.len() == 1
        && get_bit_sequence_size(&members, ast) == 0
        && members.first().unwrap().tag.is_none()
    {
        decode_func(&members.first().unwrap().data_type, &ns, ast)
    } else {
        format!(
            "decoder => {{ {decode} }}",
            decode = decode_operation(operation, true, ast).indent()
        )
        .into()
    }
}
