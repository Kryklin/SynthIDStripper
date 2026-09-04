use crate::lexicon::{STOPWORDS, TECHNICAL_TERMS};
use crate::tokenization::{Casing, Token, TokenType};
use crate::types::PosTag;

pub struct EntityClassifier;

impl Default for EntityClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl EntityClassifier {
    pub fn new() -> Self {
        Self
    }

    /// Determines if a token represents a protected entity (Proper Noun, Technical Term, Code, Number, URL, Stopword).
    pub fn is_protected(&self, token: &Token) -> bool {
        // 1. Non-word tokens
        if token.token_type != TokenType::Word {
            return true;
        }

        let lower = token.text.to_lowercase();

        // 2. Stopwords & closed-class words
        if STOPWORDS.contains(lower.as_str()) {
            return true;
        }

        // 3. Technical keywords & programming terms
        if TECHNICAL_TERMS.contains(lower.as_str()) {
            return true;
        }

        // 4. Proper Nouns (NNP / NNPS)
        if token.pos == PosTag::NNP || token.pos == PosTag::NNPS {
            return true;
        }

        // 5. Capitalization within sentence (unless sentence start)
        if token.casing == Casing::Title && !token.is_sentence_start {
            return true;
        }

        // 6. All-uppercase acronyms (> 1 char) e.g., NASA, USA, WSD, NLP
        if token.casing == Casing::Upper && token.text.len() >= 2 {
            return true;
        }

        // 7. Contractions e.g. don't, can't, wouldn't
        if token.text.contains('\'') {
            return true;
        }

        // 8. Closed-class grammatical tags
        if !token.pos.is_content_word() {
            return true;
        }

        false
    }
}
