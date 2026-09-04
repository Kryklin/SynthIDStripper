#!/usr/bin/env python3
"""
Standardized 3-Tier Classification & Paired Cohort Benchmark Reporter.

Evaluates the benchmark dataset with explicit 3-tier statistical thresholds:
1. Z < 1.645       : Statistically Non-Significant (Clean / Stripped, alpha=0.05)
2. 1.645 <= Z < 3.0: Borderline Trace
3. Z >= 3.0        : Strong Signal (3-sigma alarm)

Reports:
- Full Corpus Distribution (N=100) across all 3 tiers
- Baseline-Detectable Cohort (N=29, Z_baseline >= 1.645) Paired Analysis
- High-Confidence Cohort (Z_baseline >= 3.0) Paired Analysis
- Dialect Turnover Stratification (General vs Domain, Protected vs Replaced)
"""

import os
import sys
import json
import statistics
from collections import defaultdict

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RESULTS_JSON = os.path.join(BASE_DIR, "data", "terminology_benchmark_results.json")

def load_data():
    if not os.path.exists(RESULTS_JSON):
        print(f"Error: Results file not found at {RESULTS_JSON}")
        sys.exit(1)
    with open(RESULTS_JSON, "r", encoding="utf-8") as f:
        return json.load(f)

def classify_z(z):
    if z is None:
        return "UNKNOWN"
    if z < 1.645:
        return "NON_SIGNIFICANT"
    elif z < 3.0:
        return "BORDERLINE"
    else:
        return "STRONG_SIGNAL"

