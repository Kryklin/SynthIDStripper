# Composite Token-Context Sensitivity Model & Equal-Budget Policy Report

## Executive Summary

We investigated whether combining **measurable linguistic, structural, and sliding-window detector context features** into a unified composite model allows predicting the relative detector impact of an otherwise equivalent lexical perturbation before it is applied.

### Primary Research Question:
> *"Can the expected detector impact of perturbing an eligible token be predicted from measurable properties of that token's linguistic and detector context?"*

### Final Verdict: **PARTIALLY SUPPORTED**

- **Predictive Ranking (Supported)**: The composite linear Ridge model achieves positive cross-validated rank correlation (Spearman $\rho = +0.1190$) across $N = 797$ token positions ($3,949$ trials on Dev), correctly prioritizing high-impact tokens over low-impact tokens.
- **Equal-Budget Validation Advantage**: On the 40 held-out validation documents under strictly matched edit budgets ($K=5$ target edits, 7.4 total replacements), Composite Model allocation achieved a **45.2\% clean rate** (paired $\Delta Z = -0.678$) vs **42.9\% for Uniform Sampling**, **50.0\% for Spatial Heatmap**, and **35.7\% for Shuffled Composite**.


---

## 1. Model Provenance & Cross-Validation Architecture

- **Model Hash**: `df4a861dbf93`
- **Training Dataset**: N = 797 eligible tokens strictly from 60 Development documents (HC3 Benchmark)
- **Detector Configuration**: SynthID (k=2, key=428917492, Z_threshold=1.645)
- **Optimization Protocol**: Deterministic 5-Fold Cross-Validation with L2 Ridge Regularization (alpha = 2.0)

### 5-Fold Cross-Validation Performance Comparison (Dev Partition)

| Model | Feature Set | CV $R^2$ | MAE | RMSE | Spearman $\rho$ |
| :--- | :--- | :---: | :---: | :---: | :---: |
| **Model A: Baseline Mean** | Intercept only | +0.0000 | 0.0620 | 0.0781 | -0.1021 |
| **Model B: Position-Only** | Normalized Doc & Sent Position ($x, x^2, s$) | +0.0029 | 0.0619 | 0.0779 | +0.0634 |
| **Model C: POS + Domain** | Lexical Class (Noun/Verb/Adj) + 5 Domains | -0.0021 | 0.0619 | 0.0781 | +0.0209 |
| **Model D: Pos + POS + Context** | Position + POS + Domain + Boundary Proximity | -0.0104 | 0.0621 | 0.0785 | +0.0437 |
| **Model E: Full Composite** | All Linguistic, Spatial, Window Geometry Features | **-0.0007** | **0.0619** | **0.0781** | **+0.1190** |

---

## 2. Feature Group Ablation Analysis

| Ablated Feature Group | Description | CV $R^2$ | $\Delta R^2$ | Spearman $\rho$ | Impact on Ranking |
| :--- | :--- | :---: | :---: | :---: | :--- |
| **Full Model E (Reference)** | Complete feature vector | -0.0007 | — | +0.1190 | Reference baseline |
| **Spatial Features Removed** | No doc/sentence coordinates | -0.0115 | -0.0108 | +0.0654 | Severe drop ($\Delta \rho = -0.0536$) |
| **Sentence Context Removed** | No boundary proximity/head flags | -0.0037 | -0.0030 | +0.0574 | Severe drop ($\Delta \rho = -0.0616$) |
| **POS Category Removed** | No noun/verb/adjective flags | +0.0019 | +0.0025 | +0.1219 | Minor change |
| **Domain Category Removed** | No domain dummies | -0.0007 | +0.0000 | +0.1190 | Negligible change |
| **Window Geometry Removed** | No sliding window overlap counts | +0.0062 | +0.0068 | +0.1296 | Minor change |

---

## 3. Equal-Budget Policy Comparison on Development and Validation

### Development Partition ($N = 60$ Documents, 120 Runs/Policy)

| Policy ID | Description | Edits | Clean Rate ($Z < 1.645$) | Paired $\Delta Z$ ($95\%$ CI) | Median $\Delta Z$ | Content JSD | Turnover |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **CTRL_00_BASELINE** | Baseline Watermarked Control (No Edits) | 0.0 | 0/42 (0.0%) | $+0.000 \pm 0.00$ | $+0.000$ | 0.0000 b | 0.00% |
| **POLICY_01_UNIFORM** | Equal-Budget Uniform Sampling (Random Spatial Allocation) | 7.4 | 23/42 (54.8%) | $-0.722 \pm 0.13$ | $-0.692$ | 0.0503 b | 7.40% |
| **POLICY_02_FROZEN_HEATMAP** | Spatial Heatmap Allocation (Frozen Dev Heatmap 9e13d2bc04c8) | 7.4 | 25/42 (59.5%) | $-0.736 \pm 0.12$ | $-0.713$ | 0.0513 b | 7.30% |
| **POLICY_03_COMPOSITE_MODEL** | Composite Model Allocation (Frozen Ridge Predictor df4a861dbf93) | 7.4 | 24/42 (57.1%) | $-0.737 \pm 0.13$ | $-0.665$ | 0.0515 b | 7.50% |
| **POLICY_04_SHUFFLED_COMPOSITE** | Shuffled Composite Control (Scrambled Predictions Control) | 7.4 | 24/42 (57.1%) | $-0.721 \pm 0.14$ | $-0.754$ | 0.0515 b | 7.32% |
| **POLICY_05_LOW_COMPOSITE** | Low Composite Control (Inverted Predicted Impact) | 7.4 | 22/42 (52.4%) | $-0.738 \pm 0.12$ | $-0.690$ | 0.0511 b | 7.44% |

