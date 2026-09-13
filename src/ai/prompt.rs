use crate::diff::DiffResult;
use crate::parser::ParsedPage;

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build(page: &ParsedPage, diff: &DiffResult) -> String {
        let mut prompt = String::new();
        prompt.push_str("Tu es un assistant charge d'analyser les mises a jour du site de mathematiques d'un professeur.\n");
        prompt.push_str("Analyse le contenu modifie ci-dessous et retourne un JSON valide avec cette structure exacte :\n");
        prompt.push_str("{\n");
        prompt.push_str("  \"title\": \"Titre concis de la notification\",\n");
        prompt.push_str(
            "  \"summary\": \"Synthese claire et sans fioritures de ce qui a ete ajoute\",\n",
        );
        prompt.push_str("  \"homework_items\": [\"Liste des exercices ou devoirs a faire avec les dates d'echeance\"],\n");
        prompt.push_str("  \"new_documents\": [\"Liste des documents ou chapitres ajoutes\"],\n");
        prompt.push_str("  \"priority\": 3\n");
        prompt.push_str("}\n\n");

        prompt.push_str(&format!("Page : {}\n", page.page_title));
        prompt.push_str(&format!("URL : {}\n\n", page.source_url));

        if let Some(cahier) = &diff.cahier_diff_text {
            prompt.push_str("=== CAHIER DE TEXTE (MODIFIE) ===\n");
            prompt.push_str(cahier);
            prompt.push_str("\n\n");
        }

        if let Some(devoir) = &diff.devoir_diff_text {
            prompt.push_str("=== DEVOIRS / INTERROGATIONS (MODIFIE) ===\n");
            prompt.push_str(devoir);
            prompt.push_str("\n\n");
        }

        if !diff.new_attachments.is_empty() {
            prompt.push_str("=== NOUVEAUX FICHIERS / LIENS JOINTS ===\n");
            for att in &diff.new_attachments {
                prompt.push_str(&format!("- {} : {}\n", att.text, att.url));
            }
            prompt.push('\n');
        }

        prompt.push_str("Regles strictes :\n");
        prompt.push_str("- Aucun emoji dans la reponse JSON.\n");
        prompt.push_str("- Sois precis, concis et factuel.\n");
        prompt.push_str("- priority doit etre un entier entre 1 et 5 (5 si DS imminent ou devoir urgent, sinon 3 ou 4).\n");

        prompt
    }
}
