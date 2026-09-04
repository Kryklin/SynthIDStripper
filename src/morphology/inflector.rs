use crate::tokenization::Casing;
use crate::types::PosTag;
use std::collections::HashMap;

lazy_static::lazy_static! {
    /// Verb inflection table: base -> (3rd_sing, past, past_part, gerund)
    static ref VERB_CONJUGATIONS: HashMap<&'static str, (&'static str, &'static str, &'static str, &'static str)> = {
        let mut m = HashMap::new();
        let list = [
            ("be", ("is", "was", "been", "being")),
            ("have", ("has", "had", "had", "having")),
            ("do", ("does", "did", "done", "doing")),
            ("say", ("says", "said", "said", "saying")),
            ("go", ("goes", "went", "gone", "going")),
            ("get", ("gets", "got", "gotten", "getting")),
            ("make", ("makes", "made", "made", "making")),
            ("know", ("knows", "knew", "known", "knowing")),
            ("think", ("thinks", "thought", "thought", "thinking")),
            ("take", ("takes", "took", "taken", "taking")),
            ("see", ("sees", "saw", "seen", "seeing")),
            ("come", ("comes", "came", "come", "coming")),
            ("find", ("finds", "found", "found", "finding")),
            ("give", ("gives", "gave", "given", "giving")),
            ("tell", ("tells", "told", "told", "telling")),
            ("become", ("becomes", "became", "become", "becoming")),
            ("show", ("shows", "showed", "shown", "showing")),
            ("leave", ("leaves", "left", "left", "leaving")),
            ("feel", ("feels", "felt", "felt", "feeling")),
            ("put", ("puts", "put", "put", "putting")),
            ("bring", ("brings", "brought", "brought", "bringing")),
            ("begin", ("begins", "began", "begun", "beginning")),
            ("keep", ("keeps", "kept", "kept", "keeping")),
            ("hold", ("holds", "held", "held", "holding")),
            ("uphold", ("upholds", "upheld", "upheld", "upholding")),
            ("withhold", ("withholds", "withheld", "withheld", "withholding")),
            ("undertake", ("undertakes", "undertook", "undertaken", "undertaking")),
            ("undergo", ("undergoes", "underwent", "undergone", "undergoing")),
            ("overcome", ("overcomes", "overcame", "overcome", "overcoming")),
            ("write", ("writes", "wrote", "written", "writing")),
            ("stand", ("stands", "stood", "stood", "standing")),
            ("hear", ("hears", "heard", "heard", "hearing")),
            ("let", ("lets", "let", "let", "letting")),
            ("mean", ("means", "meant", "meant", "meaning")),
            ("set", ("sets", "set", "set", "setting")),
            ("meet", ("meets", "met", "met", "meeting")),
            ("run", ("runs", "ran", "run", "running")),
            ("pay", ("pays", "paid", "paid", "paying")),
            ("sit", ("sits", "sat", "sat", "sitting")),
            ("speak", ("speaks", "spoke", "spoken", "speaking")),
            ("lie", ("lies", "lay", "lain", "lying")),
            ("lead", ("leads", "led", "led", "leading")),
            ("read", ("reads", "read", "read", "reading")),
            ("grow", ("grows", "grew", "grown", "growing")),
            ("lose", ("loses", "lost", "lost", "losing")),
            ("fall", ("falls", "fell", "fallen", "falling")),
            ("send", ("sends", "sent", "sent", "sending")),
            ("build", ("builds", "built", "built", "building")),
            ("understand", ("understands", "understood", "understood", "understanding")),
            ("draw", ("draws", "drew", "drawn", "drawing")),
            ("break", ("breaks", "broke", "broken", "breaking")),
            ("spend", ("spends", "spent", "spent", "spending")),
            ("cut", ("cuts", "cut", "cut", "cutting")),
            ("rise", ("rises", "rose", "risen", "rising")),
            ("drive", ("drives", "drove", "driven", "driving")),
            ("buy", ("buys", "bought", "bought", "buying")),
            ("wear", ("wears", "wore", "worn", "wearing")),
            ("choose", ("chooses", "chose", "chosen", "choosing")),
            ("seek", ("seeks", "sought", "sought", "seeking")),
            ("deal", ("deals", "dealt", "dealt", "dealing")),
            ("win", ("wins", "won", "won", "winning")),
            ("fly", ("flies", "flew", "flown", "flying")),
            ("teach", ("teaches", "taught", "taught", "teaching")),
            ("catch", ("catches", "caught", "caught", "catching")),
            ("throw", ("throws", "threw", "thrown", "throwing")),
            ("sleep", ("sleeps", "slept", "slept", "sleeping")),
            ("fight", ("fights", "fought", "fought", "fighting")),
            ("eat", ("eats", "ate", "eaten", "eating")),
            ("sing", ("sings", "sang", "sung", "singing")),
            ("swim", ("swims", "swam", "swum", "swimming")),
            ("drink", ("drinks", "drank", "drunk", "drinking")),
            ("arise", ("arises", "arose", "arisen", "arising")),
            ("bind", ("binds", "bound", "bound", "binding")),
            ("blow", ("blows", "blew", "blown", "blowing")),
            ("cast", ("casts", "cast", "cast", "casting")),
            ("feed", ("feeds", "fed", "fed", "feeding")),
            ("forget", ("forgets", "forgot", "forgotten", "forgetting")),
            ("freeze", ("freezes", "froze", "frozen", "freezing")),
            ("hang", ("hangs", "hung", "hung", "hanging")),
            ("hide", ("hides", "hid", "hidden", "hiding")),
            ("hit", ("hits", "hit", "hit", "hitting")),
            ("hurt", ("hurts", "hurt", "hurt", "hurting")),
            ("lay", ("lays", "laid", "laid", "laying")),
            ("lend", ("lends", "lent", "lent", "lending")),
            ("ride", ("rides", "rode", "ridden", "riding")),
            ("ring", ("rings", "rang", "rung", "ringing")),
            ("shake", ("shakes", "shook", "shaken", "shaking")),
            ("shine", ("shines", "shone", "shone", "shining")),
            ("shoot", ("shoots", "shot", "shot", "shooting")),
            ("shut", ("shuts", "shut", "shut", "shutting")),
            ("slide", ("slides", "slid", "slid", "sliding")),
            ("spread", ("spreads", "spread", "spread", "spreading")),
            ("steal", ("steals", "stole", "stolen", "stealing")),
            ("stick", ("sticks", "stuck", "stuck", "sticking")),
            ("strike", ("strikes", "struck", "struck", "striking")),
            ("swear", ("swears", "swore", "sworn", "swearing")),
            ("sweep", ("sweeps", "swept", "swept", "sweeping")),
            ("tear", ("tears", "tore", "torn", "tearing")),
            ("wake", ("wakes", "woke", "woken", "waking")),
            ("yield", ("yields", "yielded", "yielded", "yielding")),
        ];
        for (b, forms) in list {
            m.insert(b, forms);
        }
        m
    };

    /// Irregular noun plurals
    static ref NOUN_PLURALS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        let list = [
            ("child", "children"), ("man", "men"), ("woman", "women"), ("person", "people"),
            ("foot", "feet"), ("tooth", "teeth"), ("goose", "geese"), ("mouse", "mice"),
            ("ox", "oxen"), ("datum", "data"), ("criterion", "criteria"), ("phenomenon", "phenomena"),
            ("analysis", "analyses"), ("hypothesis", "hypotheses"), ("crisis", "crises"),
            ("thesis", "theses"), ("diagnosis", "diagnoses"), ("oasis", "oases"),
            ("basis", "bases"), ("ellipsis", "ellipses"), ("axis", "axes"),
            ("matrix", "matrices"), ("index", "indices"), ("appendix", "appendices"),
            ("fungus", "fungi"), ("cactus", "cacti"), ("nucleus", "nuclei"), ("stimulus", "stimuli"),
            ("syllabus", "syllabi"), ("radius", "radii"), ("focus", "foci"),
            ("life", "lives"), ("wife", "wives"), ("knife", "knives"), ("wolf", "wolves"),
            ("calf", "calves"), ("half", "halves"), ("leaf", "leaves"), ("loaf", "loaves"),
            ("shelf", "shelves"), ("thief", "thieves"),
        ];
        for (sing, pl) in list {
            m.insert(sing, pl);
        }
        m
    };

    /// Irregular adjectives / adverbs: base -> (comparative, superlative)
    static ref ADJ_COMPARISONS: HashMap<&'static str, (&'static str, &'static str)> = {
        let mut m = HashMap::new();
        let list = [
            ("good", ("better", "best")),
            ("well", ("better", "best")),
            ("bad", ("worse", "worst")),
            ("badly", ("worse", "worst")),
            ("far", ("further", "furthest")),
            ("little", ("less", "least")),
            ("much", ("more", "most")),
            ("many", ("more", "most")),
            ("old", ("older", "oldest")),
        ];
        for (b, comp) in list {
            m.insert(b, comp);
        }
        m
    };
}

