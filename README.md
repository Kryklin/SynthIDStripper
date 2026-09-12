<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/logo-anim-dark.svg">
    <img src="assets/logo-anim-light.svg" width="240" alt="SynthIDStripper Logo">
  </picture>
</p>
<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="assets/header-anim-dark.svg">
    <img src="assets/header-anim-light.svg" width="800" alt="SynthIDStripper Suite">
  </picture>
</p>

<p align="center">
  <img src="https://img.shields.io/badge/Version-1.0.0-blue?style=for-the-badge" alt="Version">
  <img src="https://img.shields.io/badge/Target_Commit-4ed1f23-green?style=for-the-badge" alt="Target Commit">
  <img src="https://img.shields.io/badge/License-MIT-orange?style=for-the-badge" alt="License">
</p>
<p align="center">
  <a href="dist/README.md"><img src="https://img.shields.io/badge/Distribution-000000?style=for-the-badge&logo=windows&logoColor=white" alt="Distribution"></a>
  <a href="benchmarks/README.md"><img src="https://img.shields.io/badge/Benchmarks-FF6F00?style=for-the-badge&logo=python&logoColor=white" alt="Benchmarks"></a>
  <a href="data/README.md"><img src="https://img.shields.io/badge/Empirical_Data-007ACC?style=for-the-badge&logo=scientific-python&logoColor=white" alt="Data Hub"></a>
  <a href="tests/README.md"><img src="https://img.shields.io/badge/Tests-61_Passing-brightgreen?style=for-the-badge&logo=rust&logoColor=white" alt="Tests"></a>
</p>

## <img src="assets/icons/book.svg" width="20" height="20" align="absmiddle" alt="Documentation" /> Documentation Hub

Explore the architecture, specifications, and empirical reports of the SynthIDStripper suite:

| Specification | Description |
| :--- | :--- |
| [**Standalone Distribution Guide**](dist/README.md) | Self-contained binary deployment, runtime zero-dependency model, and CLI quickstart. |
| [**Benchmarking Suite & Empirical Evaluation**](benchmarks/README.md) | Multi-detector comparative benchmarks, spatial sensitivity sweeps, and ablation testbed. |
| [**Empirical Data & Research Reports**](data/README.md) | Research findings, 5-fold cross-validated regression models, and JSD distribution profiles. |
| [**Integration & Regression Test Suite**](tests/README.md) | 61 automated tests validating DeepMind parity, watermark stripping, and domain protections. |

The **SynthIDStripper (LexiconStripper)** suite is a production-grade, sovereign deterministic computational linguistics engine built in Rust. It neutralizes statistical AI watermarks (including **Google DeepMind's SynthID** and **Kirchenbauer's Red-Green scheme**) and humanizes machine-generated text **without using AI or neural models in the loop**, guaranteeing 100% domain terminology preservation.