def main():
    raw_records = load_data()
    
    # Group by (domain, sample_id)
    doc_map = defaultdict(dict)
    for r in raw_records:
        key = (r["domain"], r["sample_id"])
        doc_map[key][r["condition"]] = r

    conditions = [
        ("A_baseline", "Baseline (Watermarked)"),
        ("B_wordnet_only", "WordNet Only (Unconstrained)"),
        ("C_terminology_on", "WordNet + IATE/EuroVoc"),
        ("D_terminology_protected", "IATE/EuroVoc (Strict Protection)"),
    ]

    print("=" * 105)
    print("  STANDARDIZED 3-TIER STATISTICAL CLASSIFICATION BENCHMARK (N = 100 Documents)")
    print("=" * 105)
    
    # 1. FULL COHORT 3-TIER DISTRIBUTION TABLE
    print("\n1. FULL COHORT STATISTICAL DISTRIBUTION (N = 100 Documents, 400 Evaluations):")
    print("-" * 105)
    print(f"{'Condition':<32} | {'Mean Z':<8} | {'Z < 1.645 (Clean)':<18} | {'1.645 <= Z < 3':<16} | {'Z >= 3.0 (Strong)':<18} | {'Turnover %':<10}")
    print("-" * 105)

    for cond_key, cond_name in conditions:
        runs = [doc_map[k][cond_key] for k in doc_map if cond_key in doc_map[k]]
        zs = [r["synthid_z"] for r in runs if r["synthid_z"] is not None]
        turnovers = [r["lexical_turnover_pct"] for r in runs]
        
        non_sig = sum(1 for z in zs if z < 1.645)
        borderline = sum(1 for z in zs if 1.645 <= z < 3.0)
        strong = sum(1 for z in zs if z >= 3.0)
        
        mean_z = statistics.mean(zs) if zs else 0.0
        mean_t = statistics.mean(turnovers) if turnovers else 0.0
        
        pct_non_sig = non_sig / len(zs) * 100.0
        pct_border = borderline / len(zs) * 100.0
        pct_strong = strong / len(zs) * 100.0
        
        print(f"{cond_name:<32} | {mean_z:8.3f} | {non_sig:2d} ({pct_non_sig:5.1f}%)       | {borderline:2d} ({pct_border:5.1f}%)     | {strong:2d} ({pct_strong:5.1f}%)        | {mean_t:8.2f}%")
    print("-" * 105)

    # 2. PAIRED BASELINE-DETECTABLE COHORT ANALYSIS (Z_baseline >= 1.645)
    detectable_keys = []
    for k, conds in doc_map.items():
        base = conds.get("A_baseline")
        if base and base["synthid_z"] is not None and base["synthid_z"] >= 1.645:
            detectable_keys.append(k)

    print(f"\n2. PAIRED BASELINE-DETECTABLE COHORT (Z_baseline >= 1.645, N = {len(detectable_keys)} Documents):")
    print("=" * 115)
    print(f"{'Condition':<32} | {'Mean Z':<8} | {'Paired Delta Z':<14} | {'P(Z < 1.645)':<14} | {'Borderline':<12} | {'Strong (>=3)':<14} | {'Turnover %':<10}")
    print("-" * 115)

    for cond_key, cond_name in conditions:
        runs = [doc_map[k][cond_key] for k in detectable_keys if cond_key in doc_map[k]]
        base_runs = [doc_map[k]["A_baseline"] for k in detectable_keys if "A_baseline" in doc_map[k]]
        
        zs = [r["synthid_z"] for r in runs if r["synthid_z"] is not None]
        base_zs = [r["synthid_z"] for r in base_runs if r["synthid_z"] is not None]
        
        delta_zs = [(z - bz) for z, bz in zip(zs, base_zs)]
        turnovers = [r["lexical_turnover_pct"] for r in runs]
        
        clean_count = sum(1 for z in zs if z < 1.645)
        border_count = sum(1 for z in zs if 1.645 <= z < 3.0)
        strong_count = sum(1 for z in zs if z >= 3.0)
        
        mean_z = statistics.mean(zs) if zs else 0.0
        mean_delta_z = statistics.mean(delta_zs) if delta_zs else 0.0
        mean_t = statistics.mean(turnovers) if turnovers else 0.0
        
        p_clean = clean_count / len(zs) * 100.0
        p_border = border_count / len(zs) * 100.0
        p_strong = strong_count / len(zs) * 100.0
        
        delta_str = f"{mean_delta_z:+8.3f}" if cond_key != "A_baseline" else "      —   "
        print(f"{cond_name:<32} | {mean_z:8.3f} | {delta_str:<14} | {clean_count:2d}/{len(zs)} ({p_clean:5.1f}%) | {border_count:2d} ({p_border:5.1f}%) | {strong_count:2d} ({p_strong:5.1f}%)   | {mean_t:8.2f}%")
    print("-" * 115)

    # 3. TERMINOLOGY DENOMINATORS IN THE DETECTABLE COHORT
    print(f"\n3. TERMINOLOGY DIALECT METRICS IN THE BASELINE-DETECTABLE COHORT (N = {len(detectable_keys)} Documents):")
    print("-" * 110)
    print(f"{'Condition':<32} | {'Detected':<9} | {'Eligible':<9} | {'Protected':<10} | {'Variants':<9} | {'Replaced':<9} | {'Gen Turn':<9} | {'Dom Turn':<9}")
    print("-" * 110)

    for cond_key, cond_name in conditions:
        runs = [doc_map[k][cond_key] for k in detectable_keys if cond_key in doc_map[k]]
        mean_det = statistics.mean(r["domain_terms_detected"] for r in runs)
        mean_elig = statistics.mean(r["domain_terms_eligible"] for r in runs)
        mean_prot = statistics.mean(r["domain_terms_protected"] for r in runs)
        mean_var = statistics.mean(r["domain_terms_with_variants"] for r in runs)
        mean_rep = statistics.mean(r["domain_terms_replaced"] for r in runs)
        mean_gt = statistics.mean(r["general_turnover_pct"] for r in runs)
        mean_dt = statistics.mean(r["domain_turnover_pct"] for r in runs)
        print(f"{cond_name:<32} | {mean_det:9.1f} | {mean_elig:9.1f} | {mean_prot:10.1f} | {mean_var:9.1f} | {mean_rep:9.1f} | {mean_gt:8.2f}% | {mean_dt:8.2f}%")
    print("-" * 110)

    # 4. HIGH-SIGNAL COHORT (Z_baseline >= 3.0, 3-sigma standard detection)
    strong_keys = []
    for k, conds in doc_map.items():
        base = conds.get("A_baseline")
        if base and base["synthid_z"] is not None and base["synthid_z"] >= 3.0:
            strong_keys.append(k)

    print(f"\n4. HIGH-CONFIDENCE COHORT (Z_baseline >= 3.0, 3-Sigma Alarm, N = {len(strong_keys)} Documents):")
    print("-" * 110)
    print(f"{'Condition':<32} | {'Mean Z':<8} | {'Delta Z':<10} | {'Demoted < 3.0':<15} | {'Stripped < 1.645':<18} | {'Turnover %':<10}")
    print("-" * 110)
    for cond_key, cond_name in conditions:
        runs = [doc_map[k][cond_key] for k in strong_keys if cond_key in doc_map[k]]
        base_runs = [doc_map[k]["A_baseline"] for k in strong_keys if "A_baseline" in doc_map[k]]
        zs = [r["synthid_z"] for r in runs]
        base_zs = [r["synthid_z"] for r in base_runs]
        delta_zs = [(z - bz) for z, bz in zip(zs, base_zs)]
        demoted = sum(1 for z in zs if z < 3.0)
        stripped = sum(1 for z in zs if z < 1.645)
        mean_z = statistics.mean(zs)
        mean_dz = statistics.mean(delta_zs) if cond_key != "A_baseline" else 0.0
        mean_t = statistics.mean(r["lexical_turnover_pct"] for r in runs)
        dz_str = f"{mean_dz:+8.3f}" if cond_key != "A_baseline" else "    —   "
        print(f"{cond_name:<32} | {mean_z:8.3f} | {dz_str:<10} | {demoted:2d}/{len(zs)} ({demoted/len(zs)*100.0:5.1f}%)   | {stripped:2d}/{len(zs)} ({stripped/len(zs)*100.0:5.1f}%)        | {mean_t:8.2f}%")
    print("=" * 110)

if __name__ == "__main__":
    main()
