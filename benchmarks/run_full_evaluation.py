#!/usr/bin/env python3
"""
Large-Scale Benchmark Evaluation Harness for Lexicon Stripper.
Evaluates SynthID watermark disruption, JSD divergence, edit distance,
and dose-response dynamics across a multi-domain corpus of verified AI texts.
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
EVAL_LOGS_DIR = os.path.join(BASE_DIR, "data", "evaluation_logs")
RESULTS_JSON = os.path.join(BASE_DIR, "data", "benchmark_results.json")

SYNTHID_KEY = 428917492
SYNTHID_K = 2

CONDITIONS = [
    {"name": "A_baseline", "desc": "Baseline (Watermarked)", "args": ["--no-lexical"]},
    {"name": "B_lexical_p20", "desc": "Lexical (prob=0.20)", "args": ["--prob", "0.20", "--seed", "101"]},
    {"name": "C_lexical_p40", "desc": "Lexical (prob=0.40)", "args": ["--prob", "0.40", "--seed", "101"]},
    {"name": "D_lexical_p60", "desc": "Lexical (prob=0.60)", "args": ["--prob", "0.60", "--seed", "101"]},
    {"name": "E_lexical_p80", "desc": "Lexical (prob=0.80)", "args": ["--prob", "0.80", "--seed", "101"]},
    {"name": "F_lexical_p100", "desc": "Lexical (prob=1.00)", "args": ["--prob", "1.00", "--seed", "101"]},
    {"name": "G_typing_noise", "desc": "Typing Noise Only (2%)", "args": ["--no-lexical", "--typing-noise", "0.02", "--typing-seed", "101"]},
    {"name": "H_func_words", "desc": "Function Words Only", "args": ["--no-lexical", "--function-words"]},
    {"name": "I_syntax_cadence", "desc": "Syntax & Cadence Only", "args": ["--no-lexical", "--syntax", "--cadence"]},
    {"name": "J_full_combined", "desc": "Full Combined Pipeline", "args": ["--combined", "--typing-noise", "0.02", "--typing-seed", "101", "--seed", "101"]},
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
    """Watermarks the raw text using the official SynthID watermarker"""
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
            with open(log_path, "r", encoding="utf-8") as lf:
                log_data = json.load(lf)
                
            synthid = log_data.get("synthid_verification", {})
            jsd = log_data.get("jsd_metrics", {})
            typing = log_data.get("typing_noise_metrics") or {}
            
            record = {
                "domain": domain,
                "sample_id": sample_id,
                "condition": cond_name,
                "condition_desc": cond["desc"],
                "experiment_id": exp_id,
                "input_sha256": log_data.get("input_sha256"),
                "output_sha256": log_data.get("output_sha256"),
                "total_words": log_data.get("total_words", 0),
                "replaced_words": log_data.get("replaced_words", 0),
                "replacement_rate_pct": log_data.get("replacement_rate_pct", 0.0),
                "lexical_turnover_pct": log_data.get("lexical_turnover_pct", 0.0),
                "mean_semantic_confidence": log_data.get("mean_semantic_confidence", 0.0),
                "total_jsd": jsd.get("total_jsd", 0.0),
                "content_words_jsd": jsd.get("content_words_jsd", 0.0),
                "adjectives_jsd": jsd.get("adjectives_jsd", 0.0),
                "nouns_jsd": jsd.get("nouns_jsd", 0.0),
                "verbs_jsd": jsd.get("verbs_jsd", 0.0),
                "function_words_jsd": jsd.get("function_words_jsd", 0.0),
                "synthid_g_value": synthid.get("mean_g_value", 0.5),
                "synthid_z_score": synthid.get("z_score", 0.0),
                "synthid_p_value": synthid.get("p_value", 1.0),
                "watermark_signal_detected": synthid.get("watermark_signal_detected", False),
                "classification": synthid.get("classification", "UNKNOWN"),
                "is_clean": synthid.get("z_score", 0.0) < 1.645,
                "char_edit_distance": typing.get("character_edit_distance", 0),
                "word_edit_distance": typing.get("word_edit_distance", 0),
            }
            sample_records.append(record)
        except Exception as log_err:
            print(f"[!] Error reading log {log_path}: {log_err}")
            
    return sample_records

def main():
    print("=================================================================================")
    print("      LARGE-SCALE MULTI-DOMAIN AI BENCHMARK EVALUATION ENGINE                    ")
    print("=================================================================================")
    
    if not os.path.exists(EXE_PATH):
        print(f"[!] Executable not found at {EXE_PATH}. Please run `cargo build --release` first.")
        sys.exit(1)
        
    domains = [d for d in os.listdir(CORPUS_DIR) if os.path.isdir(os.path.join(CORPUS_DIR, d))]
    print(f"[*] Found {len(domains)} corpus domains: {', '.join(domains)}")
    
    all_sample_tasks = []
    for domain in domains:
        domain_path = os.path.join(CORPUS_DIR, domain)
        samples = [f for f in os.listdir(domain_path) if f.endswith(".txt") and not f.endswith(".meta.txt")]
        for s in samples:
            all_sample_tasks.append((domain, s))
            
    total_evaluations = len(all_sample_tasks) * len(CONDITIONS)
    print(f"[*] Total Samples: {len(all_sample_tasks)}")
    print(f"[*] Total Experimental Runs: {total_evaluations} ({len(CONDITIONS)} conditions per sample)")
    print("[*] Running parallel evaluation harness across all CPU cores...\n")
    
    all_records = []
    completed_samples = 0
    
    # Process samples in parallel across worker threads
    with ThreadPoolExecutor(max_workers=8) as executor:
        futures = [executor.submit(evaluate_sample, dom, samp) for dom, samp in all_sample_tasks]
        for f in futures:
            res = f.result()
            all_records.extend(res)
            completed_samples += 1
            if completed_samples % 10 == 0 or completed_samples == len(all_sample_tasks):
                pct = (completed_samples / len(all_sample_tasks)) * 100.0
                print(f"    -> Progress: {completed_samples}/{len(all_sample_tasks)} samples processed ({pct:.1f}%) ...")
                
    print(f"\n[+] Executed {len(all_records)} / {total_evaluations} experimental evaluations successfully.")
    
    # Save raw records to JSON
    with open(RESULTS_JSON, "w", encoding="utf-8") as rf:
        json.dump(all_records, rf, indent=2)
    print(f"[+] Saved complete empirical dataset to {RESULTS_JSON}")
    
    # Group and aggregate statistics
    by_condition = defaultdict(list)
    by_domain_condition = defaultdict(lambda: defaultdict(list))
    
    for r in all_records:
        cond = r["condition"]
        dom = r["domain"]
        by_condition[cond].append(r)
        by_domain_condition[dom][cond].append(r)
        
    print("\n" + "=" * 125)
    print(f"{'Condition':<26} | {'N':<4} | {'Turnover (%)':<14} | {'Mean Z':<8} | {'95% CI':<14} | {'Mean p-val':<10} | {'Clean Rate':<10} | {'Tot JSD':<8} | {'Cont JSD':<8}")
    print("-" * 125)
    
    for cond in CONDITIONS:
        c_name = cond["name"]
        recs = by_condition[c_name]
        if not recs:
            continue
        n = len(recs)
        turnovers = [r["lexical_turnover_pct"] for r in recs]
        z_scores = [r["synthid_z_score"] for r in recs]
        p_vals = [r["synthid_p_value"] for r in recs]
        clean_flags = [1 if r["is_clean"] else 0 for r in recs]
        tot_jsds = [r["total_jsd"] for r in recs]
        cont_jsds = [r["content_words_jsd"] for r in recs]
        
        mean_turnover = statistics.mean(turnovers)
        std_turnover = statistics.stdev(turnovers) if n > 1 else 0.0
        
        mean_z = statistics.mean(z_scores)
        std_z = statistics.stdev(z_scores) if n > 1 else 0.0
        ci_z = 1.96 * (std_z / math.sqrt(n)) if n > 1 else 0.0
        
        mean_p = statistics.mean(p_vals)
        clean_rate = (sum(clean_flags) / n) * 100.0
        mean_tot_jsd = statistics.mean(tot_jsds)
        mean_cont_jsd = statistics.mean(cont_jsds)
        
        desc = cond["desc"]
        print(f"{desc:<26} | {n:<4} | {mean_turnover:>5.1f}% ± {std_turnover:>4.1f}% | {mean_z:>8.3f} | ± {ci_z:>11.3f} | {mean_p:>10.4f} | {clean_rate:>8.1f}% | {mean_tot_jsd:>7.4f}b | {mean_cont_jsd:>7.4f}b")
        
    print("=" * 125)

if __name__ == "__main__":
    main()
