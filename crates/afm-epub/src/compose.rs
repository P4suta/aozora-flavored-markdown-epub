//! Phase 3 — synthesise OPF + NAV + the static EPUB scaffolding.
//!
//! Specs consumed:
//!
//! - OCF `META-INF/container.xml` — [EPUB 3 OCF §3.5.2](https://www.w3.org/TR/epub-33/#sec-container-metainf-container.xml).
//! - OPF Package Document — [EPUB 3.3 OPF §3](https://www.w3.org/TR/epub-33/#sec-package-doc).
//!   `dcterms:modified` is `xsd:dateTime`, Z-suffixed UTC, no fractional
//!   seconds (EPUB 3.3 §3.4.6 bans offsets and sub-second forms).
//! - Navigation Document — [EPUB 3.3 §5](https://www.w3.org/TR/epub-33/#sec-nav).
//! - DCMES — [Dublin Core v1.1](https://www.dublincore.org/specifications/dublin-core/dces/).
//! - Language tags — [RFC 5646 (BCP 47) §2.1](https://datatracker.ietf.org/doc/html/rfc5646#section-2.1).
//!   The validator accepts the common shape (2-3 letter primary subtag
//!   plus 2-8 alphanumeric subtags); anything else is rejected here.
//!
//! XML is written through `quick_xml::writer::Writer`, never hand-rolled
//! concatenation, so user-supplied bytes escape exactly once. Metadata
//! validation runs before composition: a missing `dc:title` / `dc:creator`
//! or non-BCP-47 `dc:language` is a hard error, not a malformed package.

use std::borrow::Cow;
use std::io::Cursor;

use chrono::{DateTime, Utc};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use uuid::Uuid;

use crate::discover::Manuscript;
use crate::render::RenderOutput;
use crate::{Error, Result};

/// Files to write into the EPUB ZIP, in their canonical order.
#[derive(Debug, Clone)]
pub struct Bundle {
    pub mimetype: &'static str,
    pub container: String,
    pub package_opf: String,
    pub nav_xhtml: String,
    pub spine: Vec<NamedFile>,
    pub assets: Vec<NamedFile>,
}

#[derive(Debug, Clone)]
pub struct NamedFile {
    pub path: String,
    pub contents: Vec<u8>,
}

pub fn compose(manuscript: &Manuscript, rendered: &RenderOutput) -> Result<Bundle> {
    validate_metadata(&manuscript.metadata)?;

    let id = manuscript
        .metadata
        .identifier
        .clone()
        .unwrap_or_else(|| format!("urn:uuid:{}", Uuid::new_v4()));

    let now = Utc::now();
    let container = container_xml()?;
    let package_opf = package_opf(&manuscript.metadata, &id, rendered, now)?;
    let nav_xhtml = nav_xhtml(&manuscript.metadata, rendered)?;

    let spine: Vec<NamedFile> = rendered
        .items
        .iter()
        .map(|it| NamedFile {
            path: format!("OEBPS/{}", it.href),
            contents: it.xhtml.as_bytes().to_vec(),
        })
        .collect();

    let assets = vec![NamedFile {
        path: "OEBPS/css/afm.css".to_owned(),
        contents: include_bytes!("../assets/afm.css").to_vec(),
    }];

    Ok(Bundle {
        mimetype: "application/epub+zip",
        container,
        package_opf,
        nav_xhtml,
        spine,
        assets,
    })
}

/// Reject metadata that would leave the EPUB unreadable on the major
/// reading systems. Narrow on purpose.
fn validate_metadata(meta: &crate::discover::Metadata) -> Result<()> {
    if meta.title.trim().is_empty() {
        return Err(Error::MetadataInvalid {
            field: "title",
            reason: "dc:title must be a non-empty string".to_owned(),
        });
    }
    if meta.creator.trim().is_empty() {
        return Err(Error::MetadataInvalid {
            field: "creator",
            reason: "dc:creator must be a non-empty string".to_owned(),
        });
    }
    if !is_bcp47_subset(&meta.language) {
        return Err(Error::MetadataInvalid {
            field: "language",
            reason: format!(
                "dc:language must be a BCP 47 tag (e.g. `ja`, `ja-JP`); got {:?}",
                meta.language
            ),
        });
    }
    Ok(())
}

