use didrit_watcher::diff::DiffEngine;
use didrit_watcher::parser::{AttachmentLink, CahierEntry, DevoirEntry, ParsedPage};
use didrit_watcher::storage::PageState;

#[test]
fn test_diff_engine_first_run() {
    let current = ParsedPage {
        source_url: "https://www.didrit.fr/Maths_TE.htm".to_string(),
        page_title: "Maths".to_string(),
        cahier_entries: Vec::new(),
        devoir_entries: Vec::new(),
        cours_entries: Vec::new(),
        all_attachments: Vec::new(),
        full_content_hash: "hash123".to_string(),
    };

    let diff = DiffEngine::compute(None, &current);
    assert!(diff.is_first_run);
    assert!(!diff.has_changes);
}

#[test]
fn test_diff_engine_no_changes() {
    let previous = PageState {
        url: "https://www.didrit.fr/Maths_TE.htm".to_string(),
        last_hash: "hash123".to_string(),
        last_checked_at: "2026-09-13T10:00:00Z".to_string(),
        known_attachments: vec!["https://www.didrit.fr/doc.pdf".to_string()],
        known_cours_titles: Vec::new(),
        last_cahier_text: "Ex 1".to_string(),
        last_devoir_text: "Devoir 1".to_string(),
    };

    let current = ParsedPage {
        source_url: "https://www.didrit.fr/Maths_TE.htm".to_string(),
        page_title: "Maths".to_string(),
        cahier_entries: vec![CahierEntry {
            raw_text: "Ex 1".to_string(),
            links: Vec::new(),
        }],
        devoir_entries: vec![DevoirEntry {
            raw_text: "Devoir 1".to_string(),
            links: Vec::new(),
        }],
        cours_entries: Vec::new(),
        all_attachments: vec![AttachmentLink {
            text: "doc".to_string(),
            url: "https://www.didrit.fr/doc.pdf".to_string(),
        }],
        full_content_hash: "hash123".to_string(),
    };

    let diff = DiffEngine::compute(Some(&previous), &current);
    assert!(!diff.is_first_run);
    assert!(!diff.has_changes);
}

#[test]
fn test_diff_engine_detects_new_attachment_and_cahier_change() {
    let previous = PageState {
        url: "https://www.didrit.fr/Maths_TE.htm".to_string(),
        last_hash: "hash123".to_string(),
        last_checked_at: "2026-09-13T10:00:00Z".to_string(),
        known_attachments: vec!["https://www.didrit.fr/doc1.pdf".to_string()],
        known_cours_titles: Vec::new(),
        last_cahier_text: "Ex 1".to_string(),
        last_devoir_text: "Devoir 1".to_string(),
    };

    let current = ParsedPage {
        source_url: "https://www.didrit.fr/Maths_TE.htm".to_string(),
        page_title: "Maths".to_string(),
        cahier_entries: vec![CahierEntry {
            raw_text: "Ex 1 + Ex 2 pour lundi".to_string(),
            links: Vec::new(),
        }],
        devoir_entries: vec![DevoirEntry {
            raw_text: "Devoir 1".to_string(),
            links: Vec::new(),
        }],
        cours_entries: Vec::new(),
        all_attachments: vec![
            AttachmentLink {
                text: "doc1".to_string(),
                url: "https://www.didrit.fr/doc1.pdf".to_string(),
            },
            AttachmentLink {
                text: "doc2".to_string(),
                url: "https://www.didrit.fr/doc2.pdf".to_string(),
            },
        ],
        full_content_hash: "hash456".to_string(),
    };

    let diff = DiffEngine::compute(Some(&previous), &current);
    assert!(!diff.is_first_run);
    assert!(diff.has_changes);
    assert_eq!(diff.new_attachments.len(), 1);
    assert_eq!(
        diff.new_attachments[0].url,
        "https://www.didrit.fr/doc2.pdf"
    );
    assert!(diff.current_cahier_text.is_some());
}
