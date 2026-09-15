use didrit_watcher::storage::AppState;

#[test]
fn test_storage_state_lifecycle() {
    let mut state = AppState::default();
    let url = "https://www.didrit.fr/Maths_TE.htm";

    state.update_page(
        url,
        "hash123".to_string(),
        vec!["https://www.didrit.fr/doc.pdf".to_string()],
        vec!["Complexes : Polynomes".to_string()],
        "cahier".to_string(),
        "devoir".to_string(),
    );

    let page = state.get_page(url).unwrap();
    assert_eq!(page.last_hash, "hash123");
    assert_eq!(page.known_attachments.len(), 1);
    assert_eq!(page.known_cours_titles.len(), 1);
    assert_eq!(page.last_cahier_text, "cahier");
    assert_eq!(page.last_devoir_text, "devoir");
}
