use itertools::{join, min, Itertools};
use khronos_registry_parse::gl::{
    Command, CommandParam, Enum, Enums, EnumsChild, ExtensionChild, InterfaceItem, Registry,
    RegistryChild,
};
use nom::branch::{alt, permutation};
use nom::bytes::complete::{tag, take_while};
use nom::character::complete::{alphanumeric1, char, hex_digit1, one_of, u16};
use nom::combinator::{complete, map, map_parser, map_res, opt, recognize, value};
use nom::error::{context, ParseError};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::Err::Error;
use nom::{Finish, IResult, Parser};
use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use regex::Regex;
use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::slice::Iter;
use syn::__private::bool;
use syn::spanned::Spanned;
use syn::token::Const;
use syn::{LitByteStr, LitStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Constant {
    Number {
        sign: Option<String>,
        number: String,
    },
    Hex(String),
}

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
    _type: TokenStream,
    name: String,
    is_array: bool,
}

struct APICommand {
    _return: Option<TokenStream>,
    name: String,
    arguments: Vec<APIArgument>,
}

struct APIGroup {
    commands: HashSet<String>,
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

fn construct_arguments(args: &[APIArgument]) -> Vec<TokenStream> {
    args.iter()
        .map(|cmd| {
            let arg_type = &cmd._type;
            let name = format_ident!("_{}", cmd.name.as_str());
            if cmd.is_array {
                quote! { #name : *mut #arg_type }
            } else {
                quote! { #name : #arg_type }
            }
        })
        .collect()
}

fn build_enum_block(collection: &HashMap<String, HashSet<APIEnum>>) -> Vec<TokenStream> {
    collection
        .iter()
        .map(|(name, enums)| {
            let e: Vec<&APIEnum> = enums.iter().collect();
            let impl_enum: Vec<TokenStream> = construct_const(e.as_slice());
            let ident = format_ident!("{}", &name);
            quote! {
                #[repr(transparent)]
                #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
                pub struct #ident(pub(crate) u64);
                impl #ident {
                   #(#impl_enum)*
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
                pub struct #ident(pub(crate) u64);
                impl #ident {
                   #(#impl_bitflags)*
                }
            }
        })
        .collect()
}

fn build_command_block(collection: &HashMap<String, APICommand>) -> Vec<TokenStream> {
    collection
        .iter()
        .map(|(name, command)| {
            let args: Vec<TokenStream> = construct_arguments(command.arguments.as_slice());
            let ident = format_ident!("PFN_{}", name.as_str());
            match &command._return {
                None => {
                    quote! {
                         #[allow(non_camel_case_types)]
                         pub type #ident = unsafe extern "system" fn(#(#args,)*);
                    }
                }
                Some(return_arg) => {
                    quote! {
                         #[allow(non_camel_case_types)]
                         pub type #ident = unsafe extern "system" fn(#(#args,)*) -> #return_arg;
                    }
                }
            }
        })
        .collect()
}

fn map_return_type(arg_type: Option<&str>) -> Option<TokenStream> {
    match arg_type {
        Some("struct _cl_context") => Some(quote! {CLContext}),
        Some("struct _cl_event") => Some(quote! {CLEvent}),
        Some("int64_t") => Some(quote! {u64}),
        Some("int32_t") => Some(quote! {u32}),
        Some("Bool") => Some(quote! {bool}),
        Some(i) => Some(format_ident!("{}", i).to_token_stream()),
        None => None,
    }
}
fn map_arg_type(arg_type: Option<&str>) -> TokenStream {
    let arg_type = map_return_type(arg_type);
    match arg_type {
        Some(e) => e,
        None => quote! {GLvoid},
    }
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
    let mut extension_cache: HashMap<String, APIGroup> = HashMap::default();

    let mut enum_cache: HashMap<String, HashSet<APIEnum>> = HashMap::default();
    let mut bitflag_cache: HashMap<String, HashSet<APIEnum>> = HashMap::default();

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
                            // treat PathFontStyle as a bitflat
                            (if is_bitmask || n.eq("PathFontStyle") {
                                &mut bitflag_cache
                            } else {
                                &mut enum_cache
                            })
                            .entry(n.trim().to_string())
                            .or_insert(HashSet::default())
                            .insert(APIEnum {
                                name: e.name.to_string(),
                                constant: parse_constant(e.value.as_ref().unwrap().as_str())
                                    .finish()
                                    .expect("failed to parse constant")
                                    .1,
                            });
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
                    _return: {
                        match c.proto.group.as_ref().map(|i| i.as_str()) {
                            Some("String") => Some(quote! {*const char}),
                            _ => map_return_type(
                                c.proto.definition.type_name.as_ref().map(|i| i.as_str()),
                            ),
                        }
                    },
                    arguments: c
                        .params
                        .iter()
                        .map(|it| {
                            println!("{}", it.definition.code.as_str());

                            let stage_type = (match it.group.as_ref() {
                                None => it.definition.type_name.as_ref(),
                                Some(group_name) => {
                                    if (enum_cache.contains_key(group_name.as_str())
                                        || bitflag_cache.contains_key(group_name.as_str()))
                                    {
                                        Some(group_name)
                                    } else {
                                        it.definition.type_name.as_ref()
                                    }
                                }
                            });
                            APIArgument {
                                _type: map_arg_type(stage_type.map(|a| a.as_str())),
                                is_array: it.len.is_some(),
                                name: it.definition.name.clone(),
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
                    });

                    for feature in &features.children {
                        match feature {
                            ExtensionChild::Require { items, .. } => {
                                for it in items {
                                    match it {
                                        InterfaceItem::Enum(_) => {}
                                        InterfaceItem::Type { .. } => {}
                                        InterfaceItem::Command { name, comment } => {
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
                    ExtensionChild::Require { items, api, .. } => {
                        for entry in items {
                            match entry {
                                InterfaceItem::Enum(_) => {}
                                InterfaceItem::Type { .. } => {}
                                InterfaceItem::Command { name, comment } => {}
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
        bitmap_cache: bitflag_cache,
        enum_cache,
        command_cache,
        feature_cache,
        extension_cache: Default::default(),
    }
}

struct Feature {}

impl quote::ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Constant::Number {
                ref sign,
                ref number,
            } => {
                let number = interleave_number('_', 4, number.as_str());
                syn::LitInt::new(
                    &format!(
                        "{}{}",
                        match sign.as_ref() {
                            None => {
                                ""
                            }
                            Some(res) => {
                                res
                            }
                        },
                        number
                    ),
                    Span::call_site(),
                )
                .to_tokens(tokens);
            }
            Constant::Hex(ref n) => {
                let number = interleave_number('_', 4, n);
                syn::LitInt::new(&format!("0x{}", number), Span::call_site()).to_tokens(tokens);
            }
        }
    }
}

fn map_type(input: Option<&String>) -> TokenStream {
    match input.map_or_else(|| None, |e| Some(e.as_str())) {
        Some("struct _cl_context") => quote! {*mut c_void},
        Some("struct _cl_event") => quote! {*mut c_void},
        Some(i) => format_ident!("{}", i).to_token_stream(),
        None => quote! {*mut c_void},
    }
}

fn parse_hex(input: &str) -> IResult<&str, Constant> {
    map_res(
        preceded(
            alt((tag("0x"), tag("0X"))),
            recognize(many1(terminated(
                one_of("0123456789abcdefABCDEF"),
                many0(char('_')),
            ))),
        ),
        |out: &str| -> Result<Constant, ()> { Ok(Constant::Hex(out.to_ascii_lowercase())) },
    )(input)
}

fn parse_number(input: &str) -> IResult<&str, Constant> {
    map_res(
        tuple((opt(one_of("+-")), many1(one_of("0123456789")))),
        |(sign, number)| -> Result<Constant, ()> {
            Ok(Constant::Number {
                sign: sign.map(|e| format!("{}", e)),
                number: number.into_iter().collect(),
            })
        },
    )(input)
}

fn parse_constant(i: &str) -> IResult<&str, Constant> {
    let hex = parse_hex(i);
    let number = parse_number(i);
    return hex.or(number);
}


// Interleaves a number, for example 100000 => 100_000. Mostly used to make clippy happy
fn interleave_number(symbol: char, count: usize, n: &str) -> String {
    let number: String = n
        .chars()
        .rev()
        .enumerate()
        .fold(String::new(), |mut acc, (idx, next)| {
            if idx != 0 && idx % count == 0 {
                acc.push(symbol);
            }
            acc.push(next);
            acc
        });
    number.chars().rev().collect()
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
    let context = construct_context(&spec);

    let enum_codes: Vec<TokenStream> = build_enum_block(&context.enum_cache);
    let bitflag_codes: Vec<TokenStream> = build_bitflag_block(&context.bitmap_cache);
    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);

    let mut features_codes: Vec<TokenStream> = Vec::new();
    for (name, group) in &context.feature_cache {
        match name {
            APIName::OPENGL { minor, major } => {
                let api_name = format_ident!("EntryFnGL{}{}", major, minor);
                let properties: Vec<TokenStream> = group
                    .commands
                    .iter()
                    .map(|cmd| {
                        let command_name = format_ident!("{}", cmd.as_str());
                        let command_type = format_ident!("PFN_{}", cmd.as_str());
                        quote! {
                            pub #command_name : crate::gl::command::#command_type
                        }
                    })
                    .collect();

                let impl_block: Vec<TokenStream> = group.commands.iter().map(|cmd| {
                    let command = &context.command_cache[cmd.as_str()];
                    let api_name = format_ident!("{}", cmd.as_str());
                    let internal_api_catch = format_ident!("__{}", cmd.as_str());
                    let args: Vec<TokenStream> = construct_arguments(command.arguments.as_slice());
                    let byte_lit = LitByteStr::new(format!("{}\0", cmd.as_str()).as_bytes(), Span::call_site());
                    let panic_message = syn::LitStr::new(format!("Unable to load {}", cmd.as_str()).as_str(), Span::call_site());
                    let function_code = match &command._return {
                        None => {
                            quote! {
                                unsafe extern "system" fn #internal_api_catch (#(#args,)*) {
                                    panic!(#panic_message)
                                }
                            }
                        }
                        Some(return_arg) => {
                            quote! {
                                unsafe extern "system" fn #internal_api_catch (#(#args,)*) -> #return_arg {
                                    panic!(#panic_message)
                                }
                            }
                        }
                    };

                    quote! {
                        #api_name : unsafe {
                           #function_code
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
                let feature_block: TokenStream = quote! {
                    #[derive(Clone)]
                    pub struct #api_name {
                         #(#properties,)*
                    }
                    impl #api_name {
                        pub fn load<F>(mut _f: F) -> Self
                        where
                            F: FnMut(&::std::ffi::CStr) -> *const c_void {
                            Self {
                                #(#impl_block,)*
                            }
                        }
                    }
                };
                features_codes.push(feature_block);
            }
            APIName::Unknown => {}
            _ => {}
        }
    }

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

        #(#features_codes)*
    };

    let mut gl_path = PathBuf::from(output);
    gl_path.push("gl");

    // let mut gl_enums_file = File::create(gl_dir.join("enums.rs")).expect("enums.rs");
    let mut bitflag_file = File::create(gl_path.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(gl_path.join("enums.rs")).expect("enums.rs");
    let mut command_file = File::create(gl_path.join("command.rs")).expect("command.rs");
    let mut feature_file = File::create(gl_path.join("feature.rs")).expect("feature.rs");

    // write!(&mut gl_enums_file, "{}", enum_code).expect("Unable to write enums.rs");
    write!(&mut bitflag_file, "{}", bitflag_code).expect("Unable to write bitflag.rs");
    write!(&mut enum_file, "{}", enum_code).expect("Unable to write bitflag.rs");
    write!(&mut command_file, "{}", command_code).expect("Unable to write command.rs");
    write!(&mut feature_file, "{}", feature_code).expect("Unable to write command.rs");

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
