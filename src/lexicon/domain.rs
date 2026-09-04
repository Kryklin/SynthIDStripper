use crate::types::{CoarsePos, Domain, LexicalClass};
use std::collections::{HashMap, HashSet};

lazy_static::lazy_static! {
    /// Curated domain-locked technical vocabularies that MUST NOT be perturbed
    static ref FINANCE_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "ebitda", "liquidity", "yield", "amortization", "insolvency", "leverage",
            "securities", "dividend", "portfolio", "collateral", "arbitrage",
            "underwriting", "macroeconomic", "inflationary", "deflationary", "derivatives",
            "annuity", "depreciation", "accrual", "solvency", "fiduciary", "equity",
            "bullish", "bearish", "nasdaq", "nyse", "sp500", "sec", "treasury"
        ] {
            s.insert(w);
        }
        s
    };

    static ref BIOMEDICAL_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "apoptosis", "cytokine", "pathogen", "oncogene", "receptor", "metabolite",
            "phagocytosis", "genomic", "mrna", "histology", "assay", "enzyme",
            "substrate", "antibody", "antigen", "polypeptide", "titer", "pathology",
            "mitochondria", "dna", "rna", "crispr", "carcinogen", "homeostasis",
            "ischemia", "necrosis", "hypertrophy", "etiology", "pharmacokinetics"
        ] {
            s.insert(w);
        }
        s
    };

    static ref CS_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "compiler", "latency", "throughput", "mutex", "semaphore", "deadlock",
            "recursion", "polymorphism", "bytecode", "stack", "heap", "hashmap",
            "bandwidth", "pipeline", "buffer", "concurrency", "asynchronous",
            "endianness", "regex", "tokenizer", "pointer", "syscall", "kernel",
            "daemon", "hypervisor", "llvm", "posix", "api", "json", "sql"
        ] {
            s.insert(w);
        }
        s
    };

    static ref LEGAL_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "subpoena", "tort", "plaintiff", "defendant", "affidavit", "jurisdiction",
            "statute", "indictment", "adjudication", "injunction", "depose", "testimony",
            "certiorari", "appellant", "appellee", "voir dire", "estoppel", "arbitration"
        ] {
            s.insert(w);
        }
        s
    };

    static ref CULINARY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "sommelier", "stagiaire", "stagiaires", "gastronomy", "michelin", "sous-chef",
            "restaurateur", "maître d'", "umami", "charcuterie", "confectionery",
            "vinaigrette", "centrifuge", "centrifuges", "biodynamic", "micro-cilantro",
            "cilantro", "turnip", "tweezers", "pan", "skillet", "braise", "blanch",
            "caramelization", "emulsify", "sauté", "fondant", "ganache", "julienne",
            "fermentation", "sous-vide", "tasting", "degrease", "reduction", "foams",
            "gels", "steak", "tartare", "fondue", "saucier", "patisserie", "boulangerie",
            "service"
        ] {
            s.insert(w);
        }
        s
    };

    static ref CREATIVE_ARTS_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "chiaroscuro", "triptych", "fresco", "sculpture", "choreography", "curator",
            "sonata", "symphony", "pentameter", "libretto", "tableau", "lithograph",
            "gouache", "surrealism", "modernism", "cubism", "impressionism", "avant-garde",
            "contrapposto", "composition", "monochrome", "polyphony", "tessellation"
        ] {
            s.insert(w);
        }
        s
    };

    static ref SCIENCE_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "spectroscopy", "photosynthesis", "entropy", "neutrino", "isotope", "catalyst",
            "electron", "proton", "neutron", "quantum", "relativity", "thermodynamics",
            "crystallography", "spectrometry", "chromatography", "electrolysis", "valence",
            "superconductivity", "boson", "hadron", "quark", "enthalpy", "optics"
        ] {
            s.insert(w);
        }
        s
    };

    static ref POLITICS_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "filibuster", "gerrymandering", "incumbent", "referendum", "parliament",
            "senate", "constituency", "caucus", "impeachment", "electorate", "plebiscite",
            "bicameral", "gubernatorial", "sovereignty", "plenary", "bipartisan",
            "demagogue", "hegemony", "geopolitics", "suffrage", "lobbyist"
        ] {
            s.insert(w);
        }
        s
    };

    static ref SPORTS_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "quarterback", "goalkeeper", "decathlon", "playoff", "tournament", "hat-trick",
            "heptathlon", "velodrome", "steeplechase", "offside", "scrimmage", "touchdown",
            "paragliding", "triathlon", "marathon", "pole vault", "relay", "olympiad"
        ] {
            s.insert(w);
        }
        s
    };

    static ref EDUCATION_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "pedagogy", "syllabus", "dissertation", "matriculation", "alumni",
            "baccalaureate", "practicum", "curriculum", "dean", "provost",
            "valedictorian", "salutatorian", "tenure", "sabbatical", "accreditation"
        ] {
            s.insert(w);
        }
        s
    };

    static ref PHILOSOPHY_PSYCHOLOGY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "epistemology", "phenomenology", "hermeneutics", "psychoanalysis",
            "subconscious", "behaviorism", "ontology", "metaphysics", "teleology",
            "existentialism", "solipsism", "empiricism", "deontology", "utilitarianism",
            "dialectic", "neurosis", "gestalt", "heuristics", "determinism"
        ] {
            s.insert(w);
        }
        s
    };

    static ref MILITARY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "brigade", "reconnaissance", "ballistics", "artillery", "battalion",
            "infantry", "garrison", "regiment", "flank", "armored", "amphibious",
            "sortie", "vanguard", "shrapnel", "howitzer", "ordnance", "platoon",
            "squadron", "insurgency", "tactical", "counteroffensive"
        ] {
            s.insert(w);
        }
        s
    };

    static ref JOURNALISM_MEDIA_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "byline", "op-ed", "broadsheet", "broadcast", "syndication", "press",
            "tabloid", "wire service", "masthead", "newsroom", "correspondent",
            "photojournalism", "investigative", "editorial", "telecast", "newscast"
        ] {
            s.insert(w);
        }
        s
    };

    static ref TRAVEL_TOURISM_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "itinerary", "excursion", "concierge", "lodging", "sightseeing",
            "ecotourism", "backpacking", "wayfinding", "passport", "layover",
            "customs", "boarding pass", "guidebook", "hospitality", "glamping"
        ] {
            s.insert(w);
        }
        s
    };

    static ref AVIATION_AEROSPACE_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "avionics", "telemetry", "fuselage", "altimeter", "yaw", "pitch", "roll",
            "mach", "orbit", "stall", "aileron", "thrust", "supersonic", "hypersonic",
            "autopilot", "transponder", "runway", "airspace", "empennage", "apogee",
            "perigee", "payload", "propulsion", "waypoint", "cockpit", "afterburner"
        ] {
            s.insert(w);
        }
        s
    };

    static ref ARCHITECTURE_CONSTRUCTION_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "cantilever", "load-bearing", "hvac", "easement", "brutalism", "masonry",
            "facade", "scaffolding", "blueprint", "cornice", "lintel", "truss",
            "fenestration", "soffit", "parapet", "mezzanine", "clerestory", "rebar",
            "joist", "girder", "fenestrations", "buttress", "cupola"
        ] {
            s.insert(w);
        }
        s
    };

    static ref GAMING_ESPORTS_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "hitbox", "aggro", "frame data", "rng", "dps", "nerfed", "buffed",
            "matchmaking", "respawn", "speedrun", "cooldown", "esports", "gameplay",
            "nerf", "buff", "meta", "loot", "npc", "mmorpg", "perk", "glitch",
            "gank", "pvp", "pve", "caster", "streamer", "speedrunner"
        ] {
            s.insert(w);
        }
        s
    };

    static ref AGRICULTURE_BOTANY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "hydroponics", "grafting", "agronomy", "cultivar", "photosynthesis",
            "npk", "herbicide", "pesticide", "tillage", "pollination", "germination",
            "chlorophyll", "silage", "monoculture", "rhizome", "angiosperm",
            "gymnosperm", "stoma", "phloem", "xylem", "mycorrhizae", "horticulture"
        ] {
            s.insert(w);
        }
        s
    };

    static ref FASHION_TEXTILES_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "selvage", "warp", "weft", "bias cut", "silhouette", "seam", "bespoke",
            "cashmere", "haute couture", "mannequin", "chiffon", "draping", "couturier",
            "houndstooth", "apparel", "tapestry", "interfacing", "pleating", "muslin",
            "bodice", "garment", "haberdashery", "millinery"
        ] {
            s.insert(w);
        }
        s
    };

    static ref THEOLOGY_RELIGION_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "exegesis", "ecclesiastical", "dogma", "liturgy", "secular", "orthodox",
            "sacrament", "theology", "eschatology", "soteriology", "homiletics",
            "canon", "doxology", "ecumenical", "scripture", "hermeneutic",
            "catechism", "synod", "deity", "pontiff", "episcopal", "diocese"
        ] {
            s.insert(w);
        }
        s
    };

    static ref AUDIO_ENGINEERING_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "reverb", "equalization", "lossless", "polyrhythm", "attenuation",
            "spectrogram", "decibel", "preamp", "dither", "subwoofer", "sibilance",
            "transducer", "frequency", "hertz", "panning", "limiter", "equalizer",
            "gain", "waveform", "bitrate", "daw", "sidechain"
        ] {
            s.insert(w);
        }
        s
    };

    static ref MARITIME_NAUTICAL_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "starboard", "ballast", "keel", "draft", "bulkhead", "knot", "halyard",
            "portside", "rudder", "anchor", "propeller", "nautical", "vessel",
            "bow", "stern", "leeward", "windward", "transom", "capstan", "sonar",
            "buoy", "maritime", "bilge", "coxswain", "bosun", "dead reckoning"
        ] {
            s.insert(w);
        }
        s
    };

    static ref AUTOMOTIVE_MOTORSPORT_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "camshaft", "chassis", "torque", "oversteer", "slipstream", "drivetrain",
            "telemetry", "understeer", "horsepower", "paddock", "piston", "transmission",
            "turbocharger", "supercharger", "aerodynamics", "downforce", "crankshaft",
            "differential", "tachometer", "powertrain", "suspension", "spoiler"
        ] {
            s.insert(w);
        }
        s
    };

    static ref FILM_TELEVISION_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "mise-en-scène", "foley", "anamorphic", "best boy", "storyboard", "render",
            "aperture", "screenplay", "cinematographer", "gaffer", "boom operator",
            "cinematography", "telephoto", "clapperboard", "dolly", "steadicam",
            "color grading", "soundstage", "b-roll", "grip", "auteur"
        ] {
            s.insert(w);
        }
        s
    };

    static ref LINGUISTICS_PHILOLOGY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "phoneme", "morphology", "fricative", "diphthong", "syntax", "semantic",
            "conjugation", "morpheme", "etymology", "orthography", "dialect",
            "allophone", "sociolinguistics", "glottochronology", "prosody", "affix",
            "inflectional", "phonetics", "philology", "lexicon", "cognate"
        ] {
            s.insert(w);
        }
        s
    };

    static ref REAL_ESTATE_PROPERTY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "escrow", "lien", "appraisal", "zoning", "sublet", "title",
            "gentrification", "mortgage", "tenant", "landlord", "foreclosure",
            "leasehold", "deed", "freehold", "easement", "realtor", "equity",
            "covenant", "closing costs", "condominium", "conveyance"
        ] {
            s.insert(w);
        }
        s
    };

    static ref LOGISTICS_SUPPLY_CHAIN_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "intermodal", "procurement", "bill of lading", "sku", "freight",
            "bottleneck", "inventory", "supply chain", "pallet", "distribution center",
            "logistics", "warehousing", "fulfillment", "manifest", "consignee",
            "consignor", "stevedore", "freight forwarder", "deadweight"
        ] {
            s.insert(w);
        }
        s
    };

    static ref FITNESS_KINESIOLOGY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "hypertrophy", "anabolic", "macros", "deadlift", "isometric", "glycogen",
            "superset", "barbell", "dumbbell", "cardio", "aerobic", "bench press",
            "squat", "biomechanics", "calisthenics", "catabolism", "plyometrics",
            "metabolism", "kinesiology", "concentric", "eccentric"
        ] {
            s.insert(w);
        }
        s
    };

    static ref OCCULT_ASTROLOGY_LOCKED: HashSet<&'static str> = {
        let mut s = HashSet::new();
        for &w in &[
            "retrograde", "sigil", "astral", "manifestation", "divination",
            "natal chart", "esoteric", "horoscope", "talisman", "occult",
            "pentacle", "clairvoyance", "tarot", "zodiac", "astrology",
            "alchemy", "grimoire", "incantation", "ascendant", "horary", "mediumship"
        ] {
            s.insert(w);
        }
        s
    };

    /// Curated domain-preferred equivalence mappings
    static ref FINANCE_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("firm", vec!["enterprise", "company", "corporation", "business"]);
        m.insert("company", vec!["firm", "enterprise", "corporation", "business"]);
        m.insert("enterprise", vec!["firm", "company", "corporation", "business"]);
        m.insert("investor", vec!["shareholder", "stakeholder", "backer"]);
        m.insert("profit", vec!["earnings", "net income", "returns"]);
        m.insert("revenue", vec!["sales", "turnover", "top-line"]);
        m.insert("capital", vec!["funds", "financing", "assets"]);
        m.insert("valuation", vec!["market value", "appraisal", "market cap"]);
        m
    };

    static ref BIOMEDICAL_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("malignant", vec!["cancerous", "invasive"]);
        m.insert("cancerous", vec!["malignant", "invasive"]);
        m.insert("dosage", vec!["dose", "administration amount"]);
        m.insert("dose", vec!["dosage", "administered amount"]);
        m.insert("physician", vec!["doctor", "clinician", "practitioner"]);
        m.insert("doctor", vec!["physician", "clinician"]);
        m.insert("efficacy", vec!["effectiveness", "potency"]);
        m.insert("effectiveness", vec!["efficacy", "potency"]);
        m.insert("symptom", vec!["manifestation", "sign", "indication"]);
        m
    };

    static ref CS_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("execute", vec!["run", "process", "invoke"]);
        m.insert("run", vec!["execute", "process"]);
        m.insert("compute", vec!["calculate", "evaluate", "process"]);
        m.insert("storage", vec!["memory", "disk space", "capacity"]);
        m.insert("retrieve", vec!["fetch", "query", "load"]);
        m.insert("fetch", vec!["retrieve", "query", "load"]);
        m.insert("throughput", vec!["bandwidth", "capacity", "data rate"]);
        m
    };

    static ref LEGAL_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("attorney", vec!["counsel", "lawyer", "advocate"]);
        m.insert("lawyer", vec!["attorney", "counsel", "advocate"]);
        m.insert("counsel", vec!["attorney", "lawyer"]);
        m.insert("statute", vec!["regulation", "legislation", "law"]);
        m.insert("regulation", vec!["statute", "rule", "ordinance"]);
        m.insert("stipulate", vec!["specify", "require", "mandate"]);
        m.insert("verdict", vec!["ruling", "decision", "judgment"]);
        m.insert("ruling", vec!["verdict", "decision", "judgment"]);
        m
    };

    static ref CULINARY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("chef", vec!["cook", "culinary artist"]);
        m.insert("cook", vec!["chef", "prepare food"]);
        m.insert("cooking", vec!["culinary preparation", "food preparation"]);
        m.insert("kitchen", vec!["cookhouse", "galley"]);
        m.insert("dining", vec!["eating", "feasting"]);
        m.insert("restaurant", vec!["bistro", "eatery", "brasserie", "dining establishment"]);
        m.insert("dish", vec!["course", "plate", "creation"]);
        m.insert("food", vec!["cuisine", "fare", "nourishment"]);
        m.insert("produce", vec!["ingredients", "farm goods", "fresh goods"]);
        m.insert("meal", vec!["repast", "dinner", "feast"]);
        m.insert("recipe", vec!["formula", "preparation guide"]);
        m.insert("cuisine", vec!["cooking style", "culinary tradition"]);
        m.insert("sommelier", vec!["wine steward", "wine director"]);
        m
    };

    static ref CREATIVE_ARTS_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("artist", vec!["creator", "artisan"]);
        m.insert("exhibition", vec!["gallery show", "display", "presentation"]);
        m.insert("painting", vec!["canvas", "artwork"]);
        m.insert("music", vec!["musical composition", "score"]);
        m.insert("aesthetic", vec!["visual style", "artistic sensibility"]);
        m.insert("sculpture", vec!["statue", "carving"]);
        m
    };

    static ref SCIENCE_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("hypothesis", vec!["theory", "conjecture", "postulate"]);
        m.insert("experiment", vec!["trial", "investigation", "test"]);
        m.insert("laboratory", vec!["lab", "research facility"]);
        m.insert("phenomenon", vec!["occurrence", "event", "observation"]);
        m.insert("theory", vec!["framework", "model", "hypothesis"]);
        m
    };

    static ref POLITICS_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("politician", vec!["legislator", "lawmaker", "statesman"]);
        m.insert("election", vec!["vote", "ballot", "poll"]);
        m.insert("campaign", vec!["candidacy drive", "canvass"]);
        m.insert("policy", vec!["statute", "strategy", "doctrine"]);
        m.insert("treaty", vec!["accord", "pact", "convention"]);
        m
    };

    static ref SPORTS_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("athlete", vec!["competitor", "player", "sportsman"]);
        m.insert("coach", vec!["trainer", "manager", "instructor"]);
        m.insert("championship", vec!["tournament title", "finals"]);
        m.insert("stadium", vec!["arena", "coliseum", "ballpark"]);
        m.insert("league", vec!["association", "conference", "circuit"]);
        m
    };

    static ref EDUCATION_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("student", vec!["pupil", "learner", "scholar"]);
        m.insert("teacher", vec!["instructor", "educator", "tutor"]);
        m.insert("course", vec!["class", "subject of study"]);
        m.insert("lecture", vec!["presentation", "instructional talk"]);
        m.insert("curriculum", vec!["course of study", "syllabus"]);
        m
    };

    static ref PHILOSOPHY_PSYCHOLOGY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("cognition", vec!["mental process", "thought process"]);
        m.insert("perception", vec!["sensory apprehension", "awareness"]);
        m.insert("ethics", vec!["moral philosophy", "morality"]);
        m.insert("belief", vec!["conviction", "creed", "view"]);
        m.insert("mind", vec!["psyche", "intellect", "consciousness"]);
        m
    };

    static ref MILITARY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("commander", vec!["officer", "leader", "chief"]);
        m.insert("soldier", vec!["troop", "fighter", "service member"]);
        m.insert("tactics", vec!["operational methods", "strategy"]);
        m.insert("weapon", vec!["armament", "munition", "arms"]);
        m.insert("base", vec!["outpost", "installation", "compound"]);
        m
    };

    static ref JOURNALISM_MEDIA_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("reporter", vec!["journalist", "correspondent", "writer"]);
        m.insert("headline", vec!["banner headline", "news title"]);
        m.insert("article", vec!["piece", "story", "report"]);
        m.insert("newspaper", vec!["daily paper", "journal", "gazette"]);
        m.insert("columnist", vec!["commentator", "essayist", "writer"]);
        m
    };

    static ref TRAVEL_TOURISM_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("traveler", vec!["tourist", "voyager", "passenger", "visitor"]);
        m.insert("vacation", vec!["holiday", "getaway", "trip"]);
        m.insert("hotel", vec!["resort", "inn", "accommodations"]);
        m.insert("destination", vec!["locale", "travel spot", "location"]);
        m.insert("journey", vec!["trip", "voyage", "excursion"]);
        m
    };

    static ref AVIATION_AEROSPACE_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("aircraft", vec!["airplane", "plane", "flight vehicle"]);
        m.insert("airplane", vec!["aircraft", "plane"]);
        m.insert("pilot", vec!["aviator", "airman", "flyer"]);
        m.insert("flight", vec!["air journey", "flying trip"]);
        m.insert("landing", vec!["touchdown", "arrival"]);
        m.insert("takeoff", vec!["departure", "liftoff"]);
        m.insert("altitude", vec!["elevation", "flight level"]);
        m
    };

    static ref ARCHITECTURE_CONSTRUCTION_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("building", vec!["structure", "edifice"]);
        m.insert("structure", vec!["building", "edifice", "construction"]);
        m.insert("architect", vec!["designer", "master builder"]);
        m.insert("foundation", vec!["footing", "base substructure"]);
        m.insert("construction", vec!["building", "structural erection"]);
        m.insert("renovation", vec!["remodeling", "refurbishment"]);
        m
    };

    static ref GAMING_ESPORTS_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("player", vec!["gamer", "competitor"]);
        m.insert("game", vec!["title", "video game"]);
        m.insert("character", vec!["avatar", "champion", "hero"]);
        m.insert("defeat", vec!["eliminate", "vanquish"]);
        m.insert("level", vec!["stage", "zone", "map"]);
        m.insert("strategy", vec!["game plan", "tactics", "playbook"]);
        m
    };

    static ref AGRICULTURE_BOTANY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("farmer", vec!["grower", "cultivator", "agriculturist"]);
        m.insert("crop", vec!["harvest", "yield", "produce"]);
        m.insert("harvest", vec!["crop yield", "gathering"]);
        m.insert("soil", vec!["earth", "topsoil", "ground"]);
        m.insert("plant", vec!["flora", "botanical specimen"]);
        m.insert("cultivate", vec!["grow", "farm", "raise"]);
        m
    };

    static ref FASHION_TEXTILES_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("garment", vec!["apparel item", "attire", "clothing"]);
        m.insert("clothing", vec!["garments", "attire", "wardrobe"]);
        m.insert("fabric", vec!["textile", "cloth", "material"]);
        m.insert("designer", vec!["couturier", "stylist"]);
        m.insert("runway", vec!["catwalk", "fashion show"]);
        m.insert("tailor", vec!["clothier", "outfitter"]);
        m
    };

    static ref THEOLOGY_RELIGION_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("clergy", vec!["ministry", "priesthood"]);
        m.insert("sermon", vec!["homily", "religious discourse"]);
        m.insert("doctrine", vec!["creed", "tenet", "teaching"]);
        m.insert("prayer", vec!["supplication", "invocation", "devotion"]);
        m.insert("faith", vec!["religious belief", "creed"]);
        m.insert("worship", vec!["veneration", "reverence", "devotion"]);
        m
    };

    static ref AUDIO_ENGINEERING_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("sound", vec!["audio", "acoustic signal"]);
        m.insert("recording", vec!["audio track", "take", "capture"]);
        m.insert("mixing", vec!["audio blending", "balance"]);
        m.insert("mastering", vec!["audio finishing", "finalizing"]);
        m.insert("loudness", vec!["sound volume", "gain level"]);
        m.insert("microphone", vec!["mic", "transducer"]);
        m
    };

    static ref MARITIME_NAUTICAL_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("ship", vec!["vessel", "boat", "craft"]);
        m.insert("boat", vec!["vessel", "craft", "skiff"]);
        m.insert("sailor", vec!["mariner", "seaman", "navigator"]);
        m.insert("captain", vec!["skipper", "master", "commanding officer"]);
        m.insert("voyage", vec!["passage", "cruise", "sail"]);
        m.insert("harbor", vec!["port", "haven", "marina"]);
        m
    };

    static ref AUTOMOTIVE_MOTORSPORT_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("car", vec!["automobile", "vehicle"]);
        m.insert("vehicle", vec!["automobile", "motorcar", "car"]);
        m.insert("driver", vec!["racer", "motorist", "pilot"]);
        m.insert("race", vec!["grand prix", "motorsport event", "competition"]);
        m.insert("speed", vec!["velocity", "pace"]);
        m.insert("engine", vec!["motor", "powertrain"]);
        m
    };

    static ref FILM_TELEVISION_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("movie", vec!["film", "motion picture", "feature"]);
        m.insert("film", vec!["movie", "motion picture", "picture"]);
        m.insert("director", vec!["filmmaker", "helmer"]);
        m.insert("actor", vec!["performer", "thespian", "player"]);
        m.insert("scene", vec!["sequence", "shot", "take"]);
        m.insert("camera", vec!["motion camera", "imaging device"]);
        m
    };

    static ref LINGUISTICS_PHILOLOGY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("language", vec!["tongue", "linguistic system", "speech"]);
        m.insert("word", vec!["lexical item", "vocable", "term"]);
        m.insert("grammar", vec!["syntax", "grammatical rules"]);
        m.insert("dialect", vec!["vernacular", "regional speech", "patois"]);
        m.insert("pronunciation", vec!["articulation", "phonetic delivery"]);
        m
    };

    static ref REAL_ESTATE_PROPERTY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("property", vec!["real estate", "premises", "realty"]);
        m.insert("house", vec!["residence", "dwelling", "home"]);
        m.insert("apartment", vec!["flat", "unit", "rental"]);
        m.insert("buyer", vec!["purchaser", "prospective homeowner"]);
        m.insert("seller", vec!["vendor", "property owner"]);
        m.insert("lease", vec!["rental agreement", "tenancy contract"]);
        m
    };

    static ref LOGISTICS_SUPPLY_CHAIN_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("shipment", vec!["consignment", "cargo delivery", "dispatch"]);
        m.insert("cargo", vec!["freight", "goods", "merchandise"]);
        m.insert("warehouse", vec!["storage facility", "distribution depot"]);
        m.insert("supplier", vec!["vendor", "distributor", "provider"]);
        m.insert("transport", vec!["haulage", "carriage", "shipping"]);
        m
    };

    static ref FITNESS_KINESIOLOGY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("exercise", vec!["workout", "physical training"]);
        m.insert("workout", vec!["training session", "exercise routine"]);
        m.insert("muscle", vec!["muscle tissue", "musculature"]);
        m.insert("strength", vec!["power", "physical force", "stamina"]);
        m.insert("trainer", vec!["fitness coach", "instructor"]);
        m.insert("gym", vec!["fitness center", "health club", "weight room"]);
        m
    };

    static ref OCCULT_ASTROLOGY_PREFERRED: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("magic", vec!["sorcery", "enchantment", "mysticism"]);
        m.insert("ritual", vec!["ceremony", "rite", "mystic practice"]);
        m.insert("spell", vec!["charm", "incantation", "hex"]);
        m.insert("spirit", vec!["astral entity", "apparition", "presence"]);
        m.insert("fortune", vec!["destiny", "fate", "future"]);
        m
    };
}

