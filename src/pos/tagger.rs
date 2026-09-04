use crate::pos::rules::{AUX_VERBS, CLOSED_CLASS_MAP, COMMON_ADJECTIVES, COMMON_ADVERBS};
use crate::tokenization::{Casing, Token, TokenType};
use crate::types::PosTag;
use std::ops::Deref;

pub struct PosTagger;

impl Default for PosTagger {
    fn default() -> Self {
        Self::new()
    }
}

impl PosTagger {
    pub fn new() -> Self {
        Self
    }

    /// Tags a slice of tokens with Penn Treebank POS tags.
    pub fn tag_tokens(&self, tokens: &mut [Token]) {
        let len = tokens.len();
        if len == 0 {
            return;
        }

        // Pass 1: Lexical and Morphological tagging
        for i in 0..len {
            if tokens[i].token_type != TokenType::Word {
                tokens[i].pos = match tokens[i].token_type {
                    TokenType::Punctuation => PosTag::PUNCT,
                    TokenType::Number => PosTag::CD,
                    TokenType::Protected(_) => PosTag::NNP,
                    TokenType::Whitespace => PosTag::UNKNOWN,
                    _ => PosTag::UNKNOWN,
                };
                continue;
            }

            let word_lower = tokens[i].text.to_lowercase();
            let is_sentence_start = tokens[i].is_sentence_start;
            let casing = tokens[i].casing;

            // 1. Check closed-class map
            if let Some(&tag) = CLOSED_CLASS_MAP.deref().get(word_lower.as_str()) {
                tokens[i].pos = tag;
                continue;
            }

            // 2. Check auxiliary verbs
            if let Some(&tag) = AUX_VERBS.deref().get(word_lower.as_str()) {
                tokens[i].pos = tag;
                continue;
            }

            // 3. Proper noun check (Capitalized and not start of sentence, or acronym)
            if (casing == Casing::Title && !is_sentence_start) || casing == Casing::Upper {
                if word_lower.ends_with('s') {
                    tokens[i].pos = PosTag::NNPS;
                } else {
                    tokens[i].pos = PosTag::NNP;
                }
                continue;
            }

            // 4. Common Adverbs
            if let Some(&tag) = COMMON_ADVERBS.deref().get(word_lower.as_str()) {
                tokens[i].pos = tag;
                continue;
            }

            // 5. Common Adjectives
            if let Some(&tag) = COMMON_ADJECTIVES.deref().get(word_lower.as_str()) {
                tokens[i].pos = tag;
                continue;
            }

            // 6. Suffix / Morphological heuristics
            tokens[i].pos = self.guess_by_morphology(&word_lower);
        }

        // Pass 2: Contextual Disambiguation Rules
        for i in 0..len {
            if tokens[i].token_type != TokenType::Word {
                continue;
            }

            let prev_word_pos = self.find_prev_word_pos(tokens, i);
            let prev_word_text = self.find_prev_word_text(tokens, i);
            let next_word_pos = self.find_next_word_pos(tokens, i);
            let word_lower = tokens[i].text.to_lowercase();

            // Rule: "to" + word -> Verb base (VB)
            if let Some(prev_text) = &prev_word_text {
                if prev_text == "to"
                    && (tokens[i].pos == PosTag::NN
                        || tokens[i].pos == PosTag::VBP
                        || tokens[i].pos == PosTag::VB)
                {
                    tokens[i].pos = PosTag::VB;
                }
            }

            // Rule: Modal (can, will, should) + word -> Verb base (VB)
            if prev_word_pos == Some(PosTag::MD)
                && tokens[i].pos != PosTag::RB {
                    tokens[i].pos = PosTag::VB;
                }

            // Rule: have/has/had + word-ed/en -> Past participle (VBN)
            if let Some(prev_text) = &prev_word_text {
                if matches!(prev_text.as_str(), "have" | "has" | "had" | "having")
                    && (tokens[i].pos == PosTag::VBD
                        || word_lower.ends_with("ed")
                        || word_lower.ends_with("en"))
                    {
                        tokens[i].pos = PosTag::VBN;
                    }
                // is/am/are/was/were/be/been + word-ed -> VBN (passive voice)
                if matches!(
                    prev_text.as_str(),
                    "is" | "am" | "are" | "was" | "were" | "be" | "been" | "being"
                )
                    && tokens[i].pos == PosTag::VBD {
                        tokens[i].pos = PosTag::VBN;
                    }
            }

            // Rule: Determiner (DT) + [Word] + Noun -> [Word] is Adjective (JJ)
            if (prev_word_pos == Some(PosTag::DT) || prev_word_pos == Some(PosTag::PRP_POSS))
                && matches!(
                    next_word_pos,
                    Some(PosTag::NN) | Some(PosTag::NNS) | Some(PosTag::NNP) | Some(PosTag::NNPS)
                )
                    && tokens[i].pos != PosTag::JJ
                        && tokens[i].pos != PosTag::JJR
                        && tokens[i].pos != PosTag::JJS
                    {
                        // If it's not already a noun modifier, it's likely an adjective
                        if !word_lower.ends_with("tion") && !word_lower.ends_with("ment") {
                            tokens[i].pos = PosTag::JJ;
                        }
                    }

            // Rule: Determiner (DT) + [Word] at end or before verb/preposition -> [Word] is Noun (NN/NNS)
            if prev_word_pos == Some(PosTag::DT) || prev_word_pos == Some(PosTag::PRP_POSS) {
                if tokens[i].pos == PosTag::VBG {
                    tokens[i].pos = PosTag::NN;
                } else if matches!(
                    next_word_pos,
                    Some(PosTag::VB)
                        | Some(PosTag::VBZ)
                        | Some(PosTag::VBD)
                        | Some(PosTag::VBP)
                        | Some(PosTag::IN)
                        | Some(PosTag::PUNCT)
                        | None
                )
                    && (tokens[i].pos == PosTag::VB || tokens[i].pos == PosTag::VBP) {
                        tokens[i].pos = if word_lower.ends_with('s') {
                            PosTag::NNS
                        } else {
                            PosTag::NN
                        };
                    }
            }

            // Rule: Adjective (JJ) + [Word-ing] before prep/verb/punct -> [Word-ing] is Noun (NN) (e.g. "distinctive building")
            if matches!(
                prev_word_pos,
                Some(PosTag::JJ) | Some(PosTag::JJR) | Some(PosTag::JJS)
            )
                && tokens[i].pos == PosTag::VBG {
                    tokens[i].pos = PosTag::NN;
                }

            // Rule: [Word] + Noun -> [Word] is Adjective (JJ) (e.g. "slow change", "warm smell")
            if matches!(
                next_word_pos,
                Some(PosTag::NN) | Some(PosTag::NNS) | Some(PosTag::NNP) | Some(PosTag::NNPS)
            )
                && (tokens[i].pos == PosTag::RB
                    || tokens[i].pos == PosTag::NN
                    || tokens[i].pos == PosTag::VB
                    || tokens[i].pos == PosTag::VBP)
                    && !CLOSED_CLASS_MAP.deref().contains_key(word_lower.as_str())
                        && !word_lower.ends_with("tion")
                        && !word_lower.ends_with("ment")
                    {
                        tokens[i].pos = PosTag::JJ;
                    }

            // Rule: Adverb (RB) + [Word] + Preposition (IN) / TO -> [Word] is Adjective (JJ) (e.g. "particularly vital to")
            if prev_word_pos == Some(PosTag::RB)
                && matches!(next_word_pos, Some(PosTag::IN) | Some(PosTag::TO))
                    && tokens[i].pos != PosTag::JJ
                        && tokens[i].pos != PosTag::JJR
                        && tokens[i].pos != PosTag::JJS
                    {
                        tokens[i].pos = PosTag::JJ;
                    }

            // Rule: Personal Pronoun (PRP) + [Word] -> [Word] is Verb
            if prev_word_pos == Some(PosTag::PRP)
                && tokens[i].pos == PosTag::NN {
                    if word_lower.ends_with('s') {
                        tokens[i].pos = PosTag::VBZ;
                    } else if word_lower.ends_with("ed") {
                        tokens[i].pos = PosTag::VBD;
                    } else {
                        tokens[i].pos = PosTag::VBP;
                    }
                }
        }
    }

