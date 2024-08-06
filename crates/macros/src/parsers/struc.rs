use syn::{Data, DeriveInput, Fields, Ident, Type};

pub fn parse_struct_fields(input: &DeriveInput) -> Vec<(Ident, Type)> {
    let mut fields_info = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(ref fields_named) = data_struct.fields {
            for field in fields_named.named.iter() {
                if let Some(ident) = &field.ident {
                    fields_info.push((ident.clone(), field.ty.clone()));
                }
            }
        }
    }

    fields_info
}
