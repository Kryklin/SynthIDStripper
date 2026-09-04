use crate::types::DistributionMode;
use std::collections::HashMap;

lazy_static::lazy_static! {
    /// Human corpus Zipf frequency scores (higher = more frequent in human texts, e.g. 1.0 to 7.0)
    static ref HUMAN_FREQUENCIES: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        let list = [
            // Core high-frequency nouns
            ("time", 6.2), ("person", 5.8), ("people", 6.1), ("year", 6.1), ("way", 5.9), ("day", 6.0),
            ("thing", 5.9), ("man", 5.7), ("world", 5.8), ("life", 5.8), ("hand", 5.5),
            ("part", 5.6), ("child", 5.4), ("eye", 5.3), ("woman", 5.4), ("place", 5.6),
            ("work", 5.7), ("week", 5.3), ("case", 5.5), ("point", 5.5), ("government", 5.2),
            ("company", 5.3), ("number", 5.6), ("group", 5.3), ("problem", 5.3), ("fact", 5.2),
            ("system", 5.7), ("word", 5.8), ("program", 5.3), ("question", 5.4),
            ("water", 5.4), ("money", 5.5), ("story", 5.3), ("book", 5.4), ("job", 5.4),
            ("business", 5.4), ("issue", 5.3), ("side", 5.4), ("kind", 5.4), ("head", 5.4),
            ("house", 5.5), ("service", 5.4), ("friend", 5.4), ("power", 5.4), ("hour", 5.3),
            ("game", 5.4), ("line", 5.4), ("end", 5.4), ("member", 5.2), ("law", 5.3),
            ("car", 5.4), ("city", 5.4), ("community", 5.2), ("name", 5.5), ("team", 5.3),
            ("minute", 5.2), ("idea", 5.4), ("kid", 5.3), ("body", 5.3), ("information", 5.4),
            ("result", 5.3), ("output", 4.9), ("data", 5.5), ("model", 5.4), ("language", 5.5),
            ("method", 5.2), ("reason", 5.3), ("research", 5.2), ("force", 5.2),
            ("process", 5.3), ("lock", 4.8), ("key", 5.2), ("token", 4.4), ("signal", 4.7),
            ("machine", 5.1), ("text", 5.4), ("noise", 4.7), ("homework", 4.6), ("spam", 4.3),
            ("detector", 4.5), ("adversary", 3.8), ("classifier", 4.2), ("mass", 4.8),

            // Core high-frequency verbs
            ("say", 6.5), ("get", 6.3), ("make", 6.2), ("go", 6.4), ("know", 6.2),
            ("take", 6.1), ("see", 6.1), ("come", 6.0), ("think", 6.0), ("look", 5.9),
            ("want", 5.9), ("give", 5.9), ("use", 5.8), ("find", 5.8), ("tell", 5.7),
            ("ask", 5.5), ("seem", 5.4), ("feel", 5.5), ("try", 5.4),
            ("leave", 5.4), ("call", 5.5), ("change", 5.4), ("help", 5.3), ("start", 5.3),
            ("show", 5.4), ("hear", 5.3), ("play", 5.2), ("run", 5.2), ("move", 5.2),
            ("like", 5.8), ("live", 5.3), ("believe", 5.1), ("hold", 5.2), ("bring", 5.2),
            ("happen", 5.2), ("write", 5.2), ("provide", 5.1), ("sit", 5.1), ("stand", 5.1),
            ("lose", 5.0), ("pay", 5.0), ("meet", 5.0), ("include", 5.1), ("continue", 5.0),
            ("set", 5.1), ("learn", 5.0), ("lead", 4.9), ("understand", 5.0), ("watch", 5.0),
            ("follow", 4.9), ("stop", 4.9), ("create", 4.9), ("speak", 4.9), ("read", 4.9),
            ("allow", 4.8), ("add", 4.9), ("spend", 4.8), ("grow", 4.8), ("open", 4.9),
            ("walk", 4.8), ("win", 4.8), ("offer", 4.8), ("remember", 4.8), ("love", 5.1),
            ("consider", 4.7), ("appear", 4.7), ("buy", 4.8), ("wait", 4.8), ("serve", 4.7),
            ("die", 4.7), ("send", 4.7), ("expect", 4.6), ("build", 4.9), ("stay", 4.7),
            ("fall", 4.7), ("cut", 4.6), ("reach", 4.9), ("kill", 4.6), ("remain", 4.6),
            ("suggest", 4.5), ("raise", 4.5), ("pass", 4.5), ("sell", 4.6), ("require", 4.5),
            ("report", 4.5), ("decide", 4.5), ("pull", 4.5), ("break", 4.5), ("produce", 4.7),
            ("choose", 5.2), ("pick", 5.2), ("select", 4.8), ("opt", 4.2), ("elect", 3.7),
            ("keep", 5.5), ("prevent", 4.8), ("maintain", 4.8), ("preserve", 4.3), ("protect", 4.8),
            ("operate", 4.7), ("function", 4.8), ("perform", 4.8), ("motivate", 4.4), ("inspire", 4.6),
            ("alter", 4.3), ("modify", 4.3), ("transform", 4.6), ("construct", 4.4),
            ("begin", 4.9), ("launch", 4.2), ("aid", 4.0), ("assist", 3.8), ("display", 4.2),

            // Core high-frequency adjectives
            ("good", 6.1), ("new", 6.0), ("first", 5.9), ("last", 5.6), ("long", 5.5),
            ("great", 5.7), ("little", 5.4), ("own", 5.3), ("other", 5.8), ("old", 5.5),
            ("right", 5.6), ("big", 5.5), ("high", 5.4), ("different", 5.3), ("small", 5.3),
            ("large", 5.2), ("next", 5.3), ("early", 5.1), ("young", 5.1), ("important", 5.1),
            ("fast", 4.6), ("quick", 4.5), ("rapid", 4.0), ("huge", 4.6), ("massive", 4.4),
            ("enormous", 4.0), ("smart", 4.5), ("clever", 4.3), ("intelligent", 4.1), ("bright", 4.4),
            ("simple", 4.9), ("clear", 5.1), ("real", 5.4), ("strong", 5.1), ("free", 5.1),
            ("true", 5.2), ("full", 5.2), ("easy", 5.0), ("hard", 5.2), ("major", 4.9),
            ("multiple", 4.9), ("various", 5.1), ("several", 5.3), ("assorted", 3.1),
            ("motivated", 4.4), ("inspired", 4.4), ("encouraged", 4.2),

            // Adverbs
            ("substantially", 3.9), ("essentially", 4.7), ("fundamentally", 4.3), ("basically", 4.9),
            ("completely", 5.1), ("entirely", 4.8), ("wholly", 3.9), ("totally", 4.9),
            ("subtly", 3.8), ("gently", 4.2), ("mildly", 4.0), ("directly", 4.8),
        ];
        for (w, s) in list {
            m.insert(w, s);
        }
        m
    };

    /// AI-characteristic vocabulary weights (favors words typical of LLM stylistic choices)
    static ref AI_CHARACTERISTIC_WEIGHTS: HashMap<&'static str, f64> = {
        let mut m = HashMap::new();
        let list = [
            ("crucial", 5.5), ("pivotal", 5.4), ("delve", 5.8), ("underscores", 5.3),
            ("underscore", 5.3), ("fosters", 5.2), ("foster", 5.2), ("tapestry", 5.5),
            ("multifaceted", 5.4), ("seamless", 5.2), ("seamlessly", 5.3), ("showcase", 5.1),
            ("showcases", 5.2), ("testament", 5.3), ("meticulous", 5.1), ("meticulously", 5.2),
            ("paramount", 5.4), ("nuanced", 5.3), ("nuance", 5.2), ("beacon", 5.0),
            ("catalyst", 5.1), ("harness", 5.2), ("harnessing", 5.3), ("realm", 5.2),
            ("bolster", 5.1), ("bolstering", 5.1), ("comprehensive", 5.0), ("intricate", 5.1),
            ("transformative", 5.2), ("imperative", 5.1), ("exemplifies", 5.2),
            ("exemplify", 5.1), ("illuminates", 5.1), ("illuminate", 5.0), ("streamline", 5.1),
            ("exponential", 4.9), ("unwavering", 5.0), ("cornerstone", 5.2), ("linchpin", 5.0),
            ("quintessential", 5.2), ("endeavor", 4.9), ("intertwined", 5.0), ("resonate", 4.9),
            ("elucidate", 5.0), ("myriad", 5.1), ("paradigm", 5.0), ("synergy", 4.9),
            ("ubiquitous", 5.0), ("leveraging", 5.2), ("leverage", 5.1), ("vital", 4.8),
            ("indispensable", 4.9), ("profound", 4.8), ("dynamic", 4.7), ("holistic", 4.9),
            ("augment", 4.8), ("navigate", 4.7), ("embark", 4.6), ("unveil", 4.7),
            ("unravel", 4.6), ("epitome", 4.8), ("pinnacle", 4.8), ("conducive", 4.7),
            ("revolutionize", 4.8), ("spearhead", 4.7), ("pave", 4.6), ("foster", 4.9),
        ];
        for (w, s) in list {
            m.insert(w, s);
        }
        m
    };
}

