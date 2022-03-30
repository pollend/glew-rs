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
use std::cmp::{min, Ordering};

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
use crate::context::{APIArgument, APICommand, APIEnum, APIName, APIProto, collect_unique_enums, construct_context, map_type};
use crate::generator::{build_command_block, build_function_block};


pub fn write_gl(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/gl.xml");

    let root_path = PathBuf::from(&output);
    let gl_path = {
        let mut result = PathBuf::from(&output);
        result.push("gl");
        result
    };


    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let mut context = construct_context(&spec);

    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);

    let unique_enums = collect_unique_enums(&context.enum_cache);
    let unique_bitfield = collect_unique_enums(&context.bitfield_cache);
    let all_enums_const: Vec<TokenStream> = unique_enums
        .iter()
        .map(|c| {
            let name = format_ident!("{}", &c.name);
            let value = &c.constant;
            quote! {
                #[allow(non_upper_case_globals)]
                pub const #name: GLenum = #value;
            }
        })
        .collect();

    let all_bitfield_const: Vec<TokenStream> = unique_bitfield
        .iter()
        .map(|c| {
            let name = format_ident!("{}", &c.name);
            let value = &c.constant;
            quote! {
                #[allow(non_upper_case_globals)]
                pub const #name: GLbitfield = #value;
            }
        })
        .collect();


    #[derive(Clone, PartialEq, Eq)]
    struct GLGroup {
        major: u16,
        minor: u16,
        commands: HashSet<String>,
    }
    let mut gl_group: Vec<GLGroup> = Vec::new();
    let mut gl_cmd_features: HashMap<String, HashSet<String>> = HashMap::default();
    let mut gl_commands: HashSet<String> = HashSet::default();

    for (major, minor) in context
        .feature_cache
        .keys()
        .into_iter()
        .filter_map(|it| match it {
            APIName::OPENGL { major, minor } => Some((major, minor)),
            _ => None,
        })
        .sorted_by(|(major1, minor1), (major2, minor2)| {
            let mut order = Ord::cmp(major1, major2);
            if order == Ordering::Equal {
                order = Ord::cmp(minor1, minor2)
            }
            order
        })
    {
        let api = APIName::OPENGL {
            major: *major,
            minor: *minor,
        };
        let feature = &context.feature_cache[&api];
        for extension in &feature.groups {
            match extension {
                ExtensionChild::Require { items, .. } => {
                    for it in items {
                        match it {
                            InterfaceItem::Enum(_) => {}
                            InterfaceItem::Type { .. } => {}
                            InterfaceItem::Command { name, .. } => {
                                gl_commands.insert(name.clone());
                            }
                            _ => {}
                        }
                    }
                }
                ExtensionChild::Removed { items, .. } => {
                    for it in items {
                        match it {
                            InterfaceItem::Enum(_) => {}
                            InterfaceItem::Type { .. } => {}
                            InterfaceItem::Command { name, .. } => {
                                gl_commands.remove(name);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        for cmd in gl_commands.iter() {
            gl_cmd_features
                .entry(cmd.clone())
                .or_insert(HashSet::default())
                .insert(format!("gl{}{}", *major, *minor));
        }

        gl_group.push(GLGroup {
            major: *major,
            minor: *minor,
            commands: gl_commands.clone(),
        });
    }

    for g in &gl_group {
        let mut method_block: Vec<TokenStream> = vec![];
        for cmd in g.commands.iter() {
            match cmd.as_str() {
                _ => {
                    let command = &context.command_cache[cmd.as_str()];
                    let api_name = format_ident!("{}", cmd.as_str());
                    let (arg_block, return_block) =
                        build_function_block(&command.proto, command.arguments.as_slice());
                    let args: Vec<TokenStream> = command
                        .arguments
                        .iter()
                        .map(|p| {
                            let name = format_ident!("_{}", p.name.as_str());
                            quote! { #name }
                        })
                        .collect();

                    method_block.push(match return_block {
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
                    })
                }
            }
        }
        let api_name = format_ident!("GL{}{}", g.major, g.minor);
        let function_codes = quote! {
            use crate::types::*;
            use crate::gl::feature::EntryGLFn;

            pub trait #api_name {
                unsafe fn entry(&self) -> &EntryGLFn;

                #(#method_block)*
            }
        };

        let mut function_file = File::create(
            gl_path
                .join(format!("gl{}{}", g.major, g.minor))
                .join("api.rs"),
        )
            .expect("functions.rs");
        write!(&mut function_file, "{}", function_codes).expect("Unable to write command.rs");
    }

    fn construct_features(features: &HashSet<String>) -> TokenStream {
        let collection: Vec<TokenStream> = features
            .iter()
            .map(|it| {
                let name = it.as_str();
                quote! { feature = #name }
            })
            .collect();
        quote! {
            #[cfg(any( #(#collection,)* ))]
        }
    }
    // for (name, features) in
    let properties: Vec<TokenStream> = gl_cmd_features
        .iter()
        .map(|(name, features)| {
            let feature_codes = construct_features(features);
            let command_name = format_ident!("{}", name.as_str());
            let command_type = format_ident!("PFN_{}", name.as_str());
            quote! {
                #feature_codes
                pub #command_name : crate::gl::command::#command_type
            }
        })
        .collect();

    let impl_block: Vec<TokenStream> = gl_cmd_features
        .iter()
        .map(|(name, features)| {
            let feature_codes = construct_features(features);
            let command = &context.command_cache[name.as_str()];
            let api_name = format_ident!("{}", name.as_str());
            let internal_api_catch = format_ident!("__{}", name.as_str());
            let (arg_block, return_block) =
                build_function_block(&command.proto, command.arguments.as_slice());
            let byte_lit =
                LitByteStr::new(format!("{}\0", name.as_str()).as_bytes(), Span::call_site());
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
        })
        .collect();

    let enum_code = quote! {
        use crate::types::*;
        #(#all_enums_const)*
    };

    let bitflag_code = quote! {
        use crate::types::*;
        #(#all_bitfield_const)*
    };

    // use std::fmt;
    // use std::ffi::c_void;
    // use gl::enums::*;
    // use gl::bitflags::*;
    // use crate::gl;
    let command_code = quote! {
        use crate::types::*;
        #(#command_codes)*
    };

    //         use gl::command::*;
    //         use gl::enums::*;
    //         use gl::bitflags::*;
    let feature_code = quote! {
        use crate::types::*;
        use std::ffi::c_void;

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

    let mut bitflag_file = File::create(gl_path.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(gl_path.join("enums.rs")).expect("enums.rs");
    let mut command_file = File::create(gl_path.join("command.rs")).expect("command.rs");
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
