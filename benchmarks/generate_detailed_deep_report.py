#!/usr/bin/env python3
"""
Deep Comprehensive Analysis Script for Parameter Optimization Study
Computes per-domain, dev vs held-out validation, terminology dialect metrics,
and statistical tables for the final report.
"""

import os
import sys
import json
import math
import statistics
from collections import defaultdict

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SWEEP_JSON = os.path.join(BASE_DIR, "data", "benchmark_parameter_sweep.json")

def mean_and_ci(values, confidence=0.95):
    if not values:
        return 0.0, 0.0
    m = statistics.mean(values)
    if len(values) < 2:
        return m, 0.0
    stdev = statistics.stdev(values)
    z_score = 1.96 if confidence == 0.95 else 2.576
    ci = z_score * (stdev / math.sqrt(len(values)))
    return m, ci

def main():
    with open(SWEEP_JSON, "r", encoding="utf-8") as f:
        records = json.load(f)

    # 1. Map baseline Z by (split, domain, sample_id, seed)
    base_map = {}
    for r in records:
        if r["config_id"] == "CFG_00_BASELINE":
            key = (r["split"], r["domain"], r["sample_id"], r["seed"])
            base_map[key] = r

    # 2. Group records by config and split
    configs = sorted(list(set(r["config_id"] for r in records)))
    
    print("=" * 100)
    print("  1. FULL PARAMETER SWEEP SUMMARY (DEVELOPMENT SET, N=60 docs, 120 evals/config)")
    print("=" * 100)
    print(f"{'Config ID':<26} | {'P(Z<1.645|Det)':<15} | {'Paired Delta Z (95% CI)':<24} | {'Mean Out Z':<10} | {'Turnover':<9} | {'JSD (bits)':<10} | {'Conf':<6}")
    print("-" * 100)

    for cfg in configs:
        dev_runs = [r for r in records if r["split"] == "dev" and r["config_id"] == cfg]
        if not dev_runs:
            continue
        
        # Detectable cohort
        det_pairs = []
        for r in dev_runs:
            key = (r["split"], r["domain"], r["sample_id"], r["seed"])
            b = base_map.get(key)
            if b and b.get("synthid_z", 0.0) >= 1.645:
                dz = r.get("synthid_z", 0.0) - b.get("synthid_z", 0.0)
                det_pairs.append((r, dz))
        
        if det_pairs:
            det_cnt = len(det_pairs)
            clean_cnt = sum(1 for (r, dz) in det_pairs if r.get("synthid_z", 0.0) < 1.645)
            p_str = f"{clean_cnt}/{det_cnt} ({clean_cnt/det_cnt*100:.1f}%)"
            dzs = [dz for (r, dz) in det_pairs]
            m_dz, ci_dz = mean_and_ci(dzs)
            dz_str = f"{m_dz:+.3f} +/- {ci_dz:.2f}"
            out_zs = [r.get("synthid_z", 0.0) for (r, dz) in det_pairs]
            m_oz = statistics.mean(out_zs)
        else:
            p_str = "N/A"
            dz_str = "N/A"
            m_oz = 0.0

        all_turn = [r.get("lexical_turnover_pct", 0.0) for r in dev_runs]
        all_jsd = [r.get("content_jsd", 0.0) for r in dev_runs]
        all_conf = [r.get("mean_confidence", 0.0) for r in dev_runs]

        m_turn = statistics.mean(all_turn) if all_turn else 0.0
        m_jsd = statistics.mean(all_jsd) if all_jsd else 0.0
        m_conf = statistics.mean(all_conf) if all_conf else 0.0

        print(f"{cfg:<26} | {p_str:<15} | {dz_str:<24} | {m_oz:9.3f}  | {m_turn:7.2f}%  | {m_jsd:8.4f} b | {m_conf:5.3f}")

    # 3. Per-Domain Breakdown across key configurations
    key_cfgs = [
        ("CFG_00_BASELINE", "Baseline Control"),
        ("CFG_01_DEFAULT_HUMAN", "Standard Human Mode (p=1.0, c=0.55)"),
        ("CFG_03_RELAXED_CONF_40", "Relaxed Confidence (c=0.40)"),
        ("CFG_07_DIST_RANDOM", "Random Sampling (p=1.0, c=0.55)"),
        ("CFG_13_TERM_STRICT_PROTECT", "Strict Domain Protection"),
    ]
    
    print("\n" + "=" * 100)
    print("  2. PER-DOMAIN COMPARATIVE BREAKDOWN (Detectable Cohort)")
    print("=" * 100)
    
    domains = sorted(list(set(r["domain"] for r in records)))
    for dom in domains:
        print(f"\n>>> Domain: {dom.upper()}")
        print(f"{'Config ID':<26} | {'Split':<5} | {'P(Z<1.645|Det)':<15} | {'Paired Delta Z':<16} | {'Mean Out Z':<10} | {'Turnover':<9} | {'JSD (bits)':<10}")
        print("-" * 95)
        for cfg_id, cfg_desc in key_cfgs:
            for sp in ["dev", "val"]:
                runs = [r for r in records if r["domain"] == dom and r["split"] == sp and r["config_id"] == cfg_id]
                det_pairs = []
                for r in runs:
                    key = (r["split"], r["domain"], r["sample_id"], r["seed"])
                    b = base_map.get(key)
                    if b and b.get("synthid_z", 0.0) >= 1.645:
                        dz = r.get("synthid_z", 0.0) - b.get("synthid_z", 0.0)
                        det_pairs.append((r, dz))
                
                if det_pairs:
                    det_cnt = len(det_pairs)
                    clean_cnt = sum(1 for (r, dz) in det_pairs if r.get("synthid_z", 0.0) < 1.645)
                    p_str = f"{clean_cnt}/{det_cnt} ({clean_cnt/det_cnt*100:.1f}%)"
                    dzs = [dz for (r, dz) in det_pairs]
                    m_dz = statistics.mean(dzs)
                    dz_str = f"{m_dz:+.3f}"
                    out_zs = [r.get("synthid_z", 0.0) for (r, dz) in det_pairs]
                    m_oz = statistics.mean(out_zs)
                    turns = [r.get("lexical_turnover_pct", 0.0) for (r, dz) in det_pairs]
                    m_turn = statistics.mean(turns)
                    jsds = [r.get("content_jsd", 0.0) for (r, dz) in det_pairs]
                    m_jsd = statistics.mean(jsds)
                else:
                    p_str = "0/0 (N/A)"
                    dz_str = "0.000"
                    m_oz = 0.0
                    m_turn = 0.0
                    m_jsd = 0.0
                
                print(f"{cfg_id:<26} | {sp:<5} | {p_str:<15} | {dz_str:<16} | {m_oz:9.3f}  | {m_turn:7.2f}%  | {m_jsd:8.4f} b")

    # 4. Domain Terminology Dialect Granular Denominators
    print("\n" + "=" * 100)
    print("  3. DOMAIN TERMINOLOGY DENOMINATORS & DIALECT PRESERVATION (All 100 Documents)")
    print("=" * 100)
    print(f"{'Domain':<22} | {'Detected':<9} | {'Eligible':<9} | {'Protected':<10} | {'With Variants':<14} | {'Replaced':<9} | {'Dom Turn %':<11}")
    print("-" * 100)

    for dom in domains:
        dom_runs = [r for r in records if r["domain"] == dom and r["config_id"] == "CFG_01_DEFAULT_HUMAN"]
        if not dom_runs:
            continue
        det_t = statistics.mean([r.get("domain_terms_detected", 0) for r in dom_runs])
        elig_t = statistics.mean([r.get("domain_terms_eligible", 0) for r in dom_runs])
        prot_t = statistics.mean([r.get("domain_terms_protected", 0) for r in dom_runs])
        var_t = statistics.mean([r.get("domain_terms_with_variants", 0) for r in dom_runs])
        rep_t = statistics.mean([r.get("domain_terms_replaced", 0) for r in dom_runs])
        dturn = statistics.mean([r.get("domain_turnover_pct", 0.0) for r in dom_runs])
        print(f"{dom:<22} | {det_t:8.1f}  | {elig_t:8.1f}  | {prot_t:9.1f}  | {var_t:13.1f}  | {rep_t:8.1f}  | {dturn:9.2f}%")

if __name__ == "__main__":
    main()