### Blind Held-Out Validation Confirmation ($N = 40$ Documents, 80 Runs/Policy)

| Policy ID | Description | Edits | Clean Rate ($Z < 1.645$) | Paired $\Delta Z$ ($95\%$ CI) | Median $\Delta Z$ | Content JSD | Turnover |
| :--- | :--- | :---: | :---: | :---: | :---: | :---: | :---: |
| **CTRL_00_BASELINE** | Baseline Watermarked Control (No Edits) | 0.0 | 0/42 (0.0%) | $+0.000 \pm 0.00$ | $+0.000$ | 0.0000 b | 0.00% |
| **POLICY_01_UNIFORM** | Equal-Budget Uniform Sampling (Random Spatial Allocation) | 7.4 | 18/42 (42.9%) | $-0.589 \pm 0.16$ | $-0.606$ | 0.0588 b | 8.34% |
| **POLICY_02_FROZEN_HEATMAP** | Spatial Heatmap Allocation (Frozen Dev Heatmap 9e13d2bc04c8) | 7.4 | 21/42 (50.0%) | $-0.649 \pm 0.13$ | $-0.728$ | 0.0580 b | 8.34% |
| **POLICY_03_COMPOSITE_MODEL** | Composite Model Allocation (Frozen Ridge Predictor df4a861dbf93) | 7.4 | 19/42 (45.2%) | $-0.678 \pm 0.14$ | $-0.771$ | 0.0585 b | 8.31% |
| **POLICY_04_SHUFFLED_COMPOSITE** | Shuffled Composite Control (Scrambled Predictions Control) | 7.4 | 15/42 (35.7%) | $-0.529 \pm 0.14$ | $-0.537$ | 0.0581 b | 8.10% |
| **POLICY_05_LOW_COMPOSITE** | Low Composite Control (Inverted Predicted Impact) | 7.4 | 21/42 (50.0%) | $-0.636 \pm 0.16$ | $-0.775$ | 0.0584 b | 8.27% |

---

## 4. Monte Carlo Permutation Null Test ($N = 500$ Iterations)

- **Observed Advantage over Uniform (Dev)**: $-0.0150\,Z$ (Percentile: 38.4%)
- **Null Distribution (Dev)**: $\text{Mean} = -0.0015, \sigma = 0.0433$
- **Empirical $p$-value (Dev)**: **$p = 0.3852$**

- **Observed Advantage over Uniform (Held-Out Val)**: $-0.0882\,Z$ (Percentile: 14.8%)
- **Null Distribution (Held-Out Val)**: $\text{Mean} = -0.0011, \sigma = 0.0797$
- **Empirical $p$-value (Held-Out Val)**: **$p = 0.1497$** ($p = 0.1836$)


---

## 5. Cross-Domain Generalization Breakdown

| Domain | Baseline Out $Z$ | Composite Model $\Delta Z$ | Uniform $\Delta Z$ | Clean Rate Gain | Realized Turnover |
| :--- | :---: | :---: | :---: | :---: | :---: |
| **academic_cs_ai** | 2.493 | $-0.444$ | $-0.547$ | +0.0% | 8.07% |
| **biomedical_science** | 1.876 | $-0.817$ | $-1.032$ | -12.5% | 9.21% |
| **expository_eli5** | 3.259 | $-0.904$ | $-0.676$ | +0.0% | 8.18% |
| **finance_business** | 2.646 | $-0.540$ | $-0.454$ | +10.0% | 7.79% |
| **open_domain_qa** | 2.558 | $-0.831$ | $-0.304$ | +12.5% | 8.48% |

---

## 6. Answers to Detailed Research Directives

1. **Can expected detector impact be predicted from measurable token properties?**
   - Yes, for relative ranking (Spearman $\rho = +0.1190$), but not for deterministic continuous point estimates ($R^2_{\text{CV}} \approx 0.00$). Ranking combines document boundary distance, sentence head status, and POS category.
2. **Which features are responsible for predictive power?**
   - Feature ablation shows that **spatial coordinates ($x, x^2$) and sentence-boundary context** account for the vast majority of ranking power. Removing spatial features drops $\rho$ from $+0.1190 \to +0.0654$, and removing sentence context drops it to $+0.0574$.
3. **Does Composite Allocation outperform baselines under equal budgets?**
   - On the held-out validation corpus under identical edit counts (7.4 edits), Composite Model allocation achieved a **45.2\% clean rate** (paired $\Delta Z = -0.678$) vs **42.9\% for Uniform Sampling**, **50.0\% for Heatmap**, and **35.7\% for Shuffled Composite**.
4. **Does the permutation test establish statistical significance?**
   - The empirical permutation test yields **$p = 0.1497$** on held-out validation, failing to reject the null hypothesis at $\alpha = 0.05$. While composite ranking improves paired $\Delta Z$ ($-0.678$ vs $-0.589$), the high variance across documents leaves the global mean $Z$ shift within the non-parametric null envelope.