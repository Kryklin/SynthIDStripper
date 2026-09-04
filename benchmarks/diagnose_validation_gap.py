#!/usr/bin/env python3
"""
Deep Diagnostic Analysis of Dev vs Held-Out Validation Discrepancy,
Document Characteristics, Failure Modes, and Mechanism Bottlenecks.
"""

import os
import sys
import json
import statistics
from collections import defaultdict

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SWEEP_JSON = os.path.join(BASE_DIR, "data", "benchmark_parameter_sweep.json")
CORPUS_DIR = os.path.join(BASE_DIR, "data", "corpus")

def main():
    with open(SWEEP_JSON, "r", encoding="utf-8") as f:
        records = json.load(f)

    # Separate Dev and Val baseline runs
    dev_base = [r for r in records if r["split"] == "dev" and r["config_id"] == "CFG_00_BASELINE"]
    val_base = [r for r in records if r["split"] == "val" and r["config_id"] == "CFG_00_BASELINE"]

    print("=" * 80)
    print("  1. CORPUS PARTITION BASELINE CHARACTERISTICS (Dev vs Validation)")
    print("=" * 80)
    
    for split_name, base_runs in [("Development (60 docs x 2 seeds)", dev_base), ("Held-Out Validation (40 docs x 2 seeds)", val_base)]:
        zs = [r["synthid_z"] for r in base_runs if r.get("synthid_z") is not None]
        words = [r["total_words"] for r in base_runs if r.get("total_words") is not None]
        elig = [r["eligible_words"] for r in base_runs if r.get("eligible_words") is not None]
        det_cnt = sum(1 for z in zs if z >= 1.645)
        strong_cnt = sum(1 for z in zs if z >= 3.0)
        
        print(f"\n>>> Split: {split_name}")
        print(f"  Total Evaluations:           {len(base_runs)}")
        print(f"  Mean Document Length:        {statistics.mean(words):.1f} words (range: {min(words)} - {max(words)})")
        print(f"  Mean Eligible Content Words: {statistics.mean(elig):.1f} words ({statistics.mean(elig)/statistics.mean(words)*100:.1f}%)")
        print(f"  Mean Baseline Z-Score:       {statistics.mean(zs):.3f} (median: {statistics.median(zs):.3f}, max: {max(zs):.3f})")
        print(f"  Detectable (Z >= 1.645):     {det_cnt}/{len(zs)} ({det_cnt/len(zs)*100.1:.1f}%)")
        print(f"  Strong Signal (Z >= 3.0):    {strong_cnt}/{len(zs)} ({strong_cnt/len(zs)*100.1:.1f}%)")
        
        # In detectable cohort:
        det_zs = [z for z in zs if z >= 1.645]
        if det_zs:
            print(f"  Detectable Cohort Mean Z:    {statistics.mean(det_zs):.3f} (min: {min(det_zs):.3f}, max: {max(det_zs):.3f})")

    # 2. Per-Domain Baseline Detectability
    print("\n" + "=" * 80)
    print("  2. PER-DOMAIN BASELINE COMPARISON (Dev vs Validation)")
    print("=" * 80)
    print(f"{'Domain':<22} | {'Split':<5} | {'N Docs':<6} | {'Mean Length':<11} | {'Mean Base Z':<11} | {'Det (>=1.645)':<13} | {'Det Mean Z':<10}")
    print("-" * 80)

    by_split_dom = defaultdict(lambda: defaultdict(list))
    for r in records:
        if r["config_id"] == "CFG_00_BASELINE":
            by_split_dom[r["split"]][r["domain"]].append(r)

    for dom in sorted(by_split_dom["dev"].keys()):
        for sp in ["dev", "val"]:
            runs = by_split_dom[sp][dom]
            zs = [r["synthid_z"] for r in runs if r.get("synthid_z") is not None]
            words = [r["total_words"] for r in runs]
            det_zs = [z for z in zs if z >= 1.645]
            det_str = f"{len(det_zs)}/{len(zs)} ({len(det_zs)/len(zs)*100:.0f}%)"
            det_mean_z = f"{statistics.mean(det_zs):.3f}" if det_zs else "N/A"
            print(f"{dom:<22} | {sp:<5} | {len(runs):<6} | {statistics.mean(words):9.1f}   | {statistics.mean(zs):9.3f}   | {det_str:<13} | {det_mean_z:<10}")

    # 3. Investigation of Failure Cases in Validation Set
    print("\n" + "=" * 80)
    print("  3. FAILURE ANALYSIS ON HELD-OUT VALIDATION SET")
    print("=" * 80)
    
    # Identify validation documents that failed to drop below 1.645 under CFG_07
    val_base_map = {}
    for r in val_base:
        key = (r["domain"], r["sample_id"], r["seed"])
        val_base_map[key] = r
        
    val_cand_runs = [r for r in records if r["split"] == "val" and r["config_id"] == "CFG_07_DIST_RANDOM"]
    
    failed_val_cases = []
    success_val_cases = []
    
    for r in val_cand_runs:
        key = (r["domain"], r["sample_id"], r["seed"])
        base_r = val_base_map.get(key)
        if base_r and base_r["synthid_z"] >= 1.645:
            delta_z = r["synthid_z"] - base_r["synthid_z"]
            item = {
                "domain": r["domain"],
                "sample_id": r["sample_id"],
                "seed": r["seed"],
                "words": r["total_words"],
                "eligible": r["eligible_words"],
                "replaced": r["replaced_words"],
                "turnover": r["lexical_turnover_pct"],
                "gen_turnover": r["general_turnover_pct"],
                "dom_turnover": r["domain_turnover_pct"],
                "det_terms": r["domain_terms_detected"],
                "prot_terms": r["domain_terms_protected"],
                "rep_terms": r["domain_terms_replaced"],
                "base_z": base_r["synthid_z"],
                "out_z": r["synthid_z"],
                "delta_z": delta_z,
                "is_clean": r["synthid_z"] < 1.645,
                "confidence": r["mean_confidence"],
                "content_jsd": r["content_jsd"],
            }
            if r["synthid_z"] >= 1.645:
                failed_val_cases.append(item)
            else:
                success_val_cases.append(item)

    print(f"\nIn Held-Out Validation Cohort (N = {len(failed_val_cases) + len(success_val_cases)} detectable runs):")
    print(f"  Successfully Stripped (< 1.645): {len(success_val_cases)} ({len(success_val_cases)/(len(failed_val_cases)+len(success_val_cases))*100:.1f}%)")
    print(f"  Failed (Z >= 1.645):              {len(failed_val_cases)} ({len(failed_val_cases)/(len(failed_val_cases)+len(success_val_cases))*100:.1f}%)")

    print("\nComparison: Successful vs Failed Validation Cases:")
    print(f"{'Metric':<30} | {'Successful (Z < 1.645)':<25} | {'Failed (Z >= 1.645)':<25}")
    print("-" * 80)
    
    metrics = [
        ("Base Z", "base_z"),
        ("Output Z", "out_z"),
        ("Delta Z", "delta_z"),
        ("Document Length (words)", "words"),
        ("Eligible Content Words", "eligible"),
        ("Realized Turnover %", "turnover"),
        ("General Turnover %", "gen_turnover"),
        ("Domain Turnover %", "dom_turnover"),
        ("Domain Terms Detected", "det_terms"),
        ("Domain Terms Protected", "prot_terms"),
        ("Semantic Confidence", "confidence"),
        ("Content JSD (bits)", "content_jsd"),
    ]
    
    for label, key in metrics:
        s_vals = [c[key] for c in success_val_cases]
        f_vals = [c[key] for c in failed_val_cases]
        mean_s = statistics.mean(s_vals) if s_vals else 0.0
        mean_f = statistics.mean(f_vals) if f_vals else 0.0
        print(f"{label:<30} | {mean_s:23.3f} | {mean_f:23.3f}")

    print("\nDetailed Failed Cases Sample (First 6):")
    for c in failed_val_cases[:6]:
        print(f"  - [{c['domain']}/{c['sample_id']}_s{c['seed']}] Len: {c['words']}w | Base Z: {c['base_z']:.3f} -> Out Z: {c['out_z']:.3f} (Delta: {c['delta_z']:+.3f}) | Turn: {c['turnover']:.1f}% | Det: {c['det_terms']} (Prot: {c['prot_terms']})")

if __name__ == "__main__":
    main()
