use crate::command_parser::{parse_argument, parse_proto, ArgumentDef, ProtoDef};
use crate::const_parser::{parse_constant, Constant};
use khronos_registry_parse::gl::{
    EnumsChild, ExtensionChild, InterfaceItem, Registry, RegistryChild,
};
use nom::error::VerboseError;
use nom::Finish;
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use regex::Regex;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[derive(Clone, Eq)]
pub struct APIEnum {
    pub name: String,
    pub constant: Constant,
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

pub struct APIArgument {
    pub group: Option<String>,
    pub name: String,
    pub def: ArgumentDef,
}

pub struct APIProto {
    pub group: Option<String>,
    pub def: ProtoDef,
}

pub struct APICommand {
    pub name: String,
    pub proto: APIProto,
    pub arguments: Vec<APIArgument>,
}

pub struct APIGroup {
    pub groups: Vec<ExtensionChild>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum APIName {
    OPENGL { minor: u16, major: u16 },
    GLES { minor: u16, major: u16 },
    GLES2 { minor: u16, major: u16 },
    GLES3 { minor: u16, major: u16 },
    Unknown,
}

impl APIName {
    pub fn order(a1: &APIName, a2: &APIName) -> Ordering {
        let order_major_minor = |major1, minor1, major2, minor2| -> Ordering {
            let mut order = Ord::cmp(major1, major2);
            if order == Ordering::Equal {
                order = Ord::cmp(minor1, minor2)
            }
            return order;
        };
        match (a1, a2) {
            (
                APIName::OPENGL {
                    major: major1,
                    minor: minor1,
                },
                APIName::OPENGL {
                    major: major2,
                    minor: minor2,
                },
            ) => order_major_minor(major1, minor1, major2, minor2),
            (
                APIName::GLES {
                    major: major1,
                    minor: minor1,
                },
                APIName::GLES {
                    major: major2,
                    minor: minor2,
                },
            ) => order_major_minor(major1, minor1, major2, minor2),
            (
                APIName::GLES2 {
                    major: major1,
                    minor: minor1,
                },
                APIName::GLES2 {
                    major: major2,
                    minor: minor2,
                },
            ) => order_major_minor(major1, minor1, major2, minor2),
            (
                APIName::GLES3 {
                    major: major1,
                    minor: minor1,
                },
                APIName::GLES3 {
                    major: major2,
                    minor: minor2,
                },
            ) => order_major_minor(major1, minor1, major2, minor2),
            _ => {
                panic!("mismatched api's")
            }
        }
    }
}

pub struct Context {
    pub constant_map: HashMap<String, APIEnum>,
    pub bitfield_cache: HashMap<String, HashSet<APIEnum>>,
    pub enum_cache: HashMap<String, HashSet<APIEnum>>,
    pub command_cache: HashMap<String, APICommand>,
    pub feature_cache: HashMap<APIName, APIGroup>,
    pub extension_cache: HashMap<String, APIGroup>,
}

pub fn parse_number_major_minor(version: &str) -> (u16, u16) {
    let r = Regex::new(r"([0-9]+).([0-9]+)").unwrap();
    let version = r
        .captures(version)
        .expect("failed to capture version string");
    (
        version.get(1).unwrap().as_str().parse().unwrap(),
        version.get(2).unwrap().as_str().parse().unwrap(),
    )
}

pub fn collect_unique_enums(collection: &HashMap<String, HashSet<APIEnum>>) -> HashSet<APIEnum> {
    let mut results: HashSet<APIEnum> = HashSet::default();
    for (name, values) in collection.iter() {
        if name.as_str().eq("SpecialNumbers") {
            continue;
        }
        for e in values {
            results.insert(e.clone());
        }
    }
    results
}

pub fn construct_context(registry: &Registry) -> Context {
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
                    feature_cache.entry(api).or_insert(APIGroup {
                        groups: features.children.clone(),
                    });
                }
                "gles2" => {
                    let (major, minor) = parse_number_major_minor(
                        features
                            .number
                            .as_ref()
                            .expect("feature version is missing"),
                    );
                    let api = APIName::GLES2 { major, minor };
                    feature_cache.entry(api).or_insert(APIGroup {
                        groups: features.children.clone(),
                    });
                }
                "gles1" => {
                    let (major, minor) = parse_number_major_minor(
                        features
                            .number
                            .as_ref()
                            .expect("feature version is missing"),
                    );
                    let api = APIName::GLES { major, minor };
                    feature_cache.entry(api).or_insert(APIGroup {
                        groups: features.children.clone(),
                    });
                }
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
        bitfield_cache: bitflag_cache,
        enum_cache,
        command_cache,
        feature_cache,
        extension_cache: Default::default(),
    }
}

pub fn map_type(arg: &str) -> TokenStream {
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
