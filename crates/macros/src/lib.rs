use error::ToCompileError;
use parsers::struc::parse_struct_fields;
use proc_macro::TokenStream;
use quote::quote;
use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};
use struct_attrs::StructAttrs;
use syn::{parse_macro_input, DeriveInput};
use ts::gen_ts_code::gen_ts_code;

mod error;
mod parsers;
mod rename_all;
mod struct_attrs;
mod ts;

#[proc_macro_derive(TsBind, attributes(ts_bind))]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match handle_derive(&input) {
        Ok(ts_code) => ts_code,
        Err(e) => e.to_compile_error(),
    }
}

fn handle_derive(input: &DeriveInput) -> anyhow::Result<TokenStream> {
    let attrs = &input.attrs;

    let mut struct_attrs = StructAttrs::new();
    struct_attrs.parse_attrs(attrs);

    let name = input.ident.to_string();
    let struct_name = struct_attrs.rename.as_ref().unwrap_or(&name);

    let fields = parse_struct_fields(&input)?;

    let ts_bind = gen_ts_code(&struct_name, &fields, &struct_attrs)?;

    let lib_path = struct_attrs.export.unwrap_or_else(|| {
        PathBuf::new()
            .join("bindings")
            .join(format!("{}.ts", struct_name))
    });

    write_to_file(&lib_path, &ts_bind);

    Ok(quote! {}.into())
}

fn write_to_file(path: &PathBuf, content: &str) {
    create_dir_all(path.parent().unwrap()).unwrap();
    write(path, content).unwrap();
}
