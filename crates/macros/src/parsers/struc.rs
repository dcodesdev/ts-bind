use syn::{Attribute, Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, MetaNameValue, Type};

#[derive(Default)]
pub struct FieldAttributes {
    pub rename: Option<String>,
}

pub fn parse_struct_fields(
    input: &DeriveInput,
) -> anyhow::Result<Vec<(Ident, Type, FieldAttributes)>> {
    let mut fields_info = Vec::new();

    if let Data::Struct(data_struct) = &input.data {
        if let Fields::Named(ref fields_named) = data_struct.fields {
            for field in fields_named.named.iter() {
                if let Some(ident) = &field.ident {
                    let attrs = parse_field_attributes(&field.attrs)?;
                    fields_info.push((ident.clone(), field.ty.clone(), attrs));
                }
            }
        }
    }

    Ok(fields_info)
}

fn parse_field_attributes(attrs: &[Attribute]) -> anyhow::Result<FieldAttributes> {
    let mut field_attrs = FieldAttributes::default();

    for attr in attrs.iter() {
        if attr.path().is_ident("ts_bind") {
            if let Ok(meta) = attr.parse_args() {
                if let Meta::NameValue(meta_name_value) = meta {
                    if meta_name_value.path.is_ident("rename") {
                        field_attrs.rename = handle_rename(&meta_name_value)?;
                    }
                }
            }
        }
    }

    Ok(field_attrs)
}

fn handle_rename(rename_meta: &MetaNameValue) -> anyhow::Result<Option<String>> {
    if let Expr::Lit(lit) = &rename_meta.value {
        if let Lit::Str(lit_str) = &lit.lit {
            return Ok(Some(lit_str.value()));
        }
    } else {
        return Err(anyhow::anyhow!("rename attribute must be a string literal"));
    }

    Ok(None)
}
