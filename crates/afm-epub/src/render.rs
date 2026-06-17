//! Phase 2 — render afm sources into XHTML spine items.
//!
//! Each source byte buffer is decoded (UTF-8 by default; SJIS via
//! [`aozora_encoding::decode_sjis`] when the source path's extension
//! signals it) and handed to
//! [`afm_markdown::render_to_string`]. The resulting HTML is wrapped
//! in an XHTML 1.1 strict envelope with the manuscript language and
//! a stylesheet link the package phase will resolve.

use afm_markdown::{Options, render_to_string};

use crate::discover::{Manuscript, WritingMode};
use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct SpineItem {
    /// Filename used inside the EPUB, e.g. `chapter-001.xhtml`.
    pub href: String,
    /// `<title>` element of the chapter.
    pub title: String,
    /// Full XHTML document — already HTML-escaped by afm-markdown.
    pub xhtml: String,
}

#[derive(Debug, Clone)]
pub struct RenderOutput {
    pub items: Vec<SpineItem>,
}

pub fn render_all(manuscript: &Manuscript) -> Result<RenderOutput> {
    let opts = Options::afm_default();
    let mut items = Vec::with_capacity(manuscript.sources.len());
    for (idx, source) in manuscript.sources.iter().enumerate() {
        let text = decode_source(source)?;
        let rendered = render_to_string(&text, &opts);
        let title = source
            .path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_owned();
        let xhtml = wrap_xhtml(
            &title,
            &rendered.html,
            &manuscript.metadata.language,
            manuscript.metadata.writing_mode,
        );
        items.push(SpineItem {
            href: format!("chapter-{:03}.xhtml", idx + 1),
            title,
            xhtml,
        });
    }
    Ok(RenderOutput { items })
}

fn decode_source(source: &crate::discover::SourceFile) -> Result<String> {
    let ext = source
        .path
        .extension()
        .and_then(|e| e.to_str())
        .map(str::to_ascii_lowercase);
    if matches!(ext.as_deref(), Some("sjis" | "shift_jis" | "shift-jis")) {
        aozora_encoding::decode_sjis(&source.bytes).map_err(Error::from)
    } else {
        std::str::from_utf8(&source.bytes)
            .map(str::to_owned)
            .map_err(Error::from)
    }
}

fn wrap_xhtml(title: &str, body_html: &str, lang: &str, mode: WritingMode) -> String {
    let title = escape_attr(title);
    let lang = escape_attr(lang);
    let body_class = match mode {
        WritingMode::Horizontal => "afm-horizontal",
        WritingMode::Vertical => "afm-vertical",
    };
    format!(
        r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xmlns:epub="http://www.idpf.org/2007/ops" xml:lang="{lang}" lang="{lang}">
  <head>
    <meta charset="utf-8" />
    <title>{title}</title>
    <link rel="stylesheet" type="text/css" href="../css/afm.css" />
  </head>
  <body class="{body_class}">
{body_html}
  </body>
</html>
"#,
    )
}

fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
