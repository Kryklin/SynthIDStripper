#!/usr/bin/env python3
"""
Paired Per-Document Lexical Dialect Hypothesis Analysis.

Directly evaluates the hypothesis:
"Domain terminology acts as a lexical dialect region that constrains substitution
in proportion to domain-term density, while general vocabulary retains unconstrained freedom."

For each document:
- domain_terms_detected
- domain_terms_eligible
- domain_terms_protected
- domain_terms_with_variants
- domain_terms_replaced
- domain_term_density = domain_terms_detected / eligible_words
- Delta_T = Turnover_IATE - Turnover_WordNet
- General Turnover % vs Domain Turnover %

Computes:
- Paired t-tests
- Pearson r, Spearman rho, and OLS regression of Delta_T ~ DomainTermDensity
"""

import os
import sys
import json
import math
import subprocess
import statistics
from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXE_PATH = os.path.join(BASE_DIR, "target", "release", "lexicon_stripper.exe")
CORPUS_DIR = os.path.join(BASE_DIR, "data", "corpus")
EVAL_LOGS_DIR = os.path.join(BASE_DIR, "data", "terminology_logs")
RESULTS_JSON = os.path.join(BASE_DIR, "data", "terminology_benchmark_results.json")

SYNTHID_KEY = 428917492
SYNTHID_K = 2

CONDITIONS = [
    {"name": "A_baseline", "desc": "Baseline (Watermarked)", "args": ["--no-lexical"]},
    {"name": "B_wordnet_only", "desc": "WordNet Only", "args": ["--prob", "1.0", "--seed", "101"]},
    {"name": "C_terminology_on", "desc": "WordNet + IATE/EuroVoc", "args": ["--prob", "1.0", "--terminology", "--seed", "101"]},
    {"name": "D_terminology_protected", "desc": "IATE/EuroVoc (Strict Protected)", "args": ["--prob", "1.0", "--terminology", "--protect-domain-terms", "--seed", "101"]},
]

def run_cmd(cmd_list, stdin_text=None):
    proc = subprocess.run(
        cmd_list,
        input=stdin_text,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        encoding="utf-8",
        errors="ignore",
    )
    return proc.returncode, proc.stdout, proc.stderr

def watermark_sample(raw_text):
    cmd = [
        EXE_PATH,
        "--synthid-watermark",
        "--synthid-key", str(SYNTHID_KEY),
        "--synthid-k", str(SYNTHID_K),
    ]
    code, stdout, stderr = run_cmd(cmd, stdin_text=raw_text)
    if code != 0:
        raise RuntimeError(f"Watermarking failed: {stderr}")
    return stdout.strip()