fn is_locked_any(lower: &str) -> bool {
    FINANCE_LOCKED.contains(lower)
        || BIOMEDICAL_LOCKED.contains(lower)
        || CS_LOCKED.contains(lower)
        || LEGAL_LOCKED.contains(lower)
        || CULINARY_LOCKED.contains(lower)
        || CREATIVE_ARTS_LOCKED.contains(lower)
        || SCIENCE_LOCKED.contains(lower)
        || POLITICS_LOCKED.contains(lower)
        || SPORTS_LOCKED.contains(lower)
        || EDUCATION_LOCKED.contains(lower)
        || PHILOSOPHY_PSYCHOLOGY_LOCKED.contains(lower)
        || MILITARY_LOCKED.contains(lower)
        || JOURNALISM_MEDIA_LOCKED.contains(lower)
        || TRAVEL_TOURISM_LOCKED.contains(lower)
        || AVIATION_AEROSPACE_LOCKED.contains(lower)
        || ARCHITECTURE_CONSTRUCTION_LOCKED.contains(lower)
        || GAMING_ESPORTS_LOCKED.contains(lower)
        || AGRICULTURE_BOTANY_LOCKED.contains(lower)
        || FASHION_TEXTILES_LOCKED.contains(lower)
        || THEOLOGY_RELIGION_LOCKED.contains(lower)
        || AUDIO_ENGINEERING_LOCKED.contains(lower)
        || MARITIME_NAUTICAL_LOCKED.contains(lower)
        || AUTOMOTIVE_MOTORSPORT_LOCKED.contains(lower)
        || FILM_TELEVISION_LOCKED.contains(lower)
        || LINGUISTICS_PHILOLOGY_LOCKED.contains(lower)
        || REAL_ESTATE_PROPERTY_LOCKED.contains(lower)
        || LOGISTICS_SUPPLY_CHAIN_LOCKED.contains(lower)
        || FITNESS_KINESIOLOGY_LOCKED.contains(lower)
        || OCCULT_ASTROLOGY_LOCKED.contains(lower)
}

