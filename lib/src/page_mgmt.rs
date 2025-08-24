use color_eyre::eyre::Result;

use crate::path_resolution::ErrorResult;

pub fn get_home_page() -> Result<String, ErrorResult> {
    Ok(r#"# Welcome to TiefBlog!

This is your best blogging platform imaginable!"#
        .to_string())
}
