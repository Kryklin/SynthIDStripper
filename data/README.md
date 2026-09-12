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

[<img src="../assets/icons/home.svg" width="13" height="13" align="absmiddle" alt="Main" /> Main](../README.md) | [<img src="../assets/icons/rocket.svg" width="13" height="13" align="absmiddle" alt="Distribution" /> Distribution](../dist/README.md) | [<img src="../assets/icons/tools.svg" width="13" height="13" align="absmiddle" alt="Benchmarks" /> Benchmarks](../benchmarks/README.md) | [<img src="../assets/icons/microscope.svg" width="13" height="13" align="absmiddle" alt="Data Hub" /> Data Hub](README.md) | [<img src="../assets/icons/shield.svg" width="13" height="13" align="absmiddle" alt="Tests" /> Tests](../tests/README.md)

</div>

# SynthIDStripper: Empirical Research Reports & Experimental Datasets

<p align="left">
  <img src="https://img.shields.io/badge/Datasets-38_Artifacts-007ACC?style=for-the-badge&logo=databricks&logoColor=white" alt="Datasets">
  <img src="https://img.shields.io/badge/Tokens_Profiled-50%2C000%2B-blue?style=for-the-badge&logo=scipy&logoColor=white" alt="Tokens">
  <img src="https://img.shields.io/badge/Cross--Validation-5--Fold-orange?style=for-the-badge&logo=scikitlearn&logoColor=white" alt="CV">
  <img src="https://img.shields.io/badge/Format-CSV_%2F_JSON_%2F_MD-brightgreen?style=for-the-badge&logo=markdown&logoColor=white" alt="Formats">
</p>

The `data/` hub archives empirical research findings, 5-fold cross-validated regression models, token sensitivity heatmaps, and ablation benchmarks generated during the validation of **SynthIDStripper (LexiconStripper)**.

> [!IMPORTANT]
> **Data Provenance & Experimental Rigor**:
> All experiments were conducted under controlled perturbation budgets using standardized null permutation tests and cross-validated against both **Google DeepMind SynthID Text** and **Kirchenbauer et al.** statistical detectors.

---

## <img src="../assets/icons/book.svg" width="20" height="20" align="absmiddle" alt="Reports Index" /> Empirical Research Reports Index

Detailed academic findings and statistical analyses are organized in the following peer-reviewable reports:

| Report Document | Focus Area | Key Metric / Result | Empirical Finding |
| :--- | :--- | :--- | :--- |
| [**Comparative Sensitivity Landscape Study**](comparative_sensitivity_report.md) | Feature Hierarchy & Spatial Modeling | Full Composite $\rho = +0.1190$ ($p < 10^{-4}$) | Contextual and spatial features significantly outperform token-level self-information alone. |
| [**Composite Model Validation Report**](composite_model_report.md) | Fixed-Budget Perturbation Allocation | Medium Budget Clean Rate: **$50.0\%$** vs $42.9\%$ | Model 6 composite ranking achieves highest stripping efficiency per edit. |
| [**Multi-Detector Benchmark Report**](multi_detector_benchmark_report.md) | Cross-Detector Generalization | Kirchenbauer Clean Rate: **$61.9\%$** | Spatial guidance transfers out-of-sample without access to detector internal state. |
| [**Large-Scale Robustness Report**](large_scale_robustness_report.md) | Corpus-Wide Stress Testing | Multi-Domain Clean Rate: **$100.0\%$** | Robust stripping confirmed across 29 specialized technical and conversational registers. |
| [**Spatial Confounder & Null Study**](spatial_confounder_report.md) | Confounder Analysis & Null Tests | Permutation $p < 0.001$ | Validates that spatial effects are structural linguistic properties, not statistical artifacts. |
| [**Spatial Sensitivity Study**](spatial_sensitivity_report.md) | Positional Sensitivity Gradients | Decay Half-Life: **$k=2$ tokens** | Establishes positional sensitivity bounds across sentence-initial and sentence-final boundaries. |
| [**Ablation Study Report**](sensitivity_report.md) | Subsystem Ablation Matrix | 8-Way Factorial Matrix | Identifies individual variance contributions of synsets, valency guards, and typing noise. |

