#!/usr/bin/env python
"""
Tool to generate rust service definitions.
"""
from __future__ import (
    absolute_import,
    print_function,
)

import sys
from contextlib import contextmanager
from typing import (
    Any,
    Dict,
    List,
    Optional,
    Set,
    Text,
)

# Hax to get around fact that google protobuf libraries aren't in typeshed yet
import google.protobuf.descriptor_pb2 as d_typed
import six
from google.protobuf.compiler import plugin_pb2 as plugin

from proto.rust import service_extensions_pb2

d = d_typed  # type: Any


GENERATED_HEADER = Text("// @" "generated, do not edit\n")

SPEC_TOML_TEMPLATE = Text(
    "# @" "generated, do not edit\n"
    """
name = "{crate}"

[dependencies]
{deps}
""")


# Keywords in rust which cannot be module names.
RESERVED_KEYWORDS = [Text("move"), Text("type"), Text("struct")]

EMPTY_TYPE = u".google.protobuf.Empty"


def is_empty(ty):
    # type: (Text) -> bool
    return ty == EMPTY_TYPE


def sanitize_name(mod):
    # type: (Text) -> Text
    return mod + "_" if mod in RESERVED_KEYWORDS else mod


def sanitized_path(proto_filename):
    # type: (Text) -> List[Text]
    filename = proto_filename.replace("dropbox/proto/", "").replace(".proto", "")
    mod_parts = [sanitize_name(mod) for mod in filename.split("/")]
    return mod_parts


def message_crate_from_proto_filename(proto_filename):
    mod_parts = sanitized_path(proto_filename)
    crate_name = "proto_" + mod_parts[0]
    return crate_name, mod_parts[1:]


class WrappedService(object):

    def __init__(self, service_proto, info, method_info):
        # type: (d.ServiceDescriptorProto, Optional[d.SourceCodeInfo.Location], Dict[str, d.SourceCodeInfo.Location]) -> None
        self.proto = service_proto
        self.info = info
        self.method_info = method_info


class TextFormatter(object):

    def __init__(self):
        self.lines = []  # type: List[Text]
        self.indent = 0

    @contextmanager
    def block(self, header, *args, **kwargs):
        self.write("{} {{", header.format(*args, **kwargs))
        self.indent += 1
        yield
        self.indent -= 1
        self.write("}}")

    @contextmanager
    def statement(self, header, *args, **kwargs):
        self.write("{} {{", header.format(*args, **kwargs))
        self.indent += 1
        yield
        self.indent -= 1
        self.write("}};")

    def write_line_broken_text_with_prefix(self, block, prefix, *args, **kwargs):
        for l in block.format(*args, **kwargs).rstrip().split('\n'):
            if l:
                self.write("{}{}", prefix, l)
            else:
                self.write("")

    def write(self, line, *args, **kwargs):
        self.lines.append(("    " * self.indent) + line.format(*args, **kwargs))

    def contents(self):
        # type: () -> Text
        return "\n".join(self.lines)


class LibRs(object):
    def __init__(self):
        self.crates = set([
            "context",
            "futures",
            "pb",
            "pb_service",
        ])  # type: Set[Text]
        self.mods = []  # type: List[Text]

    def add_crate(self, crate):
        # type: (Text) -> None
        self.crates.add(crate)

    def add_mod(self, mod):
        # type: (Text) -> None
        self.mods.append(mod)

    def to_file_contents(self):
        # type: () -> Text
        lines = [GENERATED_HEADER]
        for crate in sorted(self.crates):
            lines.append("extern crate {};".format(crate))
        lines.append("")

        for mod in sorted(self.mods):
            lines.append("pub mod {};".format(mod))

        return "\n".join(lines)


