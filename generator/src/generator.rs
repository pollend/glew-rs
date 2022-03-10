use itertools::{join, min, Itertools};
use khronos_registry_parse::gl::{
    Command, CommandParam, Enum, Enums, EnumsChild, ExtensionChild, InterfaceItem, Registry,
    RegistryChild,
};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, hex_digit1, one_of, u16};
use nom::combinator::{complete, map, map_parser, map_res, opt, recognize, value};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::{Finish, IResult};
use proc_macro2::{Group, Ident, Punct, Spacing, Span, TokenStream};
use quote::{format_ident, quote, ToTokens, TokenStreamExt};
use regex::Regex;
use std::borrow::{Borrow, BorrowMut};
use std::cmp;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::token::Const;

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
    _type: String,
    name: String,
}

struct APICommand {
    _return: Option<String>,
    name: String,
    arguments: Vec<APIArgument>,
}

struct APIGroup {
    commands: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum APIName {
    OPENGL { minor: u16, major: u16 },
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
    extension_cache: HashMap<APIName, APIGroup>,
}

fn construct_context(registry: &Registry) -> Context {
    let mut command_cache: HashMap<String, APICommand> = HashMap::default();
    let mut feature_cache: HashMap<APIName, APIGroup> = HashMap::default();
    let mut extension_cache: HashMap<APIName, APIGroup> = HashMap::default();

    let mut enum_cache: HashMap<String, HashSet<APIEnum>> = HashMap::default();
    let mut bitmap_cache: HashMap<String, HashSet<APIEnum>> = HashMap::default();

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
                        for n in test.split(",") {
                            (if is_bitmask {
                                &mut bitmap_cache
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
            let type_name = &c.proto.type_name;
            command_cache.insert(
                c.proto.name.clone(),
                APICommand {
                    _return: match type_name.as_ref().map(|a| a.as_str()) {
                        Some("struct _cl_context") => Some("CLContext".to_string()),
                        Some("struct _cl_event") => Some("CLEvent".to_string()),
                        Some(i) => Some(i.to_string()),
                        None => None,
                    },
                    name: c.proto.name.to_string(),
                    arguments: c
                        .params
                        .iter()
                        .map(|it| {
                            let stage_type = it
                                .group
                                .as_ref()
                                .or_else(|| it.definition.type_name.as_ref());
                            APIArgument {
                                _type: match stage_type.map(|a| a.as_str()) {
                                    Some("struct _cl_context") => "CLContext".to_string(),
                                    Some("struct _cl_event") => "CLEvent".to_string(),
                                    Some(i) => i.to_string(),
                                    None => "GLvoid".to_string(),
                                },
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
            println!("feature api: {} name: {}", api.as_str(), features.name.as_ref().map_or( "UNKNOWN", |a|  a.as_str()));
            if api.eq("gl") {
                let r = Regex::new(r"([0-9]+).([0-9]+)").unwrap();
                let version = features.number.as_ref().expect("feature version is missing");
                let version = r.captures(version.as_str()).unwrap();
                let api = APIName::OPENGL {
                    major: version.get(1).unwrap().as_str().parse().unwrap(),
                    minor: version.get(2).unwrap().as_str().parse().unwrap(),
                };
                let api_group = feature_cache
                    .entry(api)
                    .or_insert(APIGroup { commands: vec![] });

                for feature in &features.children {
                    match feature {
                        ExtensionChild::Require { items, .. } => {
                            for it in items {
                                match it {
                                    InterfaceItem::Enum(_) => {}
                                    InterfaceItem::Type { .. } => {}
                                    InterfaceItem::Command { name, comment } => {
                                        api_group.commands.push(name.to_string());
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
        }
    }
    for features in registry
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Extensions(e) => Some(e),
            _ => None,
        })
        .into_iter()
    {}

    Context {
        bitmap_cache,
        enum_cache,
        command_cache,
        feature_cache,
        extension_cache: Default::default(),
    }
}

struct Feature {}
//
// impl Argument {
//     fn arguments(cmd: &Command) -> Vec<Argument> {
//         cmd.params
//             .iter()
//             .map(|p| {
//                 // let parameter = format_ident!("_{}",  p.definition.name.clone()).to_token_stream();
//                 let stage_type = p
//                     .group
//                     .as_ref()
//                     .or_else(|| p.definition.type_name.as_ref())
//                     .clone();
//                 let parameter_type = match stage_type.map_or(None, |a| Some(a.as_str())) {
//                     Some("struct _cl_context") => "CLContext".to_string(),
//                     Some("struct _cl_event") => "CLEvent".to_string(),
//                     Some(i) => i.to_string(),
//                     None => "GLvoid".to_string(),
//                 };
//                 Argument {
//                     parameter_type,
//                     parameter: format!("_{}", p.definition.name.clone()),
//                 }
//             })
//             .collect()
//     }
// }
//
// impl quote::ToTokens for Argument {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         tokens.append(Ident::new(self.parameter.as_str(), Span::call_site()));
//         tokens.append(Punct::new(':', Spacing::Alone));
//         tokens.append(Ident::new(self.parameter_type.as_str(), Span::call_site()));
//     }
// }

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
    let gl_xml = headers_dir.join("xml/gl.xml");
    let glx_xml = headers_dir.join("xml/glx.xml");
    let wgl_xml = headers_dir.join("xml/wgl.xml");

    write_gl(headers_dir, PathBuf::from(src_dir.as_ref()));
}

fn write_gl(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/gl.xml");

    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let context = construct_context(&spec);

    let enum_codes: Vec<TokenStream> = context.enum_cache.iter().map(|(name, enums)| {
        let impl_enum: Vec<TokenStream> = enums.iter().map(|c| {
            let name = format_ident!("{}", &c.name);
            let value = &c.constant;
            quote! {
                pub const #name: Self = Self(#value);
            }
        }).collect();
        let ident = format_ident!("{}", &name.as_str());
        quote! {
            #[repr(transparent)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            // #[doc = #khronos_link]
            pub struct #ident(pub(crate) u64);
            // vk_bitflags_wrapped!(#ident, u64);
            impl #ident {
                #(#impl_enum)*
            }
        }
    }).collect();

    let bitflag_codes: Vec<TokenStream>  = context.bitmap_cache.iter().map(|(name, enums)| {
        let impl_bitflags: Vec<TokenStream> = enums.iter().map(|c| {
            let name = format_ident!("{}", &c.name);
            let value = &c.constant;
            quote! {
                pub const #name: Self = Self(#value);
            }
        }).collect();
        let ident = format_ident!("{}", &name.as_str());
        quote! {
            #[repr(transparent)]
            #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
            // #[doc = #khronos_link]
            pub struct #ident(pub(crate) u64);
            // vk_bitflags_wrapped!(#ident, u64);
            impl #ident {
                #(#impl_bitflags)*
            }
        }
    }).collect();


    for (name , group) in &context.feature_cache {

    }
    //
    // let extensions = spec
    //     .0
    //     .iter()
    //     .filter_map(|item| match item {
    //         RegistryChild::Extensions(e) => Some(&e.children),
    //         _ => None,
    //     })
    //     .next()
    //     .expect("extension");
    //
    // let mut enum_collection: HashMap<String, HashSet<_Enum>> = HashMap::new();
    // let mut bitmask_collection: HashMap<String, HashSet<_Enum>> = HashMap::new();
    // for enums in spec
    //     .0
    //     .iter()
    //     .filter_map(|item| match item {
    //         RegistryChild::Enums(e) => Some(e),
    //         _ => None,
    //     })
    //     .into_iter()
    // {
    //     let is_bitmask = match &enums.enum_type {
    //         None => false,
    //         Some(en) => en.eq("bitmask"),
    //     };
    //
    //     let base_name = enums.group.as_ref();
    //     for child in &enums.children {
    //         if let EnumsChild::Enum(e) = child {
    //             let handle_enum = |name: &String, coll: &mut HashMap<String, HashSet<_Enum>>| {
    //                 for n in name.split(',') {
    //                     coll.entry(n.trim().to_string())
    //                         .or_insert(HashSet::new())
    //                         .insert(_Enum(e.clone()));
    //                 }
    //             };
    //             if let Some(name) = base_name.or_else(|| e.group.as_ref()) {
    //                 if name.as_str().eq("TransformFeedbackTokenNV") {
    //                     continue;
    //                 }
    //             }
    //
    //             if let Some(name) = base_name {
    //                 handle_enum(
    //                     name,
    //                     (if is_bitmask {
    //                         &mut bitmask_collection
    //                     } else {
    //                         &mut enum_collection
    //                     }),
    //                 );
    //             }
    //             if let Some(name) = e.group.as_ref() {
    //                 handle_enum(
    //                     name,
    //                     (if is_bitmask {
    //                         &mut bitmask_collection
    //                     } else {
    //                         &mut enum_collection
    //                     }),
    //                 );
    //             }
    //         }
    //     }
    // }

    // let mut command_cache: HashMap<String, Vec<Command>> = HashMap::new();
    // let mut command_codes: Vec<TokenStream> = Vec::new();
    // for commands in spec
    //     .0
    //     .iter()
    //     .filter_map(|item| match item {
    //         RegistryChild::Commands(e) => Some(e),
    //         _ => None,
    //     })
    //     .into_iter()
    // {
    //     for c in &commands.children {
    //         command_cache
    //             .entry(c.proto.name.clone())
    //             .or_insert(Vec::new())
    //             .push(c.clone());
    //
    //         let args = Argument::arguments(c);
    //         let method_name = format_ident!("PFN_{}", c.proto.name);
    //         command_codes.push(quote! {
    //              pub type #method_name = unsafe extern "system" fn(#(#args,)*);
    //         });
    //     }
    // }
    //
    // let mut api_functions: HashMap<String, Vec<TokenStream>> = HashMap::new();
    // for features in spec
    //     .0
    //     .iter()
    //     .filter_map(|item| match item {
    //         RegistryChild::Features(e) => Some(e),
    //         _ => None,
    //     })
    //     .into_iter()
    // {
    //     let feature_name = features.name.as_ref().unwrap();
    //     for feature in &features.children {
    //         match feature {
    //             ExtensionChild::Require { items, .. } => {
    //                 for it in items {
    //                     match it {
    //                         InterfaceItem::Enum(_) => {}
    //                         InterfaceItem::Type { .. } => {}
    //                         InterfaceItem::Command { name, comment } => {
    //                             println!("cmd name: {}", name.as_str());
    //                             for cmd in &command_cache[name] {
    //                                 let method_name = format_ident!("{}", cmd.proto.name);
    //                                 let command_type = format_ident!("PFN_{}", cmd.proto.name);
    //                                 api_functions
    //                                     .entry(feature_name.clone())
    //                                     .or_insert(Vec::new())
    //                                     .push(quote! {
    //                                         pub #method_name : crate::gl::command::#command_type
    //                                     });
    //                             }
    //                         }
    //                         _ => {}
    //                     }
    //                 }
    //             }
    //             ExtensionChild::Removed { .. } => {}
    //             _ => {}
    //         }
    //     }
    // }

    // let mut enum_codes: Vec<TokenStream> = enum_collection
    //     .into_iter()
    //     .map(|(key, enums)| {
    //         let ident = format_ident!("{}", key.as_str());
    //         let impl_enum = enums.iter().map(|c| {
    //             // let value = Constant::Hex(c.value.as_ref().unwrap().clone());
    //             // let value =
    //             let name = format_ident!("{}", &c.0.name);
    //             let value = parse_constant(c.0.value.as_ref().unwrap())
    //                 .finish()
    //                 .expect("failed to parse constant")
    //                 .1;
    //             quote! {
    //                 pub const #name: Self = Self(#value);
    //             }
    //         });
    //         quote! {
    //             #[repr(transparent)]
    //             #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    //             // #[doc = #khronos_link]
    //             pub struct #ident(pub(crate) u64);
    //             // vk_bitflags_wrapped!(#ident, u64);
    //             impl #ident {
    //                 #(#impl_enum)*
    //             }
    //         }
    //     })
    //     .collect();
    //
    // let mut bitflag_codes: Vec<TokenStream> = bitmask_collection
    //     .into_iter()
    //     .map(|(key, enums)| {
    //         let ident = format_ident!("{}", key.as_str());
    //         let impl_enum = enums.iter().map(|c| {
    //             // let value = Constant::Hex(c.value.as_ref().unwrap().clone());
    //             // let value =
    //             let name = format_ident!("{}", &c.0.name);
    //             let value = parse_constant(c.0.value.as_ref().unwrap())
    //                 .finish()
    //                 .expect("failed to parse constant")
    //                 .1;
    //             quote! {
    //                 pub const #name: Self = Self(#value);
    //             }
    //         });
    //         quote! {
    //             #[repr(transparent)]
    //             #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    //             // #[doc = #khronos_link]
    //             pub struct #ident(pub(crate) u64);
    //             // vk_bitflags_wrapped!(#ident, u64);
    //             impl #ident {
    //                 #(#impl_enum)*
    //             }
    //         }
    //     })
    //     .collect();
    //
    // let api_implementation_code = api_functions.iter().map(|(key, value)| {
    //     let ident = format_ident!("{}", key);
    //     quote! {
    //         #[derive(Clone)]
    //         pub struct #ident {
    //              #(#value,)*
    //         }
    //     }
    // });

    let enum_code = quote! {
        use std::fmt;
        #(#enum_codes)*
    };

    let bitflag_code = quote! {
        use std::fmt;
        #(#bitflag_codes)*
    };

    let command_code = quote! {
        use std::fmt;
        use crate::gl;
        use std::ffi::c_void;
        use gl::types::*;
        use gl::enums::*;
        use gl::bitflags::*;
        // #(#command_codes)*
    };

    let feature_code = quote! {
        use crate::gl;
        use gl::command::*;

        // #(#api_implementation_code)*
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
