use std::cmp::{min, Ordering};
use itertools::Itertools;
use khronos_registry_parse::gl::{
    Command, CommandParam, Enum, Enums, EnumsChild, ExtensionChild, InterfaceItem, Registry,
    RegistryChild,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, char, hex_digit1, one_of, u16};
use nom::combinator::{complete, map, map_parser, map_res, opt, recognize, value};
use nom::error::{ParseError, VerboseError};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};

use nom::{Finish, IResult, Parser};
use quote::{format_ident, quote, ToTokens};
use regex::Regex;

use std::collections::{HashMap, HashSet};

use proc_macro2::{Span, TokenStream};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::os::raw::c_int;
use std::path::{Path, PathBuf};

use syn::spanned::Spanned;

use crate::command_parser::{
    parse_argument, parse_proto, Arg, ArgumentDef, FundamentalType, PointerType, ProtoDef,
    ProtoReturn,
};
use crate::const_parser::{parse_constant, Constant};
use syn::LitByteStr;

#[derive(Clone, Eq)]
struct APIEnum {
    name: String,
    constant: Constant,
}

impl PartialEq for APIEnum {
    fn eq(&self, other: &Self) -> bool {
        self.name.as_str() == other.name.as_str()
    }
}
impl Hash for APIEnum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

struct APIArgument {
    group: Option<String>,
    name: String,
    def: ArgumentDef,
}

struct APIProto {
    group: Option<String>,
    def: ProtoDef,
}

struct APICommand {
    name: String,
    proto: APIProto,
    arguments: Vec<APIArgument>,
}

struct APIGroup {
    commands: HashSet<String>,
    groups: Vec<ExtensionChild>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum APIName {
    OPENGL { minor: u16, major: u16 },
    GLES { minor: u16, major: u16 },
    GLES2 { minor: u16, major: u16 },
    GLES3 { minor: u16, major: u16 },
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct _Enum(Enum);

impl Hash for _Enum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state)
    }
}

struct Context {
    constant_map: HashMap<String, APIEnum>,
    bitmap_cache: HashMap<String, HashSet<APIEnum>>,
    enum_cache: HashMap<String, HashSet<APIEnum>>,
    command_cache: HashMap<String, APICommand>,
    feature_cache: HashMap<APIName, APIGroup>,
    extension_cache: HashMap<String, APIGroup>,
}

fn construct_const(enums: &[&APIEnum]) -> Vec<TokenStream> {
    enums
        .iter()
        .map(|c| {
            let name = format_ident!("{}", &c.name);
            let value = &c.constant;
            quote! {
                pub const #name: Self = Self(#value);
            }
        })
        .collect()
}

fn map_type(arg: &str) -> TokenStream {
    match arg {
        "_cl_context" => quote! {CLContext},
        "_cl_event" => quote! {CLContext},
        "void" => quote! { std::os::raw::c_void },
        "int64_t" => quote! {u64},
        "int32_t" => quote! {u32},
        "Bool" => quote! {bool},
        "float" => quote! { f32 },
        i => format_ident!("{}", i).to_token_stream(),
    }
}

fn map_fundamental_type(fundamental_type: &FundamentalType) -> TokenStream {
    match fundamental_type {
        FundamentalType::SignedShortInt => {
            quote! { std::os::raw::c_short }
        }
        FundamentalType::SignedInt => {
            quote! { std::os::raw::c_int }
        }
        FundamentalType::SignedLongInt => {
            quote! { std::os::raw::c_long }
        }
        FundamentalType::SignedLongLongInt => {
            quote! { std::os::raw::c_longlong }
        }
        FundamentalType::UnsignedShortInt => {
            quote! { std::os::raw::c_ushort }
        }
        FundamentalType::UnsignedInt => {
            quote! { std::os::raw::c_uint }
        }
        FundamentalType::UnsignedLongInt => {
            quote! { std::os::raw::c_ulong }
        }
        FundamentalType::UnsignedLongLongInt => {
            quote! { std::os::raw::c_ulonglong }
        }
    }
}

