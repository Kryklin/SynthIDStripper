# 🛡️ SynthIDStripper (LexiconStripper)

[![Rust Stable](https://img.shields.io/badge/Rust-1.75%2B-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Zero AI in the Loop](https://img.shields.io/badge/AI%20in%20Loop-0%25%20(Deterministic)-blue.svg?style=for-the-badge)](https://github.com)
[![Automated Tests](https://img.shields.io/badge/Tests-61%20Passed%20%7C%2017%20Suites-brightgreen.svg?style=for-the-badge)](https://github.com)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg?style=for-the-badge)](https://github.com)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](LICENSE)

> **A production-grade, zero-AI deterministic computational linguistics engine that neutralizes statistical AI watermarks (Google DeepMind SynthID, Kirchenbauer et al.) and humanizes machine-generated text while guaranteeing 100% domain terminology preservation.**

---

## ⚡ Highlights

* **100% Deterministic — Zero AI in the Loop**: Runs entirely on discrete symbolic linguistics, WordNet synset graphs, and physical keyboard ergonomics. No LLMs, no neural networks, and no external API calls required.
* **1:1 Google DeepMind SynthID Parity**: Benchmarked and mathematically verified against Google DeepMind's official open-source repository (`google-deepmind/synthid-text`).
* **29 Specialized Linguistic Domains**: Features locked jargon registers across 29 specialized fields (Finance, Biomedical, CS, Culinary, Legal, Aviation, Maritime, etc.) ensuring technical terms are never corrupted.
* **IATE / EuroVoc Multi-Word Terminology DB**: Longest-Match-First atomic phrase protection for institutional multi-word terms (e.g., *"air traffic control"*, *"bill of lading"*, *"dead reckoning"*).
* **Biomechanical Human Typing Noise Simulator**: Models natural typing imperfections using physical 104-key ANSI QWERTY Euclidean key distances across 6 biomechanical error classes.
* **Single Standalone Binary**: Compiles to a self-contained ~6 MB executable with all databases, synsets, and rules embedded directly inside.

---

## 🔬 How It Works

Generative language models embed statistical watermarks by altering token sampling distributions based on pseudo-random hashes of preceding $n$-grams ($k$-token contexts). Over multi-sentence spans, this creates an unnatural concentration of "green" tokens or elevated $G$-values detectable via standardized $Z$-score hypothesis testing ($Z \ge 3.00$, $p < 0.0013$).

```
                                  [ Input Text ]
                                         │
 ┌───────────────────────────────────────▼───────────────────────────────────────┐
 │ 0. Tokenization & POS Tagging                                                 │
 │    • Custom Penn Treebank Tagger     • Lemma & Case Normalizer                 │
 │    • Named Entity Recognition (NER)  • Semantic Anchor Immunity                │
 └───────────────────────────────────────┬───────────────────────────────────────┘
                                         │
 ┌───────────────────────────────────────▼───────────────────────────────────────┐
 │ 1. Domain Detection & Terminology Locking (29 Domains + IATE / EuroVoc)        │
 │    • Multi-word Longest-Match-First  • 29 Specialized Vocabulary Registries   │
 │    • Continuous Density / Dominance  • Technical Jargon Protection Locks      │
 └───────────────────────────────────────┬───────────────────────────────────────┘
                                         │
 ┌───────────────────────────────────────▼───────────────────────────────────────┐
 │ 2. Primary Core: Semantic Lexical Resampling                                  │
 │    • Curated WordNet Synset Graphs   • Contextual Compatibility & POS Match   │
 │    • Valency & Collocation Guards    • Zipf Human Frequency Distributions     │
 │    • Selection Strategies (A - E)    • Fixed-Point Protection Invariant       │
 └───────────────────────────────────────┬───────────────────────────────────────┘
                                         │
 ┌───────────────────────────────────────▼───────────────────────────────────────┐
 │ 3. Optional Linguistic Modifiers                                              │
 │    • Function-Word / Discourse Modulation                                     │
 │    • Conservative Syntactic Clause Restructuring & Cadence Variation          │
 └───────────────────────────────────────┬───────────────────────────────────────┘
                                         │
 ┌───────────────────────────────────────▼───────────────────────────────────────┐
 │ 4. Biomechanical Human Typing Noise Simulator                                 │
 │    • Physical 104-key ANSI QWERTY Geometry & Euclidean Key Coordinates        │
 │    • 6 Error Classes: Substitution, Transposition, Omission, Insertion, etc.  │
 └───────────────────────────────────────┬───────────────────────────────────────┘
                                         │
 ┌───────────────────────────────────────▼───────────────────────────────────────┐
 │ 5. Empirical Verification & Watermark Analytics                               │
 │    • SynthID & Kirchenbauer Statistical Detectors (Z-Score, p-value)          │
 │    • Multi-Category Jensen-Shannon Divergence (JSD)                           │
 │    • Token-Level Sensitivity Heatmap Profiling & Ablation Matrix              │
 └───────────────────────────────────────┬───────────────────────────────────────┘
                                         │
                                 [ Clean Output ]
```

**SynthIDStripper** applies mathematically bounded lexical perturbations at critical positions. This breaks the periodic hash alignment across consecutive evaluation windows, driving the detector's standardized $Z$-score down to the unwatermarked baseline ($Z < 1.64$, `NOT_SIGNIFICANT`).

---

## 🚀 Quick Start

### Building From Source

Prerequisites: [Rust 1.75+](https://rustup.rs/)

```bash
git clone https://github.com/your-org/SynthIDStripper.git
cd SynthIDStripper
cargo build --release
```

The compiled standalone executable will be located at:
* **Windows**: `target/release/lexicon_stripper.exe`
* **Linux/macOS**: `target/release/lexicon_stripper`

---

## 📖 CLI Usage & Examples

### Basic File Transformation
```bash
lexicon_stripper -f input.txt -o output.txt --report
```

### Direct Inline String
```bash
lexicon_stripper "The company achieved significant revenue growth." --report
```

### Colorized Terminal Diff
```bash
lexicon_stripper -f essay.txt -o cleaned.txt --diff --report
```

### Interactive REPL Mode
```bash
lexicon_stripper -i
```

---

## 💡 Built-in Help System

The CLI includes an intuitive help routing system:

```bash
# General CLI help and options overview
lexicon_stripper help

# Comprehensive guide to the human typing noise simulator
lexicon_stripper help typos
# (Aliases: help typo, help typing, typos, --help-typos)

# Directory of all 29 supported specialized domains and locked jargon
lexicon_stripper help domains

# Statistical watermark architecture guide (SynthID & Kirchenbauer)
lexicon_stripper help watermark
```

---

## 🏛️ Supported Specialized Domains (29 Total)

The engine automatically detects the document's domain register (or accepts an explicit `--domain <NAME>` flag). Technical jargon terms are strictly locked against modification:

| # | Domain Identifier | Scope & Representative Locked Jargon |
|---|:---|:---|
| 1 | **`culinary`** | *sommelier, stagiaire, gastronomy, Michelin, sous-chef, turnip, reduction, foams, steak, service* |
| 2 | **`finance`** | *ebitda, liquidity, yield, amortization, insolvency, leverage, securities, dividend, portfolio, collateral* |
| 3 | **`biomedical`** | *apoptosis, cytokine, pathogen, oncogene, receptor, metabolite, phagocytosis, mrna, histology* |
| 4 | **`computer_science`** | *compiler, latency, throughput, mutex, semaphore, deadlock, recursion, bytecode, stack, heap, api* |
| 5 | **`legal`** | *subpoena, tort, plaintiff, defendant, affidavit, jurisdiction, statute, indictment, injunction* |
| 6 | **`creative_arts`** | *chiaroscuro, triptych, fresco, sculpture, choreography, curator, sonata, symphony, contrapposto* |
| 7 | **`science`** | *spectroscopy, photosynthesis, entropy, neutrino, isotope, catalyst, quantum, relativity* |
| 8 | **`politics`** | *filibuster, gerrymandering, incumbent, referendum, parliament, senate, bipartisan, hegemony* |
| 9 | **`sports`** | *quarterback, goalkeeper, decathlon, playoff, tournament, hat-trick, steeplechase, triathlon* |
| 10 | **`education`** | *pedagogy, syllabus, dissertation, matriculation, alumni, baccalaureate, practicum, curriculum* |
| 11 | **`philosophy_psychology`**| *epistemology, phenomenology, hermeneutics, psychoanalysis, subconscious, behaviorism, ontology* |
| 12 | **`military`** | *brigade, reconnaissance, ballistics, artillery, battalion, infantry, garrison, regiment, flank* |
| 13 | **`journalism_media`** | *byline, op-ed, broadsheet, broadcast, syndication, press, newsroom, correspondent, photojournalism* |
| 14 | **`travel_tourism`** | *itinerary, excursion, concierge, lodging, sightseeing, ecotourism, wayfinding, boarding pass* |
| 15 | **`aviation_aerospace`** | *avionics, telemetry, fuselage, altimeter, yaw, pitch, roll, mach, orbit, stall, aileron, thrust* |
| 16 | **`architecture_construction`** | *cantilever, load-bearing, hvac, easement, brutalism, masonry, facade, scaffolding, blueprint, rebar* |
| 17 | **`gaming_esports`** | *hitbox, aggro, frame data, rng, dps, nerfed, buffed, matchmaking, respawn, speedrun, cooldown* |
| 18 | **`agriculture_botany`** | *hydroponics, grafting, agronomy, cultivar, photosynthesis, npk, herbicide, tillage, pollination* |
| 19 | **`fashion_textiles`** | *selvage, warp, weft, bias cut, silhouette, seam, bespoke, cashmere, haute couture, mannequin* |
| 20 | **`theology_religion`** | *exegesis, ecclesiastical, dogma, liturgy, secular, orthodox, sacrament, eschatology, scripture* |
| 21 | **`audio_engineering`** | *reverb, equalization, lossless, polyrhythm, attenuation, spectrogram, decibel, preamp, dither* |
| 22 | **`maritime_nautical`** | *starboard, ballast, keel, draft, bulkhead, knot, halyard, portside, rudder, anchor, dead reckoning* |
| 23 | **`automotive_motorsport`** | *camshaft, chassis, torque, oversteer, slipstream, drivetrain, telemetry, understeer, paddock* |
| 24 | **`film_television`** | *mise-en-scène, foley, anamorphic, best boy, storyboard, render, aperture, screenplay, gaffer* |
| 25 | **`linguistics_philology`** | *phoneme, morphology, fricative, diphthong, syntax, semantic, conjugation, morpheme, etymology* |
| 26 | **`real_estate_property`** | *escrow, lien, appraisal, zoning, sublet, title, gentrification, mortgage, tenant, landlord, deed* |
| 27 | **`logistics_supply_chain`** | *intermodal, procurement, bill of lading, sku, freight, bottleneck, inventory, distribution center* |
| 28 | **`fitness_kinesiology`** | *hypertrophy, anabolic, macros, deadlift, isometric, glycogen, superset, barbell, biomechanics* |
| 29 | **`occult_astrology`** | *retrograde, sigil, astral, manifestation, divination, natal chart, esoteric, horoscope, talisman* |

---

## ⌨️ Biomechanical Human Typing Noise Simulator

The engine includes an optional keyboard physics model that injects realistic human typing mistakes based on **104-key ANSI QWERTY geometry**:
* **Distance Metric**: Euclidean key distance $d = \sqrt{(\Delta x)^2 + (\Delta y)^2}$ with exponential spatial decay:
  $$P(\text{key}' \mid \text{key}) \propto \exp(-\alpha \cdot d)$$
* **The 6 Biomechanical Error Classes**:
  1. **`substitution`** (35%): Strikes an adjacent physical key (`d` $\to$ `s`, `e`, `r`, `f`, `x`, `c`).
  2. **`transposition`** (20%): Timing slip inverting adjacent letters (`the` $\to$ `teh`).
  3. **`omission`** (15%): Dropped or under-pressed keystroke (`industry` $\to$ `idustry`). Words $\le 3$ chars are protected.
  4. **`insertion`** (15%): Stray double-contact adjacent keystrike (`recipe` $\to$ `recxipe`).
  5. **`duplication`** (10%): Key bounce double-strike repeating a valid letter (`coffee` $\to$ `coffeey`).
  6. **`temporal`** (5%): Finger reach trajectory transition error between preceding and succeeding strokes.
* **Semantic Anchor Immunity**: Named entities (`NNP`/`NNPS`), numbers, currency amounts, dates, URLs, code tokens, and CLI flags are 100% immune from mutation.

```bash
# Apply subtle 3% realistic typing noise with deterministic reproducibility:
lexicon_stripper -f input.txt -o output.txt --typing-noise 0.03 --typing-seed 1337 --report
```

---

## 🔬 Official Google DeepMind SynthID Parity

**SynthIDStripper** is directly benchmarked and mathematically validated against **Google DeepMind's official SynthID Text repository** (`google-deepmind/synthid-text`):

| Component | Official DeepMind Module | Rust Engine Implementation |
|:---|:---|:---|
| **LCG Hashing** | `hashing_function.py` | `accumulate_hash` using Knuth 64-bit Linear Congruential Generator (`multiplier = 6364136223846793005`, `increment = 1`). |
| **Hash IV Generation** | `logits_processing.py` | SHA-256 digest of depth keys converted to 64-bit signed modulo initialization vector (`hash_iv`). |
| **G-Value Extraction** | `logits_processing.py` | 12-round right-shift key mixing with bitwise extraction: `(key_hash >> 30) % 2`. |
| **Tournament Sampling**| `logits_processing.py` | 8 depth layers using official default keys: `[654, 400, 336, 679, 700, 901, 12, 444]`. |
| **Hypothesis Testing** | `detector_mean.py` | Standardized $Z$-score and $p$-value hypothesis testing against null baseline $\mu = 0.5000$. |

### Testing SynthID via CLI
```bash
# Scan any text for official SynthID watermarking:
lexicon_stripper -f my_text.txt --synthid-detect

# Transform text and view live SynthID verification in report:
lexicon_stripper -f input.txt -o output.txt --report

# Watermark text using official SynthID tournament sampling:
lexicon_stripper "Text to watermark" --synthid-watermark --synthid-key 428917492
```

---

## 🛠️ CLI Reference Table

| Flag | Parameter | Default | Description |
|:---|:---|:---|:---|
| `-f, --file` | `<PATH>` | *stdin* | Path to input document. |
| `-o, --output` | `<PATH>` | *stdout* | Path to write transformed text. |
| `-r, --report` | *(Flag)* | `false` | Prints formatted statistical report to stderr. |
| `-d, --diff` | *(Flag)* | `false` | Displays colorized terminal diff of word replacements. |
| `-j, --json` | *(Flag)* | `false` | Outputs structured transformation metrics in JSON. |
| `-l, --log` | `<PATH>` | `None` | Saves detailed empirical transformation audit logs to JSON. |
| `-i, --interactive` | *(Flag)* | `false` | Launches interactive REPL terminal session. |
| `-m, --mode` | `neutral` \| `human` \| `ai` \| `random` | `human` | Token frequency sampling distribution. |
| `--domain` | `auto` or any of the 29 domains | `auto` | Specialized linguistic domain register. |
| `--strategy` | `baseline` \| `uniform` \| `context-neutral` \| `efficiency-guided` \| `conservative-efficiency` | `conservative-efficiency` | Candidate selection algorithm. |
| `-p, --prob` | `0.0 .. 1.0` | `0.30` | Replacement probability for eligible content words. |
| `-c, --min-confidence` | `0.0 .. 1.0` | `0.55` | Minimum semantic compatibility threshold. |
| `-s, --seed` | `u64` | `None` | Deterministic RNG master seed for reproducibility. |
| `-n, --passes` | `usize` | `1` | Number of iterative transformation passes. |
| `--typing-noise` | `0.0 .. 1.0` | `0.0` | Probability of applying human typing noise. |
| `--typing-seed` | `u64` | `None` | Deterministic RNG seed for typing noise. |
| `--typing-errors` | `sub,trans,omit,ins,dup,temp` | *All* | Comma-separated list of enabled error classes. |
| `--syntax` | *(Flag)* | `false` | Enables conservative clause and syntactic restructuring. |
| `--cadence` | *(Flag)* | `false` | Enables sentence pattern and cadence variation. |
| `--function-words` | *(Flag)* | `false` | Enables discourse connector and function-word alternatives. |
| `--combined` | *(Flag)* | `false` | Enables all layers (Lexical + Syntax + Cadence + Function Words). |
| `--synthid-detect` | *(Flag)* | `false` | Runs standalone SynthID watermark scan on input text. |
| `--kirchenbauer-detect` | *(Flag)* | `false` | Runs standalone Kirchenbauer red/green watermark scan. |
| `--ablation` | *(Flag)* | `false` | Executes an 8-way controlled ablation experiment matrix. |
| `--analyze-sensitivity`| *(Flag)* | `false` | Generates token-position empirical sensitivity profiles. |

---

## 🧪 Testing & Validation

The test suite contains **61 tests across 17 test suites** validating all subsystems:

```bash
cargo test
```

### Key Test Suites
* **`tests/official_deepmind_parity_tests.rs`**: Mathematical equivalence with DeepMind's official algorithm.
* **`tests/synthid_tests.rs`**: End-to-end watermark injection followed by successful stripping.
* **`tests/domain_expansion_tests.rs`**: Jargon protection and auto-detection across all 29 domains.
* **`tests/typing_tests.rs`**: Physical keyboard geometry, distance decay, and the 6 error classes.
* **`tests/cli_tests.rs`**: Verification of CLI help commands, topic routing, and manual contents.
* **`tests/fluency_tests.rs`**: Valency constraints and semantic collocation preservation.

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
