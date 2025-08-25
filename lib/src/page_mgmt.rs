use crate::path_resolution::{PageRetrievalResult, ServerResponseResult};

pub fn get_home_page() -> PageRetrievalResult {
    Ok(ServerResponseResult {
        status_code: 200,
        content: r#"# Welcome to TiefBlog!

This is your best blogging platform imaginable!"#
            .to_string(),
    })
}
