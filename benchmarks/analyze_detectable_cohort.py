#!/usr/bin/env python3
"""
Rigorous paired-difference & cohort stratification analysis.
Specifically isolates baseline-detectable documents (Z_baseline >= threshold)
and evaluates within-document Delta Z, retention probabilities, and pure dose-response curves.
"""

import os
import json
import math
import statistics
from collections import defaultdict

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RESULTS_JSON = os.path.join(BASE_DIR, "data", "benchmark_results.json")

def main():
    if not os.path.exists(RESULTS_JSON):
        print(f"Results file not found: {RESULTS_JSON}")
        return

    with open(RESULTS_JSON, "r", encoding="utf-8") as f:
        records = json.load(f)

    # Group by (domain, sample_id) -> condition -> record
    doc_map = defaultdict(dict)
    for r in records:
        key = (r["domain"], r["sample_id"])
        doc_map[key][r["condition"]] = r

    total_docs = len(doc_map)
    print("===================================================================================================================")
    print("           PAIRED-SAMPLE COHORT ANALYSIS & WATERMARK RETENTION PROBABILITY                                         ")
    print("===================================================================================================================")
    print(f"Total Unique Documents: {total_docs}")

    # Thresholds to evaluate: Z >= 1.645 (standard alpha=0.05 one-tailed detection) and Z >= 1.0 (elevated signal)
    for threshold_name, thresh in [("Standard Alpha=0.05 (Z >= 1.645)", 1.645), ("Elevated Baseline (Z >= 1.000)", 1.000)]:
        print(f"\n" + "=" * 115)
        print(f"  COHORT STRATIFICATION CRITERION: {threshold_name.upper()}")
        print("=" * 115)

        detectable_docs = []
        undetectable_docs = []

        for key, conds in doc_map.items():
            base_rec = conds.get("A_baseline")
            if not base_rec:
                continue
            z_base = base_rec["synthid_z_score"]
            if z_base >= thresh:
                detectable_docs.append(key)
            else:
                undetectable_docs.append(key)

        n_detectable = len(detectable_docs)
        n_undetectable = len(undetectable_docs)
        print(f"  * Baseline Detectable Cohort:   {n_detectable} / {total_docs} ({n_detectable/total_docs*100:.1f}%)")
        print(f"  * Baseline Sub-Threshold Cohort: {n_undetectable} / {total_docs} ({n_undetectable/total_docs*100:.1f}%)")

        if n_detectable == 0:
            print("    [!] No documents met this threshold.")
            continue

        conditions_ordered = [
            ("A_baseline", "Baseline (Watermarked)"),
            ("B_lexical_p20", "Lexical (prob=0.20)"),
            ("C_lexical_p40", "Lexical (prob=0.40)"),
            ("D_lexical_p60", "Lexical (prob=0.60)"),
            ("E_lexical_p80", "Lexical (prob=0.80)"),
            ("F_lexical_p100", "Lexical (prob=1.00)"),
            ("G_typing_noise", "Typing Noise Only (2%)"),
            ("H_func_words", "Function Words Only"),
            ("I_syntax_cadence", "Syntax & Cadence Only"),
            ("J_full_combined", "Full Combined Pipeline"),
        ]

        print(f"\n  --- PAIRED WITHIN-DOCUMENT EFFECT SIZES (Detectable Cohort N = {n_detectable}) ---")
        print(f"  {'Condition':<26} | {'Mean Z':<8} | {'Mean dZ':<9} | {'Mean d|Z|':<10} | {'95% CI (dZ)':<12} | {'Still Detectable (%)':<20} | {'P(Stripped | Base)':<18}")
        print("  " + "-" * 115)

        for cond_id, cond_desc in conditions_ordered:
            z_after_list = []
            delta_z_list = []
            delta_abs_z_list = []
            still_detectable_count = 0

            for doc_key in detectable_docs:
                base_r = doc_map[doc_key]["A_baseline"]
                cond_r = doc_map[doc_key].get(cond_id)
                if not cond_r:
                    continue

                z_base = base_r["synthid_z_score"]
                z_after = cond_r["synthid_z_score"]

                dz = z_after - z_base
                d_abs_z = abs(z_after) - abs(z_base)

                z_after_list.append(z_after)
                delta_z_list.append(dz)
                delta_abs_z_list.append(d_abs_z)

                if z_after >= thresh:
                    still_detectable_count += 1

            m_z = statistics.mean(z_after_list)
            m_dz = statistics.mean(delta_z_list)
            m_d_abs_z = statistics.mean(delta_abs_z_list)

            std_dz = statistics.stdev(delta_z_list) if len(delta_z_list) > 1 else 0.0
            ci_dz = 1.96 * (std_dz / math.sqrt(len(delta_z_list))) if len(delta_z_list) > 1 else 0.0

            det_pct = (still_detectable_count / n_detectable) * 100.0
            stripped_prob = 100.0 - det_pct

            print(f"  {cond_desc:<26} | {m_z:>8.3f} | {m_dz:>9.3f} | {m_d_abs_z:>10.3f} | +/- {ci_dz:>7.3f} | {still_detectable_count:>3}/{n_detectable} ({det_pct:>5.1f}%)        | {stripped_prob:>6.1f}%")

    print("\n===================================================================================================================")
    print("  DOMAIN BREAKDOWN OF BASELINE DETECTABILITY (Z_baseline >= 1.645)")
    print("===================================================================================================================")
    by_domain_counts = defaultdict(lambda: {"total": 0, "detectable": 0, "stripped_at_F": 0})
    for (dom, sid), conds in doc_map.items():
        by_domain_counts[dom]["total"] += 1
        base_z = conds["A_baseline"]["synthid_z_score"]
        if base_z >= 1.645:
            by_domain_counts[dom]["detectable"] += 1
            f_z = conds["F_lexical_p100"]["synthid_z_score"]
            if f_z < 1.645:
                by_domain_counts[dom]["stripped_at_F"] += 1

    for dom, counts in sorted(by_domain_counts.items()):
        det = counts["detectable"]
        tot = counts["total"]
        strp = counts["stripped_at_F"]
        pct_det = (det / tot) * 100.0
        p_strip = (strp / det * 100.0) if det > 0 else 0.0
        print(f"  * {dom:<22}: {det:>2}/{tot} baseline detectable ({pct_det:>4.1f}%) -> {strp}/{det} stripped at 100% lexical (Retention Collapse: {p_strip:>5.1f}%)")

    print("===================================================================================================================")

if __name__ == "__main__":
    main()
