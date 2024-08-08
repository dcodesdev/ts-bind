use convert_case::{Case, Casing};

#[derive(Debug)]
pub enum RenameAll {
    CamelCase,
    SnakeCase,
    UpperCase,
    LowerCase,
    PascalCase,
    // TODO: kebab
    //KebabCase,
}

impl RenameAll {
    pub fn to_case(&self, s: &str) -> String {
        match self {
            Self::CamelCase => s.to_case(Case::Camel),
            Self::SnakeCase => s.to_case(Case::Snake),
            Self::UpperCase => s.to_case(Case::Upper),
            Self::LowerCase => s.to_case(Case::Lower),
            Self::PascalCase => s.to_case(Case::Pascal),
        }
    }
}
