use error::ToCompileError;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use ts_bind::handle_ts_bind;

mod error;
mod files;
mod parsers;
mod rename_all;
mod struct_attrs;
mod ts;
mod ts_bind;

#[proc_macro_derive(TsBind, attributes(ts_bind))]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match handle_ts_bind(&input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}
