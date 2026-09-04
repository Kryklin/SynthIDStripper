use crate::types::PosTag;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Casing {
    Lower,
    Title,
    Upper,
    Mixed,
}

impl Casing {
    pub fn detect(s: &str) -> Self {
        let mut chars = s.chars().filter(|c| c.is_alphabetic());
        let first = chars.next();
        if first.is_none() {
            return Casing::Lower;
        }
        let first = first.unwrap();
        let rest: Vec<char> = chars.collect();

        if first.is_uppercase() {
            if rest.is_empty() {
                Casing::Title
            } else if rest.iter().all(|c| c.is_uppercase()) {
                Casing::Upper
            } else if rest.iter().all(|c| c.is_lowercase()) {
                Casing::Title
            } else {
                Casing::Mixed
            }
        } else {
            if rest.iter().all(|c| c.is_lowercase()) {
                Casing::Lower
            } else {
                Casing::Mixed
            }
        }
    }

    pub fn apply(&self, word: &str) -> String {
        match self {
            Casing::Lower => word.to_lowercase(),
            Casing::Upper => word.to_uppercase(),
            Casing::Title => {
                let mut c = word.chars();
                match c.next() {
                    None => String::new(),
                    Some(f) => f.to_uppercase().collect::<String>() + &c.as_str().to_lowercase(),
                }
            }
            Casing::Mixed => word.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TokenType {
    Word,
    Whitespace,
    Punctuation,
    Number,
    Protected(String), // URL, email, code, markdown tag
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub text: String,
    pub original_text: String,
    pub token_type: TokenType,
    pub casing: Casing,
    pub start_offset: usize,
    pub end_offset: usize,
    pub pos: PosTag,
    pub lemma: Option<String>,
    pub is_sentence_start: bool,
    pub is_transformed: bool,
    pub transformation_history: Vec<String>,
}

impl Token {
    pub fn is_word(&self) -> bool {
        self.token_type == TokenType::Word
    }
}

pub struct Tokenizer {
    regex: Regex,
}

impl Default for Tokenizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Tokenizer {
    pub fn new() -> Self {
        // Regex matches:
        // 1. URLs / Emails / Code-like: https?://\S+ | \w+@\w+\.\w+ | `[^`]+`
        // 2. English contractions and words (with full Unicode letters): [\p{Alphabetic}]+(?:'[\p{Alphabetic}]+)?
        // 3. Numbers: \d+(?:,\d+)*(?:\.\d+)?%?
        // 4. Whitespaces: \s+
        // 5. Punctuations and symbols: [^\w\s]
        let pattern = r"(?x)
            (?P<protected>https?://[^\s]+|[\p{Alphabetic}\d_.+-]+@[\p{Alphabetic}\d.-]+\.[\p{Alphabetic}\d.-]+|`[^`]+`)
            |(?P<word>[\p{Alphabetic}]+(?:'[\p{Alphabetic}]+)?)
            |(?P<number>\d+(?:,\d+)*(?:\.\d+)?%?)
            |(?P<space>\s+)
            |(?P<punct>[^\w\s])
        ";
        let regex = Regex::new(pattern).expect("Valid tokenizer regex");
        Self { regex }
    }

    pub fn tokenize(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut is_sentence_start = true;

        for cap in self.regex.captures_iter(text) {
            let mat = cap.get(0).unwrap();
            let raw_text = mat.as_str().to_string();
            let start = mat.start();
            let end = mat.end();

            let (token_type, casing) = if cap.name("protected").is_some() {
                (TokenType::Protected("url_or_code".into()), Casing::Mixed)
            } else if cap.name("word").is_some() {
                let casing = Casing::detect(&raw_text);
                (TokenType::Word, casing)
            } else if cap.name("number").is_some() {
                (TokenType::Number, Casing::Lower)
            } else if cap.name("space").is_some() {
                (TokenType::Whitespace, Casing::Lower)
            } else {
                (TokenType::Punctuation, Casing::Lower)
            };

            let current_is_sentence_start = if token_type == TokenType::Word {
                let s = is_sentence_start;
                is_sentence_start = false;
                s
            } else if token_type == TokenType::Punctuation {
                if raw_text == "." || raw_text == "!" || raw_text == "?" || raw_text == "\n" {
                    is_sentence_start = true;
                }
                false
            } else {
                false
            };

            tokens.push(Token {
                text: raw_text.clone(),
                original_text: raw_text,
                token_type,
                casing,
                start_offset: start,
                end_offset: end,
                pos: PosTag::UNKNOWN,
                lemma: None,
                is_sentence_start: current_is_sentence_start,
                is_transformed: false,
                transformation_history: Vec::new(),
            });
        }

        tokens
    }

    /// Reconstructs string from tokens with potential replacements
    pub fn reconstruct(&self, tokens: &[Token], replacements: &[(usize, String)]) -> String {
        let mut map = std::collections::HashMap::new();
        for (idx, rep) in replacements {
            map.insert(*idx, rep);
        }

        let mut output = String::new();
        for (i, token) in tokens.iter().enumerate() {
            if let Some(rep) = map.get(&i) {
                output.push_str(rep);
            } else {
                output.push_str(&token.text);
            }
        }
        output
    }
}
