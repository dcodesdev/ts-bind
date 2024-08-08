use std::{
    fs::{create_dir_all, write},
    path::PathBuf,
};

pub fn write_to_file(path: &PathBuf, content: &str) -> anyhow::Result<()> {
    let parent = path.parent().ok_or(anyhow::anyhow!(
        "Failed to get parent directory of path: {}",
        path.display()
    ))?;

    create_dir_all(parent)?;

    write(path, content)?;

    Ok(())
}
