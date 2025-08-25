use color_eyre::eyre::Result;
use log::debug;

use crate::{blog_mgmt, page_mgmt::get_home_page};

pub type PageRetrievalResult = Result<ServerResponseResult>;
pub type PathResolverResult = Result<Fn(Option<&str>) -> PageRetrievalResult>;
pub type PathResolver = fn(&str) -> PathResolverResult;

pub struct ServerResponseResult {
    pub status_code: u16,
    pub content: String,
}

pub fn resolve_get_path(path: &str) -> PathResolverResult {
    debug!("Resolving GET path: {}", path);
    match path {
        p if p == "/" => Ok(|_| get_home_page()),
        p if p.starts_with("/blog") => blog_mgmt::get_blog_post_resolver(&p[5..]),
        _ => Err(color_eyre::eyre::eyre!("404 Not Found")), // You may want to handle this differently
    }
}

pub fn resolve_unknown_path(_path: &str) -> PathResolverResult {
    debug!("Resolving non-get method");
    Ok(|_| {
        Ok(ServerResponseResult {
            status_code: 404,
            content: r#"# Method Not Allowed

The method you used is not allowed. Please use GET."#
                .to_string(),
        })
    })
}
