use color_eyre::eyre::Result;
use log::debug;

use crate::{blog_mgmt, page_mgmt::get_home_page, render::render_page};

pub fn resolve_get_path(path: &str) -> Result<String, ErrorResult> {
    debug!("Resolving GET path: {}", path);
    match path {
        p if p == "/" => get_home_page(),
        p if p.starts_with("/blog") => blog_mgmt::get_blog_post(&p[5..]),
        _ => Err(ErrorResult {
            message: format!("Path not found: {}", path),
            status_code: 404,
        }),
    }
}

pub fn resolve_unknown_path(_path: &str) -> Result<String, ErrorResult> {
    debug!("Resolving non-get method");
    Ok(render_page(
        r#"# Method Not Allowed

The method you used is not allowed. Please use GET."#,
    )?)
}

pub type PathResolver = fn(&str) -> Result<String, ErrorResult>;

pub struct ErrorResult {
    pub status_code: u16,
    pub message: String,
}
