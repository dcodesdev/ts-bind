use quote::quote;
use syn::Type;

#[derive(Debug)]
pub struct TsRsMap {
    pub mapped: String,
    pub imports: Vec<String>,
}

pub fn ts_rs_map(token: &Type) -> TsRsMap {
    let mut imports = Vec::new();

    let mapped = match token {
        Type::Path(type_path) if is_option(type_path) => {
            let inner = unwrap_option(type_path);
            let inner_mapped = ts_rs_map(&inner);
            imports.extend(inner_mapped.imports);
            format!("{} | null", inner_mapped.mapped)
        }
        Type::Path(type_path) if type_path.path.is_ident("String") => "string".to_string(),
        Type::Reference(type_ref) => {
            let ty = type_ref.elem.as_ref();

            match ty {
                Type::Path(type_path) if type_path.path.is_ident("str") => "string".to_string(),
                _ => {
                    let map_result = ts_rs_map(ty);
                    imports.extend(map_result.imports);
                    map_result.mapped
                }
            }
        }
        Type::Path(type_path)
            if [
                "i8", "i16", "i32", "i64", "i128", "u8", "u16", "u32", "u64", "u128", "f32", "f64",
                "usize", "isize",
            ]
            .contains(
                &type_path
                    .path
                    .get_ident()
                    .map(|i| i.to_string())
                    .unwrap_or_default()
                    .as_str(),
            ) =>
        {
            "number".to_string()
        }
        Type::Path(type_path) if type_path.path.is_ident("bool") => "boolean".to_string(),
        Type::Path(type_path) if is_vec(type_path) => {
            let inner = unwrap_vec(type_path);
            let inner_mapped = ts_rs_map(&inner);
            imports.extend(inner_mapped.imports);
            format!("({})[]", inner_mapped.mapped)
        }
        Type::Path(type_path) if is_hashmap(type_path) => {
            let (key, value) = unwrap_hashmap(type_path);
            let key_mapped = ts_rs_map(&key);
            let value_mapped = ts_rs_map(&value);
            imports.extend(key_mapped.imports);

            format!(
                "{{ [key: {}]: {} }}",
                key_mapped.mapped, value_mapped.mapped
            )
        }
        _ => {
            let value = quote! {#token}.to_string();
            imports.push(value.clone());
            value
        }
    };

    TsRsMap { mapped, imports }
}

fn is_option(type_path: &syn::TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "Option")
        .unwrap_or(false)
}

fn unwrap_option(type_path: &syn::TypePath) -> syn::Type {
    let last_segment = type_path
        .path
        .segments
        .last()
        .expect("Could not get last segment");
    if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return ty.clone();
        }
    }
    panic!("Could not unwrap option");
}

fn is_vec(type_path: &syn::TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "Vec")
        .unwrap_or(false)
}

fn unwrap_vec(type_path: &syn::TypePath) -> syn::Type {
    let last_segment = type_path
        .path
        .segments
        .last()
        .expect("Could not get last segment");
    if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
        if let Some(syn::GenericArgument::Type(ty)) = args.args.first() {
            return ty.clone();
        }
    }
    panic!("Could not unwrap Vec");
}

fn is_hashmap(type_path: &syn::TypePath) -> bool {
    type_path
        .path
        .segments
        .last()
        .map(|seg| seg.ident == "HashMap")
        .unwrap_or(false)
}

fn unwrap_hashmap(type_path: &syn::TypePath) -> (syn::Type, syn::Type) {
    let last_segment = type_path
        .path
        .segments
        .last()
        .expect("Could not get last segment");
    if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
        if args.args.len() == 2 {
            if let (
                Some(syn::GenericArgument::Type(key)),
                Some(syn::GenericArgument::Type(value)),
            ) = (args.args.first(), args.args.get(1))
            {
                return (key.clone(), value.clone());
            }
        }
    }
    panic!("Could not unwrap HashMap");
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, Type};

    #[test]
    fn test_handle_option() {
        let ty: Type = parse_quote!(Option<String>);
        let result = ts_rs_map(&ty).mapped;
        assert_eq!(result, "string | null");

        let ty: Type = parse_quote!(Option<Option<String>>);
        let result = ts_rs_map(&ty).mapped;
        assert_eq!(result, "string | null | null");
    }

    #[test]
    fn test_unwrap_option() {
        let ty: Type = parse_quote!(Option<String>);
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            _ => panic!("Could not get type path"),
        };
        let result = unwrap_option(&type_path);
        if let Type::Path(type_path) = result {
            assert_eq!(type_path.path.get_ident().unwrap().to_string(), "String");
        } else {
            panic!("Could not get type path");
        }
    }

    #[test]
    fn test_is_option() {
        let ty: Type = parse_quote!(Option<String>);
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            _ => panic!("Could not get type path"),
        };
        let result = is_option(&type_path);
        assert_eq!(result, true);

        let ty: Type = parse_quote!(String);
        let type_path = match ty {
            Type::Path(type_path) => type_path,
            _ => panic!("Could not get type path"),
        };
        let result = is_option(&type_path);
        assert_eq!(result, false);
    }

    #[test]
    fn test_vector_map() {
        let ty: Type = parse_quote!(Vec<String>);
        let result = ts_rs_map(&ty).mapped;
        assert_eq!(result, "(string)[]");
    }

    #[test]
    fn test_map_map() {
        let ty: Type = parse_quote!(std::collections::HashMap<String, String>);
        let result = ts_rs_map(&ty).mapped;
        assert_eq!(result, "{ [key: string]: string }");
    }

    #[test]
    fn test_str_ref() {
        {
            let ty: Type = parse_quote!(&str);
            let result = ts_rs_map(&ty).mapped;
            assert_eq!(result, "string");
        }

        {
            // with lifetime
            let ty: Type = parse_quote!(&'a str);
            let result = ts_rs_map(&ty).mapped;
            assert_eq!(result, "string");
        }
    }

    #[test]
    fn test_imports() {
        {
            let ty: Type = parse_quote!(Vec<Posts>);
            let result = ts_rs_map(&ty).imports;
            assert_eq!(result, vec!["Posts".to_string()]);
        }

        {
            let ty: Type = parse_quote!(Option<String>);
            let result = ts_rs_map(&ty).imports;

            assert_eq!(result, Vec::<String>::new());
        }

        {
            let ty: Type = parse_quote!(Users);
            let result = ts_rs_map(&ty).imports;
            assert_eq!(result, vec!["Users".to_string()]);
        }
    }
}