pub struct FrequencyDb;

impl Default for FrequencyDb {
    fn default() -> Self {
        Self::new()
    }
}

impl FrequencyDb {
    pub fn new() -> Self {
        Self
    }

    /// Computes the frequency/preference weight for a candidate lemma under the given distribution mode.
    pub fn get_candidate_weight(
        &self,
        lemma: &str,
        mode: DistributionMode,
        temperature: f64,
    ) -> f64 {
        let temp = if temperature <= 0.0 { 1.0 } else { temperature };
        let lower = lemma.to_lowercase();

        let raw_score = match mode {
            DistributionMode::Neutral => return 1.0,
            DistributionMode::Random => return 1.0,
            DistributionMode::Human => {
                if let Some(&score) = HUMAN_FREQUENCIES.get(lower.as_str()) {
                    score
                } else {
                    // Default baseline for standard vetted English words
                    4.2
                }
            }
            DistributionMode::Ai => {
                if let Some(&ai_score) = AI_CHARACTERISTIC_WEIGHTS.get(lower.as_str()) {
                    ai_score * 1.5
                } else if let Some(&human_score) = HUMAN_FREQUENCIES.get(lower.as_str()) {
                    human_score * 0.9
                } else {
                    3.8
                }
            }
        };

        // Tempered power-scaling centered around neutral baseline 4.2
        // Prevents wild exponential blowout while maintaining natural Zipf human distribution
        let normalized = (raw_score - 4.2) * (0.65 / temp);
        normalized.exp()
    }
}
