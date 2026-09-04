use std::collections::HashSet;

lazy_static::lazy_static! {
    /// Stopwords and closed-class words that should never be substituted as lexical items.
    pub static ref STOPWORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        let list = [
            // Articles & Determiners
            "a", "an", "the", "this", "that", "these", "those", "each", "every", "either",
            "neither", "any", "some", "no", "all", "both", "half", "such", "what", "which",
            // Pronouns
            "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your",
            "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her",
            "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs",
            "themselves", "who", "whom", "whose", "which", "what", "whatever", "whoever",
            // Prepositions
            "in", "on", "at", "by", "for", "with", "about", "against", "between", "into",
            "through", "during", "before", "after", "above", "below", "to", "from", "up",
            "down", "in", "out", "on", "off", "over", "under", "again", "further", "then",
            "once", "here", "there", "when", "where", "why", "how", "all", "any", "both",
            "of", "as", "if", "until", "while", "toward", "towards", "upon", "without", "within",
            // Conjunctions
            "and", "but", "or", "nor", "for", "yet", "so", "because", "although", "since",
            "unless", "whereas", "whether",
            // Auxiliary and modal verbs
            "be", "am", "is", "are", "was", "were", "being", "been", "have", "has", "had",
            "having", "do", "does", "did", "doing", "can", "could", "shall", "should",
            "will", "would", "may", "might", "must", "ought",
            // Common particles, negation, adverbs
            "not", "n't", "no", "yes", "too", "very", "just", "now", "so", "than",
            "there", "here", "why", "how", "when", "where",
        ];
        for w in list {
            s.insert(w);
        }
        s
    };

    /// Common technical & programming keywords to preserve
    pub static ref TECHNICAL_TERMS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        let list = [
            "api", "sdk", "http", "https", "url", "uri", "html", "css", "json", "xml",
            "yaml", "toml", "sql", "nosql", "rest", "graphql", "grpc", "protobuf", "jwt",
            "ssh", "tcp", "udp", "ip", "dns", "ssl", "tls", "cpu", "gpu", "tpu", "ram",
            "rom", "os", "linux", "windows", "macos", "ios", "android", "docker", "kubernetes",
            "git", "github", "gitlab", "cargo", "npm", "pip", "rust", "python", "javascript",
            "typescript", "c", "cpp", "java", "go", "ruby", "php", "swift", "kotlin",
            "fn", "struct", "enum", "impl", "trait", "async", "await", "pub", "mut",
            "const", "let", "var", "def", "class", "function", "return", "import", "export",
            "bool", "int", "float", "str", "string", "vec", "map", "array", "null", "nil",
            "true", "false", "undefined", "nan", "void", "char", "byte", "bit",
        ];
        for w in list {
            s.insert(w);
        }
        s
    };
}
