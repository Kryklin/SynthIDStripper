use crate::types::PosTag;
use std::collections::HashMap;

lazy_static::lazy_static! {
    /// Irregular verbs map (inflected -> base)
    static ref IRREGULAR_VERBS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        let list = [
            ("am", "be"), ("is", "be"), ("are", "be"), ("was", "be"), ("were", "be"), ("been", "be"), ("being", "be"),
            ("have", "have"), ("has", "have"), ("had", "have"), ("having", "have"),
            ("do", "do"), ("does", "do"), ("did", "do"), ("done", "do"), ("doing", "do"),
            ("say", "say"), ("says", "say"), ("said", "say"), ("saying", "say"),
            ("go", "go"), ("goes", "go"), ("went", "go"), ("gone", "go"), ("going", "go"),
            ("get", "get"), ("gets", "get"), ("got", "get"), ("gotten", "get"), ("getting", "get"),
            ("make", "make"), ("makes", "make"), ("made", "make"), ("making", "make"),
            ("know", "know"), ("knows", "know"), ("knew", "know"), ("known", "know"), ("knowing", "know"),
            ("think", "think"), ("thinks", "think"), ("thought", "think"), ("thinking", "think"),
            ("take", "take"), ("takes", "take"), ("took", "take"), ("taken", "take"), ("taking", "take"),
            ("see", "see"), ("sees", "see"), ("saw", "see"), ("seen", "see"), ("seeing", "see"),
            ("come", "come"), ("comes", "come"), ("came", "come"), ("coming", "come"),
            ("find", "find"), ("finds", "find"), ("found", "find"), ("finding", "find"),
            ("give", "give"), ("gives", "give"), ("gave", "give"), ("given", "give"), ("giving", "give"),
            ("tell", "tell"), ("tells", "tell"), ("told", "tell"), ("telling", "tell"),
            ("become", "become"), ("becomes", "become"), ("became", "become"), ("becoming", "become"),
            ("show", "show"), ("shows", "show"), ("showed", "show"), ("shown", "show"), ("showing", "show"),
            ("leave", "leave"), ("leaves", "leave"), ("left", "leave"), ("leaving", "leave"),
            ("feel", "feel"), ("feels", "feel"), ("felt", "feel"), ("feeling", "feel"),
            ("put", "put"), ("puts", "put"), ("putting", "put"),
            ("bring", "bring"), ("brings", "bring"), ("brought", "bring"), ("bringing", "bring"),
            ("begin", "begin"), ("begins", "begin"), ("began", "begin"), ("begun", "begin"), ("beginning", "begin"),
            ("keep", "keep"), ("keeps", "keep"), ("kept", "keep"), ("keeping", "keep"),
            ("hold", "hold"), ("holds", "hold"), ("held", "hold"), ("holding", "hold"),
            ("write", "write"), ("writes", "write"), ("wrote", "write"), ("written", "write"), ("writing", "write"),
            ("stand", "stand"), ("stands", "stand"), ("stood", "stand"), ("standing", "stand"),
            ("hear", "hear"), ("hears", "hear"), ("heard", "hear"), ("hearing", "hear"),
            ("let", "let"), ("lets", "let"), ("letting", "let"),
            ("mean", "mean"), ("means", "mean"), ("meant", "mean"), ("meaning", "mean"),
            ("set", "set"), ("sets", "set"), ("setting", "set"),
            ("meet", "meet"), ("meets", "meet"), ("met", "meet"), ("meeting", "meet"),
            ("run", "run"), ("runs", "run"), ("ran", "run"), ("running", "run"),
            ("pay", "pay"), ("pays", "pay"), ("paid", "pay"), ("paying", "pay"),
            ("sit", "sit"), ("sits", "sit"), ("sat", "sit"), ("sitting", "sit"),
            ("speak", "speak"), ("speaks", "speak"), ("spoke", "speak"), ("spoken", "speak"), ("speaking", "speak"),
            ("lie", "lie"), ("lies", "lie"), ("lay", "lie"), ("lain", "lie"), ("lying", "lie"),
            ("lead", "lead"), ("leads", "lead"), ("led", "lead"), ("leading", "lead"),
            ("read", "read"), ("reads", "read"), ("reading", "read"),
            ("grow", "grow"), ("grows", "grow"), ("grew", "grow"), ("grown", "grow"), ("growing", "grow"),
            ("lose", "lose"), ("loses", "lose"), ("lost", "lose"), ("losing", "lose"),
            ("fall", "fall"), ("falls", "fall"), ("fell", "fall"), ("fallen", "fall"), ("falling", "fall"),
            ("send", "send"), ("sends", "send"), ("sent", "send"), ("sending", "send"),
            ("build", "build"), ("builds", "build"), ("built", "build"), ("building", "build"),
            ("understand", "understand"), ("understands", "understand"), ("understood", "understand"), ("understanding", "understand"),
            ("draw", "draw"), ("draws", "draw"), ("drew", "draw"), ("drawn", "draw"), ("drawing", "draw"),
            ("break", "break"), ("breaks", "break"), ("broke", "break"), ("broken", "break"), ("breaking", "break"),
            ("spend", "spend"), ("spends", "spend"), ("spent", "spend"), ("spending", "spend"),
            ("cut", "cut"), ("cuts", "cut"), ("cutting", "cut"),
            ("rise", "rise"), ("rises", "rise"), ("rose", "rise"), ("risen", "rise"), ("rising", "rise"),
            ("drive", "drive"), ("drives", "drive"), ("drove", "drive"), ("driven", "drive"), ("driving", "drive"),
            ("buy", "buy"), ("buys", "buy"), ("bought", "buy"), ("buying", "buy"),
            ("wear", "wear"), ("wears", "wear"), ("wore", "wear"), ("worn", "wear"), ("wearing", "wear"),
            ("choose", "choose"), ("chooses", "choose"), ("chose", "choose"), ("chosen", "choose"), ("choosing", "choose"),
            ("seek", "seek"), ("seeks", "seek"), ("sought", "seek"), ("seeking", "seek"),
            ("deal", "deal"), ("deals", "deal"), ("dealt", "deal"), ("dealing", "deal"),
            ("win", "win"), ("wins", "win"), ("won", "win"), ("winning", "win"),
            ("fly", "fly"), ("flies", "fly"), ("flew", "fly"), ("flown", "fly"), ("flying", "fly"),
            ("teach", "teach"), ("teaches", "teach"), ("taught", "teach"), ("teaching", "teach"),
            ("catch", "catch"), ("catches", "catch"), ("caught", "catch"), ("catching", "catch"),
            ("throw", "throw"), ("throws", "throw"), ("threw", "throw"), ("thrown", "throw"), ("throwing", "throw"),
            ("sleep", "sleep"), ("sleeps", "sleep"), ("slept", "sleep"), ("sleeping", "sleep"),
            ("fight", "fight"), ("fights", "fight"), ("fought", "fight"), ("fighting", "fight"),
            ("eat", "eat"), ("eats", "eat"), ("ate", "eat"), ("eaten", "eat"), ("eating", "eat"),
            ("sing", "sing"), ("sings", "sing"), ("sang", "sing"), ("sung", "sing"), ("singing", "sing"),
            ("swim", "swim"), ("swims", "swim"), ("swam", "swim"), ("swum", "swim"), ("swimming", "swim"),
            ("drink", "drink"), ("drinks", "drink"), ("drank", "drink"), ("drunk", "drink"), ("drinking", "drink"),
            ("arise", "arise"), ("arises", "arise"), ("arose", "arise"), ("arisen", "arise"), ("arising", "arise"),
            ("bind", "bind"), ("binds", "bind"), ("bound", "bind"), ("binding", "bind"),
            ("blow", "blow"), ("blows", "blow"), ("blew", "blow"), ("blown", "blow"), ("blowing", "blow"),
            ("cast", "cast"), ("casts", "cast"), ("casting", "cast"),
            ("feed", "feed"), ("feeds", "feed"), ("fed", "feed"), ("feeding", "feed"),
            ("forget", "forget"), ("forgets", "forget"), ("forgot", "forget"), ("forgotten", "forget"), ("forgetting", "forget"),
            ("freeze", "freeze"), ("freezes", "freeze"), ("froze", "freeze"), ("frozen", "freeze"), ("freezing", "freeze"),
            ("hang", "hang"), ("hangs", "hang"), ("hung", "hang"), ("hanging", "hang"),
            ("hide", "hide"), ("hides", "hide"), ("hid", "hide"), ("hidden", "hide"), ("hiding", "hide"),
            ("hit", "hit"), ("hits", "hit"), ("hitting", "hit"),
            ("hurt", "hurt"), ("hurts", "hurt"), ("hurting", "hurt"),
            ("lay", "lay"), ("lays", "lay"), ("laid", "lay"), ("laying", "lay"),
            ("lend", "lend"), ("lends", "lend"), ("lent", "lend"), ("lending", "lead"),
            ("ride", "ride"), ("rides", "ride"), ("rode", "ride"), ("ridden", "ride"), ("riding", "ride"),
            ("ring", "ring"), ("rings", "ring"), ("rang", "ring"), ("rung", "ring"), ("ringing", "ring"),
            ("shake", "shake"), ("shakes", "shake"), ("shook", "shake"), ("shaken", "shake"), ("shaking", "shake"),
            ("shine", "shine"), ("shines", "shine"), ("shone", "shine"), ("shining", "shine"),
            ("shoot", "shoot"), ("shoots", "shoot"), ("shot", "shoot"), ("shooting", "shoot"),
            ("shut", "shut"), ("shuts", "shut"), ("shutting", "shut"),
            ("slide", "slide"), ("slides", "slide"), ("slid", "slide"), ("sliding", "slide"),
            ("spread", "spread"), ("spreads", "spread"), ("spreading", "spread"),
            ("steal", "steal"), ("steals", "steal"), ("stole", "steal"), ("stolen", "steal"), ("stealing", "steal"),
            ("stick", "stick"), ("sticks", "stick"), ("stuck", "stick"), ("sticking", "stick"),
            ("strike", "strike"), ("strikes", "strike"), ("struck", "strike"), ("striking", "strike"),
            ("swear", "swear"), ("swears", "swear"), ("swore", "swear"), ("sworn", "swear"), ("swearing", "swear"),
            ("sweep", "sweep"), ("sweeps", "sweep"), ("swept", "sweep"), ("sweeping", "sweep"),
            ("tear", "tear"), ("tears", "tear"), ("tore", "tear"), ("torn", "tear"), ("tearing", "tear"),
            ("wake", "wake"), ("wakes", "wake"), ("woke", "wake"), ("woken", "wake"), ("waking", "wake"),
            ("yield", "yield"), ("yields", "yield"), ("yielded", "yield"), ("yielding", "yield"),
        ];
        for (inf, base) in list {
            m.insert(inf, base);
        }
        m
    };

    /// Irregular nouns map (plural -> singular)
    static ref IRREGULAR_NOUNS: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        let list = [
            ("children", "child"), ("men", "man"), ("women", "woman"), ("people", "person"),
            ("feet", "foot"), ("teeth", "tooth"), ("geese", "goose"), ("mice", "mouse"),
            ("oxen", "ox"), ("data", "datum"), ("criteria", "criterion"), ("phenomena", "phenomenon"),
            ("analyses", "analysis"), ("hypotheses", "hypothesis"), ("crises", "crisis"),
            ("theses", "thesis"), ("diagnoses", "diagnosis"), ("oases", "oasis"),
            ("bases", "basis"), ("ellipses", "ellipsis"), ("axes", "axis"),
            ("matrices", "matrix"), ("indices", "index"), ("appendices", "appendix"),
            ("fungi", "fungus"), ("cacti", "cactus"), ("nuclei", "nucleus"), ("stimuli", "stimulus"),
            ("syllabi", "syllabus"), ("radii", "radius"), ("foci", "focus"),
            ("lives", "life"), ("wives", "wife"), ("knives", "knife"), ("wolves", "wolf"),
            ("calves", "calf"), ("halves", "half"), ("leaves", "leaf"), ("loaves", "loaf"),
            ("shelves", "shelf"), ("thieves", "thief"), ("ourselves", "myself"),
        ];
        for (pl, sing) in list {
            m.insert(pl, sing);
        }
        m
    };

    /// Irregular adjectives/adverbs (comparative/superlative -> positive)
    static ref IRREGULAR_ADJ_ADV: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        let list = [
            ("better", "good"), ("best", "good"),
            ("worse", "bad"), ("worst", "bad"),
            ("farther", "far"), ("farthest", "far"),
            ("further", "far"), ("furthest", "far"),
            ("less", "little"), ("least", "little"),
            ("more", "much"), ("most", "much"),
            ("older", "old"), ("oldest", "old"),
            ("elder", "old"), ("eldest", "old"),
        ];
        for (comp, base) in list {
            m.insert(comp, base);
        }
        m
    };
}

