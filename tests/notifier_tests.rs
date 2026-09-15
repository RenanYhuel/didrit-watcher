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
        previous_cahier_text: None,
        current_cahier_text: Some("Ex 56 p 29".to_string()),
        previous_devoir_text: None,
        current_devoir_text: None,
        new_cours: Vec::new(),
    };

    let analysis = AiAnalysis {
        title: "Maths Expertes : Seance du 11/09".to_string(),
        fait_en_classe: Some("Cor. Ch.1 Ex. 27, 28 p 27 + Cours Ch.1-III".to_string()),
        a_faire: Some("Chercher Ch.1 Ex. 56, 61 p 29".to_string()),
        date_echeance: Some("Lundi 14/09".to_string()),
        evaluation: None,
        nouveaux_documents: vec!["DM1.pdf".to_string()],
        priority: 4,
    };

    let payload = notifier.build_payload(&page, &diff, &analysis);

    assert_eq!(payload.title, "Maths Expertes : Seance du 11/09");
    assert_eq!(payload.priority, 4);
    assert!(payload
        .message
        .contains("FAIT EN CLASSE :\nCor. Ch.1 Ex. 27, 28 p 27 + Cours Ch.1-III"));
    assert!(payload
        .message
        .contains("A FAIRE (pour le Lundi 14/09) :\nChercher Ch.1 Ex. 56, 61 p 29"));
    assert!(payload
        .message
        .contains("NOUVEAUX DOCUMENTS :\n- Devoir Maison 1 : https://www.didrit.fr/DM1.pdf"));
    assert_eq!(payload.actions.len(), 2);
    assert_eq!(payload.actions[0].url, "https://www.didrit.fr/Maths_TE.htm");
    assert_eq!(payload.actions[1].url, "https://www.didrit.fr/DM1.pdf");
}