class ServiceMod(object):

    def __init__(self, pkg_name, services, index):
        # type: (Text, List[WrappedService], Dict[Text, Text]) -> None
        self.services = services
        self.pkg_name = pkg_name
        self.index = index

    def fully_qualified_name(self, name):
        # type: (Text) -> Text
        if is_empty(name):
            return "()"
        return "::" + self.index[name]

    def get_rpc_name(self, service, method_name):
        # type: (WrappedService, Text) -> Text
        service_name = (self.pkg_name + "." if self.pkg_name else "") + service.proto.name
        return "/{}/{}".format(service_name, method_name)

    def gen_service(self, service):
        # type: (WrappedService) -> Text
        f = TextFormatter()

        if service.info:
            for comment in service.info.leading_detached_comments:
                f.write_line_broken_text_with_prefix(comment, "//")
                f.write("")

            if service.info.leading_comments:
                f.write_line_broken_text_with_prefix(service.info.leading_comments, "///")

        with f.block("pub trait {name}Service", name=service.proto.name):
            f.write("type Error: Debug;")
            f.write("")
            for method in service.proto.method:
                assert not method.client_streaming, "Client streaming not supported"

                info = service.method_info.get(method.name)
                if info:
                    for comment in info.leading_detached_comments:
                        f.write_line_broken_text_with_prefix(comment, "//")
                        f.write("")

                    if info.leading_comments:
                        f.write_line_broken_text_with_prefix(info.leading_comments, "///")
                        f.write("///")

                f.write("/// This method routes to `{}`.", self.get_rpc_name(service, method.name))

                timeout_ms = self.get_timeout_ms(service, method)
                if timeout_ms is not None:
                    f.write("/// This method has a timeout of {} ms.", timeout_ms)

                method_fmt_str = "fn {method_name}(&mut self, {maybe_request}) -> Box<dyn {kind}<Item = {response}, Error = Self::Error>>;"
                f.write(
                    method_fmt_str,
                    method_name=method.name,
                    kind="Stream" if method.server_streaming else "Future",
                    maybe_request="request: " + self.fully_qualified_name(method.input_type) if not is_empty(method.input_type) else "",
                    response=self.fully_qualified_name(method.output_type),
                )
                if info and info.trailing_comments:
                    f.write_line_broken_text_with_prefix(comment, "// ")
                f.write("")

        if service.info and service.info.trailing_comments:
            f.write("")
            f.write_line_broken_text_with_prefix(service.info.trailing_comments, "// ")

        return f.contents()

    def gen_dispatcher(self, service):
        # type: (WrappedService) -> Text
        f = TextFormatter()

        f.write("#[derive(Debug)]")
        with f.block("pub struct {name}Dispatcher<'a, S: {name}Service + 'a>", name=service.proto.name):
            f.write("service: &'a mut S,")
        f.write("")

        with f.block("impl<'a, S: {name}Service + 'a> {name}Dispatcher<'a, S> where S::Error: From<::std::io::Error> + 'static", name=service.proto.name):
            f.write("")
            f.write("/// Creates a new `{name}Dispatcher`.", name=service.proto.name)
            with f.block("pub fn new(service: &'a mut S) -> Self"):
                f.write("Self {{ service }}")
            f.write("")
            f.write("/// Dispatches requests by `route` to the appropriate method on [S] (for unary requests).")
            with f.block("pub fn dispatch_unary(&mut self, route: &str, req: &[u8]) -> Box<dyn Future<Item = Vec<u8>, Error = S::Error>>"):
                with f.block("match route"):
                    for method in service.proto.method:
                        if not method.server_streaming:
                            with f.block("\"{rpc_name}\" =>", rpc_name=self.get_rpc_name(service, method.name)):
                                empty_input = is_empty(method.input_type)
                                if not empty_input:
                                    with f.statement("let req_msg = match {req_type}::deserialize_from_slice(req)", req_type=self.fully_qualified_name(method.input_type)):
                                        f.write("Ok(m) => m,")
                                        f.write("Err(e) => return Box::new(::futures::future::err(e.into())),")

                                f.write("let f = self.service.{method_name}({req_msg});",
                                        method_name=method.name,
                                        req_msg="" if empty_input else "req_msg")
                                if is_empty(method.output_type):
                                    f.write("Box::new(f.map(|()| Vec::new()))")
                                else:
                                    f.write("Box::new(f.map(|resp| resp.serialize_to_vec()))")
                    with f.block("_ =>"):
                        f.write("Box::new(::futures::future::err(::std::io::Error::from(::std::io::ErrorKind::NotFound).into()))")
            f.write("")
            f.write("/// Dispatches requests by `route` to the appropriate method on [S] (for stream requests).")
            with f.block("pub fn dispatch_stream(&mut self, route: &str, req: &[u8]) -> Box<dyn Stream<Item = Vec<u8>, Error = S::Error>>"):
                with f.block("match route"):
                    for method in service.proto.method:
                        if method.server_streaming:
                            with f.block("\"{rpc_name}\" =>", rpc_name=self.get_rpc_name(service, method.name)):
                                empty_input = is_empty(method.input_type)
                                if not empty_input:
                                    with f.statement("let req_msg = match {req_type}::deserialize_from_slice(req)", req_type=self.fully_qualified_name(method.input_type)):
                                        f.write("Ok(m) => m,")
                                        f.write("Err(e) => return Box::new(::futures::future::err(e.into()).into_stream()),")

                                f.write("let f = self.service.{method_name}({req_msg});",
                                        method_name=method.name,
                                        req_msg="" if empty_input else "req_msg")
                                if is_empty(method.output_type):
                                    f.write("Box::new(f.map(|()| Vec::new()))")
                                else:
                                    f.write("Box::new(f.map(|resp| resp.serialize_to_vec()))")
                    with f.block("_ =>"):
                        f.write("Box::new(::futures::future::err(::std::io::Error::from(::std::io::ErrorKind::NotFound).into()).into_stream())")
        f.write("")

        with f.block("impl<'a, S: {name}Service + 'a> ::pb_service::Dispatch<S> for {name}Dispatcher<'a, S> where S::Error: From<::std::io::Error> + 'static", name=service.proto.name):
            f.write("type Error = S::Error;")
            with f.block("fn dispatch_unary(service: &mut S, route: &str, req: &[u8]) -> Box<dyn Future<Item = Vec<u8>, Error = S::Error>>"):
                f.write("{name}Dispatcher {{ service }}.dispatch_unary(route, req)", name=service.proto.name)
            with f.block("fn dispatch_stream(service: &mut S, route: &str, req: &[u8]) -> Box<dyn Stream<Item = Vec<u8>, Error = S::Error>>"):
                f.write("{name}Dispatcher {{ service }}.dispatch_stream(route, req)", name=service.proto.name)
        f.write("")

        return f.contents()

    def get_timeout_ms(self, service, method):
        # type: (WrappedService, d.MethodDescriptorProto) -> Optional[int]
        if method.options.HasExtension(service_extensions_pb2.method_default_deadline_ms):
            return method.options.Extensions[service_extensions_pb2.method_default_deadline_ms]

        if service.proto.options.HasExtension(service_extensions_pb2.service_default_deadline_ms):
            return service.proto.options.Extensions[service_extensions_pb2.service_default_deadline_ms]

        return None

    def gen_client(self, service):
        # type: (WrappedService) -> Text

        f = TextFormatter()

        f.write("#[derive(Debug, Clone)]")
        with f.block("pub struct {name}Client<T: ::pb_service::Transport>", name=service.proto.name):
            f.write("pub transport: T,")
        f.write("")

        with f.block("impl<T: ::pb_service::Transport> {name}Client<T>", name=service.proto.name):
            with f.block("pub fn new(transport: T) -> Self"):
                f.write("Self {{ transport }}")
        f.write("")

        with f.block("impl<T: ::pb_service::Transport> {name}Service for {name}Client<T>" +
                     " where T::Error : Debug", name=service.proto.name):
            f.write("type Error = T::Error;")
            f.write("")
            for method in service.proto.method:
                assert not method.client_streaming, "Client streaming not supported"

                method_fmt_str = "fn {method_name}(&mut self, {maybe_request}) -> Box<dyn {kind}<Item = {response}, Error = Self::Error>>"
                input_empty = is_empty(method.input_type)
                with f.block(
                    method_fmt_str,
                    method_name=method.name,
                    kind="Stream" if method.server_streaming else "Future",
                    maybe_request="request: " + self.fully_qualified_name(method.input_type) if not input_empty else "",
                    response=self.fully_qualified_name(method.output_type),
                ):
                    if input_empty:
                        f.write("let request = ();")
                    if method.server_streaming:
                        f.write(
                            "self.transport.unary_streaming(Context::background(), \"{}\", request)",
                            self.get_rpc_name(service, method.name),
                        )
                    else:
                        timeout_ms = self.get_timeout_ms(service, method)
                        if timeout_ms is not None:
                            f.write(
                                "let ctx = Context::background().with_timeout(::std::time::Duration::from_millis({}));",
                                timeout_ms,
                            )
                        else:
                            f.write("let ctx = Context::background();")
                        f.write(
                            "self.transport.unary_unary(ctx, \"{}\", request)",
                            self.get_rpc_name(service, method.name),
                        )
                f.write("")

        return f.contents()

    def to_file_contents(self):
        # type: () -> Text
        lines = [
            Text(GENERATED_HEADER),
            Text(""),
            Text("// Note, we allow <unused> specifically to avoid warnings about dispatch's `request` being unused,"),
            Text("// in case of services that don't have any streaming methods"),
            Text("#![allow(non_snake_case, unused)]"),
            Text(""),
            Text("use std::fmt::Debug;"),
            Text("use context::Context;"),
            Text("use futures::{Future, Stream};"),
            Text("use pb::Message;"),
        ]
        lines.extend(self.gen_service(s) for s in self.services)
        lines.append(Text(""))
        lines.extend(self.gen_client(s) for s in self.services)
        lines.append(Text(""))
        lines.extend(self.gen_dispatcher(s) for s in self.services)
        return "\n".join(lines)

    def extra_crates(self):
        # type: () -> Set[Text]
        extra_crates = set()
        for service in self.services:
            for method in service.proto.method:
                for ty in [method.input_type, method.output_type]:
                    if not is_empty(ty):
                        extra_crates.add(self.fully_qualified_name(ty).split('::')[1])
        return extra_crates