pub struct Lemmatizer;

impl Default for Lemmatizer {
    fn default() -> Self {
        Self::new()
    }
}

impl Lemmatizer {
    pub fn new() -> Self {
        Self
    }

    /// Lemmatizes a word given its POS tag.
    pub fn lemmatize(&self, word: &str, pos: PosTag) -> String {
        let lower = word.to_lowercase();

        match pos {
            PosTag::NNS | PosTag::NNPS => self.lemmatize_noun(&lower),
            PosTag::VB | PosTag::VBD | PosTag::VBG | PosTag::VBN | PosTag::VBP | PosTag::VBZ => {
                self.lemmatize_verb(&lower)
            }
            PosTag::JJR | PosTag::JJS => self.lemmatize_adj(&lower),
            PosTag::RBR | PosTag::RBS => self.lemmatize_adv(&lower),
            _ => lower,
        }
    }

    pub fn lemmatize_noun(&self, word: &str) -> String {
        if let Some(&sing) = IRREGULAR_NOUNS.get(word) {
            return sing.to_string();
        }

        if word.ends_with("ies") && word.len() > 4 {
            let base = &word[..word.len() - 3];
            return format!("{}y", base);
        }

        if word.ends_with("ves") && word.len() > 4 {
            let base = &word[..word.len() - 3];
            return format!("{}f", base);
        }

        if word.ends_with("ses")
            || word.ends_with("xes")
            || word.ends_with("shes")
            || word.ends_with("ches")
        {
            return word[..word.len() - 2].to_string();
        }

        if word.ends_with('s')
            && !word.ends_with("ss")
            && !word.ends_with("us")
            && !word.ends_with("is")
        {
            return word[..word.len() - 1].to_string();
        }

        word.to_string()
    }