    fn guess_by_morphology(&self, word: &str) -> PosTag {
        if word.ends_with("ly") && word.len() > 3 {
            // Special exceptions: friendly, lovely, lonely, deadly
            if matches!(
                word,
                "friendly"
                    | "lovely"
                    | "lonely"
                    | "deadly"
                    | "orderly"
                    | "ugly"
                    | "silly"
                    | "holy"
                    | "lively"
            ) {
                return PosTag::JJ;
            }
            return PosTag::RB;
        }

        if word.ends_with("ing") && word.len() > 4 {
            return PosTag::VBG;
        }

        if (word.ends_with("ed") || word.ends_with("ied")) && word.len() > 3 {
            return PosTag::VBD;
        }

        if word.ends_with("est") && word.len() > 4 {
            return PosTag::JJS;
        }

        if (word.ends_with("er") || word.ends_with("ier")) && word.len() > 3 {
            // Can be agent noun (teacher) or comparative (faster)
            if matches!(
                word,
                "better"
                    | "faster"
                    | "slower"
                    | "higher"
                    | "lower"
                    | "greater"
                    | "larger"
                    | "smaller"
                    | "longer"
                    | "shorter"
                    | "easier"
                    | "harder"
                    | "older"
                    | "younger"
                    | "richer"
                    | "poorer"
                    | "stronger"
                    | "weaker"
            ) {
                return PosTag::JJR;
            }
            return PosTag::NN;
        }

        if word.ends_with("tion")
            || word.ends_with("sion")
            || word.ends_with("ment")
            || word.ends_with("ness")
            || word.ends_with("ity")
            || word.ends_with("ance")
            || word.ends_with("ence")
            || word.ends_with("ship")
            || word.ends_with("hood")
            || word.ends_with("ism")
            || word.ends_with("ist")
            || word.ends_with("dom")
            || word.ends_with("logy")
        {
            return PosTag::NN;
        }

        if word.ends_with("tions")
            || word.ends_with("sions")
            || word.ends_with("ments")
            || word.ends_with("nesses")
            || word.ends_with("ities")
            || word.ends_with("ances")
            || word.ends_with("ences")
            || word.ends_with("ships")
            || word.ends_with("hoods")
            || word.ends_with("isms")
            || word.ends_with("ists")
        {
            return PosTag::NNS;
        }

        if word.ends_with("ful")
            || word.ends_with("less")
            || word.ends_with("able")
            || word.ends_with("ible")
            || word.ends_with("ous")
            || word.ends_with("ious")
            || word.ends_with("ive")
            || word.ends_with("ic")
            || word.ends_with("ical")
            || word.ends_with("al")
            || word.ends_with("ish")
            || word.ends_with("ary")
            || word.ends_with("ory")
        {
            return PosTag::JJ;
        }

        if word.ends_with("ize")
            || word.ends_with("ise")
            || word.ends_with("ify")
            || word.ends_with("ate")
        {
            return PosTag::VB;
        }

        if word.ends_with('s')
            && !word.ends_with("ss")
            && !word.ends_with("us")
            && !word.ends_with("is")
        {
            return PosTag::NNS;
        }

        // Default open class word to singular noun
        PosTag::NN
    }

    fn find_prev_word_pos(&self, tokens: &[Token], current_idx: usize) -> Option<PosTag> {
        let mut idx = current_idx;
        while idx > 0 {
            idx -= 1;
            if tokens[idx].token_type == TokenType::Word {
                return Some(tokens[idx].pos);
            }
        }
        None
    }

    fn find_prev_word_text(&self, tokens: &[Token], current_idx: usize) -> Option<String> {
        let mut idx = current_idx;
        while idx > 0 {
            idx -= 1;
            if tokens[idx].token_type == TokenType::Word {
                return Some(tokens[idx].text.to_lowercase());
            }
        }
        None
    }

    fn find_next_word_pos(&self, tokens: &[Token], current_idx: usize) -> Option<PosTag> {
        for token in tokens.iter().skip(current_idx + 1) {
            if token.token_type == TokenType::Word {
                return Some(token.pos);
            }
            if token.token_type == TokenType::Punctuation {
                return Some(PosTag::PUNCT);
            }
        }
        None
    }
}