pub struct Inflector;

impl Default for Inflector {
    fn default() -> Self {
        Self::new()
    }
}

impl Inflector {
    pub fn new() -> Self {
        Self
    }

    /// Inflects a base lemma to match the target POS tag, and applies original casing.
    pub fn inflect(&self, lemma: &str, target_pos: PosTag, casing: Casing) -> String {
        let lower = lemma.to_lowercase();
        let inflected_lower = match target_pos {
            // Nouns
            PosTag::NN | PosTag::NNP => lower.clone(),
            PosTag::NNS | PosTag::NNPS => self.inflect_noun_plural(&lower),

            // Verbs
            PosTag::VB | PosTag::VBP => lower.clone(),
            PosTag::VBZ => self.inflect_verb_3sg(&lower),
            PosTag::VBD => self.inflect_verb_past(&lower),
            PosTag::VBN => self.inflect_verb_past_participle(&lower),
            PosTag::VBG => self.inflect_verb_gerund(&lower),

            // Adjectives
            PosTag::JJ => lower.clone(),
            PosTag::JJR => self.inflect_adj_comparative(&lower),
            PosTag::JJS => self.inflect_adj_superlative(&lower),

            // Adverbs
            PosTag::RB => lower.clone(),
            PosTag::RBR => self.inflect_adv_comparative(&lower),
            PosTag::RBS => self.inflect_adv_superlative(&lower),

            _ => lower.clone(),
        };

        casing.apply(&inflected_lower)
    }

