use std::borrow::{Borrow, BorrowMut};
use itertools::{Itertools, join};
use khronos_registry_parse::gl::{Command, Enum, Enums, EnumsChild, ExtensionChild, InterfaceItem, RegistryChild};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, hex_digit1, one_of};
use nom::combinator::{complete, map, map_parser, map_res, opt, recognize, value};
use nom::multi::{many0, many1};
use nom::sequence::{preceded, terminated, tuple};
use nom::{Finish, IResult};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::token::Const;

#[derive(Clone, Debug)]
pub enum Constant {
    Number {
        sign: Option<String>,
        number: String
    },
    Hex(String),
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
            recognize(
                many1(
                    terminated(one_of("0123456789abcdefABCDEF"), many0(char('_')))
                )
            )
        ),
        |out: &str| -> Result<Constant, ()> { Ok(Constant::Hex(out.to_ascii_lowercase())) },
    )(input)
}


fn parse_number(input: &str) -> IResult<&str, Constant> {
    map_res(
        tuple(( opt(one_of("+-")), many1(one_of("0123456789")))),
        |(sign,number )| -> Result<Constant, ()> { Ok(Constant::Number {
            sign: sign.map(|e| format!("{}", e)),
            number: number.into_iter().collect()
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

impl quote::ToTokens for Constant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match *self {
            Constant::Number { ref sign, ref number}  => {
                let number = interleave_number('_', 4, number.as_str());
                syn::LitInt::new(&format!("{}{}", match sign.as_ref() {
                    None => { "" }
                    Some(res) => {res}
                }, number), Span::call_site()).to_tokens(tokens);
            }
            Constant::Hex(ref n) => {
                let number = interleave_number('_', 4, n);
                syn::LitInt::new(&format!("0x{}", number), Span::call_site()).to_tokens(tokens);
            }
        }
    }
}

pub fn write_source_code<P: AsRef<Path>>(headers_dir: &Path, src_dir: P) {
    let gl_xml = headers_dir.join("xml/gl.xml");
    let glx_xml = headers_dir.join("xml/glx.xml");
    let wgl_xml = headers_dir.join("xml/wgl.xml");

    write_gl( headers_dir, PathBuf::from(src_dir.as_ref()));

    // write_part_code(&gl_xml, {
    //     let mut gl_path = PathBuf::from(src_dir.as_ref());
    //     gl_path.push("gl");
    //     gl_path
    // });
    // write_part_code(&glx_xml, {
    //     let mut gl_path = PathBuf::from(src_dir.as_ref());
    //     gl_path.push("glx");
    //     gl_path
    // });
    // write_part_code(&wgl_xml, {
    //     let mut gl_path = PathBuf::from(src_dir.as_ref());
    //     gl_path.push("wgl");
    //     gl_path
    // });
}
//
// impl Hash for Enum {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.name.hash(state);
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct _Enum(Enum);

impl Hash for _Enum {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.name.hash(state)
    }
}

// #[derive(Debug, Clone, PartialEq, Eq)]
// pub struct _Command(Command);
//
// impl Default for _Command {
//     fn default() -> Self {
//         todo!()
//     }
// }
//
// impl Hash for _Command {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0.proto.name.hash(state)
//     }
// }


fn write_gl(opengl_registry: &Path, output: PathBuf) {
    let mut p = PathBuf::from(opengl_registry);
    p.push("xml/gl.xml");

    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(p.as_path()).expect("invalid xml file");

    let extensions = spec
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Extensions(e) => Some(&e.children),
            _ => None,
        })
        .next()
        .expect("extension");

    let mut enum_collection: HashMap<String, HashSet<_Enum>> = HashMap::new();
    let mut bitmask_collection: HashMap<String, HashSet<_Enum>> = HashMap::new();
    for enums in spec
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
                let handle_enum = |name: &String, coll: &mut HashMap<String, HashSet<_Enum>>| {
                    for n in name.split(',') {
                        coll.entry(n.trim().to_string())
                            .or_insert(HashSet::new())
                            .insert(_Enum(e.clone()));
                    }
                };
                if let Some(name) = base_name.or_else(|| e.group.as_ref()) {
                    if name.as_str().eq("TransformFeedbackTokenNV") {
                        continue;
                    }
                }

                if let Some(name) = base_name {
                    handle_enum(
                        name,
                        (if is_bitmask {
                            &mut bitmask_collection
                        } else {
                            &mut enum_collection
                        }),
                    );
                }
                if let Some(name) = e.group.as_ref() {
                    handle_enum(
                        name,
                        (if is_bitmask {
                            &mut bitmask_collection
                        } else {
                            &mut enum_collection
                        }),
                    );
                }
            }
        }
    }

    // let mut command_collection: HashMap<String, HashSet<Command>> = HashMap::new();
    let mut command_codes: Vec<TokenStream> = Vec::new();
    for commands in spec
        .0
        .iter()
        .filter_map(|item| match item {
            RegistryChild::Commands(e) => Some(e),
            _ => None,
        })
        .into_iter()
    {
        for c in &commands.children {
            let type_name = format_ident!("PFN_{}", c.proto.name);
            println!("{}", type_name);
            let parameter_iter = c.params.iter()
                .map(|p| {
                    let paramter_def = &p.definition;
                    let parameter_name: String = paramter_def.name.clone();
                    // let void_type = "*mut c_void".to_string();
                    // let type_name: String = paramter_def.type_name.as_ref().unwrap_or_else(|| &void_type).to_string();
                    let paramter_type = map_type(paramter_def.type_name.as_ref());

                    let parameter_ident = format_ident!("_{}", parameter_name);
                    // let type_ident = format_ident!("{}",  p.group.as_ref().unwrap_or_else(|| &final_type));
                    quote! {
                        #parameter_ident: #paramter_type
                    }
                });
            let parameters = quote!(#(#parameter_iter,)*);
            // let return_type = c.proto.type_name.as_ref().unwrap();
            command_codes.push(quote! {
                #[allow(non_camel_case_types)]
                pub type #type_name = unsafe extern "system" fn(#parameters);
            });
        }
    }


    let mut enum_codes: Vec<TokenStream> = enum_collection.into_iter().map(|(key, enums)|  {
        let ident = format_ident!("{}", key.as_str());
        let impl_enum = enums.iter().map(|c| {
            // let value = Constant::Hex(c.value.as_ref().unwrap().clone());
            // let value =
            let name = format_ident!("{}", &c.0.name);
            let value = parse_constant(c.0.value.as_ref().unwrap())
                .finish()
                .expect("failed to parse constant")
                .1;
            quote! {
                pub const #name: Self = Self(#value);
            }
        });
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


    let mut bitflag_codes: Vec<TokenStream> = bitmask_collection.into_iter().map(|(key, enums)|  {
        let ident = format_ident!("{}", key.as_str());
        let impl_enum = enums.iter().map(|c| {
            // let value = Constant::Hex(c.value.as_ref().unwrap().clone());
            // let value =
            let name = format_ident!("{}", &c.0.name);
            let value = parse_constant(c.0.value.as_ref().unwrap())
                .finish()
                .expect("failed to parse constant")
                .1;
            quote! {
                pub const #name: Self = Self(#value);
            }
        });
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
        use gl::platform_types::*;
        use gl::enums::*;
        use gl::bitflags::*;
        #(#command_codes)*
    };



    let mut gl_path = PathBuf::from(output);
    gl_path.push("gl");

    // let mut gl_enums_file = File::create(gl_dir.join("enums.rs")).expect("enums.rs");
    let mut bitflag_file = File::create(gl_path.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(gl_path.join("enums.rs")).expect("enums.rs");
    let mut command_file = File::create(gl_path.join("command.rs")).expect("command.rs");

    // write!(&mut gl_enums_file, "{}", enum_code).expect("Unable to write enums.rs");
    write!(&mut bitflag_file, "{}", bitflag_code).expect("Unable to write bitflag.rs");
    write!(&mut enum_file, "{}", enum_code).expect("Unable to write bitflag.rs");
    write!(&mut command_file, "{}", command_code).expect("Unable to write command.rs");

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