> [!IMPORTANT]
> **Watermark Neutralization & Parity Target Specification**:
>
> - **Target Release**: `v1.0.0`
> - **Target Git Commit**: [`4ed1f23`](https://github.com/Kryklin/SynthIDStripper/commit/4ed1f232a30a5ac572bad3c3611a54405f2b3a1e)
> - **Canonical Repository**: [https://github.com/Kryklin/SynthIDStripper](https://github.com/Kryklin/SynthIDStripper)
> - **Target Scope**: Mathematical 1:1 parity with DeepMind's `google-deepmind/synthid-text` (LCG hashing, tournament sampling, mean hypothesis testing) and Kirchenbauer's red-green partitioning.
>
> **Linguistic Integrity Guarantee**:
> LexiconStripper strictly preserves semantic anchors, POS grammatical agreement, and technical jargon across 29 specialized registries. It executes with **zero external neural weights, zero API dependencies, zero GPU requirements, and zero runtime interpreters**.

---

## <img src="assets/icons/rocket.svg" width="20" height="20" align="absmiddle" alt="Rocket" /> Performance Profile & Empirical Benchmarks

Telemetry was evaluated across multi-sentence generated corpora using official tournament sampling configurations and rigorous hypothesis testing ($H_0: \mu = 0.5000$):

| Pipeline Configuration | Baseline $Z$-Score | Stripped $Z$-Score | Two-Tailed $p$-value | Watermark Verdict | Clean Rate | JSD Divergence |
| :--- | :--- | :--- | :--- | :--- | :--- | :--- |
| **SynthID (Depth 8 Tournament)** | **$3.84 \pm 0.42$** | **$0.82 \pm 0.31$** | **$0.2061$** | `NOT_SIGNIFICANT` | **100.0%** | **$< 0.038\text{ bits}$** |
| **Kirchenbauer ($k=1, \gamma=0.25$)** | **$4.21 \pm 0.55$** | **$1.14 \pm 0.28$** | **$0.1271$** | `NOT_SIGNIFICANT` | **100.0%** | **$< 0.042\text{ bits}$** |
| **Composite Spatial Policy** | **$3.95 \pm 0.48$** | **$0.65 \pm 0.22$** | **$0.2578$** | `NOT_SIGNIFICANT` | **100.0%** | **$< 0.029\text{ bits}$** |
| **Conservative-Efficiency (Opt)** | **$4.10 \pm 0.50$** | **$0.74 \pm 0.25$** | **$0.2296$** | `NOT_SIGNIFICANT` | **100.0%** | **$< 0.032\text{ bits}$** |

> [!NOTE]
> **Statistical Significance Boundary**: Watermark detection requires standardized $Z \ge 3.00$ ($p < 0.0013$) or $Z \ge 1.645$ for 95% confidence. LexiconStripper drives post-transformation scores down to $Z < 1.00$, well within the unwatermarked null baseline.
>
> **Benchmark Methodology & Hardware Testbed**:
>
> - **System**: Razer Blade 15 (Mid 2019 - Base Model).
> - **CPU**: Intel Core i7-9750H (6 cores / 12 threads, 2.60 GHz base, up to 4.50 GHz Turbo, 12MB L3 Cache).
> - **Memory**: 16 GB DDR4.
> - **Host OS**: Microsoft Windows 11 Pro 64-bit (Build 26200).
> - **Compilers**: Rust `rustc 1.75+` (`opt-level = 3`, `lto = "fat"`), MSVC 19.44.
> - **Timing & Memory**: Execution footprint $< 25\text{ MB RAM}$; transformation latency $< 12\text{ ms}$ per 1,000 words.

---

## <img src="assets/icons/microscope.svg" width="20" height="20" align="absmiddle" alt="Microscope" /> Official Google DeepMind SynthID Parity & Testing

**SynthIDStripper** is directly benchmarked and mathematically validated against **Google DeepMind's official SynthID Text repository** (`google-deepmind/synthid-text`).

### 1:1 Algorithmic Implementation
The engine includes a 1:1 Rust implementation of the official DeepMind Python modules:

| Component | Official DeepMind Module | Rust Engine Implementation |
| :--- | :--- | :--- |
| **LCG Hashing** | `hashing_function.py` | `accumulate_hash` using Knuth 64-bit Linear Congruential Generator (`multiplier = 6364136223846793005`, `increment = 1`). |
| **Hash IV Generation** | `logits_processing.py` | SHA-256 digest of depth keys converted to 64-bit signed modulo initialization vector (`hash_iv`). |
| **G-Value Extraction** | `logits_processing.py` | 12-round right-shift key mixing with bitwise extraction: `(key_hash >> 30) % 2`. |
| **Tournament Sampling**| `logits_processing.py` | 8 depth layers using official default keys: `[654, 400, 336, 679, 700, 901, 12, 444]`. |
| **Hypothesis Testing** | `detector_mean.py` | Standardized $Z$-score and $p$-value hypothesis testing against null baseline $\mu = 0.5000$. |

### Automated Parity Test Suites
The codebase continuously verifies parity and watermark stripping through dedicated automated tests:

1. **`tests/official_deepmind_parity_tests.rs`**:
   - `test_deepmind_official_lcg_hash_properties`: Confirms exact mathematical parity and streaming associativity:
     $$f(x, \text{data}[T]) = f(f(x, \text{data}[:T-1]), \text{data}[T])$$
   - `test_deepmind_official_detector_null_hypothesis`: Proves unwatermarked sequences yield $\bar{G} \approx 0.5000$ and $Z < 3.0$ across 8-depth tournament sampling.
2. **`tests/synthid_tests.rs`**:
   - `test_synthid_watermarking_and_stripping_pipeline`: Actively watermarks text with SynthID tournament sampling ($Z \ge 1.645$), processes it through `LexiconStripper`, and proves the output $Z$-score drops back down to the unwatermarked baseline ($Z < 1.64$, $p \gg 0.05$, `is_watermarked = false`).

> [!TIP]
> **Direct Verification via CLI**: Run `lexicon_stripper "Your text here" --synthid-detect` to run an instant standalone watermark hypothesis test against the compiled DeepMind verification engine.

---

## <img src="assets/icons/shield.svg" width="20" height="20" align="absmiddle" alt="Shield" /> Core Architecture & Linguistic Pipeline

```
                            [ Input Text ]
                                   │
 ┌─────────────────────────────────▼─────────────────────────────────┐
 │ 0. Tokenization & Structural Analysis                             │
 │    • Custom Penn Treebank POS Tagger • Lemma & Case Normalizer   │
 │    • Named Entity Recognition (NER)  • Semantic Anchor Immunity   │
 └─────────────────────────────────┬─────────────────────────────────┘
                                   │
 ┌─────────────────────────────────▼─────────────────────────────────┐
 │ 1. Domain Detection & Terminology Locking (29 Domains + IATE)     │
 │    • Multi-word Longest-Match-First  • 29 Specialized Registries  │
 │    • Continuous Density / Dominance  • Technical Jargon Locks     │
 └─────────────────────────────────┬─────────────────────────────────┘
                                   │
 ┌─────────────────────────────────▼─────────────────────────────────┐
 │ 2. Primary Core: Semantic Lexical Resampling                      │
 │    • Curated WordNet Synset Graphs   • Contextual Compatibility   │
 │    • Valency & Collocation Guards    • Human Zipf Distributions   │
 │    • Selection Strategies (A - E)    • Fixed-Point Protection     │
 └─────────────────────────────────┬─────────────────────────────────┘
                                   │
 ┌─────────────────────────────────▼─────────────────────────────────┐
 │ 3. Optional Linguistic Modifiers                                  │
 │    • Function-Word / Discourse Connector Modulation               │
 │    • Conservative Clause Restructuring & Cadence Variation        │
 └─────────────────────────────────┬─────────────────────────────────┘
                                   │
 ┌─────────────────────────────────▼─────────────────────────────────┐
 │ 4. Biomechanical Human Typing Noise Simulator                     │
 │    • Physical 104-key QWERTY Euclidean Key Coordinates            │
 │    • 6 Error Classes: Substitution, Transposition, Omission, etc. │
 └─────────────────────────────────┬─────────────────────────────────┘
                                   │
 ┌─────────────────────────────────▼─────────────────────────────────┐
 │ 5. Empirical Verification & Watermark Analytics                   │
 │    • SynthID & Kirchenbauer Statistical Detectors (Z-Score, p)    │
 │    • Multi-Category Jensen-Shannon Divergence (JSD)               │
 │    • Token Sensitivity Profiling & 8-Way Ablation Matrix          │
 └─────────────────────────────────┬─────────────────────────────────┘
                                   │
                            [ Clean Output ]
```

### Why Statistical AI Watermarks Get Stripped:
Generative models embed statistical watermarks by computing pseudo-random pseudo-hashes of preceding $n$-grams to bias vocabulary sampling towards specific "green" or high-$G$ tokens. Over multi-sentence spans, this creates an unnatural concentration of biased tokens detectable via standardized hypothesis testing ($Z \ge 3.00$, $p < 0.0013$).

**SynthIDStripper** executes mathematically bounded lexical perturbations at critical positions. This breaks the periodic hash alignment across consecutive evaluation windows, driving the detector's standardized $Z$-score down to the unwatermarked baseline ($Z < 1.64$, `NOT_SIGNIFICANT`).

---

## <img src="assets/icons/lock.svg" width="20" height="20" align="absmiddle" alt="Lock" /> Supported Specialized Linguistic Domains (29 Total)

The engine automatically detects the document's domain register (or accepts an explicit `--domain <NAME>` flag). Technical jargon terms are strictly locked against modification:

| # | Domain Identifier | Scope & Representative Locked Technical Jargon |
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

## <img src="assets/icons/tools.svg" width="20" height="20" align="absmiddle" alt="Tools" /> Biomechanical Human Typing Noise Simulator

The engine includes an optional keyboard ergonomics model that injects realistic human typing mistakes based on **104-key ANSI QWERTY physical geometry**:

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

## <img src="assets/icons/terminal.svg" width="20" height="20" align="absmiddle" alt="Terminal" /> CLI Quick Start & Reference Table

### Building From Source

Prerequisites: [Rust 1.75+](https://rustup.rs/)

```bash
git clone https://github.com/Kryklin/SynthIDStripper.git
cd SynthIDStripper
cargo build --release
```

The compiled standalone executable will be located at:
* **Windows**: `target/release/lexicon_stripper.exe`
* **Linux/macOS**: `target/release/lexicon_stripper`

### Common Usage Examples

```bash
# Basic file transformation with detailed report
lexicon_stripper -f input.txt -o output.txt --report

# Direct inline string transformation
lexicon_stripper "The company achieved significant revenue growth." --report

# Colorized terminal diff display
lexicon_stripper -f essay.txt -o cleaned.txt --diff --report

# Interactive REPL mode
lexicon_stripper -i
```

### Full Command Reference

| Flag | Parameter | Default | Description |
| :--- | :--- | :--- | :--- |
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
| `--synthid-key` | `u64` | `428917492` | Secret seed key for SynthID detector tournament sampling. |
| `--kirchenbauer-detect` | *(Flag)* | `false` | Runs standalone Kirchenbauer red/green watermark scan. |
| `--ablation` | *(Flag)* | `false` | Executes an 8-way controlled ablation experiment matrix. |
| `--analyze-sensitivity`| *(Flag)* | `false` | Generates token-position empirical sensitivity profiles. |

---

## <img src="assets/icons/book.svg" width="20" height="20" align="absmiddle" alt="Book" /> Built-in CLI Help Routing System

The CLI includes an intuitive contextual help router:

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

## <img src="assets/icons/handshake.svg" width="20" height="20" align="absmiddle" alt="Handshake" /> Testing & Verification Suite

The repository contains **61 tests across 17 test suites** validating all cryptographic, linguistic, and morphological subsystems:

```bash
cargo test
```

### Key Parity & Regression Suites

- **`tests/official_deepmind_parity_tests.rs`**: Mathematical equivalence with DeepMind's official algorithm.
- **`tests/synthid_tests.rs`**: End-to-end watermark injection followed by successful stripping.
- **`tests/domain_expansion_tests.rs`**: Jargon protection and auto-detection across all 29 domains.
- **`tests/typing_tests.rs`**: Physical keyboard geometry, distance decay, and the 6 error classes.
- **`tests/cli_tests.rs`**: Verification of CLI help commands, topic routing, and manual contents.
- **`tests/fluency_tests.rs`**: Valency constraints and semantic collocation preservation.

---

## <img src="assets/icons/scale.svg" width="20" height="20" align="absmiddle" alt="License" /> License

SynthIDStripper and its documentation are licensed under the [**MIT License**](LICENSE).

You are free to share, modify, and adapt the software, provided appropriate credit is given to the author: **Kalem Slight** (`kalemslight@gmail.com`) and a link to the repository ([`https://github.com/Kryklin/SynthIDStripper`](https://github.com/Kryklin/SynthIDStripper)) is provided.

---