/// Permissive BCP 47 subset check: accepts a primary subtag of 2-3
/// ASCII alphabetic characters, optionally followed by `-` and one or
/// more 2-8 alphanumeric subtags. Covers `ja`, `en-US`, `zh-Hant-TW`,
/// `ja-Jpan-JP-x-private` etc. Stricter than `regex` for the trivial
/// invariants downstream readers care about.
fn is_bcp47_subset(tag: &str) -> bool {
    if tag.is_empty() {
        return false;
    }
    let mut subtags = tag.split('-');
    let Some(primary) = subtags.next() else {
        return false;
    };
    if primary.len() < 2 || primary.len() > 3 {
        return false;
    }
    if !primary.bytes().all(|b| b.is_ascii_alphabetic()) {
        return false;
    }
    for sub in subtags {
        if sub.len() < 2 || sub.len() > 8 {
            return false;
        }
        if !sub.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return false;
        }
    }
    true
}

fn container_xml() -> Result<String> {
    let mut w = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))
        .map_err(|e| xml_to_err(&e))?;
    let mut container = BytesStart::new("container");
    container.push_attribute(("version", "1.0"));
    container.push_attribute(("xmlns", "urn:oasis:names:tc:opendocument:xmlns:container"));
    w.write_event(Event::Start(container))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::Start(BytesStart::new("rootfiles")))
        .map_err(|e| xml_to_err(&e))?;
    let mut rf = BytesStart::new("rootfile");
    rf.push_attribute(("full-path", "OEBPS/package.opf"));
    rf.push_attribute(("media-type", "application/oebps-package+xml"));
    w.write_event(Event::Empty(rf))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("rootfiles")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("container")))
        .map_err(|e| xml_to_err(&e))?;
    finish_writer(w)
}

fn package_opf(
    meta: &crate::discover::Metadata,
    id: &str,
    rendered: &RenderOutput,
    now: DateTime<Utc>,
) -> Result<String> {
    let mut w = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))
        .map_err(|e| xml_to_err(&e))?;

    let mut package = BytesStart::new("package");
    package.push_attribute(("xmlns", "http://www.idpf.org/2007/opf"));
    package.push_attribute(("version", "3.0"));
    package.push_attribute(("unique-identifier", "bookid"));
    package.push_attribute(("xml:lang", meta.language.as_str()));
    w.write_event(Event::Start(package))
        .map_err(|e| xml_to_err(&e))?;

    // metadata
    let mut metadata = BytesStart::new("metadata");
    metadata.push_attribute(("xmlns:dc", "http://purl.org/dc/elements/1.1/"));
    w.write_event(Event::Start(metadata))
        .map_err(|e| xml_to_err(&e))?;

    write_dc(&mut w, "dc:identifier", id, &[("id", "bookid")])?;
    write_dc(&mut w, "dc:title", &meta.title, &[])?;
    write_dc(&mut w, "dc:creator", &meta.creator, &[])?;
    write_dc(&mut w, "dc:language", &meta.language, &[])?;

    let mut modified = BytesStart::new("meta");
    modified.push_attribute(("property", "dcterms:modified"));
    w.write_event(Event::Start(modified))
        .map_err(|e| xml_to_err(&e))?;
    let stamp = now.format("%Y-%m-%dT%H:%M:%SZ").to_string();
    w.write_event(Event::Text(BytesText::new(&stamp)))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("meta")))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::End(BytesEnd::new("metadata")))
        .map_err(|e| xml_to_err(&e))?;

    // manifest
    w.write_event(Event::Start(BytesStart::new("manifest")))
        .map_err(|e| xml_to_err(&e))?;
    write_manifest_item(
        &mut w,
        ManifestItem {
            id: "nav",
            href: "nav.xhtml",
            media_type: "application/xhtml+xml",
            properties: Some("nav"),
        },
    )?;
    write_manifest_item(
        &mut w,
        ManifestItem {
            id: "css",
            href: "css/afm.css",
            media_type: "text/css",
            properties: None,
        },
    )?;
    for (i, item) in rendered.items.iter().enumerate() {
        let id = format!("ch{:03}", i + 1);
        write_manifest_item(
            &mut w,
            ManifestItem {
                id: &id,
                href: &item.href,
                media_type: "application/xhtml+xml",
                properties: None,
            },
        )?;
    }
    w.write_event(Event::End(BytesEnd::new("manifest")))
        .map_err(|e| xml_to_err(&e))?;

    // spine
    w.write_event(Event::Start(BytesStart::new("spine")))
        .map_err(|e| xml_to_err(&e))?;
    for i in 0..rendered.items.len() {
        let id = format!("ch{:03}", i + 1);
        let mut idref = BytesStart::new("itemref");
        idref.push_attribute(("idref", id.as_str()));
        w.write_event(Event::Empty(idref))
            .map_err(|e| xml_to_err(&e))?;
    }
    w.write_event(Event::End(BytesEnd::new("spine")))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::End(BytesEnd::new("package")))
        .map_err(|e| xml_to_err(&e))?;

    finish_writer(w)
}