    pub fn inflect_noun_plural(&self, lemma: &str) -> String {
        if let Some(&pl) = NOUN_PLURALS.get(lemma) {
            return pl.to_string();
        }

        if lemma.ends_with("sis") && lemma.len() > 3 {
            return format!("{}ses", &lemma[..lemma.len() - 3]);
        }

        if lemma.ends_with('y') && lemma.len() > 2 {
            let last2 = lemma.as_bytes();
            let prev_char = last2[last2.len() - 2] as char;
            if !"aeiou".contains(prev_char) {
                return format!("{}ies", &lemma[..lemma.len() - 1]);
            }
        }

        if lemma.ends_with('s')
            || lemma.ends_with('x')
            || lemma.ends_with('z')
            || lemma.ends_with("ch")
            || lemma.ends_with("sh")
        {
            return format!("{}es", lemma);
        }

        if lemma.ends_with('f') && !lemma.ends_with("ff") && lemma.len() > 2
            && matches!(
                lemma,
                "leaf" | "half" | "calf" | "thief" | "loaf" | "shelf" | "wolf"
            ) {
                return format!("{}ves", &lemma[..lemma.len() - 1]);
            }

        if lemma.ends_with("fe") && lemma.len() > 3
            && matches!(lemma, "life" | "wife" | "knife") {
                return format!("{}ves", &lemma[..lemma.len() - 2]);
            }

        format!("{}s", lemma)
    }

    pub fn inflect_verb_3sg(&self, lemma: &str) -> String {
        if let Some(&(v3, _, _, _)) = VERB_CONJUGATIONS.get(lemma) {
            return v3.to_string();
        }

        if lemma.ends_with('y') && lemma.len() > 2 {
            let bytes = lemma.as_bytes();
            let prev_char = bytes[bytes.len() - 2] as char;
            if !"aeiou".contains(prev_char) {
                return format!("{}ies", &lemma[..lemma.len() - 1]);
            }
        }

        if lemma.ends_with('s')
            || lemma.ends_with('x')
            || lemma.ends_with('z')
            || lemma.ends_with("ch")
            || lemma.ends_with("sh")
            || lemma.ends_with('o')
        {
            return format!("{}es", lemma);
        }

        format!("{}s", lemma)
    }

    pub fn inflect_verb_past(&self, lemma: &str) -> String {
        if let Some(&(_, past, _, _)) = VERB_CONJUGATIONS.get(lemma) {
            return past.to_string();
        }

        if lemma.ends_with('e') {
            return format!("{}d", lemma);
        }

        if lemma.ends_with('y') && lemma.len() > 2 {
            let bytes = lemma.as_bytes();
            let prev_char = bytes[bytes.len() - 2] as char;
            if !"aeiou".contains(prev_char) {
                return format!("{}ied", &lemma[..lemma.len() - 1]);
            }
        }

        // CVC consonant doubling: plan -> planned, stop -> stopped, fit -> fitted, rob -> robbed
        if self.is_cvc(lemma) {
            let last_char = lemma.chars().last().unwrap();
            return format!("{}{}ed", lemma, last_char);
        }

        format!("{}ed", lemma)
    }

