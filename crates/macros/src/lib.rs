use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

use convert_case::{Case, Casing};
use error::ToCompileError;
use parsers::struc::{get_meta_name_value, parse_struct_fields};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta};
use ts::ts_map::ts_rs_map;

mod error;
mod parsers;
mod ts;

#[derive(Debug)]
enum RenameAll {
    CamelCase,
    SnakeCase,
    UpperCase,
    LowerCase,
    PascalCase,
    // TODO: kebab
    //KebabCase,
}

impl RenameAll {
    pub fn to_case(&self, s: &str) -> String {
        match self {
            Self::CamelCase => s.to_case(Case::Camel),
            Self::SnakeCase => s.to_case(Case::Snake),
            Self::UpperCase => s.to_case(Case::Upper),
            Self::LowerCase => s.to_case(Case::Lower),
            Self::PascalCase => s.to_case(Case::Pascal),
        }
    }
}

#[derive(Default, Debug)]
struct StructAttributes {
    pub rename_all: Option<RenameAll>,
    pub rename: Option<String>,
    pub export: Option<PathBuf>,
}

impl StructAttributes {
    pub fn new() -> Self {
        Self::default()
    }
}

#[proc_macro_derive(TsBind, attributes(ts_bind))]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let attrs = &input.attrs;

    let mut struct_attrs = StructAttributes::new();
    attrs.iter().for_each(|attr| {
        if attr.path().is_ident("ts_bind") {
            if let Ok(meta) = attr.parse_args() {
                match meta {
                    Meta::Path(_meta_path) => {}
                    Meta::List(_meta_list) => {}
                    Meta::NameValue(name) => {
                        let path = &name.path;
                        if path.is_ident("rename") {
                            let value = get_meta_name_value(&name)
                                .expect("Failed to parse rename attribute")
                                .expect("Rename attribute is empty");

                            struct_attrs.rename = Some(value);
                        }
                        if path.is_ident("rename_all") {
                            let value = get_meta_name_value(&name)
                                .expect("Failed to parse rename attribute")
                                .expect("Rename attribute is empty");

                            match value.as_str() {
                                "camel_case" => {
                                    struct_attrs.rename_all = Some(RenameAll::CamelCase);
                                }
                                "snake_case" => {
                                    struct_attrs.rename_all = Some(RenameAll::SnakeCase);
                                }
                                "upper_case" => {
                                    struct_attrs.rename_all = Some(RenameAll::UpperCase);
                                }
                                "lower_case" => {
                                    struct_attrs.rename_all = Some(RenameAll::LowerCase);
                                }
                                "pascal_case" => {
                                    struct_attrs.rename_all = Some(RenameAll::PascalCase);
                                }
                                _ => {
                                    panic!("Invalid attribute name: {}", value);
                                }
                            }
                        }
                        if path.is_ident("export") {
                            let value = get_meta_name_value(&name)
                                .expect("Failed to parse export attribute")
                                .expect("Export attribute is empty");

                            struct_attrs.export = Some(PathBuf::from(value));
                        }
                    }
                }
            }
        }
    });

    let name = if let Some(rename) = struct_attrs.rename {
        rename
    } else {
        input.ident.to_string()
    };

    let fields = parse_struct_fields(&input);

    if let Err(e) = fields {
        return e.to_compile_error();
    }

    let fields = fields.unwrap();

    let mut ts_bind = String::from(format!("\nexport interface {} {{\n", name));
    let mut imports = Vec::new();
    for (ident, ty, attrs) in fields.iter() {
        if attrs.skip {
            continue;
        }

        let field_name = if let Some(rename_all) = &struct_attrs.rename_all {
            rename_all.to_case(&ident.to_string())
        } else {
            ident.to_string()
        };

        let field_name = attrs.rename.as_ref().unwrap_or(&field_name);

        let map_result = ts_rs_map(ty, &mut imports);

        ts_bind.push_str(&format!("   {}: {};\n", field_name, map_result));
    }

    ts_bind.push_str("}");

    sort_imports(&mut imports);
    for to_import in imports {
        ts_bind = format!(
            "import type {{ {} }} from \"./{}\";\n{}",
            to_import, to_import, ts_bind
        );
    }

    ts_bind = format!(
        "// This file was automatically generated by ts_bind, do not modify it manually\n{}",
        ts_bind
    );

    let lib_path = if let Some(export_path) = struct_attrs.export {
        export_path.join(format!("{}.ts", name))
    } else {
        PathBuf::new().join("bindings").join(format!("{}.ts", name))
    };

    write_to_file(&lib_path, &ts_bind);

    quote! {}.into()
}

fn write_to_file(path: &PathBuf, content: &str) {
    create_dir_all(path.parent().unwrap()).unwrap();
    write(path, content).unwrap();
}

fn sort_imports(imports: &mut Vec<String>) {
    imports.sort();
    imports.dedup();
}