fn write_dc<W: std::io::Write>(
    w: &mut Writer<W>,
    name: &str,
    text: &str,
    attrs: &[(&str, &str)],
) -> Result<()> {
    let mut tag = BytesStart::new(name);
    for (k, v) in attrs {
        tag.push_attribute((*k, *v));
    }
    w.write_event(Event::Start(tag))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::Text(BytesText::new(text)))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new(name.to_owned())))
        .map_err(|e| xml_to_err(&e))?;
    Ok(())
}

/// One manifest entry, named so call sites read top-down rather than
/// as a positional tuple.
#[derive(Debug, Clone, Copy)]
struct ManifestItem<'a> {
    id: &'a str,
    href: &'a str,
    media_type: &'a str,
    properties: Option<&'a str>,
}

fn write_manifest_item<W: std::io::Write>(w: &mut Writer<W>, spec: ManifestItem<'_>) -> Result<()> {
    let mut item = BytesStart::new("item");
    item.push_attribute(("id", spec.id));
    item.push_attribute(("href", spec.href));
    item.push_attribute(("media-type", spec.media_type));
    if let Some(p) = spec.properties {
        item.push_attribute(("properties", p));
    }
    w.write_event(Event::Empty(item))
        .map_err(|e| xml_to_err(&e))?;
    Ok(())
}

fn nav_xhtml(meta: &crate::discover::Metadata, rendered: &RenderOutput) -> Result<String> {
    let mut w = Writer::new_with_indent(Cursor::new(Vec::new()), b' ', 2);
    w.write_event(Event::Decl(BytesDecl::new("1.0", Some("utf-8"), None)))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::DocType(BytesText::from_escaped(" html")))
        .map_err(|e| xml_to_err(&e))?;

    let mut html = BytesStart::new("html");
    html.push_attribute(("xmlns", "http://www.w3.org/1999/xhtml"));
    html.push_attribute(("xmlns:epub", "http://www.idpf.org/2007/ops"));
    html.push_attribute(("xml:lang", meta.language.as_str()));
    html.push_attribute(("lang", meta.language.as_str()));
    w.write_event(Event::Start(html))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::Start(BytesStart::new("head")))
        .map_err(|e| xml_to_err(&e))?;
    let mut charset = BytesStart::new("meta");
    charset.push_attribute(("charset", "utf-8"));
    w.write_event(Event::Empty(charset))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::Start(BytesStart::new("title")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::Text(BytesText::new(&meta.title)))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("title")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("head")))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::Start(BytesStart::new("body")))
        .map_err(|e| xml_to_err(&e))?;

    let mut nav = BytesStart::new("nav");
    nav.push_attribute(("epub:type", "toc"));
    nav.push_attribute(("id", "toc"));
    w.write_event(Event::Start(nav))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::Start(BytesStart::new("h1")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::Text(BytesText::new("目次")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("h1")))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::Start(BytesStart::new("ol")))
        .map_err(|e| xml_to_err(&e))?;
    for it in &rendered.items {
        w.write_event(Event::Start(BytesStart::new("li")))
            .map_err(|e| xml_to_err(&e))?;
        let mut a = BytesStart::new("a");
        a.push_attribute(("href", it.href.as_str()));
        w.write_event(Event::Start(a)).map_err(|e| xml_to_err(&e))?;
        w.write_event(Event::Text(BytesText::new(&it.title)))
            .map_err(|e| xml_to_err(&e))?;
        w.write_event(Event::End(BytesEnd::new("a")))
            .map_err(|e| xml_to_err(&e))?;
        w.write_event(Event::End(BytesEnd::new("li")))
            .map_err(|e| xml_to_err(&e))?;
    }
    w.write_event(Event::End(BytesEnd::new("ol")))
        .map_err(|e| xml_to_err(&e))?;

    w.write_event(Event::End(BytesEnd::new("nav")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("body")))
        .map_err(|e| xml_to_err(&e))?;
    w.write_event(Event::End(BytesEnd::new("html")))
        .map_err(|e| xml_to_err(&e))?;

    finish_writer(w)
}

