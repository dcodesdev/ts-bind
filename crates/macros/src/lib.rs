use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

use parsers::struc::parse_struct_fields;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use ts::ts_map::ts_rs_map;

mod parsers;

#[proc_macro_derive(TsBind)]
pub fn ts_bind_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let fields = parse_struct_fields(&input);

    let mut ts_bind = String::from(format!("interface {} {{\n", name));
    for (ident, ty) in fields.iter() {
        ts_bind.push_str(&format!("   {}: {};\n", ident.to_string(), ts_rs_map(ty)));
    }
    ts_bind.push_str("}");

    let lib_path = PathBuf::new().join("bindings").join(format!("{}.ts", name));

    write_to_file(lib_path.to_str().unwrap(), &ts_bind);

    quote! {}.into()
}

fn write_to_file(path: &str, content: &str) {
    create_dir_all(PathBuf::from(path).parent().unwrap()).unwrap();
    write(path, content).unwrap();
}