fn check_domain(
    locked: &HashSet<&'static str>,
    preferred: &HashMap<&'static str, Vec<&'static str>>,
    lower: &str,
) -> LexicalClass {
    if locked.contains(lower) {
        LexicalClass::DomainLocked
    } else if preferred.contains_key(lower) {
        LexicalClass::DomainPreferred
    } else {
        LexicalClass::General
    }
}

/// Domain classification and lexical region controller
#[derive(Debug, Clone, Default)]
pub struct DomainClassifier;

impl DomainClassifier {
    pub fn new() -> Self {
        Self
    }

    /// Automatically infers the predominant domain of the text based on domain density
    pub fn detect_domain(&self, word_lemmas: &[String]) -> Domain {
        let mut counts: [(Domain, usize); 29] = [
            (Domain::Finance, 0),
            (Domain::Biomedical, 0),
            (Domain::ComputerScience, 0),
            (Domain::Legal, 0),
            (Domain::Culinary, 0),
            (Domain::CreativeArts, 0),
            (Domain::Science, 0),
            (Domain::Politics, 0),
            (Domain::Sports, 0),
            (Domain::Education, 0),
            (Domain::PhilosophyPsychology, 0),
            (Domain::Military, 0),
            (Domain::JournalismMedia, 0),
            (Domain::TravelTourism, 0),
            (Domain::AviationAerospace, 0),
            (Domain::ArchitectureConstruction, 0),
            (Domain::GamingEsports, 0),
            (Domain::AgricultureBotany, 0),
            (Domain::FashionTextiles, 0),
            (Domain::TheologyReligion, 0),
            (Domain::AudioEngineering, 0),
            (Domain::MaritimeNautical, 0),
            (Domain::AutomotiveMotorsport, 0),
            (Domain::FilmTelevision, 0),
            (Domain::LinguisticsPhilology, 0),
            (Domain::RealEstateProperty, 0),
            (Domain::LogisticsSupplyChain, 0),
            (Domain::FitnessKinesiology, 0),
            (Domain::OccultAstrology, 0),
        ];

        for lemma in word_lemmas {
            let lower = lemma.to_lowercase();
            let l = lower.as_str();

            if FINANCE_LOCKED.contains(l) || FINANCE_PREFERRED.contains_key(l) {
                counts[0].1 += 1;
            }
            if BIOMEDICAL_LOCKED.contains(l) || BIOMEDICAL_PREFERRED.contains_key(l) {
                counts[1].1 += 1;
            }
            if CS_LOCKED.contains(l) || CS_PREFERRED.contains_key(l) {
                counts[2].1 += 1;
            }
            if LEGAL_LOCKED.contains(l) || LEGAL_PREFERRED.contains_key(l) {
                counts[3].1 += 1;
            }
            if CULINARY_LOCKED.contains(l) || CULINARY_PREFERRED.contains_key(l) {
                counts[4].1 += 1;
            }
            if CREATIVE_ARTS_LOCKED.contains(l) || CREATIVE_ARTS_PREFERRED.contains_key(l) {
                counts[5].1 += 1;
            }
            if SCIENCE_LOCKED.contains(l) || SCIENCE_PREFERRED.contains_key(l) {
                counts[6].1 += 1;
            }
            if POLITICS_LOCKED.contains(l) || POLITICS_PREFERRED.contains_key(l) {
                counts[7].1 += 1;
            }
            if SPORTS_LOCKED.contains(l) || SPORTS_PREFERRED.contains_key(l) {
                counts[8].1 += 1;
            }
            if EDUCATION_LOCKED.contains(l) || EDUCATION_PREFERRED.contains_key(l) {
                counts[9].1 += 1;
            }
            if PHILOSOPHY_PSYCHOLOGY_LOCKED.contains(l) || PHILOSOPHY_PSYCHOLOGY_PREFERRED.contains_key(l) {
                counts[10].1 += 1;
            }
            if MILITARY_LOCKED.contains(l) || MILITARY_PREFERRED.contains_key(l) {
                counts[11].1 += 1;
            }
            if JOURNALISM_MEDIA_LOCKED.contains(l) || JOURNALISM_MEDIA_PREFERRED.contains_key(l) {
                counts[12].1 += 1;
            }
            if TRAVEL_TOURISM_LOCKED.contains(l) || TRAVEL_TOURISM_PREFERRED.contains_key(l) {
                counts[13].1 += 1;
            }
            if AVIATION_AEROSPACE_LOCKED.contains(l) || AVIATION_AEROSPACE_PREFERRED.contains_key(l) {
                counts[14].1 += 1;
            }
            if ARCHITECTURE_CONSTRUCTION_LOCKED.contains(l) || ARCHITECTURE_CONSTRUCTION_PREFERRED.contains_key(l) {
                counts[15].1 += 1;
            }
            if GAMING_ESPORTS_LOCKED.contains(l) || GAMING_ESPORTS_PREFERRED.contains_key(l) {
                counts[16].1 += 1;
            }
            if AGRICULTURE_BOTANY_LOCKED.contains(l) || AGRICULTURE_BOTANY_PREFERRED.contains_key(l) {
                counts[17].1 += 1;
            }
            if FASHION_TEXTILES_LOCKED.contains(l) || FASHION_TEXTILES_PREFERRED.contains_key(l) {
                counts[18].1 += 1;
            }
            if THEOLOGY_RELIGION_LOCKED.contains(l) || THEOLOGY_RELIGION_PREFERRED.contains_key(l) {
                counts[19].1 += 1;
            }
            if AUDIO_ENGINEERING_LOCKED.contains(l) || AUDIO_ENGINEERING_PREFERRED.contains_key(l) {
                counts[20].1 += 1;
            }
            if MARITIME_NAUTICAL_LOCKED.contains(l) || MARITIME_NAUTICAL_PREFERRED.contains_key(l) {
                counts[21].1 += 1;
            }
            if AUTOMOTIVE_MOTORSPORT_LOCKED.contains(l) || AUTOMOTIVE_MOTORSPORT_PREFERRED.contains_key(l) {
                counts[22].1 += 1;
            }
            if FILM_TELEVISION_LOCKED.contains(l) || FILM_TELEVISION_PREFERRED.contains_key(l) {
                counts[23].1 += 1;
            }
            if LINGUISTICS_PHILOLOGY_LOCKED.contains(l) || LINGUISTICS_PHILOLOGY_PREFERRED.contains_key(l) {
                counts[24].1 += 1;
            }
            if REAL_ESTATE_PROPERTY_LOCKED.contains(l) || REAL_ESTATE_PROPERTY_PREFERRED.contains_key(l) {
                counts[25].1 += 1;
            }
            if LOGISTICS_SUPPLY_CHAIN_LOCKED.contains(l) || LOGISTICS_SUPPLY_CHAIN_PREFERRED.contains_key(l) {
                counts[26].1 += 1;
            }
            if FITNESS_KINESIOLOGY_LOCKED.contains(l) || FITNESS_KINESIOLOGY_PREFERRED.contains_key(l) {
                counts[27].1 += 1;
            }
            if OCCULT_ASTROLOGY_LOCKED.contains(l) || OCCULT_ASTROLOGY_PREFERRED.contains_key(l) {
                counts[28].1 += 1;
            }
        }

        let best = counts.iter().max_by_key(|(_, c)| *c).unwrap();
        if best.1 < 2 {
            Domain::General
        } else {
            best.0
        }
    }

