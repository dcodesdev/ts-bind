use proc_macro::TokenStream;
use quote::quote;

pub trait ToCompileError {
    fn to_compile_error(&self) -> TokenStream;
}

impl ToCompileError for anyhow::Error {
    fn to_compile_error(&self) -> TokenStream {
        let error = format!("{}", self);

        quote! {
            compile_error!(#error);
        }
        .into()
    }
}
