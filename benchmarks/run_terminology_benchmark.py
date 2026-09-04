#!/usr/bin/env python3
"""
Domain-Aware Terminology Layer Benchmark Evaluation Harness.
Executes paired multi-domain runs comparing:
1. Baseline Watermarked
2. WordNet Only (Unconstrained Substitution)
3. Domain-Aware Terminology Layer (IATE / EuroVoc)
4. Domain-Aware Terminology + Strict Protection (--protect-domain-terms)

Computes across domains:
- Realized Lexical Turnover % (General vs Domain)
- Carrier Disruption (Delta Z, p-value, Clean Rate)
- Semantic Preservation & JSD
- Domain Expression Retention Rates
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
    {"name": "B_wordnet_only", "desc": "WordNet Only (No Terminology)", "args": ["--prob", "1.0", "--seed", "101"]},
    {"name": "C_terminology_on", "desc": "WordNet + IATE/EuroVoc Terminology", "args": ["--prob", "1.0", "--terminology", "--seed", "101"]},
    {"name": "D_terminology_protected", "desc": "IATE/EuroVoc + Strict Protection", "args": ["--prob", "1.0", "--terminology", "--protect-domain-terms", "--seed", "101"]},
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
                
            sample_records.append({
                "domain": domain,
                "sample_id": sample_id,
                "condition": cond_name,
                "condition_desc": cond["desc"],
                "total_words": log_data["total_words"],
                "eligible_words": log_data["eligible_words"],
                "replaced_words": log_data["replaced_words"],
                "lexical_turnover_pct": log_data["lexical_turnover_pct"],
                "general_turnover_pct": log_data.get("terminology_metrics", {}).get("general_lexical_turnover_pct", log_data["lexical_turnover_pct"]) if log_data.get("terminology_metrics") else log_data["lexical_turnover_pct"],
                "domain_turnover_pct": log_data.get("terminology_metrics", {}).get("domain_lexical_turnover_pct", 0.0) if log_data.get("terminology_metrics") else 0.0,
                "matched_terms": log_data.get("terminology_metrics", {}).get("matched_terms_count", 0) if log_data.get("terminology_metrics") else 0,
                "domain_locked_terms": log_data.get("terminology_metrics", {}).get("domain_locked_terms_count", 0) if log_data.get("terminology_metrics") else 0,
                "mean_confidence": log_data["mean_semantic_confidence"],
                "content_jsd": log_data["jsd_metrics"]["content_words_jsd"],
                "synthid_z": log_data["synthid_verification"]["z_score"] if log_data.get("synthid_verification") else None,
                "synthid_p": log_data["synthid_verification"]["p_value"] if log_data.get("synthid_verification") else None,
                "signal_detected": log_data["synthid_verification"]["watermark_signal_detected"] if log_data.get("synthid_verification") else False,
            })
        except Exception as e:
            print(f"[!] Failed to parse log {log_path}: {e}")
            
    return sample_records

def main():
    print("=" * 70)
    print("  DOMAIN-AWARE TERMINOLOGY LAYER BENCHMARK (IATE / EuroVoc)")
    print("=" * 70)
    
    if not os.path.exists(CORPUS_DIR):
        print(f"Error: Corpus directory not found at {CORPUS_DIR}")
        sys.exit(1)
        
    domains = [d for d in os.listdir(CORPUS_DIR) if os.path.isdir(os.path.join(CORPUS_DIR, d))]
    print(f"Found {len(domains)} linguistic domains: {', '.join(domains)}")
    
    tasks = []
    for domain in sorted(domains):
        domain_path = os.path.join(CORPUS_DIR, domain)
        files = [f for f in os.listdir(domain_path) if f.endswith(".txt")]
        for f in sorted(files):
            tasks.append((domain, f))
            
    print(f"Total documents to evaluate: {len(tasks)} across {len(CONDITIONS)} conditions ({len(tasks) * len(CONDITIONS)} runs)")
    
    all_results = []
    with ThreadPoolExecutor(max_workers=4) as executor:
        futures = [executor.submit(evaluate_sample, dom, f) for dom, f in tasks]
        for idx, fut in enumerate(futures, 1):
            res = fut.result()
            all_results.extend(res)
            if idx % 10 == 0 or idx == len(futures):
                print(f"  [Progress] Processed {idx}/{len(futures)} documents ({len(all_results)} condition runs completed)...")

    os.makedirs(os.path.dirname(RESULTS_JSON), exist_ok=True)
    with open(RESULTS_JSON, "w", encoding="utf-8") as f:
        json.dump(all_results, f, indent=2)
        
    print(f"\nSaved raw results to {RESULTS_JSON}")

    # Aggregations
    by_cond = defaultdict(list)
    by_domain_cond = defaultdict(lambda: defaultdict(list))
    
    for r in all_results:
        cond = r["condition"]
        dom = r["domain"]
        by_cond[cond].append(r)
        by_domain_cond[dom][cond].append(r)
        
    print("\n" + "=" * 80)
    print(f"{'Condition':<32} | {'Turnover %':<10} | {'Gen Turn %':<10} | {'Dom Turn %':<10} | {'Mean Z':<8} | {'Clean %':<8} | {'JSD (bits)':<10}")
    print("-" * 80)
    
    for cond_meta in CONDITIONS:
        c_name = cond_meta["name"]
        rows = by_cond[c_name]
        if not rows:
            continue
        n = len(rows)
        turnovers = [r["lexical_turnover_pct"] for r in rows]
        gen_turnovers = [r["general_turnover_pct"] for r in rows]
        dom_turnovers = [r["domain_turnover_pct"] for r in rows]
        zs = [r["synthid_z"] for r in rows if r["synthid_z"] is not None]
        jsds = [r["content_jsd"] for r in rows]
        clean_count = sum(1 for r in rows if r["synthid_z"] is not None and r["synthid_z"] < 1.645)
        
        mean_t = statistics.mean(turnovers)
        mean_gt = statistics.mean(gen_turnovers)
        mean_dt = statistics.mean(dom_turnovers)
        mean_z = statistics.mean(zs) if zs else 0.0
        mean_jsd = statistics.mean(jsds) if jsds else 0.0
        clean_pct = (clean_count / len(zs) * 100.0) if zs else 0.0
        
        print(f"{cond_meta['desc']:<32} | {mean_t:9.2f}% | {mean_gt:9.2f}% | {mean_dt:9.2f}% | {mean_z:8.3f} | {clean_pct:7.1f}% | {mean_jsd:9.4f} b")
        
    print("=" * 80)
    
    print("\n" + "=" * 80)
    print("PER-DOMAIN COMPARISON: WordNet Unconstrained vs Terminology-Aware")
    print("=" * 80)
    
    for dom in sorted(by_domain_cond.keys()):
        print(f"\n>>> Domain: {dom.upper()}")
        print(f"{'Condition':<32} | {'Turnover %':<10} | {'Dom Locked':<10} | {'Mean Z':<8} | {'Clean %':<8} | {'JSD (bits)':<10}")
        print("-" * 80)
        for cond_meta in CONDITIONS:
            c_name = cond_meta["name"]
            rows = by_domain_cond[dom][c_name]
            if not rows:
                continue
            turnovers = [r["lexical_turnover_pct"] for r in rows]
            locked = [r["domain_locked_terms"] for r in rows]
            zs = [r["synthid_z"] for r in rows if r["synthid_z"] is not None]
            jsds = [r["content_jsd"] for r in rows]
            clean_count = sum(1 for r in rows if r["synthid_z"] is not None and r["synthid_z"] < 1.645)
            
            mean_t = statistics.mean(turnovers)
            mean_locked = statistics.mean(locked)
            mean_z = statistics.mean(zs) if zs else 0.0
            mean_jsd = statistics.mean(jsds) if jsds else 0.0
            clean_pct = (clean_count / len(zs) * 100.0) if zs else 0.0
            
            print(f"{cond_meta['desc']:<32} | {mean_t:9.2f}% | {mean_locked:9.1f}  | {mean_z:8.3f} | {clean_pct:7.1f}% | {mean_jsd:9.4f} b")

if __name__ == "__main__":
    main()