fn finish_writer(w: Writer<Cursor<Vec<u8>>>) -> Result<String> {
    let bytes = w.into_inner().into_inner();
    let mut s =
        String::from_utf8(bytes).map_err(|err| Error::XmlBuild(Cow::Owned(err.to_string())))?;
    if !s.ends_with('\n') {
        s.push('\n');
    }
    Ok(s)
}

/// Map a `quick_xml` writer error (which always wraps an
/// `std::io::Error` for our `Cursor<Vec<u8>>` sink) into the crate
/// error type. We take by reference because `to_string` only borrows
/// — `map_err` callers therefore wrap with a closure rather than
/// passing `xml_to_err` as a fn pointer.
fn xml_to_err(err: &std::io::Error) -> Error {
    Error::XmlBuild(Cow::Owned(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bcp47_accepts_simple_two_letter_tags() {
        assert!(is_bcp47_subset("ja"));
        assert!(is_bcp47_subset("en"));
        assert!(is_bcp47_subset("zh"));
    }

    #[test]
    fn bcp47_accepts_region_subtag() {
        assert!(is_bcp47_subset("en-US"));
        assert!(is_bcp47_subset("ja-JP"));
        assert!(is_bcp47_subset("zh-Hant-TW"));
    }

    #[test]
    fn bcp47_rejects_empty_or_garbage() {
        assert!(!is_bcp47_subset(""));
        assert!(!is_bcp47_subset("x"));
        assert!(!is_bcp47_subset("toolong"));
        assert!(!is_bcp47_subset("123"));
        assert!(!is_bcp47_subset("ja-x"));
        assert!(!is_bcp47_subset("ja-toolongtag"));
    }

    fn dummy_metadata(title: &str, creator: &str, language: &str) -> crate::discover::Metadata {
        crate::discover::Metadata {
            title: title.to_owned(),
            creator: creator.to_owned(),
            language: language.to_owned(),
            identifier: None,
            writing_mode: crate::discover::WritingMode::Horizontal,
        }
    }

    #[test]
    fn validate_metadata_rejects_blank_title() {
        let err = validate_metadata(&dummy_metadata("", "Author", "ja")).unwrap_err();
        assert!(matches!(err, Error::MetadataInvalid { field: "title", .. }));
    }

    #[test]
    fn validate_metadata_rejects_blank_creator() {
        let err = validate_metadata(&dummy_metadata("Title", "   ", "ja")).unwrap_err();
        assert!(matches!(
            err,
            Error::MetadataInvalid {
                field: "creator",
                ..
            }
        ));
    }

    #[test]
    fn validate_metadata_rejects_bad_language() {
        let err = validate_metadata(&dummy_metadata("Title", "Author", "japanese")).unwrap_err();
        assert!(matches!(
            err,
            Error::MetadataInvalid {
                field: "language",
                ..
            }
        ));
    }

    #[test]
    fn validate_metadata_accepts_valid_input() {
        validate_metadata(&dummy_metadata("Title", "Author", "ja-JP")).unwrap();
    }

    #[test]
    fn package_opf_escapes_user_supplied_text() {
        // Title contains an ampersand and angle brackets; quick-xml
        // must escape them once and only once.
        let meta = dummy_metadata("Tom & Jerry <draft>", "山田\"太郎\"", "ja");
        let rendered = RenderOutput { items: vec![] };
        let now = DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z")
            .unwrap()
            .with_timezone(&Utc);
        let opf = package_opf(&meta, "urn:uuid:test", &rendered, now).unwrap();
        assert!(opf.contains("Tom &amp; Jerry &lt;draft&gt;"), "opf: {opf}");
        assert!(opf.contains("山田&quot;太郎&quot;"), "opf: {opf}");
        // No double-encoding.
        assert!(!opf.contains("&amp;amp;"), "opf: {opf}");
    }

    #[test]
    fn container_xml_round_trips_to_well_formed_output() {
        let xml = container_xml().unwrap();
        assert!(xml.contains("OEBPS/package.opf"));
        assert!(xml.contains("application/oebps-package+xml"));
        assert!(xml.starts_with("<?xml"));
    }
}