    pub fn inflect_verb_past_participle(&self, lemma: &str) -> String {
        if let Some(&(_, _, pp, _)) = VERB_CONJUGATIONS.get(lemma) {
            return pp.to_string();
        }
        self.inflect_verb_past(lemma)
    }

    pub fn inflect_verb_gerund(&self, lemma: &str) -> String {
        if let Some(&(_, _, _, ger)) = VERB_CONJUGATIONS.get(lemma) {
            return ger.to_string();
        }

        if let Some(stripped) = lemma.strip_suffix("ie") {
            return format!("{}ying", stripped);
        }

        if lemma.ends_with("ee") || lemma.ends_with("oe") || lemma.ends_with("ye") {
            return format!("{}ing", lemma);
        }

        if lemma.ends_with('e') && lemma.len() > 2 {
            return format!("{}ing", &lemma[..lemma.len() - 1]);
        }

        if self.is_cvc(lemma) {
            let last_char = lemma.chars().last().unwrap();
            return format!("{}{}ing", lemma, last_char);
        }

        format!("{}ing", lemma)
    }

    pub fn inflect_adj_comparative(&self, lemma: &str) -> String {
        if let Some(&(comp, _)) = ADJ_COMPARISONS.get(lemma) {
            return comp.to_string();
        }

        if lemma.ends_with('y') && lemma.len() > 2 {
            return format!("{}ier", &lemma[..lemma.len() - 1]);
        }

        if lemma.ends_with('e') {
            return format!("{}r", lemma);
        }

        if self.is_cvc(lemma) && lemma.len() <= 4 {
            let last_char = lemma.chars().last().unwrap();
            return format!("{}{}er", lemma, last_char);
        }

        if lemma.len() <= 5 {
            return format!("{}er", lemma);
        }

        // For longer adjectives, default to standard base form
        lemma.to_string()
    }

    pub fn inflect_adj_superlative(&self, lemma: &str) -> String {
        if let Some(&(_, sup)) = ADJ_COMPARISONS.get(lemma) {
            return sup.to_string();
        }

        if lemma.ends_with('y') && lemma.len() > 2 {
            return format!("{}iest", &lemma[..lemma.len() - 1]);
        }

        if lemma.ends_with('e') {
            return format!("{}st", lemma);
        }

        if self.is_cvc(lemma) && lemma.len() <= 4 {
            let last_char = lemma.chars().last().unwrap();
            return format!("{}{}est", lemma, last_char);
        }

        if lemma.len() <= 5 {
            return format!("{}est", lemma);
        }

        lemma.to_string()
    }

    pub fn inflect_adv_comparative(&self, lemma: &str) -> String {
        if let Some(&(comp, _)) = ADJ_COMPARISONS.get(lemma) {
            return comp.to_string();
        }
        lemma.to_string()
    }

    pub fn inflect_adv_superlative(&self, lemma: &str) -> String {
        if let Some(&(_, sup)) = ADJ_COMPARISONS.get(lemma) {
            return sup.to_string();
        }
        lemma.to_string()
    }

    fn is_cvc(&self, word: &str) -> bool {
        if word.len() < 3 || word.len() > 6 {
            return false;
        }

        // Exceptions: common verbs that do NOT double the final consonant
        if matches!(
            word,
            "offer"
                | "enter"
                | "open"
                | "happen"
                | "listen"
                | "visit"
                | "target"
                | "market"
                | "focus"
                | "gather"
                | "order"
                | "cover"
                | "discover"
                | "recover"
                | "deliver"
                | "alter"
                | "foster"
                | "bolster"
                | "prosper"
                | "limit"
                | "benefit"
                | "profit"
                | "credit"
                | "edit"
                | "audit"
        ) {
            return false;
        }

        let bytes = word.as_bytes();
        let len = bytes.len();
        let c1 = bytes[len - 3] as char;
        let v = bytes[len - 2] as char;
        let c2 = bytes[len - 1] as char;

        let vowels = "aeiou";
        !vowels.contains(c1) && vowels.contains(v) && !vowels.contains(c2) && !"wxy".contains(c2)
    }
}