fn construct_arguments(args: &[APIArgument]) -> Vec<TokenStream> {
    args.iter()
        .map(|cmd| {
            let pointer_defs: Vec<TokenStream> = if let Some(p) = &cmd.def.pointer {
                let mut ptrs: Vec<TokenStream> = p
                    .iter()
                    .map(|p| match p {
                        PointerType::Normal => {
                            quote! {*mut }
                        }
                        PointerType::ConstPointer => {
                            quote! {*const }
                        }
                    })
                    .collect();
                if cmd.def.is_const && ptrs.len() > 0 {
                    let length = ptrs.len();
                    ptrs[length - 1] = quote! {*const };
                }
                ptrs
            } else {
                vec![]
            };
            let type_def: TokenStream = {
                match &cmd.def.argument {
                    Arg::Fundamental(fund_type) => map_fundamental_type(fund_type),
                    Arg::Struct(struct_type) => {
                        let mut value = struct_type.as_str();
                        let struct_name = map_type(value);
                        quote! { #struct_name }
                    }
                    Arg::Alias(alias_type) => {
                        let mut value = alias_type.as_str();
                        if alias_type.as_str().eq("GLenum") || alias_type.as_str().eq("GLbitfield")
                        {
                            if let Some(group_name) = &cmd.group {
                                value = group_name.as_str();
                            }
                        }
                        let alias_name = map_type(value);
                        quote! { #alias_name }
                    }
                }
            };
            let name = format_ident!("_{}", cmd.def.name.as_str());
            quote! { #name : #(#pointer_defs)* #type_def }
        })
        .collect()
}

fn build_enum_block(collection: &HashMap<String, HashSet<APIEnum>>) -> Vec<TokenStream> {
    collection
        .iter()
        .map(|(name, enums)| {
            if (name.as_str().eq("SpecialNumbers")) {
                quote! {}
            } else {
                let e: Vec<&APIEnum> = enums.iter().collect();
                let impl_enum: Vec<TokenStream> = construct_const(e.as_slice());
                let ident = format_ident!("{}", &name);
                quote! {
                    #[repr(transparent)]
                    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    pub struct #ident(pub(crate) std::os::raw::c_uint);
                    impl #ident {
                       #(#impl_enum)*
                    }
                }
            }
        })
        .collect()
}

fn build_enum_type_block(collection: &HashMap<String, HashSet<APIEnum>>) -> Vec<TokenStream>{
    collection
        .iter()
        .map(|(name, enums)| {
            if (name.as_str().eq("SpecialNumbers")) {
                quote! {}
            } else {
                let e: Vec<&APIEnum> = enums.iter().collect();
                let impl_enum: Vec<TokenStream> = construct_const(e.as_slice());
                let ident = format_ident!("{}", &name);
                quote! {
                    #[repr(transparent)]
                    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                    pub struct #ident(pub(crate) std::os::raw::c_uint);
                    impl #ident {
                       #(#impl_enum)*
                    }
                }
            }
        })
        .collect()
}

fn build_bitflag_block(collection: &HashMap<String, HashSet<APIEnum>>) -> Vec<TokenStream> {
    collection
        .iter()
        .map(|(name, enums)| {
            let e: Vec<&APIEnum> = enums.iter().collect();
            let impl_bitflags: Vec<TokenStream> = construct_const(e.as_slice());
            let ident = format_ident!("{}", &name);
            quote! {
                #[repr(transparent)]
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct #ident(pub(crate) std::os::raw::c_uint);
                impl #ident {
                   #(#impl_bitflags)*
                }
            }
        })
        .collect()
}

