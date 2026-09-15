use crate::parser::{AttachmentLink, CoursEntry, ParsedPage};
use crate::storage::PageState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffResult {
    pub has_changes: bool,
    pub is_first_run: bool,
    pub new_attachments: Vec<AttachmentLink>,
    pub previous_cahier_text: Option<String>,
    pub current_cahier_text: Option<String>,
    pub previous_devoir_text: Option<String>,
    pub current_devoir_text: Option<String>,
    pub new_cours: Vec<CoursEntry>,
}

pub struct DiffEngine;

impl DiffEngine {
    pub fn compute(previous: Option<&PageState>, current: &ParsedPage) -> DiffResult {
        let prev = match previous {
            Some(p) => p,
            None => {
                return DiffResult {
                    has_changes: false,
                    is_first_run: true,
                    new_attachments: Vec::new(),
                    previous_cahier_text: None,
                    current_cahier_text: None,
                    previous_devoir_text: None,
                    current_devoir_text: None,
                    new_cours: Vec::new(),
                };
            }
        };

        if prev.last_hash == current.full_content_hash {
            return DiffResult {
                has_changes: false,
                is_first_run: false,
                new_attachments: Vec::new(),
                previous_cahier_text: None,
                current_cahier_text: None,
                previous_devoir_text: None,
                current_devoir_text: None,
                new_cours: Vec::new(),
            };
        }

        let new_attachments: Vec<AttachmentLink> = current
            .all_attachments
            .iter()
            .filter(|att| !prev.known_attachments.contains(&att.url))
            .cloned()
            .collect();

        let current_cahier_text = current
            .cahier_entries
            .iter()
            .map(|e| e.raw_text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let (cahier_prev, cahier_curr) = if current_cahier_text != prev.last_cahier_text {
            (
                Some(prev.last_cahier_text.clone()),
                Some(current_cahier_text),
            )
        } else {
            (None, None)
        };

        let current_devoir_text = current
            .devoir_entries
            .iter()
            .map(|e| e.raw_text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let (devoir_prev, devoir_curr) = if current_devoir_text != prev.last_devoir_text {
            (
                Some(prev.last_devoir_text.clone()),
                Some(current_devoir_text),
            )
        } else {
            (None, None)
        };

        let new_cours: Vec<CoursEntry> = current
            .cours_entries
            .iter()
            .filter(|c| !prev.known_cours_titles.contains(&c.title))
            .cloned()
            .collect();

        let has_changes = !new_attachments.is_empty()
            || cahier_curr.is_some()
            || devoir_curr.is_some()
            || !new_cours.is_empty();

        DiffResult {
            has_changes,
            is_first_run: false,
            new_attachments,
            previous_cahier_text: cahier_prev,
            current_cahier_text: cahier_curr,
            previous_devoir_text: devoir_prev,
            current_devoir_text: devoir_curr,
            new_cours,
        }
    }
}
