use super::model::{AttachmentLink, CahierEntry, CoursEntry, DevoirEntry, ParsedPage};
use scraper::{Html, Selector};
use sha2::{Digest, Sha256};
use url::Url;

pub struct HtmlExtractor;

impl HtmlExtractor {
    pub fn parse(html_content: &str, base_url_str: &str) -> ParsedPage {
        let document = Html::parse_document(html_content);
        let base_url = Url::parse(base_url_str).ok();

        let page_title = Self::extract_title(&document);
        let all_attachments = Self::extract_all_attachments(&document, &base_url);
        let cahier_entries = Self::extract_cahier_entries(&document, &base_url);
        let devoir_entries = Self::extract_devoir_entries(&document, &base_url);
        let cours_entries = Self::extract_cours_entries(&document, &base_url);

        let mut hasher = Sha256::new();
        hasher.update(html_content.as_bytes());
        let full_content_hash = format!("{:x}", hasher.finalize());

        ParsedPage {
            source_url: base_url_str.to_string(),
            page_title,
            cahier_entries,
            devoir_entries,
            cours_entries,
            all_attachments,
            full_content_hash,
        }
    }

    fn extract_title(document: &Html) -> String {
        let h1_selector = Selector::parse("h1").ok();
        if let Some(sel) = h1_selector {
            if let Some(h1) = document.select(&sel).next() {
                let text = h1.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !text.is_empty() {
                    return text;
                }
            }
        }

        let title_selector = Selector::parse("title").ok();
        if let Some(sel) = title_selector {
            if let Some(t) = document.select(&sel).next() {
                let text = t.text().collect::<Vec<_>>().join(" ").trim().to_string();
                if !text.is_empty() {
                    return text;
                }
            }
        }

        "Maths".to_string()
    }

    fn resolve_url(raw_href: &str, base_url: &Option<Url>) -> String {
        let trimmed = raw_href.trim();
        if trimmed.starts_with("http://") || trimmed.starts_with("https://") {
            return trimmed.to_string();
        }
        if let Some(base) = base_url {
            if let Ok(joined) = base.join(trimmed) {
                return joined.to_string();
            }
        }
        trimmed.to_string()
    }

    fn extract_all_attachments(document: &Html, base_url: &Option<Url>) -> Vec<AttachmentLink> {
        let a_selector = match Selector::parse("a[href]") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut attachments = Vec::new();
        for el in document.select(&a_selector) {
            if let Some(href) = el.value().attr("href") {
                let lower = href.to_lowercase();
                if lower.ends_with(".pdf")
                    || lower.ends_with(".doc")
                    || lower.ends_with(".docx")
                    || lower.ends_with(".zip")
                    || lower.ends_with(".py")
                {
                    let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                    let full_url = Self::resolve_url(href, base_url);
                    let label = if text.is_empty() {
                        full_url
                            .split('/')
                            .next_back()
                            .unwrap_or("document.pdf")
                            .to_string()
                    } else {
                        text
                    };

                    let link = AttachmentLink {
                        text: label,
                        url: full_url,
                    };
                    if !attachments.contains(&link) {
                        attachments.push(link);
                    }
                }
            }
        }
        attachments
    }

    fn extract_cahier_entries(document: &Html, base_url: &Option<Url>) -> Vec<CahierEntry> {
        let text_selector = match Selector::parse(".OEWEText, .OEPageXbody, div") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::new();
        for el in document.select(&text_selector) {
            let text = el.text().collect::<Vec<_>>().join(" ");
            let has_day = text.contains("Lundi")
                || text.contains("Mardi")
                || text.contains("Mercredi")
                || text.contains("Jeudi")
                || text.contains("Vendredi")
                || text.contains("Samedi");

            let has_school_context = text.contains("Ch.")
                || text.contains("Ex.")
                || text.contains("Cor.")
                || text.contains("Cours")
                || text.contains("Chercher");

            if has_day && has_school_context && text.len() > 30 {
                let mut links = Vec::new();
                let a_selector = Selector::parse("a[href]").ok();
                if let Some(a_sel) = a_selector {
                    for a_el in el.select(&a_sel) {
                        if let Some(href) = a_el.value().attr("href") {
                            let label =
                                a_el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                            let full_url = Self::resolve_url(href, base_url);
                            links.push(AttachmentLink {
                                text: if label.is_empty() {
                                    full_url.clone()
                                } else {
                                    label
                                },
                                url: full_url,
                            });
                        }
                    }
                }

                let cleaned_text = text.split_whitespace().collect::<Vec<_>>().join(" ");
                results.push(CahierEntry {
                    raw_text: cleaned_text,
                    links,
                });
                break;
            }
        }
        results
    }

    fn extract_devoir_entries(document: &Html, base_url: &Option<Url>) -> Vec<DevoirEntry> {
        let text_selector = match Selector::parse(".OEWEText, div") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::new();
        for el in document.select(&text_selector) {
            let text = el.text().collect::<Vec<_>>().join(" ");
            let has_devoir_keyword = text.contains("Devoir")
                || text.contains("Interrogation")
                || text.contains("Bac Blanc")
                || text.contains("BAC Jour");

            let has_date_or_placeholder = text.contains("2025")
                || text.contains("2026")
                || text.contains("2027")
                || text.contains("../../202");

            if has_devoir_keyword && has_date_or_placeholder && text.len() < 2000 {
                let mut links = Vec::new();
                let a_selector = Selector::parse("a[href]").ok();
                if let Some(a_sel) = a_selector {
                    for a_el in el.select(&a_sel) {
                        if let Some(href) = a_el.value().attr("href") {
                            let label =
                                a_el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                            let full_url = Self::resolve_url(href, base_url);
                            links.push(AttachmentLink {
                                text: if label.is_empty() {
                                    full_url.clone()
                                } else {
                                    label
                                },
                                url: full_url,
                            });
                        }
                    }
                }

                let cleaned_text = text.split_whitespace().collect::<Vec<_>>().join(" ");
                if !cleaned_text.is_empty() {
                    results.push(DevoirEntry {
                        raw_text: cleaned_text,
                        links,
                    });
                }
            }
        }
        results
    }

    fn extract_cours_entries(document: &Html, base_url: &Option<Url>) -> Vec<CoursEntry> {
        let li_selector = match Selector::parse("ol li, ul li") {
            Ok(s) => s,
            Err(_) => return Vec::new(),
        };

        let mut results = Vec::new();
        for el in document.select(&li_selector) {
            let text = el.text().collect::<Vec<_>>().join(" ").trim().to_string();
            if text.to_lowercase().contains("cours") || text.contains("Ch.") {
                let mut links = Vec::new();
                let a_selector = Selector::parse("a[href]").ok();
                if let Some(a_sel) = a_selector {
                    for a_el in el.select(&a_sel) {
                        if let Some(href) = a_el.value().attr("href") {
                            let label =
                                a_el.text().collect::<Vec<_>>().join(" ").trim().to_string();
                            let full_url = Self::resolve_url(href, base_url);
                            links.push(AttachmentLink {
                                text: if label.is_empty() {
                                    full_url.clone()
                                } else {
                                    label
                                },
                                url: full_url,
                            });
                        }
                    }
                }

                if !links.is_empty() {
                    results.push(CoursEntry { title: text, links });
                }
            }
        }
        results
    }
}
