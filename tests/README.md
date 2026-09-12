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

[<img src="../assets/icons/home.svg" width="13" height="13" align="absmiddle" alt="Main" /> Main](../README.md) | [<img src="../assets/icons/rocket.svg" width="13" height="13" align="absmiddle" alt="Distribution" /> Distribution](../dist/README.md) | [<img src="../assets/icons/tools.svg" width="13" height="13" align="absmiddle" alt="Benchmarks" /> Benchmarks](../benchmarks/README.md) | [<img src="../assets/icons/microscope.svg" width="13" height="13" align="absmiddle" alt="Data Hub" /> Data Hub](../data/README.md) | [<img src="../assets/icons/shield.svg" width="13" height="13" align="absmiddle" alt="Tests" /> Tests](README.md)

</div>

# SynthIDStripper: Integration & Regression Test Suite

<p align="left">
  <img src="https://img.shields.io/badge/Total_Tests-61_Passing-brightgreen?style=for-the-badge&logo=rust&logoColor=white" alt="Tests">
  <img src="https://img.shields.io/badge/Test_Suites-17_Modules-blue?style=for-the-badge&logo=codecov&logoColor=white" alt="Suites">
  <img src="https://img.shields.io/badge/Parity-1%3A1_DeepMind-orange?style=for-the-badge" alt="Parity">
  <img src="https://img.shields.io/badge/Deterministic-100%25-green?style=for-the-badge" alt="Deterministic">
</p>

The `tests/` directory houses the integration test harness for **SynthIDStripper (LexiconStripper)**. The suite validates mathematical parity against Google DeepMind's reference Python algorithms, verifies end-to-end watermark stripping, tests biomechanical keyboard error models, and enforces strict terminology locking across all 29 specialized linguistic domains.

> [!IMPORTANT]
> **Continuous Verification Requirement**:
> All 61 automated tests run in parallel with `cargo test`. Parity tests must achieve 100% mathematical bit-identity with official DeepMind reference values before any release artifact is cut.

---

## <img src="../assets/icons/shield.svg" width="20" height="20" align="absmiddle" alt="Test Suites" /> Test Suite Coverage Matrix

| Test Suite File | Subsystem Verified | Key Verification Properties |
| :--- | :--- | :--- |
| **`official_deepmind_parity_tests.rs`** | *DeepMind Algorithm Parity* | Knuth 64-bit LCG streaming associativity, SHA-256 depth-key IV mapping, tournament $G$-value extraction. |
| **`synthid_tests.rs`** | *End-to-End Watermark Pipeline* | Synthesizes watermarked sequences ($Z \ge 1.645$), executes neutralization, and asserts $Z < 1.64$. |
| **`detector_tests.rs`** | *Statistical Detectors* | SynthID and Kirchenbauer detector boundary checks, null hypothesis distributions, and $p$-value calibration. |
| **`domain_expansion_tests.rs`** | *29 Specialized Registries* | Jargon lock assertions, multi-word phrase isolation, and automatic domain classifier precision. |
| **`domain_tests.rs`** | *Domain Registry Integrity* | Collision-free domain registration, density thresholds, and fallback mechanics. |
| **`terminology_tests.rs`** | *IATE / EuroVoc Databases* | Longest-Match-First atomic locking for compound legal, medical, maritime, and aerospace terms. |
| **`typing_tests.rs`** | *Keyboard Physics Engine* | 104-key ANSI QWERTY Euclidean distances, exponential decay probabilities, and 6 error classes. |
| **`fluency_tests.rs`** | *Linguistic Fluency & Valency* | Verb valency matching, prepositional collocation constraints, and grammatical agreement. |
| **`inflection_tests.rs`** | *Morphology & Inflection* | Penn Treebank tag preservation across pluralization, past participles, and gerunds. |
| **`distribution_tests.rs`** | *Frequency Sampling* | Human Zipf distributions, neutral sampling, and Jensen-Shannon Divergence bounds. |
| **`ner_tests.rs`** | *Named Entity Recognition* | Capitalization anchors, proper nouns (`NNP`/`NNPS`), and acronym immunity. |
| **`sensitivity_tests.rs`** | *Sensitivity Heatmaps* | Token-level perturbation response gradients and ablation ranking consistency. |
| **`cli_tests.rs`** | *CLI Routing & User Experience* | Topic router (`help typos`, `help domains`, `help watermark`), flag parsing, and diff rendering. |
| **`engine_tests.rs`** | *Core Resampling Pipeline* | Multi-pass stability, RNG determinism, and candidate confidence thresholds. |
| **`framework_tests.rs`** | *System Orchestration* | End-to-end integration between tokenizer, domain registry, resampler, and reporter. |

---

## <img src="../assets/icons/terminal.svg" width="20" height="20" align="absmiddle" alt="Execution" /> Running the Test Harness

### Execute Full Test Suite
```bash
cargo test
```

### Run Official DeepMind Parity Tests
```bash
cargo test --test official_deepmind_parity_tests -- --nocapture
```

### Run End-to-End Watermark Stripping Tests
```bash
cargo test --test synthid_tests -- --nocapture
```

### Run Domain Jargon Protection Tests
```bash
cargo test --test domain_expansion_tests
```

### Run Biomechanical Typing Simulator Tests
```bash
cargo test --test typing_tests
```

---

## <img src="../assets/icons/scale.svg" width="20" height="20" align="absmiddle" alt="License" /> License

The test harness and integration suites are licensed under the [**MIT License**](../LICENSE).

Author: **Kalem Slight** (`kalemslight@gmail.com`) — Repository: [`https://github.com/Kryklin/SynthIDStripper`](https://github.com/Kryklin/SynthIDStripper).

---