    /// Classifies a lemma into its lexical domain class
    pub fn classify_lexical_class(&self, lemma: &str, domain: Domain) -> LexicalClass {
        let lower = lemma.to_lowercase();
        let l = lower.as_str();

        match domain {
            Domain::Finance => check_domain(&FINANCE_LOCKED, &FINANCE_PREFERRED, l),
            Domain::Biomedical => check_domain(&BIOMEDICAL_LOCKED, &BIOMEDICAL_PREFERRED, l),
            Domain::ComputerScience => check_domain(&CS_LOCKED, &CS_PREFERRED, l),
            Domain::Legal => check_domain(&LEGAL_LOCKED, &LEGAL_PREFERRED, l),
            Domain::Culinary => check_domain(&CULINARY_LOCKED, &CULINARY_PREFERRED, l),
            Domain::CreativeArts => check_domain(&CREATIVE_ARTS_LOCKED, &CREATIVE_ARTS_PREFERRED, l),
            Domain::Science => check_domain(&SCIENCE_LOCKED, &SCIENCE_PREFERRED, l),
            Domain::Politics => check_domain(&POLITICS_LOCKED, &POLITICS_PREFERRED, l),
            Domain::Sports => check_domain(&SPORTS_LOCKED, &SPORTS_PREFERRED, l),
            Domain::Education => check_domain(&EDUCATION_LOCKED, &EDUCATION_PREFERRED, l),
            Domain::PhilosophyPsychology => check_domain(&PHILOSOPHY_PSYCHOLOGY_LOCKED, &PHILOSOPHY_PSYCHOLOGY_PREFERRED, l),
            Domain::Military => check_domain(&MILITARY_LOCKED, &MILITARY_PREFERRED, l),
            Domain::JournalismMedia => check_domain(&JOURNALISM_MEDIA_LOCKED, &JOURNALISM_MEDIA_PREFERRED, l),
            Domain::TravelTourism => check_domain(&TRAVEL_TOURISM_LOCKED, &TRAVEL_TOURISM_PREFERRED, l),
            Domain::AviationAerospace => check_domain(&AVIATION_AEROSPACE_LOCKED, &AVIATION_AEROSPACE_PREFERRED, l),
            Domain::ArchitectureConstruction => check_domain(&ARCHITECTURE_CONSTRUCTION_LOCKED, &ARCHITECTURE_CONSTRUCTION_PREFERRED, l),
            Domain::GamingEsports => check_domain(&GAMING_ESPORTS_LOCKED, &GAMING_ESPORTS_PREFERRED, l),
            Domain::AgricultureBotany => check_domain(&AGRICULTURE_BOTANY_LOCKED, &AGRICULTURE_BOTANY_PREFERRED, l),
            Domain::FashionTextiles => check_domain(&FASHION_TEXTILES_LOCKED, &FASHION_TEXTILES_PREFERRED, l),
            Domain::TheologyReligion => check_domain(&THEOLOGY_RELIGION_LOCKED, &THEOLOGY_RELIGION_PREFERRED, l),
            Domain::AudioEngineering => check_domain(&AUDIO_ENGINEERING_LOCKED, &AUDIO_ENGINEERING_PREFERRED, l),
            Domain::MaritimeNautical => check_domain(&MARITIME_NAUTICAL_LOCKED, &MARITIME_NAUTICAL_PREFERRED, l),
            Domain::AutomotiveMotorsport => check_domain(&AUTOMOTIVE_MOTORSPORT_LOCKED, &AUTOMOTIVE_MOTORSPORT_PREFERRED, l),
            Domain::FilmTelevision => check_domain(&FILM_TELEVISION_LOCKED, &FILM_TELEVISION_PREFERRED, l),
            Domain::LinguisticsPhilology => check_domain(&LINGUISTICS_PHILOLOGY_LOCKED, &LINGUISTICS_PHILOLOGY_PREFERRED, l),
            Domain::RealEstateProperty => check_domain(&REAL_ESTATE_PROPERTY_LOCKED, &REAL_ESTATE_PROPERTY_PREFERRED, l),
            Domain::LogisticsSupplyChain => check_domain(&LOGISTICS_SUPPLY_CHAIN_LOCKED, &LOGISTICS_SUPPLY_CHAIN_PREFERRED, l),
            Domain::FitnessKinesiology => check_domain(&FITNESS_KINESIOLOGY_LOCKED, &FITNESS_KINESIOLOGY_PREFERRED, l),
            Domain::OccultAstrology => check_domain(&OCCULT_ASTROLOGY_LOCKED, &OCCULT_ASTROLOGY_PREFERRED, l),
            Domain::General | Domain::Auto => {
                if is_locked_any(l) {
                    LexicalClass::DomainLocked
                } else {
                    LexicalClass::General
                }
            }
        }
    }

