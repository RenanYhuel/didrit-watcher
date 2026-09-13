use didrit_watcher::parser::{AttachmentLink, HtmlExtractor};

#[test]
fn test_html_extractor_basic() {
    let html = r#"
        <!DOCTYPE html>
        <html>
        <head><title>Maths Expertes</title></head>
        <body>
            <h1>Terminale Option Maths Expertes</h1>
            <div class="OEWEText">
                Lundi 14/09/26 : Vendredi 11/09/26 : Cor. Ch.1 Ex. n 27 -> Chercher Ch.1 Ex. n 56 pour L. 14/09
                <a href="Files/Other/Cours/TE_C01.pdf">cours</a>
            </div>
            <div class="OEWEText">
                17/10/2026 Devoir - 2h - Histogramme
                <a href="Files/Other/Devoirs/2026_TE_D01.pdf">Devoir</a>
            </div>
            <ol>
                <li>Suites. <a href="Files/Other/Cours/TS_C01.pdf">cours</a></li>
            </ol>
        </body>
        </html>
    "#;

    let base_url = "https://www.didrit.fr/Maths_TE.htm";
    let parsed = HtmlExtractor::parse(html, base_url);

    assert_eq!(parsed.page_title, "Terminale Option Maths Expertes");
    assert!(!parsed.full_content_hash.is_empty());
    assert!(!parsed.cahier_entries.is_empty());
    assert!(!parsed.devoir_entries.is_empty());
    assert_eq!(parsed.all_attachments.len(), 3);

    assert!(parsed.all_attachments.contains(&AttachmentLink {
        text: "cours".to_string(),
        url: "https://www.didrit.fr/Files/Other/Cours/TE_C01.pdf".to_string(),
    }));
    assert!(parsed.all_attachments.contains(&AttachmentLink {
        text: "Devoir".to_string(),
        url: "https://www.didrit.fr/Files/Other/Devoirs/2026_TE_D01.pdf".to_string(),
    }));
}

#[test]
fn test_html_extractor_relative_url_resolution() {
    let html = r#"
        <html>
        <body>
            <a href="Files/Other/Cours/TS_C02.pdf">Chapitre 2</a>
            <a href="https://example.com/external.pdf">Externe</a>
        </body>
        </html>
    "#;

    let parsed = HtmlExtractor::parse(html, "https://www.didrit.fr/Maths_TE.htm");
    assert_eq!(parsed.all_attachments.len(), 2);
    assert_eq!(
        parsed.all_attachments[0].url,
        "https://www.didrit.fr/Files/Other/Cours/TS_C02.pdf"
    );
    assert_eq!(
        parsed.all_attachments[1].url,
        "https://example.com/external.pdf"
    );
}
