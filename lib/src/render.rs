use color_eyre::eyre::Result;
use comrak::{ComrakOptions, markdown_to_html};

pub fn render_page(markdown: &str) -> Result<String> {
    let mut options = ComrakOptions::default();
    // options.extension.strikethrough = true;
    // options.extension.table = true;
    // options.extension.autolink = true;
    // options.extension.tasklist = true;
    options.extension.superscript = true;
    // options.extension.footnotes = true;

    Ok(markdown_to_html(markdown, &options))
}
