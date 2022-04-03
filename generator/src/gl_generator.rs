use itertools::{min, Itertools};
use khronos_registry_parse::gl::{
    Command, CommandParam, Enum, Enums, EnumsChild, ExtensionChild, InterfaceItem, Registry,
    RegistryChild,
};
use nom::error::ParseError;
use nom::Parser;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use syn::spanned::Spanned;

use crate::context::{
    collect_unique_enums, construct_context, map_type, APIArgument, APICommand, APIEnum, APIGroup,
    APIName, APIProto, Context,
};
use crate::generator::{build_command_block, build_function_block, construct_field_block};
use crate::gl_command_generator::construct_method_block;
use syn::LitByteStr;

fn build_api_load_block<'a>(
    name: &str,
    proto: &'a APIProto,
    args: &'a [APIArgument],
) -> TokenStream {
    let internal_api_catch = format_ident!("__{}", name);
    let (arg_block, return_block) = build_function_block(proto, args);
    let byte_lit = LitByteStr::new(format!("{}\0", name).as_bytes(), Span::call_site());
    let panic_message = syn::LitStr::new(
        format!("Unable to load {}", name).as_str(),
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
        unsafe {
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
}

fn construct_features<'a>(features: impl Iterator<Item = &'a str>) -> TokenStream {
    let collection: Vec<TokenStream> = features
        .map(|it| {
            let name = it;
            quote! { feature = #name }
        })
        .collect();
    quote! {
        #[cfg(any( #(#collection,)* ))]
    }
}

#[derive(Clone, PartialEq, Eq)]
struct APIDetails<'a, 'b> {
    name: &'a APIName,
    commands: &'b BTreeSet<String>,
}

pub fn build_features<'a>(
    context: &Context,
    apis: impl Iterator<Item = &'a APIName>,
    mut handler: impl FnMut(APIDetails),
) {
    let mut temp_commands: BTreeSet<String> = BTreeSet::default();
    apis.for_each(|api| {
        let feature = &context.feature_cache[api];
        for extension in &feature.groups {
            match extension {
                ExtensionChild::Require { items, .. } => {
                    for it in items {
                        match it {
                            InterfaceItem::Command { name, .. } => {
                                temp_commands.insert(name.clone());
                            }
                            _ => {}
                        }
                    }
                }
                ExtensionChild::Removed { items, .. } => {
                    for it in items {
                        match it {
                            InterfaceItem::Command { name, .. } => {
                                temp_commands.remove(name);
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        let details = APIDetails {
            name: &api,
            commands: &temp_commands,
        };
        handler(details);
    })
}

pub fn write_gles(context: &Context, output: &PathBuf) {
    let _root_path = PathBuf::from(output);
    let gles_path = {
        let mut result = PathBuf::from(&output);
        result.push("gles");
        result
    };

    let mut cmd_features: HashMap<String, HashSet<String>> = HashMap::default();
    build_features(
        &context,
        context
            .feature_cache
            .keys()
            .filter_map(|it| {
                if let APIName::GLES { .. } = it {
                    return Some(it);
                }
                return None;
            })
            .sorted_by(|it1, it2| APIName::order_same_api(it1, it2))
            .into_iter(),
        |details| {
            if let APIName::GLES { major, minor } = details.name {
                for cmd in details.commands.iter() {
                    cmd_features
                        .entry(cmd.clone())
                        .or_insert(HashSet::default())
                        .insert(format!("gles{}{}", *major, *minor));
                }

                let mut method_block: Vec<TokenStream> = construct_method_block(
                    details.name,
                    details.commands.iter().map(|i| i.as_str()),
                    |c| &context.command_cache[c],
                );
                let api_name = format_ident!("GLES{}{}", *major, *minor);

                let function_codes = quote! {
                    use crate::types::*;
                    use crate::gles::feature::EntryGLESFn;
                    use std::mem;


                    pub trait #api_name {
                        unsafe fn entry(&self) -> &EntryGLESFn;

                        #(#method_block)*
                    }
                };
                let api_path = gles_path
                    .join(format!("gles{}{}", *major, *minor))
                    .join("api.rs");
                let mut function_file = File::create(&api_path)
                    .expect(format!("failed to create: {}", api_path.to_str().unwrap()).as_str());
                write!(&mut function_file, "{}", function_codes)
                    .expect("Unable to write command.rs");
            }
        },
    );

    build_features(
        &context,
        context
            .feature_cache
            .keys()
            .filter_map(|it| {
                if let APIName::GLES2 { .. } = it {
                    return Some(it);
                }
                return None;
            })
            .sorted_by(|it1, it2| APIName::order_same_api(it1, it2))
            .into_iter(),
        |details| {
            if let APIName::GLES2 { major, minor } = details.name {
                for cmd in details.commands.iter() {
                    cmd_features
                        .entry(cmd.clone())
                        .or_insert(HashSet::default())
                        .insert(format!("gles{}{}", *major, *minor));
                }

                let mut method_block: Vec<TokenStream> = construct_method_block(
                    details.name,
                    details.commands.iter().map(|i| i.as_str()),
                    |c| &context.command_cache[c],
                );
                let api_name = format_ident!("GLES{}{}", *major, *minor);

                let function_codes = quote! {
                    use crate::types::*;
                    use crate::gles::feature::EntryGLESFn;
                    use std::mem;

                    pub trait #api_name {
                        unsafe fn entry(&self) -> &EntryGLESFn;

                        #(#method_block)*
                    }
                };
                let api_path = gles_path
                    .join(format!("gles{}{}", *major, *minor))
                    .join("api.rs");
                let mut function_file = File::create(&api_path)
                    .expect(format!("failed to create: {}", api_path.to_str().unwrap()).as_str());
                write!(&mut function_file, "{}", function_codes)
                    .expect("Unable to write command.rs");
            }
        },
    );

    let properties: Vec<TokenStream> = cmd_features
        .iter()
        .map(|(name, features)| {
            let feature_codes = construct_features(features.iter().map(|i| i.as_str()));
            let command_name = format_ident!("{}", name.as_str());
            let command_type = format_ident!("PFN_{}", name.as_str());
            quote! {
                #feature_codes
                pub #command_name : crate::command::#command_type
            }
        })
        .collect();

    let impl_block: Vec<TokenStream> = cmd_features
        .iter()
        .map(|(name, features)| {
            let command = &context.command_cache[name.as_str()];
            let feature_codes = construct_features(features.iter().map(|i| i.as_str()));
            let api_name = format_ident!("{}", name.as_str());
            let api_load_block =
                build_api_load_block(name.as_str(), &command.proto, command.arguments.as_slice());
            quote! {
                #feature_codes
                #api_name : #api_load_block
            }
        })
        .collect();

    let feature_code = quote! {
        use crate::types::*;
        use std::ffi::c_void;

        #[derive(Clone)]
        pub struct EntryGLESFn {
             #(#properties,)*
        }
        impl EntryGLESFn {
            pub fn load<F>(mut _f: F) -> Self
                where
                    F: FnMut(&::std::ffi::CStr) -> *const c_void {
                    Self {
                        #(#impl_block,)*
                    }
            }
        }
    };
    let mut features_file = File::create(gles_path.join("feature.rs")).expect("feature.rs");
    write!(&mut features_file, "{}", feature_code).expect("Unable to write command.rs");
}

pub fn write_opengl(context: &Context, output: &PathBuf) {
    let _root_path = PathBuf::from(output);
    let gl_path = {
        let mut result = PathBuf::from(&output);
        result.push("gl");
        result
    };

    let mut cmd_features: HashMap<String, HashSet<String>> = HashMap::default();
    build_features(
        &context,
        context
            .feature_cache
            .keys()
            .filter_map(|it| {
                if let APIName::OPENGL { .. } = it {
                    return Some(it);
                }
                return None;
            })
            .sorted_by(|it1, it2| APIName::order_same_api(it1, it2))
            .into_iter(),
        |details| {
            if let APIName::OPENGL { major, minor } = details.name {
                for cmd in details.commands.iter() {
                    cmd_features
                        .entry(cmd.clone())
                        .or_insert(HashSet::default())
                        .insert(format!("gl{}{}", *major, *minor));
                }

                let mut method_block: Vec<TokenStream> = construct_method_block(
                    details.name,
                    details.commands.iter().map(|i| i.as_str()),
                    |c| &context.command_cache[c],
                );
                let api_name = format_ident!("GL{}{}", *major, *minor);

                let function_codes = quote! {
                    use crate::types::*;
                    use crate::gl::feature::EntryGLFn;
                    use std::mem;


                    pub trait #api_name {
                        unsafe fn entry(&self) -> &EntryGLFn;

                        #(#method_block)*
                    }
                };
                let mut function_file = File::create(
                    gl_path
                        .join(format!("gl{}{}", *major, *minor))
                        .join("api.rs"),
                )
                .expect("functions.rs");
                write!(&mut function_file, "{}", function_codes)
                    .expect("Unable to write command.rs");
            }
        },
    );

    let properties: Vec<TokenStream> = cmd_features
        .iter()
        .map(|(name, features)| {
            let feature_codes = construct_features(features.iter().map(|i| i.as_str()));
            let command_name = format_ident!("{}", name.as_str());
            let command_type = format_ident!("PFN_{}", name.as_str());
            quote! {
                #feature_codes
                pub #command_name : crate::command::#command_type
            }
        })
        .collect();

    let impl_block: Vec<TokenStream> = cmd_features
        .iter()
        .map(|(name, features)| {
            let command = &context.command_cache[name.as_str()];
            let feature_codes = construct_features(features.iter().map(|i| i.as_str()));
            let api_name = format_ident!("{}", name.as_str());
            let api_load_block =
                build_api_load_block(name.as_str(), &command.proto, command.arguments.as_slice());
            quote! {
                #feature_codes
                #api_name : #api_load_block
            }
        })
        .collect();

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
    let mut features_file = File::create(gl_path.join("feature.rs")).expect("feature.rs");
    write!(&mut features_file, "{}", feature_code).expect("Unable to write command.rs");
}

pub fn write_gl(opengl_registry: &Path, output: PathBuf) {
    let mut xml_path = PathBuf::from(opengl_registry);
    xml_path.push("xml/gl.xml");
    let (spec, _errors) =
        khronos_registry_parse::gl::parse_file(xml_path.as_path()).expect("invalid xml file");
    let context = construct_context(&spec);

    let command_codes: Vec<TokenStream> = build_command_block(&context.command_cache);
    let unique_enums = collect_unique_enums(&context.enum_cache);
    let unique_bitfield = collect_unique_enums(&context.bitfield_cache);
    let bitfield_tokens: Vec<TokenStream> = construct_field_block(unique_bitfield.iter(), |a| {
        format_ident!("GLbitfield").to_token_stream()
    });
    let enum_tokens: Vec<TokenStream> = construct_field_block(unique_enums.iter(), |a| {
        format_ident!("GLenum").to_token_stream()
    });

    let bitflag_code = quote! {
        use crate::types::*;
        #(#bitfield_tokens)*
    };

    let command_code = quote! {
        use crate::types::*;
        #(#command_codes)*
    };

    let enum_code = quote! {
        use crate::types::*;
        #(#enum_tokens)*
    };

    let mut command_file = File::create(output.join("command.rs")).expect("command.rs");
    let mut bitflag_file = File::create(output.join("bitflags.rs")).expect("bitflags.rs");
    let mut enum_file = File::create(output.join("enums.rs")).expect("enums.rs");

    write!(&mut command_file, "{}", command_code).expect("Unable to write command.rs");
    write!(&mut bitflag_file, "{}", bitflag_code).expect("Unable to write bitflag.rs");
    write!(&mut enum_file, "{}", enum_code).expect("Unable to write bitflag.rs");

    write_opengl(&context, &output);
    write_gles(&context, &output);
    // write_gles2(&context, &output);
}
