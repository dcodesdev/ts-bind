use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

use error::ToCompileError;
use parsers::struc::parse_struct_fields;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use ts::ts_map::ts_rs_map;

mod error;
mod parsers;
mod ts;

#[proc_macro_derive(TsBind, attributes(ts_bind))]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
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

        let field_name = ident.to_string();
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

    let lib_path = PathBuf::new().join("bindings").join(format!("{}.ts", name));

    write_to_file(lib_path.to_str().unwrap(), &ts_bind);

    quote! {}.into()
}

fn write_to_file(path: &str, content: &str) {
    create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    write(path, content).unwrap();
}

fn sort_imports(imports: &mut Vec<String>) {
    imports.sort();
    imports.dedup();
}