class Crate(object):

    def __init__(self, pkg_name, service_mods):
        # type: (Text, Dict[Text, ServiceMod]) -> None
        self.pkg_name = pkg_name
        self.lib_rs = LibRs()
        self.service_mods = service_mods

    def crate_relpath(self):
        # type: () -> List[Text]
        mod_parts = sanitized_path(self.pkg_name)
        crate_name = Text("proto_{}_svc".format(mod_parts[0]))
        return [crate_name] + mod_parts[1:]

    def append_to_response(self, resp):
        # type: (plugin.CodeGeneratorResponse) -> None

        # Note that output filenames must use "/" even on windows. It is part of the
        # protoc plugin protocol. The plugin speaks os-independent in "/". Thus, we
        # should not use os.path.sep or os.path.join
        crate_relpath = self.crate_relpath()

        # add the service files
        for modname, svc_mod in six.iteritems(self.service_mods):
            self.lib_rs.add_mod(modname)

            service_file = resp.file.add()
            service_file.name = "/".join(crate_relpath + ["src", sanitize_name(modname) + '.rs'])
            service_file.content = svc_mod.to_file_contents()
            for crate in svc_mod.extra_crates():
                self.lib_rs.add_crate(crate)

        # add the lib.rs
        lib_rs_file = resp.file.add()
        lib_rs_file.name = "/".join(crate_relpath + ["src", "lib.rs"])
        lib_rs_file.content = self.lib_rs.to_file_contents()

        # add the Spec.toml
        spec_toml_file = resp.file.add()
        spec_toml_file.name = "/".join(crate_relpath + ["Spec.toml"])
        spec_toml_file.content = SPEC_TOML_TEMPLATE.format(
            crate=crate_relpath[0],
            deps="\n".join("{} = {{}}".format(c) for c in sorted(self.lib_rs.crates))
        )


