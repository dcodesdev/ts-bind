use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

use crate::{
    files::write_to_file, parsers::struc::parse_struct_fields, struct_attrs::StructAttrs,
    ts::gen_ts_code::gen_ts_code,
};

pub fn handle_ts_bind(input: &DeriveInput) -> anyhow::Result<TokenStream> {
    let struct_attrs = StructAttrs::from(input.ident.to_string(), &input.attrs);

    let fields = parse_struct_fields(&input)?;

    let ts_bind = gen_ts_code(struct_attrs.get_name(), &fields, &struct_attrs)?;

    write_to_file(&struct_attrs.get_export_path(), &ts_bind)?;

    Ok(quote! {}.into())
}
