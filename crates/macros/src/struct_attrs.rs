use convert_case::{Case, Casing};
use syn::Attribute;

use crate::{parsers::struc::get_nested_value, rename_all::RenameAll};
use std::path::PathBuf;

#[derive(Debug)]
pub struct StructAttrs {
    name: StructName,
    rename_all: Option<RenameAll>,
    export: Option<PathBuf>,
}

#[derive(Debug)]
pub struct StructName(String);

impl StructName {
    pub fn new(name: String) -> Self {
        Self(name.to_case(Case::Pascal))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl StructAttrs {
    pub fn from(struct_name: String, attrs: &Vec<Attribute>) -> Self {
        let mut struct_attrs = Self {
            name: StructName::new(struct_name),
            rename_all: None,
            export: None,
        };

        Self::parse_attrs(&mut struct_attrs, attrs);

        struct_attrs
    }

    fn parse_attrs(struct_attrs: &mut Self, attrs: &Vec<Attribute>) {
        attrs.iter().for_each(|attr| {
            if attr.path().is_ident("ts_bind") {
                attr.parse_nested_meta(|meta| {
                    let path = &meta.path;

                    let ident = path.get_ident();

                    if let Some(ident) = ident {
                        let ident_str = ident.to_string();

                        match ident_str.as_str() {
                            "rename" => {
                                let value = get_nested_value(&meta)
                                    .expect("Failed to parse rename attribute");

                                struct_attrs.name = StructName::new(value);
                            }
                            "rename_all" => {
                                let value = get_nested_value(&meta)
                                    .expect("Failed to parse rename_all attribute");

                                match value.as_str() {
                                    "camelCase" => {
                                        struct_attrs.rename_all = Some(RenameAll::CamelCase);
                                    }
                                    "snake_case" => {
                                        struct_attrs.rename_all = Some(RenameAll::SnakeCase);
                                    }
                                    "UPPERCASE" => {
                                        struct_attrs.rename_all = Some(RenameAll::UpperCase);
                                    }
                                    "lowercase" => {
                                        struct_attrs.rename_all = Some(RenameAll::LowerCase);
                                    }
                                    "PascalCase" => {
                                        struct_attrs.rename_all = Some(RenameAll::PascalCase);
                                    }
                                    _ => {
                                        panic!("Invalid attribute name: {}", value);
                                    }
                                }
                            }
                            "export" => {
                                let value = get_nested_value(&meta)
                                    .expect("Failed to parse export attribute");

                                struct_attrs.export = Some(PathBuf::from(value));
                            }
                            _ => {
                                panic!("Invalid attribute name: {}", ident_str);
                            }
                        }
                    }

                    Ok(())
                })
                .expect("Failed to parse nested meta");
            }
        });
    }

    pub fn get_name(&self) -> &StructName {
        &self.name
    }

    pub fn get_export_path(&self) -> PathBuf {
        self.export
            .clone()
            .unwrap_or_else(|| PathBuf::new().join("bindings"))
            .join(format!("{}.ts", self.get_name().as_str()))
    }

    pub fn get_rename_all(&self) -> Option<&RenameAll> {
        self.rename_all.as_ref()
    }
}
