use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

use error::ToCompileError;
use parsers::struc::parse_struct_fields;
use proc_macro::TokenStream;
use quote::quote;
use struct_attrs::StructAttrs;
use syn::{parse_macro_input, DeriveInput};
use ts::ts_map::ts_rs_map;

mod error;
mod parsers;
mod rename_all;
mod struct_attrs;
mod ts;

#[proc_macro_derive(TsBind, attributes(ts_bind))]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match gen_ts_code(&input) {
        Ok(ts_code) => ts_code,
        Err(e) => e.to_compile_error(),
    }
}

fn gen_ts_code(input: &DeriveInput) -> anyhow::Result<TokenStream> {
    let attrs = &input.attrs;
    let mut struct_attrs = StructAttrs::new();

    struct_attrs.parse_attrs(attrs);

    let name = if let Some(rename) = struct_attrs.rename {
        rename
    } else {
        input.ident.to_string()
    };

    let fields = parse_struct_fields(&input)?;

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

    Ok(quote! {}.into())
}

fn write_to_file(path: &PathBuf, content: &str) {
    create_dir_all(path.parent().unwrap()).unwrap();
    write(path, content).unwrap();
}

fn sort_imports(imports: &mut Vec<String>) {
    imports.sort();
    imports.dedup();
}