fn build_function_block(
    proto: &APIProto,
    arguments: &[APIArgument],
) -> (TokenStream, Option<TokenStream>) {
    let return_defs: Option<TokenStream> = match &proto.def.return_arg {
        ProtoReturn::Fundamental(fund_type) => Some(map_fundamental_type(fund_type)),
        ProtoReturn::Alias(alias_type) => {
            let mut value = alias_type.as_str();
            // if alias_type.as_str().eq("GLenum") || alias_type.as_str().eq("GLbitfield") {
            //     if let Some(group_name) = &proto.group {
            //         value = group_name.as_str();
            //     }
            // }
            if proto.def.pointer.is_none() && value.eq("void") {
                None
            } else {
                Some(map_type(value))
            }
        }
    };
    let args: Vec<TokenStream> = construct_arguments(arguments);
    match &return_defs {
        None => (
            quote! {
                #(#args,)*
            },
            None,
        ),
        Some(return_arg) => {
            let pointer_defs: Vec<TokenStream> = if let Some(p) = &proto.def.pointer {
                let mut ptrs: Vec<TokenStream> = p
                    .iter()
                    .map(|p| match p {
                        PointerType::Normal => {
                            quote! {*mut }
                        }
                        PointerType::ConstPointer => {
                            quote! {*const }
                        }
                    })
                    .collect();
                if proto.def.is_const && ptrs.len() > 0 {
                    let length = ptrs.len();
                    ptrs[length - 1] = quote! {*const };
                }
                ptrs
            } else {
                vec![]
            };
            (
                quote! {
                    #(#args,)*
                },
                Some(quote! {#(#pointer_defs)* #return_arg}),
            )
        }
    }
}

fn build_command_block(collection: &HashMap<String, APICommand>) -> Vec<TokenStream> {
    collection
        .iter()
        .map(|(name, command)| {
            let ident = format_ident!("PFN_{}", name.as_str());
            let (arg_block, return_block) =
                build_function_block(&command.proto, command.arguments.as_slice());
            match return_block {
                None => {
                    quote! {
                        #[allow(non_camel_case_types)]
                        pub type #ident = unsafe extern "system" fn (#arg_block);
                    }
                }
                Some(return_block) => {
                    quote! {
                        #[allow(non_camel_case_types)]
                        pub type #ident = unsafe extern "system" fn (#arg_block) -> #return_block;
                    }
                }
            }
        })
        .collect()
}

fn parse_number_major_minor(version: &str) -> (u16, u16) {
    let r = Regex::new(r"([0-9]+).([0-9]+)").unwrap();
    let version = r
        .captures(version)
        .expect("failed to capture version string");
    (
        version.get(1).unwrap().as_str().parse().unwrap(),
        version.get(2).unwrap().as_str().parse().unwrap(),
    )
}

fn construct_context(registry: &Registry) -> Context {
    let mut command_cache: HashMap<String, APICommand> = HashMap::default();
    let mut feature_cache: HashMap<APIName, APIGroup> = HashMap::default();
    let _extension_cache: HashMap<String, APIGroup> = HashMap::default();
    let mut enum_cache: HashMap<String, HashSet<APIEnum>> = HashMap::default();
    let mut bitflag_cache: HashMap<String, HashSet<APIEnum>> = HashMap::default();
    let mut constant_map: HashMap<String, APIEnum> = HashMap::default();

    for enums in registry
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Enums(e) => Some(e),
            _ => None,
        })
        .into_iter()
    {
        let is_bitmask = match &enums.enum_type {
            None => false,
            Some(en) => en.eq("bitmask"),
        };
        let base_name = enums.group.as_ref();

        for child in &enums.children {
            if let EnumsChild::Enum(e) = child {
                let names = [base_name, e.group.as_ref()];
                for nn in names.iter() {
                    if let Some(test) = nn {
                        if test.as_str().eq("TransformFeedbackTokenNV") {
                            // TODO: skip for now
                            continue;
                        }
                        for n in test.split(",") {
                            let api_enum = APIEnum {
                                name: e.name.to_string(),
                                constant: parse_constant(e.value.as_ref().unwrap().as_str())
                                    .finish()
                                    .expect("failed to parse constant")
                                    .1,
                            };

                            // treat PathFontStyle as a bitflat
                            (if is_bitmask || n.eq("PathFontStyle") {
                                &mut bitflag_cache
                            } else {
                                &mut enum_cache
                            })
                            .entry(n.trim().to_string())
                            .or_insert(HashSet::default())
                            .insert(api_enum.clone());
                            constant_map.insert(api_enum.name.clone(), api_enum.clone());
                        }
                    }
                }
            }
        }
    }
    for commands in registry
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Commands(e) => Some(e),
            _ => None,
        })
        .into_iter()
    {
        for c in &commands.children {
            command_cache.insert(
                c.proto.definition.name.clone(),
                APICommand {
                    name: c.proto.definition.name.to_string(),
                    proto: APIProto {
                        def: {
                            let result =
                                parse_proto::<VerboseError<&str>>(c.proto.definition.code.as_str());
                            match result.finish() {
                                Ok(def) => def.1.clone(),
                                Err(error) => {
                                    panic!(
                                        "failed to parse {}: {}",
                                        c.proto.definition.code.as_str(),
                                        error
                                    );
                                }
                            }
                        },
                        group: c.proto.group.as_ref().map(|i| i.clone()),
                    },
                    arguments: c
                        .params
                        .iter()
                        .map(|it| {
                            let group_name = match it.group.as_ref() {
                                None => None,
                                Some(group_name) => {
                                    if enum_cache.contains_key(group_name.as_str())
                                        || bitflag_cache.contains_key(group_name.as_str())
                                    {
                                        Some(group_name)
                                    } else {
                                        None
                                    }
                                }
                            };
                            let result =
                                parse_argument::<VerboseError<&str>>(it.definition.code.as_str());
                            match result.finish() {
                                Ok(def) => APIArgument {
                                    group: group_name.map(|i| i.clone()),
                                    name: it.definition.name.clone(),
                                    def: def.1,
                                },
                                Err(error) => {
                                    panic!("failed to parse {}: {}", it.definition.code, error);
                                }
                            }
                        })
                        .collect(),
                },
            );
        }
    }

    for features in registry
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Features(e) => Some(e),
            _ => None,
        })
        .into_iter()
    {
        if let Some(api) = &features.api {
            // println!("feature api: {} name: {}", api.as_str(), features.name.as_ref().map_or( "UNKNOWN", |a|  a.as_str()));
            match api.as_str() {
                "gl" => {
                    let (major, minor) = parse_number_major_minor(
                        features
                            .number
                            .as_ref()
                            .expect("feature version is missing"),
                    );
                    let api = APIName::OPENGL { major, minor };
                    let api_group = feature_cache.entry(api).or_insert(APIGroup {
                        commands: HashSet::default(),
                        groups: features.children.clone()
                    });

                    for feature in &features.children {
                        match feature {
                            ExtensionChild::Require { items, .. } => {
                                for it in items {
                                    match it {
                                        InterfaceItem::Enum(_) => {}
                                        InterfaceItem::Type { .. } => {}
                                        InterfaceItem::Command { name, comment: _ } => {
                                            api_group.commands.insert(name.to_string());
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            ExtensionChild::Removed { .. } => {}
                            _ => {}
                        }
                    }
                }
                "gles2" => {}
                "gles1" => {}
                api => {
                    println!("unhandled api {}", api)
                }
            }
        }
    }
    for extension in registry
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Extensions(e) => Some(e),
            _ => None,
        })
        .into_iter()
    {
        for child_ext in &extension.children {
            for child in &child_ext.children {
                match child {
                    ExtensionChild::Require { items, api: _, .. } => {
                        for entry in items {
                            match entry {
                                InterfaceItem::Enum(_) => {}
                                InterfaceItem::Type { .. } => {}
                                InterfaceItem::Command {
                                    name: _,
                                    comment: _,
                                } => {}
                                _ => {}
                            }
                        }
                    }
                    ExtensionChild::Removed { .. } => {}
                    _ => {}
                }
            }
        }
    }

    Context {
        constant_map,
        bitmap_cache: bitflag_cache,
        enum_cache,
        command_cache,
        feature_cache,
        extension_cache: Default::default(),
    }
}

pub fn write_source_code<P: AsRef<Path>>(headers_dir: &Path, src_dir: P) {
    write_gl(headers_dir, PathBuf::from(src_dir.as_ref()));
    write_glx(headers_dir, PathBuf::from(src_dir.as_ref()));
    write_wgl(headers_dir, PathBuf::from(src_dir.as_ref()));
}

fn write_glx(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/glx.xml");

    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let mut context = construct_context(&spec);

    let enum_codes: Vec<TokenStream> = build_enum_block(&context.enum_cache);
    let bitflag_codes: Vec<TokenStream> = build_bitflag_block(&context.bitmap_cache);
    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);

    let enum_code = quote! {
        use std::fmt;
        #(#enum_codes)*
    };

    let bitflag_code = quote! {
        use std::fmt;
        #(#bitflag_codes)*
    };

    let command_code = quote! {
        use crate::glx;
        use std::fmt;
        use std::ffi::c_void;
        use crate::types::*;
        use glx::enums::*;
        use glx::bitflags::*;
        use glx::types::*;
        #(#command_codes)*
    };

    let mut glx_path = PathBuf::from(output);
    glx_path.push("glx");

    let mut bitflag_file = File::create(glx_path.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(glx_path.join("enums.rs")).expect("enums.rs");
    let mut command_file = File::create(glx_path.join("command.rs")).expect("command.rs");

    write!(&mut bitflag_file, "{}", bitflag_code).expect("Unable to write bitflags.rs");
    write!(&mut enum_file, "{}", enum_code).expect("Unable to write enums.rs");
    write!(&mut command_file, "{}", command_code).expect("Unable to write command.rs");
}

// helper utility just for generating the start of the wrapper
// fn helper_generate_start_wrapper(cmd: &APICommand) -> TokenStream {
//
//     let api_name = format_ident!("{}", cmd.as_str());
//     let (arg_block, return_block) = build_function_block(&command.proto, command.arguments.as_slice());
//
//     let args: Vec<TokenStream> = cmd.arguments.iter()
//         .map(|p| format_ident!("{}", p.name.as_str()))
//         .collect();
//
//     match return_block {
//         None => {
//             quote! {
//                 pub unsafe fn #api_name(#arg_block) {
//                    (self.#api_name)(#(#args,));
//                 }
//             }
//         }
//         Some(return_block) => {
//             quote!{
//                 pub unsafe fn #api_name(#arg_block) -> #return_block {
//                     (self.#api_name)(#(#args,))
//                 }
//             }
//         }
//     }
//
// }

fn write_wgl(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/wgl.xml");

    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let context = construct_context(&spec);

    let enum_codes: Vec<TokenStream> = build_enum_block(&context.enum_cache);
    let bitflag_codes: Vec<TokenStream> = build_bitflag_block(&context.bitmap_cache);
    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);

    let enum_code = quote! {
        use std::fmt;
        #(#enum_codes)*
    };

    let bitflag_code = quote! {
        use std::fmt;
        #(#bitflag_codes)*
    };

    let command_code = quote! {
        use crate::wgl;
        use std::fmt;
        use std::ffi::c_void;
        use crate::types::*;
        use wgl::enums::*;
        use wgl::bitflags::*;
        use wgl::types::*;
        #(#command_codes)*
    };

    let mut wgl_path = PathBuf::from(output);
    wgl_path.push("wgl");

    let mut bitflag_file = File::create(wgl_path.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(wgl_path.join("enums.rs")).expect("enums.rs");
    let mut command_file = File::create(wgl_path.join("command.rs")).expect("command.rs");

    write!(&mut bitflag_file, "{}", bitflag_code).expect("Unable to write bitflags.rs");
    write!(&mut enum_file, "{}", enum_code).expect("Unable to write enums.rs");
    write!(&mut command_file, "{}", command_code).expect("Unable to write command.rs");
}

fn write_gl(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/gl.xml");

    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let mut context = construct_context(&spec);

    let enum_codes: Vec<TokenStream> = build_enum_block(&context.enum_cache);
    let bitflag_codes: Vec<TokenStream> = build_bitflag_block(&context.bitmap_cache);
    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);

    let mut gl_path = PathBuf::from(output);
    gl_path.push("gl");

    #[derive(Clone, PartialEq, Eq)]
    struct GLGroup {
        major: u16,
        minor: u16,
        commands: HashSet<String>
    }
    let mut gl_group: Vec<GLGroup> = Vec::new();
    let mut cmd_features: HashMap<String, HashSet<String>> = HashMap::default();

    let mut commands: HashSet<String> = HashSet::default();
    for (major, minor) in context.feature_cache.keys().into_iter().filter_map(|it| match it {
        APIName::OPENGL { major, minor } => { Some((major, minor))}
        _ => None
    }).sorted_by(|(major1, minor1), (major2, minor2)| {
        let mut order = Ord::cmp(major1, major2);
        if order == Ordering::Equal {
            order = Ord::cmp(minor1, minor2)
        }
        order
    }) {
        let api = APIName::OPENGL { major: *major, minor: *minor};
        let feature = &context.feature_cache[&api];
        for extension in &feature.groups {
            match extension {
                ExtensionChild::Require { items, .. } => {
                    for it in items {
                        match it {
                            InterfaceItem::Enum(_) => {}
                            InterfaceItem::Type { .. } => {}
                            InterfaceItem::Command { name, .. } => {
                                commands.insert(name.clone());
                            }
                            _ => {}
                        }
                    }
                }
                ExtensionChild::Removed { items, ..} => {
                    for it in items {
                        match it {
                            InterfaceItem::Enum(_) => {}
                            InterfaceItem::Type { .. } => {}
                            InterfaceItem::Command { name, .. } => {
                                commands.remove(name);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        for cmd in commands.iter() {
            cmd_features.entry(cmd.clone())
                .or_insert(HashSet::default())
                .insert(format!("gl{}{}", *major, *minor));
        }

        gl_group.push(GLGroup {
            major: *major,
            minor: *minor,
            commands: commands.clone()
        });
    }

    for g in &gl_group {
        let method_block: Vec<TokenStream> = g.commands.iter().map(|cmd| {
            let command = &context.command_cache[cmd.as_str()];
            let api_name = format_ident!("{}", cmd.as_str());
            let (arg_block, return_block) =
                build_function_block(&command.proto, command.arguments.as_slice());
            let args: Vec<TokenStream> = command
                .arguments
                .iter()
                .map(|p| {
                    let name = format_ident!("_{}", p.name.as_str());
                    quote!{ #name }
                })
                .collect();

            match return_block {
                None => {
                    quote! {
                        unsafe fn #api_name(&self,#arg_block) {
                             (self.entry().#api_name)(#(#args,)*)
                        }
                    }
                }
                Some(return_block) => {
                    quote! {
                         unsafe fn #api_name(&self,#arg_block) -> #return_block{
                            (self.entry().#api_name)(#(#args,)*)
                        }
                    }
                }
            }
        }).collect();
        let api_name = format_ident!("GL{}{}", g.major, g.minor);
        let function_codes = quote! {
            use crate::gl;
            use std::fmt;
            use std::ffi::c_void;
            use crate::types::*;
            use gl::enums::*;
            use gl::bitflags::*;
            use crate::gl::feature::EntryGLFn;

            pub trait #api_name {
                unsafe fn entry(&self) -> &EntryGLFn;

                #(#method_block)*
            }
        };

        let mut function_file = File::create(gl_path.join(format!("gl{}{}", g.major, g.minor)).join("api.rs")).expect("functions.rs");
        write!(&mut function_file, "{}", function_codes).expect("Unable to write command.rs");

    }

    fn construct_features(features: &HashSet<String>) -> TokenStream {
        let collection: Vec<TokenStream> = features.iter().map(|it| {
            let name = it.as_str();
            quote! { feature = #name }
        }).collect();
        quote! {
            #[cfg(any( #(#collection,)* ))]
        }
    }


    // for (name, features) in
    let properties: Vec<TokenStream> = cmd_features.iter().map(|(name, features)| {
        let feature_codes = construct_features(features);
        let command_name = format_ident!("{}", name.as_str());
        let command_type = format_ident!("PFN_{}", name.as_str());
        quote! {
            #feature_codes
            pub #command_name : crate::gl::command::#command_type
        }
    }).collect();

    let impl_block: Vec<TokenStream> = cmd_features.iter().map(|(name, features)| {
        let feature_codes = construct_features(features);
        let command = &context.command_cache[name.as_str()];
        let api_name = format_ident!("{}", name.as_str());
        let internal_api_catch = format_ident!("__{}", name.as_str());
        let (arg_block, return_block) =
            build_function_block(&command.proto, command.arguments.as_slice());
        let byte_lit = LitByteStr::new(
            format!("{}\0", name.as_str()).as_bytes(),
            Span::call_site(),
        );
        let panic_message = syn::LitStr::new(
            format!("Unable to load {}", name.as_str()).as_str(),
            Span::call_site(),
        );

        let function_block = match return_block {
            None => {
                quote! {
                   (#arg_block)
                }
            }
            Some(return_block) => {
                quote! { (#arg_block) -> #return_block }
            }
        };

        quote! {
            #feature_codes
            #api_name : unsafe {
                unsafe extern "system" fn #internal_api_catch #function_block {
                    panic!(#panic_message)
                }
                let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
                    #byte_lit
                );
                let val = _f(cname);
                if val.is_null() {
                    #internal_api_catch
                } else {
                    ::std::mem::transmute(val)
                }
            }
        }
    }).collect();
    let feature_quote = quote! {
         #[derive(Clone)]
        pub struct EntryGLFn {
             #(#properties,)*
        }
        impl EntryGLFn {
            pub fn load<F>(mut _f: F) -> Self
                where
                    F: FnMut(&::std::ffi::CStr) -> *const c_void {
                    Self {
                        #(#impl_block,)*
                    }
            }
        }
    };

    // let mut features_codes: Vec<TokenStream> = Vec::new();
    // for (name, group) in &context.feature_cache {
    //     match name {
    //         APIName::OPENGL { minor, major } => {
    //             let api_name = format_ident!("EntryFnGL{}{}", major, minor);
    //             let properties: Vec<TokenStream> = group
    //                 .commands
    //                 .iter()
    //                 .map(|cmd| {
    //                     let command_name = format_ident!("{}", cmd.as_str());
    //                     let command_type = format_ident!("PFN_{}", cmd.as_str());
    //                     quote! {
    //                         #command_name : crate::gl::command::#command_type
    //                     }
    //                 })
    //                 .collect();
    //
    //             let method_block: Vec<TokenStream> = group
    //                 .commands
    //                 .iter()
    //                 .map(|cmd| {
    //                     let command = &context.command_cache[cmd.as_str()];
    //                     let api_name = format_ident!("{}", cmd.as_str());
    //                     let (arg_block, return_block) =
    //                         build_function_block(&command.proto, command.arguments.as_slice());
    //
    //                     let args: Vec<TokenStream> = command
    //                         .arguments
    //                         .iter()
    //                         .map(|p| {
    //                             let name = format_ident!("_{}", p.name.as_str());
    //                             quote!{ #name }
    //                         })
    //                         .collect();
    //
    //                     match return_block {
    //                         None => {
    //                             quote! {
    //                                 pub unsafe fn #api_name(&self,#arg_block) {
    //                                    (self.#api_name)(#(#args,)*);
    //                                 }
    //                             }
    //                         }
    //                         Some(return_block) => {
    //                             quote! {
    //                                 pub unsafe fn #api_name(&self,#arg_block) -> #return_block {
    //                                     (self.#api_name)(#(#args,)*)
    //                                 }
    //                             }
    //                         }
    //                     }
    //                 })
    //                 .collect();
    //
    //             let impl_block: Vec<TokenStream> = group
    //                 .commands
    //                 .iter()
    //                 .map(|cmd| {
    //                     let command = &context.command_cache[cmd.as_str()];
    //                     let api_name = format_ident!("{}", cmd.as_str());
    //                     let byte_lit = LitByteStr::new(
    //                         format!("{}\0", cmd.as_str()).as_bytes(),
    //                         Span::call_site(),
    //                     );
    //                     let panic_message = syn::LitStr::new(
    //                         format!("Unable to load {}", cmd.as_str()).as_str(),
    //                         Span::call_site(),
    //                     );
    //
    //                     let internal_api_catch = format_ident!("__{}", cmd.as_str());
    //                     let (arg_block, return_block) =
    //                         build_function_block(&command.proto, command.arguments.as_slice());
    //
    //                     let function_block = match return_block {
    //                         None => {
    //                             quote! {
    //                                (#arg_block)
    //                             }
    //                         }
    //                         Some(return_block) => {
    //                             quote! { (#arg_block) -> #return_block }
    //                         }
    //                     };
    //
    //                     quote! {
    //                         #api_name : unsafe {
    //                             unsafe extern "system" fn #internal_api_catch #function_block {
    //                                 panic!(#panic_message)
    //                             }
    //                             let cname = ::std::ffi::CStr::from_bytes_with_nul_unchecked(
    //                                 #byte_lit
    //                             );
    //                             let val = _f(cname);
    //                             if val.is_null() {
    //                                 #internal_api_catch
    //                             } else {
    //                                 ::std::mem::transmute(val)
    //                             }
    //                         }
    //                     }
    //                 })
    //                 .collect();
    //
    //             let feature_block: TokenStream = quote! {
    //                 #[derive(Clone)]
    //                 pub struct #api_name {
    //                      #(#properties,)*
    //                 }
    //                 impl #api_name {
    //                     pub fn load<F>(mut _f: F) -> Self
    //                     where
    //                         F: FnMut(&::std::ffi::CStr) -> *const c_void {
    //                         Self {
    //                             #(#impl_block,)*
    //                         }
    //                     }
    //
    //                     #(#method_block)*
    //                 }
    //             };
    //             features_codes.push(feature_block);
    //         }
    //         APIName::Unknown => {}
    //         _ => {}
    //     }
    // }

    let enum_code = quote! {
        use std::fmt;
        #(#enum_codes)*
    };

    let bitflag_code = quote! {
        use std::fmt;
        #(#bitflag_codes)*
    };

    let command_code = quote! {
        use crate::gl;
        use std::fmt;
        use std::ffi::c_void;
        use crate::types::*;
        use gl::enums::*;
        use gl::bitflags::*;
        #(#command_codes)*
    };

    let feature_code = quote! {
        use crate::gl;
        use crate::types::*;
        use gl::command::*;
        use gl::enums::*;
        use gl::bitflags::*;

        use std::ffi::c_void;

        #feature_quote
    };

    // let entry_code = quote! {
    //     use crate::gl;
    //     use crate::types::*;
    //     use gl::command::*;
    //     use gl::enums::*;
    //     use gl::bitflags::*;
    //
    //     use std::ffi::c_void;
    //
    //     #feature_code
    // };

    // let mut gl_enums_file = File::create(gl_dir.join("enums.rs")).expect("enums.rs");
    let mut bitflag_file = File::create(gl_path.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(gl_path.join("enums.rs")).expect("enums.rs");
    let mut command_file = File::create(gl_path.join("command.rs")).expect("command.rs");
    let mut feature_file = File::create(gl_path.join("feature.rs")).expect("feature.rs");
    let mut features_file = File::create(gl_path.join("feature.rs")).expect("feature.rs");

    // write!(&mut gl_enums_file, "{}", enum_code).expect("Unable to write enums.rs");
    write!(&mut bitflag_file, "{}", bitflag_code).expect("Unable to write bitflag.rs");
    write!(&mut enum_file, "{}", enum_code).expect("Unable to write bitflag.rs");
    write!(&mut command_file, "{}", command_code).expect("Unable to write command.rs");
    write!(&mut features_file, "{}", feature_code).expect("Unable to write command.rs");

    // // generate bindings
    // let mut bindings = bindgen::Builder::default();
    // let mut include_path =  PathBuf::from(opengl_registry);
    // include_path.push("api");
    // bindings = bindings.header(include_path.join("GL/glcorearb.h").to_str().unwrap());
    // bindings.generate()
    //     .expect("Unable to generate native bindings")
    //     .write_to_file(gl_path.join("natives.rs"))
    //     .expect("Couln't write native bindings");
}
