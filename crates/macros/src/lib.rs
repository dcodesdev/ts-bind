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
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}

fn handle_derive(input: &DeriveInput) -> anyhow::Result<TokenStream> {
    let struct_attrs = StructAttrs::from(input.ident.to_string(), &input.attrs);

    let fields = parse_struct_fields(&input)?;

    let ts_bind = gen_ts_code(struct_attrs.get_name(), &fields, &struct_attrs)?;

    write_to_file(&struct_attrs.get_export_path(), &ts_bind)?;

    Ok(quote! {}.into())
}

fn write_to_file(path: &PathBuf, content: &str) -> anyhow::Result<()> {
    let parent = path.parent().ok_or(anyhow::anyhow!(
        "Failed to get parent directory of path: {}",
        path.display()
    ))?;

    create_dir_all(parent)?;

    write(path, content)?;

    Ok(())
}
