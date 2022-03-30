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
use crate::gl_generator::write_gl;


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
                        // if alias_type.as_str().eq("GLenum") || alias_type.as_str().eq("GLbitfield")
                        // {
                        //     if let Some(group_name) = &cmd.group {
                        //         value = group_name.as_str();
                        //     }
                        // }
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

fn build_enum_type_block(collection: &HashMap<String, HashSet<APIEnum>>) -> Vec<TokenStream> {
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

pub fn build_function_block(
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

pub fn build_command_block(collection: &HashMap<String, APICommand>) -> Vec<TokenStream> {
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


pub fn write_source_code<P: AsRef<Path>>(headers_dir: &Path, src_dir: P) {
    write_gl(headers_dir, PathBuf::from(src_dir.as_ref()));
    // write_glx(headers_dir, PathBuf::from(src_dir.as_ref()));
    // write_wgl(headers_dir, PathBuf::from(src_dir.as_ref()));
}

fn write_glx(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/glx.xml");

    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let mut context = construct_context(&spec);

    let unique_enums = collect_unique_enums(&context.enum_cache);
    let unique_bitfield = collect_unique_enums(&context.bitfield_cache);
    let all_enums_const: Vec<TokenStream> = unique_enums
        .iter()
        .map(|c| {
            let name = format_ident!("{}", &c.name);
            let value = &c.constant;
            quote! {
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
                pub const #name: GLbitfield = #value;
            }
        })
        .collect();

    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);

    let enum_code = quote! {
        use std::fmt;
        #(#all_enums_const)*
    };

    let bitflag_code = quote! {
        use std::fmt;
        #(#all_bitfield_const)*
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
    let bitflag_codes: Vec<TokenStream> = build_bitflag_block(&context.bitfield_cache);
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

