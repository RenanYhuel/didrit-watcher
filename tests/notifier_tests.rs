use didrit_watcher::ai::AiAnalysis;
use didrit_watcher::diff::DiffResult;
use didrit_watcher::notifier::{NtfyNotifier, NtfyPayload};
use didrit_watcher::parser::{AttachmentLink, ParsedPage};

#[test]
fn test_ntfy_payload_builder() {
    let payload = NtfyPayload::new("didrit", "Test Title", "Test Message")
        .with_priority(5)
        .with_tags(vec!["school".to_string(), "warning".to_string()])
        .with_click("https://www.didrit.fr/Maths_TE.htm")
        .with_attachment("https://www.didrit.fr/devoir.pdf", Some("Devoir 1.pdf"))
        .add_action("Ouvrir", "https://www.didrit.fr/Maths_TE.htm")
        .add_action("Voir PDF", "https://www.didrit.fr/devoir.pdf");

    assert_eq!(payload.topic, "didrit");
    assert_eq!(payload.title, "Test Title");
    assert_eq!(payload.priority, 5);
    assert_eq!(payload.actions.len(), 2);
    assert_eq!(payload.actions[0].label, "Ouvrir");
    assert_eq!(payload.actions[1].label, "Voir PDF");
    assert_eq!(
        payload.click.as_deref(),
        Some("https://www.didrit.fr/Maths_TE.htm")
    );
    assert_eq!(
        payload.attach.as_deref(),
        Some("https://www.didrit.fr/devoir.pdf")
    );
}

#[test]
fn test_notifier_build_payload_from_analysis() {
    let notifier =
        NtfyNotifier::new("https://ntfy.sh".to_string(), "didrit".to_string(), None).unwrap();

    let page = ParsedPage {
        source_url: "https://www.didrit.fr/Maths_TE.htm".to_string(),
        page_title: "Terminale Maths Expertes".to_string(),
        cahier_entries: Vec::new(),
        devoir_entries: Vec::new(),
        cours_entries: Vec::new(),
        all_attachments: Vec::new(),
        full_content_hash: "hash".to_string(),
    };

    let diff = DiffResult {
        has_changes: true,
        is_first_run: false,
        new_attachments: vec![AttachmentLink {
            text: "Devoir Maison 1".to_string(),
            url: "https://www.didrit.fr/DM1.pdf".to_string(),
        }],
        cahier_diff_text: Some("Ex 56 p 29".to_string()),
        devoir_diff_text: None,
        new_cours: Vec::new(),
    };

    let analysis = AiAnalysis {
        title: "Nouveau devoir pour lundi".to_string(),
        summary: "Deux exercices sur les complexes a faire.".to_string(),
        homework_items: vec!["Ex 56 p 29 pour lundi".to_string()],
        new_documents: vec!["DM1.pdf".to_string()],
        priority: 4,
    };

    let payload = notifier.build_payload(&page, &diff, &analysis);

    assert_eq!(payload.title, "Nouveau devoir pour lundi");
    assert_eq!(payload.priority, 4);
    assert!(payload.tags.contains(&"warning".to_string()));
    assert!(payload.message.contains("Ex 56 p 29 pour lundi"));
    assert_eq!(payload.actions.len(), 2);
    assert_eq!(payload.actions[0].url, "https://www.didrit.fr/Maths_TE.htm");
    assert_eq!(payload.actions[1].url, "https://www.didrit.fr/DM1.pdf");
}
