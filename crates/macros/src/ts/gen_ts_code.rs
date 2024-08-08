use super::ts_map::ts_rs_map;
use crate::{parsers::struc::ParsedField, struct_attrs::StructAttrs};

pub fn gen_ts_code(
    struct_name: &str,
    fields: &Vec<ParsedField>,
    struct_attrs: &StructAttrs,
) -> anyhow::Result<String> {
    let mut ts_bind = String::from(format!("\nexport interface {} {{\n", struct_name));
    let mut imports = Vec::new();
    for (ident, ty, attrs) in fields.iter() {
        if attrs.skip {
            continue;
        }

        let field_name = if let Some(rename_all) = struct_attrs.get_rename_all() {
            rename_all.to_case(&ident.to_string())
        } else {
            ident.to_string()
        };

        let field_name = attrs.rename.as_ref().unwrap_or(&field_name);

        let map_result = ts_rs_map(ty, &mut imports);

        ts_bind.push_str(&format!("   {}: {};\n", field_name, map_result));
    }

    ts_bind.push_str("}");

    sorter(&mut imports);
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

    Ok(ts_bind)
}

fn sorter(imports: &mut Vec<String>) {
    imports.sort();
    imports.dedup();
}
