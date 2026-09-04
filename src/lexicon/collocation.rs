use std::collections::HashSet;

lazy_static::lazy_static! {
    /// Disallowed / unnatural word combinations (candidate, neighbor) that sound ungrammatical or stilted
    static ref ANTI_COLLOCATIONS: HashSet<(&'static str, &'static str)> = {
        let mut s = HashSet::new();
        let pairs = [
            // Verb + preposition/particle or object
            ("maintain", "from"),
            ("preserve", "from"),
            ("sustain", "from"),
            ("achieve", "point"),
            ("accomplish", "point"),
            ("execute", "point"),
            ("elect", "word"),
            ("elect", "token"),
            ("electing", "word"),
            ("single", "word"),
            ("produce", "lock"),

            // Adjective + Noun / Noun + Noun
            ("raw", "outcome"),
            ("raw", "outcomes"),
            ("raw", "effect"),
            ("raw", "effects"),
            ("encouraged", "adversary"),
            ("encouraged", "adversaries"),
            ("assorted", "translation"),
            ("assorted", "translations"),
            ("extensive", "language"),
            ("extensive", "model"),
            ("lazy", "evolution"),
            ("lazy", "evolutions"),
            ("persons", "paste"),
            ("persons", "pasting"),

            // Culinary & Verb-Complement anti-collocations
            ("freshly brewed", "pan"),
            ("freshly brewed", "skillet"),
            ("freshly brewed", "pot"),
            ("freshly made", "pan"),
            ("fresh", "pan"),
            ("sustain", "entertained"),
            ("maintain", "entertained"),
            ("preserve", "entertained"),
            ("conserve", "entertained"),
            ("retain", "entertained"),
        ];
        for (a, b) in pairs {
            s.insert((a, b));
        }
        s
    };

    /// Natural English collocations that receive an affinity bonus
    static ref AFFINITY_COLLOCATIONS: HashSet<(&'static str, &'static str)> = {
        let mut s = HashSet::new();
        let pairs = [
            ("reach", "point"),
            ("come", "point"),
            ("get", "point"),
            ("keep", "from"),
            ("prevent", "from"),
            ("stop", "from"),
            ("pick", "word"),
            ("choose", "word"),
            ("select", "word"),
            ("raw", "output"),
            ("raw", "outputs"),
            ("raw", "data"),
            ("motivated", "adversary"),
            ("motivated", "adversaries"),
            ("massive", "model"),
            ("massive", "language"),
            ("large", "model"),
            ("large", "language"),
            ("build", "lock"),
            ("construct", "lock"),
            ("foreign", "language"),
            ("statistical", "signal"),
            ("delicate", "signal"),
            ("silver", "bullet"),
            ("arms", "race"),
            ("hot", "pan"),
            ("sizzling", "pan"),
            ("keep", "entertained"),
            ("stay", "entertained"),
            ("fine", "dining"),
            ("head", "chef"),
            ("line", "cook"),
            ("tasting", "menu"),
        ];
        for (a, b) in pairs {
            s.insert((a, b));
        }
        s
    };
}

/// Evaluates statistical n-gram and collocation compatibility for candidate substitutions.
#[derive(Debug, Clone, Default)]
pub struct CollocationDb;

impl CollocationDb {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates candidate fluency and compatibility with immediate context.
    /// Returns 0.0 for strict anti-collocations, 1.5+ for high-affinity natural collocations, and 1.0 for neutral.
    pub fn evaluate_collocation_affinity(
        &self,
        candidate_inflected: &str,
        candidate_lemma: &str,
        words_before: &[&str],
        words_after: &[&str],
    ) -> f64 {
        let cand_inf = candidate_inflected.to_lowercase();
        let cand_lem = candidate_lemma.to_lowercase();

        // Extract immediate left and right neighbors (up to 6 words)
        let left_neighbors: Vec<String> = words_before
            .iter()
            .rev()
            .take(6)
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();

        let right_neighbors: Vec<String> = words_after
            .iter()
            .take(6)
            .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()).to_lowercase())
            .filter(|w| !w.is_empty())
            .collect();

        // 1. Anti-collocation check (candidate on the right: left_word + candidate)
        for lw in &left_neighbors {
            if ANTI_COLLOCATIONS.contains(&(lw.as_str(), cand_inf.as_str()))
                || ANTI_COLLOCATIONS.contains(&(lw.as_str(), cand_lem.as_str()))
            {
                return 0.0;
            }
        }

        // 2. Anti-collocation check (candidate on the left: candidate + right_word)
        for rw in &right_neighbors {
            if ANTI_COLLOCATIONS.contains(&(cand_inf.as_str(), rw.as_str()))
                || ANTI_COLLOCATIONS.contains(&(cand_lem.as_str(), rw.as_str()))
            {
                return 0.0;
            }
        }

        // 3. Affinity collocation check
        let mut bonus: f64 = 1.0;

        for lw in &left_neighbors {
            if AFFINITY_COLLOCATIONS.contains(&(lw.as_str(), cand_inf.as_str()))
                || AFFINITY_COLLOCATIONS.contains(&(lw.as_str(), cand_lem.as_str()))
            {
                bonus *= 1.6;
            }
        }

        for rw in &right_neighbors {
            if AFFINITY_COLLOCATIONS.contains(&(cand_inf.as_str(), rw.as_str()))
                || AFFINITY_COLLOCATIONS.contains(&(cand_lem.as_str(), rw.as_str()))
            {
                bonus *= 1.6;
            }
        }

        bonus.min(2.5)
    }
}