def walk(proto):
    enums, messages, services = [], [], []

    locations_by_path = {}

    if proto.source_code_info:
        for loc in proto.source_code_info.location:
            locations_by_path[tuple(loc.path)] = loc

    def _walk(proto, parents, index_path):
        if isinstance(proto, d.EnumDescriptorProto):
            enums.append((parents, proto))
        elif isinstance(proto, d.ServiceDescriptorProto):
            method_descriptions = {}
            for (field_descriptor, field_values) in proto.ListFields():
                if field_descriptor.name == 'method':
                    for (i, value) in enumerate(field_values):
                        key = tuple(index_path + [field_descriptor.number, i])
                        if key in locations_by_path:
                            method_descriptions[value.name] = locations_by_path[key]

            services.append((
                parents,
                WrappedService(proto, locations_by_path.get(tuple(index_path)), method_descriptions),
            ))
        elif isinstance(proto, d.DescriptorProto):
            messages.append((parents, proto))

            for (field_descriptor, field_values) in proto.ListFields():
                if field_descriptor.name not in ('nested_type', 'enum_type'):
                    continue
                for (i, v) in enumerate(field_values):
                    _walk(v, parents + [proto.name], index_path + [field_descriptor.number, i])
        elif isinstance(proto, d.FileDescriptorProto):
            for (field_descriptor, field_values) in proto.ListFields():
                if field_descriptor.name not in ('enum_type', 'message_type', 'service'):
                    continue
                for (i, v) in enumerate(field_values):
                    _walk(v, parents, index_path + [field_descriptor.number, i])

    _walk(proto, [], [])
    return enums, messages, services