    pub fn lemmatize_verb(&self, word: &str) -> String {
        if let Some(&base) = IRREGULAR_VERBS.get(word) {
            return base.to_string();
        }

        // Present participle / gerund (-ing)
        if word.ends_with("ing") && word.len() > 4 {
            let base = &word[..word.len() - 3];
            // Double consonants: stopping -> stop, running -> run
            if base.len() >= 3 {
                let bytes = base.as_bytes();
                let last = bytes[bytes.len() - 1];
                let second_last = bytes[bytes.len() - 2];
                if last == second_last && !"aeiou".contains(last as char) {
                    return base[..base.len() - 1].to_string();
                }
            }
            // If base ends in consonant cluster (e.g. transforming, working, helping, starting)
            if self.is_consonant_cluster_ending(base)
                || base.ends_with("ow")
                || base.ends_with("ay")
                || base.ends_with("ey")
                || base.ends_with("oy")
            {
                return base.to_string();
            }
            // E-dropping: making -> make, taking -> take, creating -> create
            if !base.ends_with('y')
                && !base.ends_with("ee")
                && !base.ends_with("oe")
                && base.len() > 2
            {
                return format!("{}e", base);
            }
            return base.to_string();
        }

        // Past tense / participle (-ed)
        if word.ends_with("ied") && word.len() > 4 {
            let base = &word[..word.len() - 3];
            return format!("{}y", base);
        }

        if word.ends_with("ed") && word.len() > 3 {
            let base = &word[..word.len() - 2];
            // Double consonants: stopped -> stop, planned -> plan
            if base.len() >= 3 {
                let bytes = base.as_bytes();
                let last = bytes[bytes.len() - 1];
                let second_last = bytes[bytes.len() - 2];
                if last == second_last && !"aeiou".contains(last as char) {
                    return base[..base.len() - 1].to_string();
                }
            }
            if base.ends_with('e') {
                return base.to_string();
            }
            // If base ends in consonant cluster or vowel dipthong: transform, help, start, look, play, follow
            if self.is_consonant_cluster_ending(base)
                || base.ends_with("ow")
                || base.ends_with("ay")
                || base.ends_with("ey")
                || base.ends_with("oy")
            {
                return base.to_string();
            }
            // If base ends in single consonant likely from e-drop (creat-ed -> create, chang-ed -> change, lov-ed -> love)
            if base.ends_with('t')
                || base.ends_with('g')
                || base.ends_with('v')
                || base.ends_with('z')
                || base.ends_with('s')
                || base.ends_with('c')
                || base.ends_with('d')
            {
                return format!("{}e", base);
            }
            return base.to_string();
        }

        // 3rd singular present (-s / -es)
        if word.ends_with("ies") && word.len() > 4 {
            let base = &word[..word.len() - 3];
            return format!("{}y", base);
        }

        if word.ends_with("es")
            && (word.ends_with("sses")
                || word.ends_with("shes")
                || word.ends_with("ches")
                || word.ends_with("xes")
                || word.ends_with("zes"))
        {
            return word[..word.len() - 2].to_string();
        }

        if word.ends_with('s') && !word.ends_with("ss") {
            return word[..word.len() - 1].to_string();
        }

        word.to_string()
    }