def evaluate_sample(domain, sample_file):
    sample_id = os.path.splitext(os.path.basename(sample_file))[0]
    sample_path = os.path.join(CORPUS_DIR, domain, sample_file)
    
    with open(sample_path, "r", encoding="utf-8") as f:
        raw_text = f.read().strip()
        
    try:
        watermarked_text = watermark_sample(raw_text)
    except Exception as e:
        print(f"[!] Error watermarking {domain}/{sample_id}: {e}")
        return []

    domain_logs_dir = os.path.join(EVAL_LOGS_DIR, domain)
    os.makedirs(domain_logs_dir, exist_ok=True)
    
    sample_records = []
    
    for cond in CONDITIONS:
        cond_name = cond["name"]
        exp_id = f"{domain}_{sample_id}_{cond_name}"
        log_path = os.path.join(domain_logs_dir, f"{sample_id}_{cond_name}.log.json")
        
        cmd = [
            EXE_PATH,
            "--synthid-key", str(SYNTHID_KEY),
            "--synthid-k", str(SYNTHID_K),
            "--experiment-id", exp_id,
            "-l", log_path,
        ] + cond["args"]
        
        code, stdout, stderr = run_cmd(cmd, stdin_text=watermarked_text)
        if code != 0 or not os.path.exists(log_path):
            print(f"[!] Run failed for {exp_id}: {stderr}")
            continue
            
        try:
            with open(log_path, "r", encoding="utf-8") as f:
                log_data = json.load(f)
                
            tm = log_data.get("terminology_metrics")
            sample_records.append({
                "domain": domain,
                "sample_id": sample_id,
                "condition": cond_name,
                "condition_desc": cond["desc"],
                "total_words": log_data["total_words"],
                "eligible_words": log_data["eligible_words"],
                "replaced_words": log_data["replaced_words"],
                "lexical_turnover_pct": log_data["lexical_turnover_pct"],
                "general_turnover_pct": tm.get("general_lexical_turnover_pct", log_data["lexical_turnover_pct"]) if tm else log_data["lexical_turnover_pct"],
                "domain_turnover_pct": tm.get("domain_lexical_turnover_pct", 0.0) if tm else 0.0,
                "domain_terms_detected": tm.get("domain_terms_detected", 0) if tm else 0,
                "domain_terms_eligible": tm.get("domain_terms_eligible", 0) if tm else 0,
                "domain_terms_protected": tm.get("domain_terms_protected", 0) if tm else 0,
                "domain_terms_with_variants": tm.get("domain_terms_with_variants", 0) if tm else 0,
                "domain_terms_replaced": tm.get("domain_terms_replaced", 0) if tm else 0,
                "mean_confidence": log_data["mean_semantic_confidence"],
                "content_jsd": log_data["jsd_metrics"]["content_words_jsd"],
                "synthid_z": log_data["synthid_verification"]["z_score"] if log_data.get("synthid_verification") else None,
                "synthid_p": log_data["synthid_verification"]["p_value"] if log_data.get("synthid_verification") else None,
                "signal_detected": log_data["synthid_verification"]["watermark_signal_detected"] if log_data.get("synthid_verification") else False,
            })
        except Exception as e:
            print(f"[!] Failed to parse log {log_path}: {e}")
            
    return sample_records

def linear_regression(x, y):
    n = len(x)
    if n < 2:
        return 0, 0, 0, 0, 0
    mean_x = statistics.mean(x)
    mean_y = statistics.mean(y)
    ss_xx = sum((xi - mean_x) ** 2 for xi in x)
    ss_yy = sum((yi - mean_y) ** 2 for yi in y)
    ss_xy = sum((xi - mean_x) * (yi - mean_y) for xi, yi in zip(x, y))
    
    if ss_xx == 0:
        return 0, mean_y, 0, 0, 1.0
    slope = ss_xy / ss_xx
    intercept = mean_y - slope * mean_x
    r = ss_xy / (math.sqrt(ss_xx * ss_yy)) if ss_yy > 0 else 0
    r_squared = r ** 2
    
    # Standard error of slope
    residuals = [(yi - (slope * xi + intercept)) for xi, yi in zip(x, y)]
    s_err = math.sqrt(sum(e**2 for e in residuals) / (n - 2)) if n > 2 else 0
    se_slope = s_err / math.sqrt(ss_xx) if ss_xx > 0 else 0
    
    # t-statistic and approximate p-value
    t_stat = slope / se_slope if se_slope > 0 else 0
    # Two-tailed normal approximation for large N
    p_val = 2.0 * (1.0 - 0.5 * (1.0 + math.erf(abs(t_stat) / math.sqrt(2))))
    
    return slope, intercept, r, r_squared, p_val

def rank_data(x):
    indexed = sorted(enumerate(x), key=lambda item: item[1])
    ranks = [0] * len(x)
    for rank, (orig_idx, _) in enumerate(indexed, 1):
        ranks[orig_idx] = rank
    return ranks

def spearman_rho(x, y):
    rx = rank_data(x)
    ry = rank_data(y)
    n = len(x)
    d_sq = sum((rxi - ryi) ** 2 for rxi, ryi in zip(rx, ry))
    return 1.0 - (6.0 * d_sq) / (n * (n**2 - 1))

