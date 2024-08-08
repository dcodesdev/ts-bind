use syn::Attribute;

use crate::{parsers::struc::get_nested_value, rename_all::RenameAll};
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct StructAttrs {
    pub rename_all: Option<RenameAll>,
    pub rename: Option<String>,
    pub export: Option<PathBuf>,
}

impl StructAttrs {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn parse_attrs(&mut self, attrs: &Vec<Attribute>) {
        attrs.iter().for_each(|attr| {
            if attr.path().is_ident("ts_bind") {
                attr.parse_nested_meta(|meta| {
                    let path = &meta.path;
                    if path.is_ident("rename") {
                        let value =
                            get_nested_value(&meta).expect("Failed to parse rename attribute");

                        self.rename = Some(value);
                    }
                    if path.is_ident("rename_all") {
                        let value =
                            get_nested_value(&meta).expect("Failed to parse rename_all attribute");

                        match value.as_str() {
                            "camelCase" => {
                                self.rename_all = Some(RenameAll::CamelCase);
                            }
                            "snake_case" => {
                                self.rename_all = Some(RenameAll::SnakeCase);
                            }
                            "UPPERCASE" => {
                                self.rename_all = Some(RenameAll::UpperCase);
                            }
                            "lowercase" => {
                                self.rename_all = Some(RenameAll::LowerCase);
                            }
                            "PascalCase" => {
                                self.rename_all = Some(RenameAll::PascalCase);
                            }
                            _ => {
                                panic!("Invalid attribute name: {}", value);
                            }
                        }
                    }
                    if path.is_ident("export") {
                        let value =
                            get_nested_value(&meta).expect("Failed to parse export attribute");

                        self.export = Some(PathBuf::from(value));
                    }

                    Ok(())
                })
                .expect("Failed to parse nested meta");
            }
        });
    }
}
