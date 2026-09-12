<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="../assets/logo-anim-dark.svg">
    <img src="../assets/logo-anim-light.svg" width="120" alt="SynthIDStripper Logo">
  </picture>
</p>
<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="../assets/header-anim-dark.svg">
    <img src="../assets/header-anim-light.svg" width="600" alt="SynthIDStripper Suite">
  </picture>
</p>

<div align="center">

[<img src="../assets/icons/home.svg" width="13" height="13" align="absmiddle" alt="Main" /> Main](../README.md) | [<img src="../assets/icons/rocket.svg" width="13" height="13" align="absmiddle" alt="Distribution" /> Distribution](README.md) | [<img src="../assets/icons/tools.svg" width="13" height="13" align="absmiddle" alt="Benchmarks" /> Benchmarks](../benchmarks/README.md) | [<img src="../assets/icons/microscope.svg" width="13" height="13" align="absmiddle" alt="Data Hub" /> Data Hub](../data/README.md) | [<img src="../assets/icons/shield.svg" width="13" height="13" align="absmiddle" alt="Tests" /> Tests](../tests/README.md)

</div>

# SynthIDStripper: Standalone Binary Distribution Guide

<p align="left">
  <img src="https://img.shields.io/badge/Platform-Windows_x86__64-blue?style=for-the-badge&logo=windows&logoColor=white" alt="Windows x86_64">
  <img src="https://img.shields.io/badge/Binary_Size-~6_MB-green?style=for-the-badge&logo=files&logoColor=white" alt="Binary Size">
  <img src="https://img.shields.io/badge/Dependencies-Zero-orange?style=for-the-badge" alt="Zero Dependencies">
  <img src="https://img.shields.io/badge/Offline-100%25-brightgreen?style=for-the-badge" alt="100% Offline">
</p>

**LexiconStripper** is a high-performance, deterministic computational linguistics engine built in Rust. It neutralizes statistical AI watermarks (including **Google DeepMind's SynthID** and **Kirchenbauer's Red-Green scheme**) and humanizes machine-generated text **without using AI or neural models in the loop**.

> [!IMPORTANT]
> **Production Binary Notice**:
> This distribution package contains the self-contained standalone executable `lexicon_stripper.exe`. All synset graphs, phonetic mappings, domain registries, and detector algorithms are statically compiled into the binary.

---

## <img src="../assets/icons/rocket.svg" width="20" height="20" align="absmiddle" alt="Portability" /> Executive Summary & Portability

- **Single Standalone Executable**: You only need `lexicon_stripper.exe` (~6 MB).
- **Zero Runtime Dependencies**:
  - No Python, Node.js, or runtime interpreters required.
  - No CUDA, PyTorch, or GPU drivers required.
  - No external DLLs, C++ redistributables, or asset folders required.
  - No internet access or API keys required (runs 100% offline, locally, and deterministically).
- **Fully Embedded Knowledge Bases**:
  - **29 Specialized Linguistic Domain Registries**: Compiled into the binary.
  - **IATE / EuroVoc Multi-Word Terminology Database**: Compiled into the binary.
  - **WordNet Synset Graphs & Valency / Collocation Rules**: Compiled into the binary.
  - **104-Key ANSI QWERTY Physical Geometry Model**: Compiled into the binary.
  - **SynthID & Kirchenbauer Statistical Detectors**: Compiled into the binary.

---

## <img src="../assets/icons/terminal.svg" width="20" height="20" align="absmiddle" alt="Quick Start" /> Quick Start Guide

Open PowerShell or Command Prompt in the directory containing `lexicon_stripper.exe`:

### Basic File Transformation
```powershell
.\lexicon_stripper.exe -f input.txt -o output.txt --report
```

### Inline String Transformation
```powershell
.\lexicon_stripper.exe "The company achieved significant revenue growth." --report
```

### Colorized Terminal Diff
```powershell
.\lexicon_stripper.exe -f essay.txt -o clean.txt --diff --report
```

### Interactive REPL Mode
```powershell
.\lexicon_stripper.exe -i
```

---

## <img src="../assets/icons/book.svg" width="20" height="20" align="absmiddle" alt="CLI Help" /> Built-in CLI Help Command System

The executable includes a comprehensive contextual help routing system:

```powershell
# 1. Main command overview with topic pointers
.\lexicon_stripper.exe help

# 2. Deep-dive guide to the Human Typing Noise / Typo Simulator
.\lexicon_stripper.exe help typos
# (Aliases: help typo, help typing, typos, --help-typos)

# 3. Directory of all 29 supported specialized domains & protected jargon
.\lexicon_stripper.exe help domains

# 4. Statistical watermark architecture guide (SynthID & Kirchenbauer)
.\lexicon_stripper.exe help watermark
```

---

## <img src="../assets/icons/shield.svg" width="20" height="20" align="absmiddle" alt="Architecture" /> How the System Works (6 Core Layers)

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

