use crate::diff::DiffResult;
use crate::parser::ParsedPage;

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build(page: &ParsedPage, diff: &DiffResult) -> String {
        let mut prompt = String::new();
        prompt.push_str("Tu es un parseur strict qui extrait les nouvelles informations d'un site de maths de lycee.\n");
        prompt.push_str("Tu dois analyser la difference entre l'ancienne et la nouvelle version de la page et extraire UNIQUEMENT les faits nouveaux et concrets.\n");
        prompt.push_str("Ne fais AUCUNE phrase de remplissage (bannis absolument les formulations comme 'Mise a jour du cahier de texte avec l'avancement...').\n");
        prompt.push_str("Retourne exclusivement un JSON valide avec cette structure exacte :\n");
        prompt.push_str("{\n");
        prompt.push_str(
            "  \"title\": \"Matiere : Seance du [Date de la seance, ex: Vendredi 11/09]\",\n",
        );
        prompt.push_str("  \"fait_en_classe\": \"Resume ultra-court des exercices corriges et notions vues (ex: Cor. Ch.1 Ex 27, 28 p 27 + Cours Ch.1-III)\",\n");
        prompt.push_str("  \"a_faire\": \"Exercices ou travail a faire pour la prochaine seance (ex: Chercher Ch.1 Ex 56, 61 p 29)\",\n");
        prompt.push_str("  \"date_echeance\": \"Date pour laquelle le travail est demande (ex: Lundi 14/09)\",\n");
        prompt.push_str(
            "  \"evaluation\": \"Annonce ou sujet de DS/DM/Interro si present, sinon null\",\n",
        );
        prompt.push_str("  \"nouveaux_documents\": [\"Nom du document UNIQUEMENT s'il apparait dans la liste NOUVEAUX FICHIERS JOINTS ci-dessous\"],\n");
        prompt.push_str("  \"priority\": 3\n");
        prompt.push_str("}\n\n");

        prompt.push_str(&format!("Matiere/Page : {}\n", page.page_title));
        prompt.push_str(&format!("URL : {}\n\n", page.source_url));

        if let Some(cahier_curr) = &diff.current_cahier_text {
            prompt.push_str("=== CAHIER DE TEXTE ACTUEL ===\n");
            prompt.push_str(cahier_curr);
            prompt.push_str("\n\n");
        }

        if let Some(cahier_prev) = &diff.previous_cahier_text {
            prompt.push_str("=== CAHIER DE TEXTE PRECEDENT (POUR COMPARER LE DELTA) ===\n");
            prompt.push_str(cahier_prev);
            prompt.push_str("\n\n");
        }

        if let Some(devoir_curr) = &diff.current_devoir_text {
            prompt.push_str("=== DEVOIRS / INTERROGATIONS ACTUEL ===\n");
            prompt.push_str(devoir_curr);
            prompt.push_str("\n\n");
        }

        if !diff.new_attachments.is_empty() {
            prompt.push_str("=== NOUVEAUX FICHIERS JOINTS DETECTES ===\n");
            for att in &diff.new_attachments {
                prompt.push_str(&format!("- {} : {}\n", att.text, att.url));
            }
            prompt.push('\n');
        } else {
            prompt.push_str("=== NOUVEAUX FICHIERS JOINTS DETECTES ===\n(Aucun nouveau fichier joint. 'nouveaux_documents' DOIT etre une liste vide [])\n\n");
        }

        prompt.push_str("Consignes de formatage strictes :\n");
        prompt.push_str("- Aucun emoji.\n");
        prompt.push_str("- Sois concis, factuel et direct.\n");
        prompt.push_str("- Ne jamais inventer de nouveaux documents s'ils ne sont pas listes dans la section NOUVEAUX FICHIERS JOINTS DETECTES.\n");
        prompt.push_str(
            "- Si aucun travail n'est a faire, mets null dans 'a_faire' et 'date_echeance'.\n",
        );

        prompt
    }
}