---

## <img src="../assets/icons/microscope.svg" width="20" height="20" align="absmiddle" alt="Model Hierarchy" /> Incremental Model Hierarchy (5-Fold Cross-Validation)

Summary of cross-validated predictive power across the 6 incremental sensitivity models:

| Model ID | Model Description | Covariates Included | CV $R^2$ | CV MAE | Spearman $\rho$ | $\Delta \rho$ vs Model 1 |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: |
| **`Model 0`** | Token-Only Baseline | 4 features | $+0.0052$ | 0.0942 | **$+0.0633$** | $+0.0070$ |
| **`Model 1`** | Token + Self-Information | 6 features | $+0.0035$ | 0.0943 | **$+0.0563$** | Baseline |
| **`Model 2`** | Token + Spatial Features | 9 features | $+0.0014$ | 0.0943 | **$+0.0976$** | **$+0.0412$** |
| **`Model 3`** | Token + Contextual Features | 7 features | $+0.0023$ | 0.0942 | **$+0.0811$** | **$+0.0247$** |
| **`Model 4`** | Token + Self-Info + Spatial | 9 features | $+0.0041$ | 0.0941 | **$+0.1059$** | **$+0.0495$** |
| **`Model 5`** | Token + Self-Info + Context | 8 features | $+0.0023$ | 0.0942 | **$+0.0806$** | **$+0.0243$** |
| **`Model 6`** | Full Composite Model | 18 features | $-0.0057$ | 0.0945 | **$+0.1190$** | **$+0.0627$** |

> [!NOTE]
> **Key Finding**: Integrating spatial coordinates (`pos_norm`, `sent_relative_pos`) and $n$-gram window overlap features doubles ranking correlation relative to self-information alone ($\rho = 0.0563 \to 0.1190$).

---

## <img src="../assets/icons/tools.svg" width="20" height="20" align="absmiddle" alt="Data Catalog" /> Dataset & Artifact Catalog

The following structured data artifacts are available for secondary analysis:

| Filename | Format | Description |
| :--- | :--- | :--- |
| **`comparative_sensitivity.csv`** | CSV (215 KB) | Token-level observations with self-information, spatial metrics, and detector response deltas. |
| **`comparative_sensitivity_raw.json`** | JSON (752 KB) | Complete uncompressed feature vectors and cross-validation fold assignments. |
| **`comparative_model_coefficients.csv`** | CSV (1 KB) | OLS and Ridge regression weights across Models 0 through 6. |
| **`comparative_budget_results.csv`** | CSV (24 KB) | Fixed-budget perturbation curves across budget levels $K \in \{1, 3, 5, 8, 12\}$. |
| **`composite_model.json`** | JSON (4 KB) | Exported production weights for the composite token-ranking policy. |
| **`composite_validation_results.csv`** | CSV (3 KB) | Out-of-sample validation runs comparing Model 6 against baseline strategies. |
| **`confounder_regression_tokens.csv`** | CSV (174 KB) | Controls for word frequency, sentence length, and POS distribution confounders. |
| **`domain_stratified_robustness.csv`** | CSV (21 KB) | Stripping efficiency stratified across all 29 specialized linguistic domains. |
| **`multi_detector_benchmark_raw.json`** | JSON (1.5 MB) | Raw evaluation metrics across SynthID, Kirchenbauer, and entropy detectors. |
| **`permutation_null_distribution.csv`** | CSV (23 KB) | 1,000-iteration Monte Carlo null distribution samples. |
| **`sensitivity_heatmap.json`** | JSON (632 KB) | Token-by-token sensitivity profiles exported for visual UI inspection. |
| **`spatial_bins_50.csv`** | CSV (12 KB) | 50-quantile spatial binned sensitivity profile across relative document position. |

---

## <img src="../assets/icons/scale.svg" width="20" height="20" align="absmiddle" alt="License" /> License

Research reports and experimental datasets are licensed under the [**MIT License**](../LICENSE).

Author: **Kalem Slight** (`kalemslight@gmail.com`) — Repository: [`https://github.com/Kryklin/SynthIDStripper`](https://github.com/Kryklin/SynthIDStripper).

---
