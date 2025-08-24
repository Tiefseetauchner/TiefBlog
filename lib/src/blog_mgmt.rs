use color_eyre::eyre::Result;

use crate::path_resolution::ErrorResult;

pub fn get_blog_post(blog_id: &str) -> Result<String, ErrorResult> {
    Ok(format!(
        r#"# Welcome to TiefBlog!

This is your first blog post. We have the id {}."#,
        blog_id
    )
    .to_string())
}
