use crate::path_resolution::{PageRetrievalResult, PathResolverResult, ServerResponseResult};

pub fn get_blog_post_resolver(blog_id: &str) -> PathResolverResult {
    Ok(|query| get_blog_post(blog_id, query))
}

pub fn get_blog_post(blog_id: &str, query: Option<&str>) -> PageRetrievalResult {
    Ok(ServerResponseResult {
        status_code: 200,
        content: format!(
            r#"# Welcome to TiefBlog!

This is your first blog post. We have the id {}. {}"#,
            blog_id,
            query.unwrap_or("")
        )
        .to_string(),
    })
}