    /// Filters candidate lemmas according to domain register validity
    pub fn filter_candidates(
        &self,
        lemma: &str,
        _coarse_pos: CoarsePos,
        domain: Domain,
        mut synset_candidates: Vec<String>,
    ) -> Vec<String> {
        let lower = lemma.to_lowercase();
        let lex_class = self.classify_lexical_class(&lower, domain);

        match lex_class {
            LexicalClass::DomainLocked => {
                // Locked domain terms must NOT be perturbed
                Vec::new()
            }
            LexicalClass::DomainPreferred => {
                // If a domain-preferred list exists, prioritize and restrict to domain-valid equivalents
                let preferred_list = match domain {
                    Domain::Finance => FINANCE_PREFERRED.get(lower.as_str()),
                    Domain::Biomedical => BIOMEDICAL_PREFERRED.get(lower.as_str()),
                    Domain::ComputerScience => CS_PREFERRED.get(lower.as_str()),
                    Domain::Legal => LEGAL_PREFERRED.get(lower.as_str()),
                    Domain::Culinary => CULINARY_PREFERRED.get(lower.as_str()),
                    Domain::CreativeArts => CREATIVE_ARTS_PREFERRED.get(lower.as_str()),
                    Domain::Science => SCIENCE_PREFERRED.get(lower.as_str()),
                    Domain::Politics => POLITICS_PREFERRED.get(lower.as_str()),
                    Domain::Sports => SPORTS_PREFERRED.get(lower.as_str()),
                    Domain::Education => EDUCATION_PREFERRED.get(lower.as_str()),
                    Domain::PhilosophyPsychology => PHILOSOPHY_PSYCHOLOGY_PREFERRED.get(lower.as_str()),
                    Domain::Military => MILITARY_PREFERRED.get(lower.as_str()),
                    Domain::JournalismMedia => JOURNALISM_MEDIA_PREFERRED.get(lower.as_str()),
                    Domain::TravelTourism => TRAVEL_TOURISM_PREFERRED.get(lower.as_str()),
                    Domain::AviationAerospace => AVIATION_AEROSPACE_PREFERRED.get(lower.as_str()),
                    Domain::ArchitectureConstruction => ARCHITECTURE_CONSTRUCTION_PREFERRED.get(lower.as_str()),
                    Domain::GamingEsports => GAMING_ESPORTS_PREFERRED.get(lower.as_str()),
                    Domain::AgricultureBotany => AGRICULTURE_BOTANY_PREFERRED.get(lower.as_str()),
                    Domain::FashionTextiles => FASHION_TEXTILES_PREFERRED.get(lower.as_str()),
                    Domain::TheologyReligion => THEOLOGY_RELIGION_PREFERRED.get(lower.as_str()),
                    Domain::AudioEngineering => AUDIO_ENGINEERING_PREFERRED.get(lower.as_str()),
                    Domain::MaritimeNautical => MARITIME_NAUTICAL_PREFERRED.get(lower.as_str()),
                    Domain::AutomotiveMotorsport => AUTOMOTIVE_MOTORSPORT_PREFERRED.get(lower.as_str()),
                    Domain::FilmTelevision => FILM_TELEVISION_PREFERRED.get(lower.as_str()),
                    Domain::LinguisticsPhilology => LINGUISTICS_PHILOLOGY_PREFERRED.get(lower.as_str()),
                    Domain::RealEstateProperty => REAL_ESTATE_PROPERTY_PREFERRED.get(lower.as_str()),
                    Domain::LogisticsSupplyChain => LOGISTICS_SUPPLY_CHAIN_PREFERRED.get(lower.as_str()),
                    Domain::FitnessKinesiology => FITNESS_KINESIOLOGY_PREFERRED.get(lower.as_str()),
                    Domain::OccultAstrology => OCCULT_ASTROLOGY_PREFERRED.get(lower.as_str()),
                    _ => None,
                };

                if let Some(pref) = preferred_list {
                    let mut domain_cands = Vec::new();
                    for &p in pref {
                        if !p.eq_ignore_ascii_case(&lower) {
                            domain_cands.push(p.to_string());
                        }
                    }
                    domain_cands
                } else {
                    synset_candidates
                }
            }
            LexicalClass::General => {
                // Filter out candidates that leak into locked domain terms improperly
                synset_candidates.retain(|c| {
                    let c_lower = c.to_lowercase();
                    !c_lower.eq_ignore_ascii_case(&lower)
                });
                synset_candidates
            }
            LexicalClass::TechnicalTerm | LexicalClass::ProperEntity => Vec::new(),
        }
    }

    /// Computes the Effective Replacement Entropy H_eff for a document given its domain search space:
    /// H_eff = (1 / |V_content|) * sum_{w} log2(1 + |C_domain_valid(w)|)
    pub fn compute_effective_entropy(
        &self,
        content_word_lemmas: &[String],
        _domain: Domain,
        candidates_per_word: &[usize],
    ) -> f64 {
        if content_word_lemmas.is_empty() {
            return 0.0;
        }

        let mut sum_entropy = 0.0f64;
        for &c_count in candidates_per_word {
            sum_entropy += (1.0 + c_count as f64).log2();
        }

        sum_entropy / (content_word_lemmas.len() as f64)
    }
}
