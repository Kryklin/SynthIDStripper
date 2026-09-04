use crate::types::PosTag;
use std::collections::HashMap;

lazy_static::lazy_static! {
    /// Closed-class function words with fixed POS tags
    pub static ref CLOSED_CLASS_MAP: HashMap<&'static str, PosTag> = {
        let mut m = HashMap::new();
        // Determiners (DT)
        for w in &["the", "a", "an", "this", "that", "these", "those", "all", "both", "half", "each", "every", "neither", "either", "any", "some", "no", "such"] {
            m.insert(*w, PosTag::DT);
        }
        // Prepositions / Subordinating conjunctions (IN)
        for w in &["in", "on", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "while", "of", "as", "if", "since", "until", "unless", "although", "because", "despite", "towards", "upon", "without", "within", "throughout"] {
            m.insert(*w, PosTag::IN);
        }
        // Coordinating Conjunctions (CC)
        for w in &["and", "but", "or", "nor", "for", "yet", "so"] {
            m.insert(*w, PosTag::CC);
        }
        // Personal Pronouns (PRP)
        for w in &["i", "me", "you", "he", "him", "she", "her", "it", "we", "us", "they", "them", "myself", "yourself", "himself", "herself", "itself", "ourselves", "themselves"] {
            m.insert(*w, PosTag::PRP);
        }
        // Possessive Pronouns (PRP$)
        for w in &["my", "your", "his", "her", "its", "our", "their", "whose"] {
            m.insert(*w, PosTag::PRP_POSS);
        }
        // Modal Auxiliaries (MD)
        for w in &["can", "could", "may", "might", "must", "shall", "should", "will", "would", "ought"] {
            m.insert(*w, PosTag::MD);
        }
        // Existential / Particle / Interjection
        m.insert("there", PosTag::EX);
        m.insert("to", PosTag::TO);
        m.insert("yes", PosTag::UH);
        m.insert("no", PosTag::UH);
        m.insert("oh", PosTag::UH);
        m.insert("ah", PosTag::UH);
        m.insert("hey", PosTag::UH);
        m.insert("hello", PosTag::UH);
        m.insert("hi", PosTag::UH);

        m
    };

    /// Common auxiliary verbs and their default tags
    pub static ref AUX_VERBS: HashMap<&'static str, PosTag> = {
        let mut m = HashMap::new();
        m.insert("be", PosTag::VB);
        m.insert("is", PosTag::VBZ);
        m.insert("are", PosTag::VBP);
        m.insert("am", PosTag::VBP);
        m.insert("was", PosTag::VBD);
        m.insert("were", PosTag::VBD);
        m.insert("been", PosTag::VBN);
        m.insert("being", PosTag::VBG);
        m.insert("have", PosTag::VBP);
        m.insert("has", PosTag::VBZ);
        m.insert("had", PosTag::VBD);
        m.insert("having", PosTag::VBG);
        m.insert("do", PosTag::VBP);
        m.insert("does", PosTag::VBZ);
        m.insert("did", PosTag::VBD);
        m.insert("doing", PosTag::VBG);
        m.insert("done", PosTag::VBN);
        m
    };

    /// Common English adverbs ending in -ly or irregular
    pub static ref COMMON_ADVERBS: HashMap<&'static str, PosTag> = {
        let mut m = HashMap::new();
        for w in &["very", "quite", "really", "almost", "always", "never", "often", "sometimes", "seldom", "rarely", "usually", "generally", "hardly", "barely", "scarcely", "nearly", "also", "even", "just", "only", "already", "still", "not", "too", "well", "enough", "now", "soon", "late", "early", "together", "apart", "instead", "perhaps", "maybe", "probably", "certainly", "definitely", "indeed", "thus", "therefore", "however", "moreover", "furthermore", "nonetheless", "nevertheless", "otherwise", "meanwhile", "again", "abroad", "anyway", "everywhere", "anywhere", "nowhere", "somewhere", "forward", "backward", "out", "away"] {
            m.insert(*w, PosTag::RB);
        }
        m
    };

    /// Common Adjectives
    pub static ref COMMON_ADJECTIVES: HashMap<&'static str, PosTag> = {
        let mut m = HashMap::new();
        for w in &["good", "new", "first", "last", "long", "great", "little", "own", "other", "old", "right", "big", "high", "different", "small", "large", "next", "early", "young", "important", "few", "public", "bad", "same", "able", "major", "better", "best", "worse", "worst", "full", "easy", "hard", "clear", "recent", "certain", "personal", "open", "red", "blue", "green", "white", "black", "dark", "light", "strong", "weak", "simple", "complex", "fine", "ready", "happy", "rich", "poor", "free", "true", "false", "real", "sure", "dead", "alive", "fine", "heavy", "deep", "hot", "cold", "fast", "slow", "fresh", "clean", "safe", "wild", "quiet", "loud", "sharp", "soft", "sweet", "bitter", "sour", "dry", "wet", "warm", "cool", "broad", "narrow", "flat", "round", "thick", "thin", "rare", "common", "unique", "crucial", "pivotal", "significant", "vital", "essential", "primary", "secondary", "effective", "efficient", "robust", "resilient", "comprehensive", "diverse", "distinct", "dynamic", "flexible", "innovative", "novel", "fundamental", "profound", "intricate", "meticulous", "seamless", "paramount", "imperative", "nuanced"] {
            m.insert(*w, PosTag::JJ);
        }
        m
    };
}