    fn is_consonant_cluster_ending(&self, base: &str) -> bool {
        if base.len() < 2 {
            return false;
        }
        let bytes = base.as_bytes();
        let last = bytes[bytes.len() - 1] as char;
        let second_last = bytes[bytes.len() - 2] as char;
        let vowels = "aeiou";
        !vowels.contains(last) && !vowels.contains(second_last)
    }

    pub fn lemmatize_adj(&self, word: &str) -> String {
        if let Some(&base) = IRREGULAR_ADJ_ADV.get(word) {
            return base.to_string();
        }

        if word.ends_with("iest") && word.len() > 5 {
            return format!("{}y", &word[..word.len() - 4]);
        }
        if word.ends_with("ier") && word.len() > 4 {
            return format!("{}y", &word[..word.len() - 3]);
        }
        if word.ends_with("est") && word.len() > 4 {
            let base = &word[..word.len() - 3];
            if base.ends_with('e') {
                return base.to_string();
            }
            return base.to_string();
        }
        if word.ends_with("er") && word.len() > 3 {
            let base = &word[..word.len() - 2];
            return base.to_string();
        }

        word.to_string()
    }

    pub fn lemmatize_adv(&self, word: &str) -> String {
        if let Some(&base) = IRREGULAR_ADJ_ADV.get(word) {
            return base.to_string();
        }
        word.to_string()
    }
}