class ProtoType(object):
    def __init__(self, proto_file, path, name):
        # type: (d.FileDescriptorProto, List[Text], Text) -> None
        self.proto_file = proto_file
        self.path = path
        self.name = name
        self.crate, self.mod_parts = message_crate_from_proto_filename(proto_file.name)

    def proto_name(self):
        # type: () -> Text
        """Name as used by proto eg. .mp.BlockMetadata.CompressionFormat"""
        r = "." + ".".join(self.path + [self.name])
        if self.proto_file.package:
            r = "." + self.proto_file.package + r
        return r

    def rust_name(self):
        # type: () -> Text
        """Name as used in rust code for proto_file"""
        mod_parts = self.mod_parts + ["_".join(self.path + [self.name])]
        mod_parts.insert(0, self.crate)
        return "::".join(mod_parts)


def generate_code(request, response):
    services_by_package = {}
    index = {}
    for proto_file_name in request.file_to_generate:
        proto_file = next(f for f in request.proto_file if f.name == proto_file_name)
        if not proto_file.package:
            continue

        enums, messages, services = walk(proto_file)

        for (parents, e) in enums:
            pt = ProtoType(proto_file, parents, e.name)
            index[pt.proto_name()] = pt.rust_name()
        for (parents, m) in messages:
            pt = ProtoType(proto_file, parents, m.name)
            index[pt.proto_name()] = pt.rust_name()

        if not services:
            continue

        for (_parents, svc) in services:
            pkg = services_by_package.setdefault(proto_file.package, {})
            pkg.setdefault(proto_file.name, []).append(svc)

    for package, services_by_file in six.iteritems(services_by_package):
        service_mods = {}
        for (fname, services) in six.iteritems(services_by_file):
            relpath = sanitized_path(fname)
            svc = ServiceMod(package, services, index)
            service_mods[relpath[-1]] = svc

        crate = Crate(package, service_mods)

        crate.append_to_response(response)


if __name__ == "__main__":
    # Read request message from stdin
    if six.PY3:
        data = sys.stdin.buffer.read()
    else:
        data = sys.stdin.read()

    # Parse request
    request = plugin.CodeGeneratorRequest()
    request.ParseFromString(data)

    # Create response
    response = plugin.CodeGeneratorResponse()

    # Generate code
    generate_code(request, response)

    # Serialise response message
    output = response.SerializeToString()

    # Write to stdout
    if six.PY3:
        sys.stdout.buffer.write(output)
    else:
        sys.stdout.write(output)
