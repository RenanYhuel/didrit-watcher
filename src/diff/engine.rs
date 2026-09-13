use crate::parser::{AttachmentLink, CoursEntry, ParsedPage};
use crate::storage::PageState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffResult {
    pub has_changes: bool,
    pub is_first_run: bool,
    pub new_attachments: Vec<AttachmentLink>,
    pub cahier_diff_text: Option<String>,
    pub devoir_diff_text: Option<String>,
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
                    cahier_diff_text: None,
                    devoir_diff_text: None,
                    new_cours: Vec::new(),
                };
            }
        };

        if prev.last_hash == current.full_content_hash {
            return DiffResult {
                has_changes: false,
                is_first_run: false,
                new_attachments: Vec::new(),
                cahier_diff_text: None,
                devoir_diff_text: None,
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

        let cahier_diff = if current_cahier_text != prev.last_cahier_text {
            Some(current_cahier_text)
        } else {
            None
        };

        let current_devoir_text = current
            .devoir_entries
            .iter()
            .map(|e| e.raw_text.as_str())
            .collect::<Vec<_>>()
            .join("\n");

        let devoir_diff = if current_devoir_text != prev.last_devoir_text {
            Some(current_devoir_text)
        } else {
            None
        };

        let has_changes =
            !new_attachments.is_empty() || cahier_diff.is_some() || devoir_diff.is_some();

        DiffResult {
            has_changes,
            is_first_run: false,
            new_attachments,
            cahier_diff_text: cahier_diff,
            devoir_diff_text: devoir_diff,
            new_cours: current.cours_entries.clone(),
        }
    }
}
