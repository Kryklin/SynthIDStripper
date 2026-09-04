use crate::types::CoarsePos;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Synset {
    pub id: &'static str,
    pub pos: CoarsePos,
    pub lemmas: &'static [&'static str],
    pub gloss: &'static str,
    pub antonyms: &'static [&'static str],
}

lazy_static::lazy_static! {
    /// Comprehensive curated WordNet-style synset database with high-precision sense disambiguation
    pub static ref SYNSET_DATABASE: Vec<Synset> = {
        vec![
            // ==========================================
            // --- VERBS ---
            // ==========================================
            Synset {
                id: "verb.transform.01",
                pos: CoarsePos::Verb,
                lemmas: &["transform", "alter", "modify", "change", "convert", "transmute", "reshape", "remodel"],
                gloss: "change or alter the form nature character structure or appearance of something",
                antonyms: &["preserve", "maintain", "keep", "retain"],
            },
            Synset {
                id: "verb.preserve.01",
                pos: CoarsePos::Verb,
                lemmas: &["preserve", "maintain", "keep", "retain", "uphold", "conserve", "sustain", "protect", "safeguard"],
                gloss: "keep in safety protect from decay harm or change maintain historic structure intact",
                antonyms: &["destroy", "ruin", "change", "alter", "modify", "eliminate"],
            },
            Synset {
                id: "verb.create.01",
                pos: CoarsePos::Verb,
                lemmas: &["create", "produce", "generate", "build", "craft", "form", "construct", "develop", "originate", "forge"],
                gloss: "bring something into existence produce through creative work or effort",
                antonyms: &["destroy", "demolish", "ruin", "eliminate", "dismantle"],
            },
            Synset {
                id: "verb.show.01",
                pos: CoarsePos::Verb,
                lemmas: &["show", "display", "exhibit", "demonstrate", "reveal", "present", "showcase", "illustrate", "manifest", "indicate"],
                gloss: "make visible or evident display demonstrate present exhibit to view",
                antonyms: &["hide", "conceal", "obscure", "cover", "mask"],
            },
            Synset {
                id: "verb.indicate.01",
                pos: CoarsePos::Verb,
                lemmas: &["indicate", "suggest", "signify", "denote", "point", "imply", "reflect", "convey"],
                gloss: "serve as a sign signal or symptom indicate point to suggest",
                antonyms: &["contradict", "disprove", "refute"],
            },
            Synset {
                id: "verb.encourage.prompt",
                pos: CoarsePos::Verb,
                lemmas: &["encourage", "prompt", "invite", "inspire", "spur", "stimulate", "welcome"],
                gloss: "persuade or stimulate someone visitors tourists passengers people to do an action explore visit",
                antonyms: &["discourage", "dissuade", "deter", "prevent"],
            },
            Synset {
                id: "verb.motivate.drive",
                pos: CoarsePos::Verb,
                lemmas: &["motivate", "drive", "incentivize", "inspire", "impel"],
                gloss: "provide a motive incentive or inner drive to act with determination and intent",
                antonyms: &["discourage", "dissuade", "deter"],
            },
            Synset {
                id: "verb.encourage.support",
                pos: CoarsePos::Verb,
                lemmas: &["encourage", "foster", "support", "bolster", "promote", "champion", "further"],
                gloss: "help stimulate or advance the growth development or success of an activity or effort",
                antonyms: &["hinder", "impede", "stifle", "suppress"],
            },
            Synset {
                id: "verb.help.01",
                pos: CoarsePos::Verb,
                lemmas: &["help", "assist", "aid", "support", "facilitate", "foster", "bolster", "further"],
                gloss: "give assistance aid support or help to someone facilitate action",
                antonyms: &["hinder", "obstruct", "impede", "prevent", "harm", "thwart"],
            },
            Synset {
                id: "verb.start.01",
                pos: CoarsePos::Verb,
                lemmas: &["start", "begin", "commence", "initiate", "launch", "embark", "originate", "inaugurate"],
                gloss: "take the first step or begin doing an action start project process",
                antonyms: &["end", "finish", "terminate", "stop", "conclude", "cease"],
            },
            Synset {
                id: "verb.stop.01",
                pos: CoarsePos::Verb,
                lemmas: &["stop", "halt", "cease", "terminate", "end", "discontinue", "pause", "conclude"],
                gloss: "come to an end pause cease motion or activity bring to completion",
                antonyms: &["start", "begin", "continue", "resume", "proceed", "initiate"],
            },
            Synset {
                id: "verb.explore.travel",
                pos: CoarsePos::Verb,
                lemmas: &["explore", "tour", "discover", "visit", "roam", "survey", "traverse"],
                gloss: "travel through an unfamiliar area countryside town landscape region to see enjoy explore",
                antonyms: &[],
            },
            Synset {
                id: "verb.explore.investigate",
                pos: CoarsePos::Verb,
                lemmas: &["explore", "investigate", "examine", "scrutinize", "delve", "analyze", "study", "probe", "evaluate"],
                gloss: "observe carefully study investigate look into inspect analyze in detail problem issue question",
                antonyms: &["ignore", "neglect", "overlook", "dismiss"],
            },
            Synset {
                id: "verb.improve.01",
                pos: CoarsePos::Verb,
                lemmas: &["improve", "enhance", "upgrade", "refine", "elevate", "boost", "augment", "enrich", "polish"],
                gloss: "make or become better improve quality condition value status facilities station",
                antonyms: &["worsen", "deteriorate", "degrade", "damage", "impair"],
            },
            Synset {
                id: "verb.reduce.01",
                pos: CoarsePos::Verb,
                lemmas: &["reduce", "decrease", "lessen", "diminish", "minimize", "lower", "curtail", "abate"],
                gloss: "make smaller or less in amount degree size intensity reduction",
                antonyms: &["increase", "augment", "expand", "boost", "maximize", "escalate"],
            },
            Synset {
                id: "verb.increase.01",
                pos: CoarsePos::Verb,
                lemmas: &["increase", "expand", "grow", "enlarge", "boost", "escalate", "augment", "heighten", "amplify"],
                gloss: "become or make greater in size amount degree intensity increase growth",
                antonyms: &["decrease", "reduce", "diminish", "lessen", "lower", "curtail"],
            },
            Synset {
                id: "verb.require.01",
                pos: CoarsePos::Verb,
                lemmas: &["require", "demand", "necessitate", "need", "entail", "warrant", "call"],
                gloss: "have need of require demand necessitate as condition or requirement",
                antonyms: &["obviate", "forgo", "waive"],
            },
            Synset {
                id: "verb.find.01",
                pos: CoarsePos::Verb,
                lemmas: &["find", "discover", "locate", "uncover", "identify", "detect", "discern", "observe"],
                gloss: "come upon discover locate find through search investigation observation",
                antonyms: &["lose", "misplace", "hide", "miss"],
            },
            Synset {
                id: "verb.select.01",
                pos: CoarsePos::Verb,
                lemmas: &["select", "choose", "pick", "opt"],
                gloss: "pick out select choose from a number of alternatives options",
                antonyms: &["reject", "discard", "dismiss", "decline"],
            },
            Synset {
                id: "verb.use.01",
                pos: CoarsePos::Verb,
                lemmas: &["use", "utilize", "employ", "apply", "leverage", "harness", "exercise", "deploy"],
                gloss: "put into service use utilize employ for a purpose apply method tool",
                antonyms: &["discard", "ignore", "neglect", "forgo"],
            },
            Synset {
                id: "verb.operate.01",
                pos: CoarsePos::Verb,
                lemmas: &["operate", "function", "work", "perform", "run", "act"],
                gloss: "perform a function work operate properly machinery system mechanism",
                antonyms: &["fail", "break", "malfunction", "stall"],
            },
            Synset {
                id: "verb.emphasize.01",
                pos: CoarsePos::Verb,
                lemmas: &["emphasize", "highlight", "stress", "underscore", "accentuate", "feature", "illuminate", "accent"],
                gloss: "give special importance or prominence emphasize stress underscore in speech writing",
                antonyms: &["downplay", "minimize", "ignore", "understate", "obscure"],
            },
            Synset {
                id: "verb.give.01",
                pos: CoarsePos::Verb,
                lemmas: &["give", "provide", "supply", "furnish", "grant", "offer", "deliver", "present", "yield"],
                gloss: "transfer possession of give provide supply grant offer to recipient",
                antonyms: &["take", "receive", "withhold", "deny", "refuse"],
            },
            Synset {
                id: "verb.reach.arrive",
                pos: CoarsePos::Verb,
                lemmas: &["reach", "arrive", "get"],
                gloss: "arrive at or reach a point stage state condition or destination",
                antonyms: &["depart", "leave"],
            },
            Synset {
                id: "verb.achieve.01",
                pos: CoarsePos::Verb,
                lemmas: &["achieve", "accomplish", "attain", "fulfill", "execute", "complete"],
                gloss: "successfully bring about achieve goal accomplish task attain objective",
                antonyms: &["fail", "abandon", "forfeit", "miss", "abort"],
            },
            Synset {
                id: "verb.explain.01",
                pos: CoarsePos::Verb,
                lemmas: &["explain", "clarify", "elucidate", "describe", "explicate", "interpret", "detail"],
                gloss: "make clear or plain explain interpret describe elucidate reasons facts",
                antonyms: &["confuse", "obscure", "complicate", "befuddle"],
            },
            Synset {
                id: "verb.understand.01",
                pos: CoarsePos::Verb,
                lemmas: &["understand", "comprehend", "grasp", "fathom", "perceive", "apprehend", "discern", "recognize", "realize"],
                gloss: "know and understand the meaning nature of grasp concept comprehend idea realize fact",
                antonyms: &["misunderstand", "misinterpret", "confuse"],
            },
            Synset {
                id: "verb.protect.01",
                pos: CoarsePos::Verb,
                lemmas: &["protect", "shield", "safeguard", "defend", "guard", "shelter", "preserve"],
                gloss: "keep safe from harm injury loss or damage shield protect safeguard preserve structure",
                antonyms: &["expose", "attack", "endanger", "harm", "damage"],
            },
            Synset {
                id: "verb.complain.01",
                pos: CoarsePos::Verb,
                lemmas: &["complain", "grumble", "object", "lament", "gripe"],
                gloss: "express dissatisfaction grievance annoyance or frustration timetable delays",
                antonyms: &["praise", "applaud"],
            },
            Synset {
                id: "verb.move.motion",
                pos: CoarsePos::Verb,
                lemmas: &["move", "drift", "float", "pass", "shift", "glide"],
                gloss: "travel through the air or across landscape drift clouds move slowly across hills",
                antonyms: &["stay", "remain"],
            },

            // ==========================================
            // --- NOUNS ---
            // ==========================================
            Synset {
                id: "noun.building.structure",
                pos: CoarsePos::Noun,
                lemmas: &["building", "structure", "edifice"],
                gloss: "a permanent structure with a roof and walls building edifice station house",
                antonyms: &[],
            },
            Synset {
                id: "noun.group.people",
                pos: CoarsePos::Noun,
                lemmas: &["group", "gathering", "cluster", "band", "circle", "party", "contingent", "assemblage", "cohort"],
                gloss: "a number of people residents citizens gathered or acting together group gathering cluster band",
                antonyms: &[],
            },
            Synset {
                id: "noun.facility.amenity",
                pos: CoarsePos::Noun,
                lemmas: &["facility", "amenity", "service", "convenience", "installation", "provision"],
                gloss: "amenities conveniences services or equipment provided for passengers visitors public",
                antonyms: &[],
            },
            Synset {
                id: "noun.objective.01",
                pos: CoarsePos::Noun,
                lemmas: &["objective", "goal", "target", "aim", "purpose", "intent", "end"],
                gloss: "the purpose or goal that one seeks to achieve target outcome destination",
                antonyms: &[],
            },
            Synset {
                id: "noun.task.01",
                pos: CoarsePos::Noun,
                lemmas: &["task", "job", "assignment", "duty", "undertaking", "endeavor", "mission", "work"],
                gloss: "a piece of work to be done assignment task job duty undertaking",
                antonyms: &[],
            },
            Synset {
                id: "noun.method.01",
                pos: CoarsePos::Noun,
                lemmas: &["method", "technique", "approach", "procedure", "strategy", "mechanism", "manner", "process"],
                gloss: "a way of doing something procedure method technique strategy process approach",
                antonyms: &[],
            },
            Synset {
                id: "noun.result.01",
                pos: CoarsePos::Noun,
                lemmas: &["result", "outcome", "consequence", "effect", "finding", "product"],
                gloss: "something that happens as a consequence result outcome effect product",
                antonyms: &["cause", "origin", "source"],
            },
            Synset {
                id: "noun.output.data",
                pos: CoarsePos::Noun,
                lemmas: &["output", "result", "yield", "export"],
                gloss: "data information or content produced or emitted by a computer model program or pipeline raw output",
                antonyms: &["input"],
            },
            Synset {
                id: "noun.problem.01",
                pos: CoarsePos::Noun,
                lemmas: &["problem", "issue", "challenge", "difficulty", "obstacle", "dilemma", "complication"],
                gloss: "a matter or situation regarded as unwelcome problem issue difficulty challenge",
                antonyms: &["solution", "remedy", "advantage", "answer"],
            },
            Synset {
                id: "noun.solution.01",
                pos: CoarsePos::Noun,
                lemmas: &["solution", "answer", "resolution", "remedy"],
                gloss: "a means of solving a problem or dealing with a difficult situation answer remedy",
                antonyms: &["problem", "complication", "issue", "obstacle"],
            },
            Synset {
                id: "noun.requirement.01",
                pos: CoarsePos::Noun,
                lemmas: &["requirement", "condition", "prerequisite", "necessity", "criterion", "specification", "demand"],
                gloss: "a thing that is needed or wanted condition prerequisite requirement necessity",
                antonyms: &[],
            },
            Synset {
                id: "noun.area.01",
                pos: CoarsePos::Noun,
                lemmas: &["area", "field", "domain", "realm", "sphere", "discipline", "sector", "arena"],
                gloss: "a subject area field of study domain realm activity sphere",
                antonyms: &[],
            },
            Synset {
                id: "noun.element.01",
                pos: CoarsePos::Noun,
                lemmas: &["element", "component", "part", "aspect", "facet", "factor", "feature"],
                gloss: "an essential part component aspect element factor feature of something complex",
                antonyms: &["whole", "aggregate", "totality"],
            },
            Synset {
                id: "noun.system.01",
                pos: CoarsePos::Noun,
                lemmas: &["system", "framework", "structure", "engine", "scheme", "architecture", "setup"],
                gloss: "a set of connected things or parts forming a complex whole system framework engine",
                antonyms: &[],
            },
            Synset {
                id: "noun.option.01",
                pos: CoarsePos::Noun,
                lemmas: &["option", "choice", "alternative", "possibility", "selection", "preference"],
                gloss: "one of a number of things from which only one can be chosen option choice alternative",
                antonyms: &[],
            },
            Synset {
                id: "noun.report.01",
                pos: CoarsePos::Noun,
                lemmas: &["report", "summary", "account", "record", "statement", "overview", "log"],
                gloss: "a written or spoken description of a situation or event report summary record",
                antonyms: &[],
            },
            Synset {
                id: "noun.category.01",
                pos: CoarsePos::Noun,
                lemmas: &["category", "class", "group", "type", "kind", "classification", "division"],
                gloss: "a classification taxonomy of things category class type",
                antonyms: &[],
            },
            Synset {
                id: "noun.candidate.01",
                pos: CoarsePos::Noun,
                lemmas: &["candidate", "applicant", "contender", "nominee", "prospect", "entry"],
                gloss: "a person or thing regarded as suitable or likely for a purpose candidate contender",
                antonyms: &[],
            },
            Synset {
                id: "noun.principle.01",
                pos: CoarsePos::Noun,
                lemmas: &["principle", "rule", "standard", "guideline", "tenet", "doctrine", "axiom"],
                gloss: "a fundamental truth or proposition principle rule standard guideline",
                antonyms: &[],
            },
            Synset {
                id: "noun.meaning.01",
                pos: CoarsePos::Noun,
                lemmas: &["meaning", "sense", "significance", "connotation", "import", "definition", "substance"],
                gloss: "what is meant by a word text concept or action meaning sense significance",
                antonyms: &[],
            },
            Synset {
                id: "noun.change.01",
                pos: CoarsePos::Noun,
                lemmas: &["change", "alteration", "modification", "transformation", "shift", "transition", "evolution"],
                gloss: "the act or process of becoming different gradual change alteration shift decades",
                antonyms: &["constancy", "stability", "permanence"],
            },

            // ==========================================
            // --- ADJECTIVES ---
            // ==========================================
            Synset {
                id: "adj.important.01",
                pos: CoarsePos::Adjective,
                lemmas: &["important", "significant", "vital", "essential", "key", "crucial", "meaningful"],
                gloss: "of great significance value or importance significant crucial vital essential key railway network",
                antonyms: &["unimportant", "insignificant", "trivial", "minor", "negligible"],
            },
            Synset {
                id: "adj.warm.sensation",
                pos: CoarsePos::Adjective,
                lemmas: &["warm", "rich", "comforting", "fragrant", "mellow"],
                gloss: "giving a pleasant feeling of heat comforting aroma or fragrance warm rich smell",
                antonyms: &["cold", "chilly"],
            },
            Synset {
                id: "adj.fresh.beverage",
                pos: CoarsePos::Adjective,
                lemmas: &["fresh", "freshly brewed", "freshly made"],
                gloss: "recently made prepared or brewed fresh coffee beverage bread",
                antonyms: &["stale"],
            },
            Synset {
                id: "adj.hot.temperature",
                pos: CoarsePos::Adjective,
                lemmas: &["hot", "scorching", "scalding", "sizzling", "warm"],
                gloss: "having a high temperature or intense thermal heat hot pan stove liquid",
                antonyms: &["cold", "freezing", "cool"],
            },
            Synset {
                id: "adj.appropriate.01",
                pos: CoarsePos::Adjective,
                lemmas: &["appropriate", "suitable", "fitting", "proper", "apt", "correct", "applicable", "relevant"],
                gloss: "suitable or proper in the circumstances fitting appropriate apt relevant",
                antonyms: &["inappropriate", "unsuitable", "improper", "unfit"],
            },
            Synset {
                id: "adj.large.physical",
                pos: CoarsePos::Adjective,
                lemmas: &["large", "big", "massive", "spacious", "broad", "sizable", "extensive"],
                gloss: "of great physical dimensions size or capacity large big massive spacious windows canopy room building town",
                antonyms: &["small", "little", "tiny", "minor", "slight"],
            },
            Synset {
                id: "adj.large.magnitude",
                pos: CoarsePos::Adjective,
                lemmas: &["considerable", "substantial", "great", "significant", "widespread", "broad", "marked"],
                gloss: "of great amount extent degree importance or backing considerable substantial support interest assistance backing",
                antonyms: &["small", "little", "insignificant", "minor", "slight"],
            },
            Synset {
                id: "adj.small.01",
                pos: CoarsePos::Adjective,
                lemmas: &["small", "little", "tiny", "modest", "slight", "compact"],
                gloss: "of limited size dimension quantity or degree small little modest town group",
                antonyms: &["large", "big", "huge", "massive", "great"],
            },
            Synset {
                id: "adj.fast.01",
                pos: CoarsePos::Adjective,
                lemmas: &["fast", "quick", "rapid", "swift", "speedy", "prompt", "brisk"],
                gloss: "moving or capable of moving at high speed fast rapid quick swift",
                antonyms: &["slow", "sluggish", "gradual", "delayed"],
            },
            Synset {
                id: "noun.timetable.01",
                pos: CoarsePos::Noun,
                lemmas: &["timetable", "schedule", "service times", "running times"],
                gloss: "a schedule showing the times at which trains depart timetable schedule",
                antonyms: &[],
            },
            Synset {
                id: "adj.unreliable.01",
                pos: CoarsePos::Adjective,
                lemmas: &["unreliable", "erratic", "unpredictable", "inconsistent", "irregular", "patchy"],
                gloss: "not able to be relied upon erratic unpredictable unreliable schedule timetable train",
                antonyms: &["reliable", "dependable", "consistent"],
            },
            Synset {
                id: "adj.dark.clouds",
                pos: CoarsePos::Adjective,
                lemmas: &["dark", "heavy", "gloomy", "somber", "shadowy", "thick", "overcast"],
                gloss: "characterized by darkness thick overcast clouds heavy sky rainy afternoon",
                antonyms: &["bright", "clear", "sunny"],
            },
            Synset {
                id: "adj.distant.01",
                pos: CoarsePos::Adjective,
                lemmas: &["distant", "far-off", "remote", "faraway", "outlying"],
                gloss: "far away in space remote far-off distant hills landscape horizon",
                antonyms: &["near", "close", "nearby"],
            },
            Synset {
                id: "adj.slow.01",
                pos: CoarsePos::Adjective,
                lemmas: &["slow", "gradual", "unhurried", "leisurely", "sluggish"],
                gloss: "moving or operating at a low speed not fast slow gradual change unhurried",
                antonyms: &["fast", "quick", "rapid", "swift"],
            },
            Synset {
                id: "adj.good.01",
                pos: CoarsePos::Adjective,
                lemmas: &["good", "excellent", "fine", "superior", "sound", "positive", "admirable", "sterling"],
                gloss: "having the qualities required for a role or purpose good excellent fine superior",
                antonyms: &["bad", "poor", "inferior", "flawed", "substandard"],
            },
            Synset {
                id: "adj.bad.01",
                pos: CoarsePos::Adjective,
                lemmas: &["bad", "poor", "inferior", "substandard", "flawed", "defective", "unfavorable"],
                gloss: "of poor quality not good substandard flawed inferior defective",
                antonyms: &["good", "excellent", "fine", "superior"],
            },
            Synset {
                id: "adj.clear.01",
                pos: CoarsePos::Adjective,
                lemmas: &["clear", "plain", "evident", "obvious", "apparent", "lucid", "distinct", "transparent"],
                gloss: "easy to perceive understand or interpret clear plain obvious distinct lucid",
                antonyms: &["unclear", "obscure", "ambiguous", "vague", "opaque"],
            },
            Synset {
                id: "adj.complex.01",
                pos: CoarsePos::Adjective,
                lemmas: &["complex", "complicated", "intricate", "sophisticated", "elaborate", "multifaceted", "involved"],
                gloss: "consisting of many different and connected parts complex complicated intricate",
                antonyms: &["simple", "basic", "straightforward", "elementary", "plain"],
            },
            Synset {
                id: "adj.simple.01",
                pos: CoarsePos::Adjective,
                lemmas: &["simple", "straightforward", "basic", "plain", "elementary", "uncomplicated", "modest"],
                gloss: "easily understood or done presenting few difficulties simple straightforward basic",
                antonyms: &["complex", "complicated", "intricate", "sophisticated"],
            },
            Synset {
                id: "adj.diverse.01",
                pos: CoarsePos::Adjective,
                lemmas: &["diverse", "varied", "various", "multiple", "multifaceted", "heterogeneous", "assorted"],
                gloss: "showing a great deal of variety very different diverse varied various",
                antonyms: &["uniform", "identical", "homogeneous", "similar"],
            },
            Synset {
                id: "adj.accurate.01",
                pos: CoarsePos::Adjective,
                lemmas: &["accurate", "precise", "exact", "correct", "meticulous", "rigorous", "faithful"],
                gloss: "correct in all details exact accurate precise rigorous meticulous",
                antonyms: &["inaccurate", "imprecise", "incorrect", "wrong", "flawed"],
            },
            Synset {
                id: "adj.comprehensive.01",
                pos: CoarsePos::Adjective,
                lemmas: &["comprehensive", "thorough", "complete", "exhaustive", "extensive", "detailed", "all-inclusive"],
                gloss: "complete and including everything that is necessary comprehensive thorough detailed",
                antonyms: &["incomplete", "partial", "cursory", "sketchy", "limited"],
            },
            Synset {
                id: "adj.strong.01",
                pos: CoarsePos::Adjective,
                lemmas: &["strong", "robust", "powerful", "resilient", "solid", "sturdy", "potent", "durable"],
                gloss: "having the power to withstand force or stress robust strong resilient solid",
                antonyms: &["weak", "fragile", "feeble", "flimsy", "vulnerable"],
            },
            Synset {
                id: "adj.primary.01",
                pos: CoarsePos::Adjective,
                lemmas: &["primary", "fundamental", "principal", "prime", "chief", "predominant"],
                gloss: "of chief importance principal primary prime fundamental",
                antonyms: &["secondary", "auxiliary", "minor"],
            },

            // ==========================================
            // --- ADVERBS ---
            // ==========================================
            Synset {
                id: "adv.simply.merely",
                pos: CoarsePos::Adverb,
                lemmas: &["simply", "merely", "just", "only", "purely"],
                gloss: "and nothing more merely just only purely watch clouds look simply",
                antonyms: &[],
            },
            Synset {
                id: "adv.simply.easily",
                pos: CoarsePos::Adverb,
                lemmas: &["easily", "readily", "effortlessly", "plainly", "smoothly"],
                gloss: "without difficulty or effort easily smoothly effortlessly readily",
                antonyms: &["hardly", "difficultly", "laboriously"],
            },
            Synset {
                id: "adv.slowly.pace",
                pos: CoarsePos::Adverb,
                lemmas: &["slowly", "gradually", "unhurriedly", "sluggishly", "leisurely"],
                gloss: "at a slow speed or pace not fast move slowly across hills clouds",
                antonyms: &["quickly", "rapidly", "swiftly", "promptly"],
            },
            Synset {
                id: "adv.quickly.01",
                pos: CoarsePos::Adverb,
                lemmas: &["quickly", "rapidly", "swiftly", "speedily", "promptly", "hastily"],
                gloss: "at a fast speed rapidly quickly swiftly promptly",
                antonyms: &["slowly", "gradually", "sluggishly"],
            },
            Synset {
                id: "adv.carefully.01",
                pos: CoarsePos::Adverb,
                lemmas: &["carefully", "meticulously", "thoroughly", "diligently", "attentively", "cautiously", "scrupulously"],
                gloss: "in a way that deliberately avoids harm errors or omissions carefully meticulously",
                antonyms: &["carelessly", "recklessly", "hastily", "negligently"],
            },
            Synset {
                id: "adv.completely.01",
                pos: CoarsePos::Adverb,
                lemmas: &["completely", "entirely", "fully", "totally", "thoroughly", "wholly", "absolutely"],
                gloss: "totally and completely in every way entirely fully thoroughly",
                antonyms: &["partially", "partly", "incompletely"],
            },
            Synset {
                id: "adv.naturally.01",
                pos: CoarsePos::Adverb,
                lemmas: &["naturally", "inherently", "genuinely", "organically", "spontaneously"],
                gloss: "as might be expected in a natural manner inherently organically",
                antonyms: &["artificially", "unauthentically"],
            },
            Synset {
                id: "adv.practically.01",
                pos: CoarsePos::Adverb,
                lemmas: &["practically", "virtually", "effectively", "essentially", "substantially", "almost"],
                gloss: "almost or nearly virtually in a practical manner effectively",
                antonyms: &[],
            },
            Synset {
                id: "adv.strictly.01",
                pos: CoarsePos::Adverb,
                lemmas: &["strictly", "rigorously", "stringently", "firmly", "rigidly"],
                gloss: "in a strict manner rigorously stringently with exact compliance",
                antonyms: &["loosely", "leniently", "flexibly"],
            },
            Synset {
                id: "adv.often.01",
                pos: CoarsePos::Adverb,
                lemmas: &["often", "frequently", "regularly", "repeatedly", "commonly"],
                gloss: "many times at short intervals regularly frequently often gather passengers",
                antonyms: &["rarely", "seldom", "infrequently"],
            },
            Synset {
                id: "adv.significantly.01",
                pos: CoarsePos::Adverb,
                lemmas: &["significantly", "substantially", "considerably", "notably", "markedly"],
                gloss: "in a sufficiently great or important way as to be worthy of attention significantly",
                antonyms: &["insignificantly", "trivially", "slightly"],
            },

            // ==========================================
            // --- ADDITIONAL CORE VERBS ---
            // ==========================================
            Synset {
                id: "verb.bark.vocalize",
                pos: CoarsePos::Verb,
                lemmas: &["bark", "vocalize", "bay", "howl", "yelp"],
                gloss: "make a sharp explosive cry or sound dogs barking vocalize",
                antonyms: &["remain silent", "stay quiet"],
            },
            Synset {
                id: "verb.scream.01",
                pos: CoarsePos::Verb,
                lemmas: &["scream", "shout", "shriek", "cry", "yell", "bellow"],
                gloss: "utter a loud sharp piercing cry voice scream shout yell",
                antonyms: &["whisper", "murmur"],
            },
            Synset {
                id: "verb.communicate.01",
                pos: CoarsePos::Verb,
                lemmas: &["communicate", "interact", "convey", "express", "transmit", "impart"],
                gloss: "share or exchange information ideas feelings communicate interact",
                antonyms: &["withhold", "conceal"],
            },
            Synset {
                id: "verb.express.01",
                pos: CoarsePos::Verb,
                lemmas: &["express", "convey", "articulate", "voice", "manifest", "communicate"],
                gloss: "convey a thought emotion idea feeling or frustration in words or actions",
                antonyms: &["suppress", "repress", "hide"],
            },
            Synset {
                id: "verb.alert.01",
                pos: CoarsePos::Verb,
                lemmas: &["alert", "warn", "notify", "inform", "signal", "apprise"],
                gloss: "warn to be prepared or inform of danger or situation alert owners",
                antonyms: &["deceive", "mislead"],
            },
            Synset {
                id: "verb.handle.01",
                pos: CoarsePos::Verb,
                lemmas: &["handle", "manage", "endure", "withstand", "sustain", "bear", "cope"],
                gloss: "manage deal with or endure strain physical stress handle load",
                antonyms: &["surrender", "collapse", "fail"],
            },
            Synset {
                id: "verb.damage.01",
                pos: CoarsePos::Verb,
                lemmas: &["damage", "harm", "injure", "impair", "hurt", "mar"],
                gloss: "inflict physical harm or impairment on cords tissue body damage vocal cords",
                antonyms: &["heal", "repair", "protect", "remedy"],
            },
            Synset {
                id: "verb.tire.01",
                pos: CoarsePos::Verb,
                lemmas: &["tire", "fatigue", "exhaust", "weary", "drain"],
                gloss: "lose energy or strength become exhausted weary tired from effort",
                antonyms: &["energize", "reinvigorate", "refresh"],
            },
            Synset {
                id: "verb.rest.01",
                pos: CoarsePos::Verb,
                lemmas: &["rest", "relax", "recover", "recuperate", "pause"],
                gloss: "cease work or movement in order to relax recover sleep recuperate",
                antonyms: &["exert", "labor", "work", "strain"],
            },
            Synset {
                id: "verb.use.01",
                pos: CoarsePos::Verb,
                lemmas: &["use", "utilize", "consume", "expend", "employ", "apply"],
                gloss: "take hold of deploy or expend energy resources time for an action",
                antonyms: &["conserve", "save"],
            },
            Synset {
                id: "verb.produce.01",
                pos: CoarsePos::Verb,
                lemmas: &["produce", "generate", "yield", "create", "manufacture", "furnish"],
                gloss: "bring forth or generate results output energy goods produce",
                antonyms: &["destroy", "consume"],
            },
            Synset {
                id: "verb.observe.01",
                pos: CoarsePos::Verb,
                lemmas: &["observe", "monitor", "watch", "track", "witness", "examine", "survey"],
                gloss: "watch carefully notice or record phenomenon behavior patients subjects",
                antonyms: &["overlook", "ignore", "neglect"],
            },
            Synset {
                id: "verb.evaluate.01",
                pos: CoarsePos::Verb,
                lemmas: &["evaluate", "assess", "appraise", "gauge", "rate", "estimate", "judge"],
                gloss: "form an idea of the amount number quality value or efficacy evaluate assess",
                antonyms: &["guess", "disregard"],
            },
            Synset {
                id: "verb.discover.01",
                pos: CoarsePos::Verb,
                lemmas: &["discover", "uncover", "identify", "find", "detect", "reveal", "ascertain"],
                gloss: "find unexpectedly or during a search uncover discover new insights findings",
                antonyms: &["lose", "miss", "overlook"],
            },
            Synset {
                id: "verb.improve.01",
                pos: CoarsePos::Verb,
                lemmas: &["improve", "enhance", "upgrade", "refine", "elevate", "boost", "advance"],
                gloss: "make or become better improve quality condition value status facilities",
                antonyms: &["worsen", "deteriorate", "degrade", "damage"],
            },
            Synset {
                id: "verb.reduce.01",
                pos: CoarsePos::Verb,
                lemmas: &["reduce", "decrease", "lessen", "diminish", "curtail", "lower", "attenuate"],
                gloss: "make smaller or less in amount degree or size reduce risk symptoms",
                antonyms: &["increase", "augment", "expand", "boost", "elevate"],
            },
            Synset {
                id: "verb.increase.01",
                pos: CoarsePos::Verb,
                lemmas: &["increase", "expand", "augment", "boost", "elevate", "enhance", "multiply"],
                gloss: "become or make greater in size amount degree or number increase turnover",
                antonyms: &["reduce", "decrease", "lessen", "lower", "diminish"],
            },
            Synset {
                id: "verb.allow.01",
                pos: CoarsePos::Verb,
                lemmas: &["allow", "permit", "enable", "facilitate", "authorize", "admit"],
                gloss: "give permission or opportunity to do something allow permit enable",
                antonyms: &["forbid", "prohibit", "prevent", "block", "restrict"],
            },
            Synset {
                id: "verb.prevent.01",
                pos: CoarsePos::Verb,
                lemmas: &["prevent", "hinder", "avert", "thwart", "obstruct", "preclude", "forestall"],
                gloss: "keep something from happening or arising prevent errors damage harm",
                antonyms: &["allow", "permit", "enable", "promote", "foster"],
            },
            Synset {
                id: "verb.suggest.01",
                pos: CoarsePos::Verb,
                lemmas: &["suggest", "propose", "recommend", "advise", "advocate", "indicate"],
                gloss: "put forward for consideration suggest propose idea recommendation",
                antonyms: &["oppose", "discourage", "reject"],
            },

            // ==========================================
            // --- ADDITIONAL CORE NOUNS ---
            // ==========================================
            Synset {
                id: "noun.dog.canine",
                pos: CoarsePos::Noun,
                lemmas: &["dog", "canine", "hound", "pooch"],
                gloss: "a domesticated carnivorous mammal canis familiaris dog canine pet",
                antonyms: &[],
            },
            Synset {
                id: "noun.owner.possessor",
                pos: CoarsePos::Noun,
                lemmas: &["owner", "possessor", "keeper", "guardian", "holder"],
                gloss: "a person who owns has possession of property or pet owners",
                antonyms: &[],
            },
            Synset {
                id: "noun.human.person",
                pos: CoarsePos::Noun,
                lemmas: &["human", "person", "individual", "being", "mortal"],
                gloss: "a human being individual person people man woman",
                antonyms: &[],
            },
            Synset {
                id: "noun.people.public",
                pos: CoarsePos::Noun,
                lemmas: &["people", "individuals", "folks"],
                gloss: "human beings considered collectively people individuals folks",
                antonyms: &[],
            },
            Synset {
                id: "noun.reason.cause",
                pos: CoarsePos::Noun,
                lemmas: &["reason", "cause", "motive", "rationale", "ground", "basis", "purpose"],
                gloss: "a cause explanation or justification for an action or event reasons variety",
                antonyms: &[],
            },
            Synset {
                id: "noun.variety.diversity",
                pos: CoarsePos::Noun,
                lemmas: &["variety", "diversity", "range", "array", "assortment", "spectrum", "selection"],
                gloss: "the quality or state of being different assortment variety of reasons choices",
                antonyms: &["uniformity", "sameness", "monotony"],
            },
            Synset {
                id: "noun.excitement.thrill",
                pos: CoarsePos::Noun,
                lemmas: &["excitement", "enthusiasm", "thrill", "exhilaration", "animation", "eagerness"],
                gloss: "a feeling of great enthusiasm and eagerness excitement joy emotion",
                antonyms: &["apathy", "boredom", "indifference"],
            },
            Synset {
                id: "noun.frustration.annoyance",
                pos: CoarsePos::Noun,
                lemmas: &["frustration", "annoyance", "irritation", "disappointment", "vexation", "distress"],
                gloss: "the feeling of being upset or annoyed as a result of being unable to change things frustration",
                antonyms: &["satisfaction", "contentment", "peace"],
            },
            Synset {
                id: "noun.breed.strain",
                pos: CoarsePos::Noun,
                lemmas: &["breed", "variety", "strain", "type", "kind", "lineage"],
                gloss: "a stock of animals or plants within a species having distinctive appearance breed type",
                antonyms: &[],
            },
            Synset {
                id: "noun.body.physique",
                pos: CoarsePos::Noun,
                lemmas: &["body", "physique", "anatomy", "organism", "system", "frame"],
                gloss: "the physical structure of a person or animal including organs bones bodies",
                antonyms: &[],
            },
            Synset {
                id: "noun.period.duration",
                pos: CoarsePos::Noun,
                lemmas: &["period", "interval", "duration", "span", "stretch", "phase", "time"],
                gloss: "an interval of time characterized by particular events period of time duration",
                antonyms: &[],
            },
            Synset {
                id: "noun.time.temporal",
                pos: CoarsePos::Noun,
                lemmas: &["time", "moment", "instant", "point in time"],
                gloss: "the indefinite continued progress of existence or specific moment instant in time",
                antonyms: &[],
            },
            Synset {
                id: "noun.strain.stress",
                pos: CoarsePos::Noun,
                lemmas: &["strain", "stress", "pressure", "tension", "burden", "exertion"],
                gloss: "a force tending to pull or stretch something to an extreme degree strain of screaming",
                antonyms: &["relief", "ease", "relaxation"],
            },
            Synset {
                id: "noun.energy.vigor",
                pos: CoarsePos::Noun,
                lemmas: &["energy", "vigor", "stamina", "vitality", "power", "effort", "strength"],
                gloss: "the strength and vitality required for sustained physical or mental activity energy",
                antonyms: &["exhaustion", "lethargy", "fatigue"],
            },
            Synset {
                id: "noun.cord.vocal",
                pos: CoarsePos::Noun,
                lemmas: &["cord", "strand", "fiber", "ligament", "membrane"],
                gloss: "folds of membranous tissue in throat vocal cords larynx",
                antonyms: &[],
            },
            Synset {
                id: "noun.method.procedure",
                pos: CoarsePos::Noun,
                lemmas: &["method", "procedure", "approach", "technique", "system", "process", "strategy"],
                gloss: "a particular procedure for accomplishing or approaching something method technique",
                antonyms: &[],
            },
            Synset {
                id: "noun.result.outcome",
                pos: CoarsePos::Noun,
                lemmas: &["result", "outcome", "consequence", "effect", "finding", "conclusion"],
                gloss: "a thing that is caused or produced by something else consequence results findings",
                antonyms: &[],
            },
            Synset {
                id: "noun.factor.element",
                pos: CoarsePos::Noun,
                lemmas: &["factor", "element", "component", "aspect", "variable", "determinant"],
                gloss: "a circumstance fact or influence that contributes to a result factor element",
                antonyms: &[],
            },
            Synset {
                id: "noun.feature.characteristic",
                pos: CoarsePos::Noun,
                lemmas: &["feature", "characteristic", "attribute", "property", "trait", "aspect"],
                gloss: "a distinctive attribute or aspect of something feature characteristic property",
                antonyms: &[],
            },
            Synset {
                id: "noun.detail.aspect",
                pos: CoarsePos::Noun,
                lemmas: &["detail", "aspect", "particular", "element", "facet", "nuance"],
                gloss: "an individual feature fact or item in a larger whole detail aspect",
                antonyms: &[],
            },

            // ==========================================
            // --- ADDITIONAL CORE ADJECTIVES ---
            // ==========================================
            Synset {
                id: "adj.vocal.expressive",
                pos: CoarsePos::Adjective,
                lemmas: &["vocal", "expressive", "outspoken", "articulate", "audible", "loud"],
                gloss: "expressing opinions or sounds freely or loudly vocal breeds vocal cords",
                antonyms: &["silent", "quiet", "mute", "reticent"],
            },
            Synset {
                id: "adj.prone.susceptible",
                pos: CoarsePos::Adjective,
                lemmas: &["prone", "susceptible", "inclined", "disposed", "vulnerable", "predisposed"],
                gloss: "likely to or liable to suffer from do or experience something prone to barking",
                antonyms: &["immune", "resistant", "unlikely"],
            },
            Synset {
                id: "adj.short.brief",
                pos: CoarsePos::Adjective,
                lemmas: &["short", "brief", "fleeting", "concise", "momentary", "transient", "limited"],
                gloss: "lasting or taking a small amount of time short period brief moment",
                antonyms: &["long", "lengthy", "extended", "prolonged", "lasting"],
            },
            Synset {
                id: "adj.long.extended",
                pos: CoarsePos::Adjective,
                lemmas: &["long", "lengthy", "extended", "prolonged", "protracted", "enduring", "sustained"],
                gloss: "lasting or taking a great amount of time long periods lengthy duration",
                antonyms: &["short", "brief", "momentary", "fleeting"],
            },
            Synset {
                id: "adj.tiring.exhausting",
                pos: CoarsePos::Adjective,
                lemmas: &["tiring", "exhausting", "strenuous", "wearisome", "fatiguing", "demanding"],
                gloss: "causing one to feel tired or fatigued exhausting tiring effort",
                antonyms: &["refreshing", "energizing", "restful", "invigorating"],
            },
            Synset {
                id: "adj.tired.weary",
                pos: CoarsePos::Adjective,
                lemmas: &["tired", "weary", "exhausted", "fatigued", "drained"],
                gloss: "in need of sleep or rest feeling exhausted tired weary",
                antonyms: &["energetic", "refreshed", "alert", "vigorous"],
            },
            Synset {
                id: "adj.different.distinct",
                pos: CoarsePos::Adjective,
                lemmas: &["different", "distinct", "dissimilar", "diverse", "disparate", "contrasting", "varied"],
                gloss: "not the same as another or each other distinct different built differently",
                antonyms: &["similar", "identical", "same", "alike", "uniform"],
            },
            Synset {
                id: "adj.easy.simple",
                pos: CoarsePos::Adjective,
                lemmas: &["easy", "effortless", "simple", "straightforward", "uncomplicated"],
                gloss: "achieved without great effort simple straightforward easy",
                antonyms: &["hard", "difficult", "tough", "complex", "arduous"],
            },
            Synset {
                id: "adj.difficult.hard",
                pos: CoarsePos::Adjective,
                lemmas: &["difficult", "hard", "demanding", "challenging", "arduous", "tough", "onerous"],
                gloss: "needing much effort or skill to accomplish difficult hard task",
                antonyms: &["easy", "simple", "effortless", "straightforward"],
            },
            Synset {
                id: "adj.complex.intricate",
                pos: CoarsePos::Adjective,
                lemmas: &["complex", "intricate", "complicated", "elaborate", "sophisticated", "nuanced"],
                gloss: "consisting of many different and connected parts complex system intricate",
                antonyms: &["simple", "basic", "elementary", "crude"],
            },
            Synset {
                id: "adj.substantial.significant",
                pos: CoarsePos::Adjective,
                lemmas: &["substantial", "significant", "considerable", "sizable", "notable", "meaningful"],
                gloss: "of considerable importance size or worth substantial progress reduction",
                antonyms: &["insignificant", "minor", "trivial", "negligible", "slight"],
            },
            Synset {
                id: "adj.rapid.swift",
                pos: CoarsePos::Adjective,
                lemmas: &["rapid", "swift", "fast", "speedy", "quick", "brisk", "expedient"],
                gloss: "happening in a short time or at great speed rapid rate swift pace",
                antonyms: &["slow", "gradual", "sluggish", "delayed"],
            },
            Synset {
                id: "adj.frequent.common",
                pos: CoarsePos::Adjective,
                lemmas: &["frequent", "recurrent", "repeated", "regular", "common", "persistent"],
                gloss: "occurring or appearing quite often frequent intervals regular events",
                antonyms: &["rare", "infrequent", "uncommon", "sporadic", "occasional"],
            },
            Synset {
                id: "adj.rare.scarce",
                pos: CoarsePos::Adjective,
                lemmas: &["rare", "scarce", "infrequent", "uncommon", "unusual", "sparse"],
                gloss: "not occurring very often scarce rare phenomenon exceptional case",
                antonyms: &["common", "frequent", "abundant", "widespread", "ubiquitous"],
            },
            Synset {
                id: "adj.crucial.essential",
                pos: CoarsePos::Adjective,
                lemmas: &["crucial", "essential", "vital", "critical", "pivotal", "fundamental", "indispensable"],
                gloss: "of the greatest importance to the success or development crucial factor vital role",
                antonyms: &["unimportant", "trivial", "optional", "dispensable", "incidental"],
            },

            // ==========================================
            // --- ADDITIONAL CORE ADVERBS ---
            // ==========================================
            Synset {
                id: "adv.differently.distinctly",
                pos: CoarsePos::Adverb,
                lemmas: &["differently", "distinctly", "variously", "disparately", "uniquely"],
                gloss: "in a different way manner or degree built differently distinctly",
                antonyms: &["similarly", "identically", "alike", "uniformly"],
            },
            Synset {
                id: "adv.easily.effortlessly",
                pos: CoarsePos::Adverb,
                lemmas: &["easily", "effortlessly", "readily", "smoothly", "simply"],
                gloss: "without difficulty or great effort make it easier readily smoothly",
                antonyms: &["hardly", "difficultly", "laboriously"],
            },
            Synset {
                id: "adv.frequently.regularly",
                pos: CoarsePos::Adverb,
                lemmas: &["frequently", "often", "regularly", "repeatedly", "commonly"],
                gloss: "regularly or many times at short intervals frequently often",
                antonyms: &["rarely", "seldom", "infrequently"],
            },
            Synset {
                id: "adv.rarely.seldom",
                pos: CoarsePos::Adverb,
                lemmas: &["rarely", "seldom", "infrequently", "scarcely", "hardly"],
                gloss: "not often or frequently rarely seldom observed",
                antonyms: &["frequently", "often", "regularly", "commonly"],
            },
            Synset {
                id: "adv.substantially.notably",
                pos: CoarsePos::Adverb,
                lemmas: &["substantially", "considerably", "significantly", "notably", "markedly"],
                gloss: "to a great or significant extent substantially considerably higher",
                antonyms: &["slightly", "marginally", "trivially", "modestly"],
            },
            Synset {
                id: "adv.primarily.chiefly",
                pos: CoarsePos::Adverb,
                lemmas: &["primarily", "chiefly", "principally", "predominantly", "largely", "mainly"],
                gloss: "for the most part mainly chiefly primarily responsible",
                antonyms: &["secondarily", "incidentally"],
            },
            Synset {
                id: "adv.eventually.ultimately",
                pos: CoarsePos::Adverb,
                lemmas: &["eventually", "ultimately", "finally", "sooner or later", "in due course"],
                gloss: "in the end especially after a long time or lot of effort eventually finally",
                antonyms: &["initially", "immediately", "promptly"],
            },
            Synset {
                id: "adv.directly.immediately",
                pos: CoarsePos::Adverb,
                lemmas: &["directly", "immediately", "straight", "forthwith", "explicitly"],
                gloss: "without changing direction or stopping directly immediately",
                antonyms: &["indirectly", "circuitously"],
            },
        ]
    };

    /// Lemma to Synsets index for O(1) candidate lookup
    pub static ref LEMMA_INDEX: HashMap<(&'static str, CoarsePos), Vec<&'static Synset>> = {
        let mut map: HashMap<(&'static str, CoarsePos), Vec<&'static Synset>> = HashMap::new();
        for synset in SYNSET_DATABASE.iter() {
            for &lemma in synset.lemmas {
                map.entry((lemma, synset.pos)).or_default().push(synset);
            }
        }
        map
    };
}

pub struct SynsetDb;

impl Default for SynsetDb {
    fn default() -> Self {
        Self::new()
    }
}

impl SynsetDb {
    pub fn new() -> Self {
        Self
    }

    /// Looks up matching synsets for a lemma and grammatical category.
    pub fn find_synsets(&self, lemma: &str, pos: CoarsePos) -> Vec<&'static Synset> {
        let lower = lemma.to_lowercase();
        let mut results = Vec::new();
        for synset in SYNSET_DATABASE.iter() {
            if synset.pos == pos
                && synset
                    .lemmas
                    .iter()
                    .any(|&l| l.eq_ignore_ascii_case(&lower))
            {
                results.push(synset);
            }
        }
        results
    }

    /// Returns all candidate synonym lemmas for a given lemma and POS, excluding the original lemma and antonyms.
    pub fn get_candidate_synonyms(
        &self,
        lemma: &str,
        pos: CoarsePos,
    ) -> Vec<(&'static str, &'static Synset)> {
        let synsets = self.find_synsets(lemma, pos);
        let mut results = Vec::new();
        let lower = lemma.to_lowercase();

        for synset in synsets {
            for &candidate in synset.lemmas {
                if candidate.eq_ignore_ascii_case(&lower) {
                    continue;
                }
                // Check antonym exclusion
                if synset
                    .antonyms
                    .iter()
                    .any(|&a| a.eq_ignore_ascii_case(candidate))
                {
                    continue;
                }
                results.push((candidate, synset));
            }
        }

        results
    }
}
