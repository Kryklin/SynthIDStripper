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

[<img src="../assets/icons/home.svg" width="13" height="13" align="absmiddle" alt="Main" /> Main](../README.md) | [<img src="../assets/icons/rocket.svg" width="13" height="13" align="absmiddle" alt="Distribution" /> Distribution](../dist/README.md) | [<img src="../assets/icons/tools.svg" width="13" height="13" align="absmiddle" alt="Benchmarks" /> Benchmarks](README.md) | [<img src="../assets/icons/microscope.svg" width="13" height="13" align="absmiddle" alt="Data Hub" /> Data Hub](../data/README.md) | [<img src="../assets/icons/shield.svg" width="13" height="13" align="absmiddle" alt="Tests" /> Tests](../tests/README.md)

</div>

# SynthIDStripper: Empirical Benchmarking Suite & Laboratory Testbed

<p align="left">
  <img src="https://img.shields.io/badge/Python-3.10%2B-3776AB?style=for-the-badge&logo=python&logoColor=white" alt="Python">
  <img src="https://img.shields.io/badge/Evaluation_Suites-25_Pipelines-blue?style=for-the-badge&logo=pytest&logoColor=white" alt="Suites">
  <img src="https://img.shields.io/badge/Cross--Validation-5--Fold-orange?style=for-the-badge" alt="CV">
  <img src="https://img.shields.io/badge/Detectors-SynthID_%26_Kirchenbauer-brightgreen?style=for-the-badge" alt="Detectors">
</p>

The `benchmarks/` directory contains the automated empirical evaluation framework for **SynthIDStripper (LexiconStripper)**. It provides end-to-end pipelines to verify 1:1 mathematical parity with DeepMind's official algorithm, profile token-level sensitivity landscapes, evaluate cross-detector transferability, and conduct ablation experiments.

> [!IMPORTANT]
> **Reproducibility & Testbed Requirements**:
> All benchmarks execute against the native release binary (`target/release/lexicon_stripper.exe`). Ensure the Rust engine is compiled with `--release` before launching Python evaluation pipelines.

---

## <img src="../assets/icons/tools.svg" width="20" height="20" align="absmiddle" alt="Pipelines" /> Benchmark Suite Catalog

The laboratory testbed is organized into specialized experimental modules:

| Script Name | Category | Primary Focus | Output Artifact |
| :--- | :--- | :--- | :--- |
| **`verify_official_parity.py`** | *Parity* | Validates Knuth LCG streaming hash equivalence with DeepMind's Python reference. | Parity verification summary. |
| **`eval_synthid.py`** | *Detection* | Standardized SynthID hypothesis evaluation ($Z$-score, two-tailed $p$-value). | Statistical evaluation report. |
| **`run_comparative_sensitivity_study.py`** | *Sensitivity* | 5-fold cross-validated regression models evaluating spatial & contextual features. | `data/comparative_sensitivity_report.md` |
| **`run_composite_policy_benchmark.py`** | *Allocation* | Evaluates Model 6 composite ranking against uniform and shuffled controls. | `data/composite_model_report.md` |
| **`run_multi_detector_benchmark.py`** | *Cross-Detector* | Tests transferability against SynthID, Kirchenbauer, and entropy detectors. | `data/multi_detector_benchmark_report.md` |
| **`run_large_scale_robustness.py`** | *Robustness* | Stress-tests stripping efficiency across large multi-genre text corpora. | `data/large_scale_robustness_report.md` |
| **`run_spatial_sensitivity_study.py`** | *Spatial* | Measures positional sensitivity decay and token context boundaries. | `data/spatial_sensitivity_report.md` |
| **`run_spatial_null_experiment.py`** | *Hypothesis* | Validates spatial feature significance against randomized null distributions. | `data/spatial_confounder_report.md` |
| **`run_ablation_study.py`** | *Ablation* | Runs 8-way controlled ablation matrix across discrete system layers. | `data/sensitivity_report.md` |
| **`run_terminology_benchmark.py`** | *Terminology* | Verifies 100% preservation of multi-word IATE terms and domain jargon. | Jargon preservation telemetry. |
| **`run_parameter_sweep.py`** | *Optimization* | Sweeps confidence thresholds ($c \in [0.4, 0.9]$) and replacement probabilities. | Parameter Pareto front. |
| **`download_corpus.py`** | *Data* | Downloads and preprocesses standardized evaluation text corpora. | `benchmarks/corpus/` |

---

## <img src="../assets/icons/rocket.svg" width="20" height="20" align="absmiddle" alt="Quick Start" /> Getting Started

### 1. Prerequisites & Environment Setup

The benchmarking pipelines require Python 3.10+ and standard numerical computing packages:

```bash
cd benchmarks
python -m venv venv
source venv/bin/activate  # On Windows: .\venv\Scripts\Activate.ps1
pip install numpy scipy scikit-learn matplotlib
```

### 2. Compile Release Binary

Compile the optimized native Rust binary:

```bash
cargo build --release
```

### 3. Verify Official DeepMind Parity

Execute the standalone parity verification check:

```bash
python verify_official_parity.py
```

Expected output:
```
[INFO] Verifying Knuth 64-bit LCG Hashing... PARITY CONFIRMED
[INFO] Verifying SHA-256 Depth Key IV Mapping... PARITY CONFIRMED
[INFO] Verifying 8-Round Tournament G-Value Generation... PARITY CONFIRMED
[INFO] Mathematical Equivalence Verified: 100.0% match across 50,000 token spans.
```

---

## <img src="../assets/icons/microscope.svg" width="20" height="20" align="absmiddle" alt="Methodology" /> Benchmark Methodology & Evaluation Metrics

Telemetry is recorded across discrete statistical and information-theoretic axes:

* **Standardized $Z$-Score**: Statistical distance from the unwatermarked null hypothesis ($\mu = 0.5000$). Watermarked text scores $Z \ge 3.00$; unwatermarked baseline scores $Z < 1.64$ (`NOT_SIGNIFICANT`).
* **Jensen-Shannon Divergence (JSD)**: Multi-category divergence measuring lexical perturbation across 6 POS bins. Lower scores ($< 0.05\text{ bits}$) confirm human-like distributional fidelity.
* **Spearman Rank Correlation ($\rho$)**: Evaluates predictive accuracy of token sensitivity regression models. Spatial and composite models achieve $\rho = +0.1190$ ($p < 10^{-4}$).
* **Clean Rate**: Proportion of watermarked documents successfully reduced below the $Z < 1.645$ detection threshold.

> [!NOTE]
> Detailed numerical logs, cross-validation splits, and ablation matrices generated by these benchmarks are preserved in the [`data/`](../data/README.md) hub.

---

## <img src="../assets/icons/scale.svg" width="20" height="20" align="absmiddle" alt="License" /> License

Benchmark scripts and test configurations are licensed under the [**MIT License**](../LICENSE).

Author: **Kalem Slight** (`kalemslight@gmail.com`) — Repository: [`https://github.com/Kryklin/SynthIDStripper`](https://github.com/Kryklin/SynthIDStripper).

---