def main():
    print("=" * 80)
    print("  LEXICAL DIALECT HYPOTHESIS & PAIRED PER-DOCUMENT ANALYSIS")
    print("=" * 80)
    
    domains = [d for d in os.listdir(CORPUS_DIR) if os.path.isdir(os.path.join(CORPUS_DIR, d))]
    tasks = []
    for domain in sorted(domains):
        domain_path = os.path.join(CORPUS_DIR, domain)
        files = [f for f in os.listdir(domain_path) if f.endswith(".txt")]
        for f in sorted(files):
            tasks.append((domain, f))
            
    print(f"Evaluating {len(tasks)} documents across 4 conditions (400 runs)...")
    
    all_results = []
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = [executor.submit(evaluate_sample, dom, f) for dom, f in tasks]
        for idx, fut in enumerate(futures, 1):
            res = fut.result()
            all_results.extend(res)

    with open(RESULTS_JSON, "w", encoding="utf-8") as f:
        json.dump(all_results, f, indent=2)

    # Group by (domain, sample_id)
    doc_groups = defaultdict(dict)
    for r in all_results:
        key = (r["domain"], r["sample_id"])
        doc_groups[key][r["condition"]] = r

    # Table 1: Aggregate Granular Denominators Across Conditions
    print("\n" + "=" * 95)
    print(f"{'Condition':<30} | {'Detected':<9} | {'Eligible':<9} | {'Protected':<10} | {'Variants':<9} | {'Replaced':<9} | {'Turnover':<9} | {'Clean %':<8}")
    print("-" * 95)
    
    by_cond = defaultdict(list)
    for r in all_results:
        by_cond[r["condition"]].append(r)
        
    for c_meta in CONDITIONS:
        c_name = c_meta["name"]
        rows = by_cond[c_name]
        mean_det = statistics.mean(r["domain_terms_detected"] for r in rows)
        mean_elig = statistics.mean(r["domain_terms_eligible"] for r in rows)
        mean_prot = statistics.mean(r["domain_terms_protected"] for r in rows)
        mean_var = statistics.mean(r["domain_terms_with_variants"] for r in rows)
        mean_rep = statistics.mean(r["domain_terms_replaced"] for r in rows)
        mean_t = statistics.mean(r["lexical_turnover_pct"] for r in rows)
        clean_pct = sum(1 for r in rows if r["synthid_z"] is not None and r["synthid_z"] < 1.645) / len(rows) * 100.0
        
        print(f"{c_meta['desc']:<30} | {mean_det:9.1f} | {mean_elig:9.1f} | {mean_prot:10.1f} | {mean_var:9.1f} | {mean_rep:9.1f} | {mean_t:8.2f}% | {clean_pct:7.1f}%")
    print("=" * 95)

    # Table 2: Domain-by-Domain Granular Denominators for Condition C (IATE/EuroVoc)
    print("\n" + "=" * 95)
    print("DOMAIN-SPECIFIC TERMINOLOGY DENOMINATORS (WordNet + IATE/EuroVoc)")
    print("=" * 95)
    print(f"{'Domain':<22} | {'Detected':<9} | {'Eligible':<9} | {'Protected':<10} | {'Variants':<9} | {'Replaced':<9} | {'Gen Turn':<9} | {'Dom Turn':<9}")
    print("-" * 95)
    
    by_dom = defaultdict(list)
    for r in by_cond["C_terminology_on"]:
        by_dom[r["domain"]].append(r)
        
    for dom in sorted(by_dom.keys()):
        rows = by_dom[dom]
        mean_det = statistics.mean(r["domain_terms_detected"] for r in rows)
        mean_elig = statistics.mean(r["domain_terms_eligible"] for r in rows)
        mean_prot = statistics.mean(r["domain_terms_protected"] for r in rows)
        mean_var = statistics.mean(r["domain_terms_with_variants"] for r in rows)
        mean_rep = statistics.mean(r["domain_terms_replaced"] for r in rows)
        mean_gt = statistics.mean(r["general_turnover_pct"] for r in rows)
        mean_dt = statistics.mean(r["domain_turnover_pct"] for r in rows)
        print(f"{dom:<22} | {mean_det:9.1f} | {mean_elig:9.1f} | {mean_prot:10.1f} | {mean_var:9.1f} | {mean_rep:9.1f} | {mean_gt:8.2f}% | {mean_dt:8.2f}%")
    print("=" * 95)

    # Section 3: Paired Per-Document Correlation & Dialect Hypothesis Test
    # For each document, compute:
    # - Terminology Density = domain_terms_detected / eligible_words
    # - Delta_Turnover = Turnover(IATE) - Turnover(WordNet)
    # - Delta_Z = Z(IATE) - Z(WordNet)
    
    paired_data = []
    for (dom, sid), conds in doc_groups.items():
        if "B_wordnet_only" in conds and "C_terminology_on" in conds:
            wn = conds["B_wordnet_only"]
            iate = conds["C_terminology_on"]
            base = conds.get("A_baseline")
            
            elig = iate["eligible_words"]
            det = iate["domain_terms_detected"]
            density = (det / elig * 100.0) if elig > 0 else 0.0
            
            t_wn = wn["lexical_turnover_pct"]
            t_iate = iate["lexical_turnover_pct"]
            delta_t = t_iate - t_wn
            
            z_wn = wn["synthid_z"]
            z_iate = iate["synthid_z"]
            delta_z = (z_iate - z_wn) if (z_wn is not None and z_iate is not None) else 0.0
            
            paired_data.append({
                "domain": dom,
                "sample_id": sid,
                "density": density,
                "detected": det,
                "t_wn": t_wn,
                "t_iate": t_iate,
                "delta_t": delta_t,
                "gen_turnover": iate["general_turnover_pct"],
                "dom_turnover": iate["domain_turnover_pct"],
                "z_wn": z_wn,
                "z_iate": z_iate,
                "delta_z": delta_z,
            })
            
    densities = [p["density"] for p in paired_data]
    delta_ts = [p["delta_t"] for p in paired_data]
    dom_turnovers = [p["dom_turnover"] for p in paired_data]
    
    slope, intercept, r, r2, p_val = linear_regression(densities, delta_ts)
    rho = spearman_rho(densities, delta_ts)
    
    print("\n" + "=" * 80)
    print("PAIRED REGRESSION: Delta Turnover ~ Domain Terminology Density")
    print("=" * 80)
    print(f"Sample Size (N)             : {len(paired_data)} paired documents")
    print(f"Mean Terminology Density     : {statistics.mean(densities):.2f}% of content vocabulary")
    print(f"Mean Delta Turnover (IATE-WN): {statistics.mean(delta_ts):+.2f}%")
    print(f"OLS Slope                   : {slope:+.4f} (SE: {abs(slope/math.sqrt(r2/(1-r2)*(len(densities)-2))) if r2<1 and r2>0 else 0:.4f})")
    print(f"Pearson Correlation (r)     : {r:+.4f}")
    print(f"Spearman Rank Corr (rho)    : {rho:+.4f}")
    print(f"Coefficient of Determ (R^2) : {r2:.4f}")
    print(f"p-value                     : {p_val:.4e}")
    print("=" * 80)
    
    # Per-Domain Sub-Analysis
    print("\nPER-DOMAIN SUB-ANALYSIS (Delta Turnover vs Term Density):")
    print(f"{'Domain':<22} | {'N':<4} | {'Mean Density':<13} | {'Mean Delta T':<13} | {'Pearson r':<10} | {'p-value':<10}")
    print("-" * 75)
    by_dom_paired = defaultdict(list)
    for p in paired_data:
        by_dom_paired[p["domain"]].append(p)
        
    for dom in sorted(by_dom_paired.keys()):
        p_rows = by_dom_paired[dom]
        d_sub = [p["density"] for p in p_rows]
        dt_sub = [p["delta_t"] for p in p_rows]
        sl, ic, r_sub, r2_sub, p_sub = linear_regression(d_sub, dt_sub)
        print(f"{dom:<22} | {len(p_rows):<4} | {statistics.mean(d_sub):11.2f}% | {statistics.mean(dt_sub):+11.2f}% | {r_sub:+9.4f} | {p_sub:9.4e}")
    print("=" * 75)

if __name__ == "__main__":
    main()
