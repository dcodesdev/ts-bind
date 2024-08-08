use crate::rename_all::RenameAll;
use std::path::PathBuf;

#[derive(Default, Debug)]
pub struct StructAttrs {
    pub rename_all: Option<RenameAll>,
    pub rename: Option<String>,
    pub export: Option<PathBuf>,
}

impl StructAttrs {
    pub fn new() -> Self {
        Self::default()
    }
}