### Why AI Watermarks Get Stripped:
AI statistical watermarks operate by pseudo-randomly hashing preceding token $n$-grams to bias sampling towards specific "green" or high-$G$ tokens. **LexiconStripper** applies mathematically bounded lexical perturbations at critical positions. This breaks the periodic hash alignment across consecutive evaluation windows, driving the detector's standardized $Z$-score down from the detection zone ($Z \ge 3.0$) to the unwatermarked baseline ($Z < 1.64$, `NOT_SIGNIFICANT`).

---

## <img src="../assets/icons/lock.svg" width="20" height="20" align="absmiddle" alt="Domains" /> Supported Specialized Linguistic Domains (29 Total)

The engine automatically detects the topic register of your text (or you can specify it via `--domain <NAME>`). All specialized jargon terms are locked against corruption:

| # | Domain Identifier | Key Protected Technical Terms & Jargon |
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

## <img src="../assets/icons/tools.svg" width="20" height="20" align="absmiddle" alt="Typing Simulator" /> Biomechanical Human Typing Noise Simulator

The engine can simulate realistic human typing noise based on physical keyboard ergonomics:
* **Physics-Based Geometry**: Evaluates Euclidean key distance $d = \sqrt{(\Delta x)^2 + (\Delta y)^2}$ between keys on a 104-key ANSI QWERTY layout with exponential neighbor strike decay:
  $$P(\text{key}' \mid \text{key}) \propto \exp(-\alpha \cdot d)$$
* **The 6 Biomechanical Error Classes**:
  1. **`substitution`** (35%): Hitting a neighboring key (`d` $\to$ `s`, `e`, `r`, `f`, `x`, `c`).
  2. **`transposition`** (20%): Rapid timing slip swapping adjacent letters (`the` $\to$ `teh`).
  3. **`omission`** (15%): Dropped or under-pressed keystroke (`industry` $\to$ `idustry`). Words $\le 3$ chars are immune.
  4. **`insertion`** (15%): Double-contact adjacent stray strike (`recipe` $\to$ `recxipe`).
  5. **`duplication`** (10%): Key bounce double strike repeating a character (`coffee` $\to$ `coffeey`).
  6. **`temporal`** (5%): Finger reach trajectory transition error between sequential keystrokes.
* **Semantic Anchor Immunity**: Named entities (`NNP`/`NNPS`), numbers, prices, URLs, code tokens, short tokens, and CLI flags are strictly protected from mutation.
* **Usage**:
  ```powershell
  # Apply subtle 3% realistic typing noise with deterministic reproducibility:
  .\lexicon_stripper.exe -f input.txt -o output.txt --typing-noise 0.03 --typing-seed 1337 --report
  ```

---

## <img src="../assets/icons/standards.svg" width="20" height="20" align="absmiddle" alt="CLI Flags" /> Full CLI Reference & Command Flags

### Input / Output Options
| Flag | Parameter | Description |
|:---|:---|:---|
| `-f, --file` | `<PATH>` | Path to input text file. |
| `-o, --output` | `<PATH>` | Path to write transformed text file. |
| `-r, --report` | *(Flag)* | Prints formatted empirical report to stderr. |
| `-d, --diff` | *(Flag)* | Shows colorized terminal diff of replacements. |
| `-j, --json` | *(Flag)* | Outputs structured metrics in JSON format. |
| `-l, --log` | `<PATH>` | Writes full transformation audit log to JSON. |
| `-i, --interactive`| *(Flag)* | Launches interactive REPL terminal session. |

### Core Transformation Controls
| Flag | Parameter | Default | Description |
|:---|:---|:---|:---|
| `-m, --mode` | `neutral` \| `human` \| `ai` \| `random` | `human` | Token frequency sampling distribution. |
| `--domain` | `auto` or any domain | `auto` | Specialized linguistic domain. |
| `--strategy` | `baseline` \| `uniform` \| `context-neutral` \| `efficiency-guided` \| `conservative-efficiency` | `conservative-efficiency` | Candidate selection strategy. |
| `-p, --prob` | `0.0 .. 1.0` | `0.30` | Replacement probability for eligible words. |
| `-c, --min-confidence` | `0.0 .. 1.0` | `0.55` | Minimum semantic compatibility threshold. |
| `-s, --seed` | `u64` | `None` | Deterministic RNG master seed. |
| `-n, --passes` | `usize` | `1` | Number of transformation passes. |

### Optional Enhancements & Typing Noise
| Flag | Parameter | Default | Description |
|:---|:---|:---|:---|
| `--typing-noise` | `0.0 .. 1.0` | `0.0` | Probability of corrupting an eligible word. Recommended: `0.02` to `0.05`. |
| `--typing-seed` | `u64` | `None` | Deterministic seed for typing noise. |
| `--typing-errors` | `sub,trans,omit,ins,dup,temp` | *All* | Comma-separated list of enabled error classes. |
| `--syntax` | *(Flag)* | `false` | Enables conservative syntactic restructuring. |
| `--cadence` | *(Flag)* | `false` | Enables sentence pattern & cadence variation. |
| `--function-words` | *(Flag)* | `false` | Enables discourse connector alternatives. |
| `--combined` | *(Flag)* | `false` | Enables all layers (Lexical + Syntax + Cadence + Function Words). |

### Watermark Analysis & Lab Tools
| Flag | Parameter | Description |
|:---|:---|:---|
| `--synthid-detect` | *(Flag)* | Performs standalone SynthID watermark scan on input text. |
| `--synthid-key` | `u64` | Secret seed key for SynthID detector (default: `428917492`). |
| `--synthid-k` | `usize` | Context window length $k$ (default: `2`). |
| `--kirchenbauer-detect` | *(Flag)* | Performs standalone Kirchenbauer red/green watermark scan. |
| `--ablation` | *(Flag)* | Executes an 8-way controlled ablation matrix (Runs A through H). |
| `--analyze-sensitivity` | *(Flag)* | Generates token-level empirical sensitivity profiles. |
| `--save-heatmap` | `<PATH>` | Exports generated sensitivity heatmap JSON model. |

---

## <img src="../assets/icons/microscope.svg" width="20" height="20" align="absmiddle" alt="DeepMind Parity" /> Official Google DeepMind SynthID Parity & Testing

**LexiconStripper** is directly benchmarked and mathematically validated against **Google DeepMind's official SynthID Text repository** (`google-deepmind/synthid-text`).

### 1:1 Algorithmic Implementation
The engine includes a 1:1 Rust implementation of the official Python modules:

| Component | Official DeepMind Module | Rust Engine Implementation |
|:---|:---|:---|
| **LCG Hashing** | `hashing_function.py` | `accumulate_hash` using Knuth 64-bit Linear Congruential Generator (`multiplier = 6364136223846793005`, `increment = 1`). |
| **Hash IV Generation** | `logits_processing.py` | SHA-256 digest of depth keys converted to 64-bit signed modulo initialization vector (`hash_iv`). |
| **G-Value Extraction** | `logits_processing.py` | 12-round right-shift key mixing with bitwise extraction: `(key_hash >> 30) % 2`. |
| **Tournament Sampling**| `logits_processing.py` | 8 depth layers using official default keys: `[654, 400, 336, 679, 700, 901, 12, 444]`. |
| **Hypothesis Testing** | `detector_mean.py` | Standardized $Z$-score and $p$-value hypothesis testing against null baseline $\mu = 0.5000$. |

### Running SynthID Tests from the CLI
```powershell
# 1. Standalone SynthID watermark scan on any text:
.\lexicon_stripper.exe -f my_text.txt --synthid-detect

# 2. Transform text and inspect live SynthID Z-score & p-value in report:
.\lexicon_stripper.exe -f input.txt -o output.txt --report

# 3. Watermark text using official SynthID tournament sampling:
.\lexicon_stripper.exe "Text to watermark" --synthid-watermark --synthid-key 428917492
```

---

## <img src="../assets/icons/target.svg" width="20" height="20" align="absmiddle" alt="Report" /> Interpreting the Transformation Report

When running with `--report`, the engine outputs a detailed empirical audit:

* **Observed Mean G-Value**: Measures alignment with the SynthID tournament key. Unwatermarked text centers at $\mu = 0.5000$.
* **Standardized Z-Score**: Statistical distance from the null baseline. Watermarked texts score $Z \ge 3.00$ ($p < 0.0013$). Unwatermarked texts score $Z < 1.64$ (`NOT_SIGNIFICANT`).
* **Multi-Category JSD (Jensen-Shannon Divergence)**: Information-theoretic divergence from natural human token distributions across Content Words, Nouns, Verbs, Adjectives, Adverbs, and Function Words. Scores $< 0.05\text{ bits}$ indicate near-zero detectable perturbation.
* **Domain-Locked Count**: Number of domain-specific jargon terms strictly protected from corruption.
* **Lexical Turnover**: Percentage of unique content words safely varied.

---

## <img src="../assets/icons/flows.svg" width="20" height="20" align="absmiddle" alt="Requirements" /> System Requirements

* **Operating System**: Microsoft Windows 10 / 11 / Server (x86_64).
* **RAM**: $< 25\text{ MB}$ footprint during execution.
* **Storage**: Single ~6 MB file (`lexicon_stripper.exe`).
* **Setup**: Zero installation required. Simply run the executable from any terminal.

---

## <img src="../assets/icons/scale.svg" width="20" height="20" align="absmiddle" alt="License" /> License

SynthIDStripper is licensed under the [**MIT License**](../LICENSE).

Author: **Kalem Slight** (`kalemslight@gmail.com`) — Repository: [`https://github.com/Kryklin/SynthIDStripper`](https://github.com/Kryklin/SynthIDStripper).

---
