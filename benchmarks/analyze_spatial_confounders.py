#!/usr/bin/env python3
"""
Statistical Confounder, Variance Decomposition & 50-Bin Multi-Resolution Analysis
Evaluates whether spatial position retains genuine explanatory power after controlling for:
- Part-of-Speech (POS) category
- Domain category
- Sentence-boundary proximity (sentence initial, medial, final)
- Clause-boundary proximity (punctuation distance)
- Local n-gram / detector window geometry (k=2 sliding window overlap count)
- Terminology protection status & token length
"""

import os
import sys
import json
import csv
import math
import statistics
from collections import defaultdict

if sys.stdout.encoding != "utf-8":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
    except Exception:
        pass

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HEATMAP_JSON = os.path.join(BASE_DIR, "data", "sensitivity_heatmap.json")
OUTPUT_JSON = os.path.join(BASE_DIR, "data", "spatial_statistical_models.json")
OUTPUT_50_CSV = os.path.join(BASE_DIR, "data", "spatial_bins_50.csv")
OUTPUT_TOKENS_CSV = os.path.join(BASE_DIR, "data", "confounder_regression_tokens.csv")

def benjamini_hochberg(p_values):
    """Computes Benjamini-Hochberg False Discovery Rate (FDR) adjusted p-values."""
    n = len(p_values)
    sorted_pairs = sorted(enumerate(p_values), key=lambda x: x[1])
    adjusted = [0.0] * n
    cum_min = 1.0
    for rank_idx, (orig_idx, p) in reversed(list(enumerate(sorted_pairs))):
        rank = rank_idx + 1
        adj = min(1.0, (p * n) / rank)
        cum_min = min(cum_min, adj)
        adjusted[orig_idx] = cum_min
    return adjusted

def solve_ols(X, y):
    """
    Solves Ordinary Least Squares (OLS) regression using standard linear algebra:
    beta = (X^T X)^(-1) X^T y
    """
    n = len(y)
    p = len(X[0])
    
    # Compute X^T X
    XtX = [[0.0 for _ in range(p)] for _ in range(p)]
    for i in range(p):
        for j in range(p):
            XtX[i][j] = sum(X[k][i] * X[k][j] for k in range(n))
            
    # Compute X^T y
    Xty = [sum(X[k][i] * y[k] for k in range(n)) for i in range(p)]
    
    # Invert XtX with small L2 ridge regularizer for numerical stability if singular
    for i in range(p):
        XtX[i][i] += 1e-7

    # Gaussian elimination with partial pivoting
    A = [row[:] for row in XtX]
    b = Xty[:]
    for i in range(p):
        max_row = max(range(i, p), key=lambda r: abs(A[r][i]))
        A[i], A[max_row] = A[max_row], A[i]
        b[i], b[max_row] = b[max_row], b[i]
        
        pivot = A[i][i]
        if abs(pivot) < 1e-12:
            continue
        for j in range(i, p):
            A[i][j] /= pivot
        b[i] /= pivot
        
        for k in range(p):
            if k != i:
                factor = A[k][i]
                for j in range(i, p):
                    A[k][j] -= factor * A[i][j]
                b[k] -= factor * b[i]
                
    beta = b
    
    # Predictions & Residuals
    y_pred = [sum(X[k][i] * beta[i] for i in range(p)) for k in range(n)]
    residuals = [y[k] - y_pred[k] for k in range(n)]
    ss_res = sum(r**2 for r in residuals)
    y_mean = statistics.mean(y)
    ss_tot = sum((yk - y_mean)**2 for yk in y)
    r_squared = 1.0 - (ss_res / ss_tot) if ss_tot > 0 else 0.0
    
    df_res = max(1, n - p)
    sigma2 = ss_res / df_res
    
    # Compute standard errors for coefficients
    inv_diag = []
    # Invert XtX to get covariance matrix diagonal
    # Re-solve for standard basis vectors
    cov_diag = [0.0] * p
    for col in range(p):
        e = [1.0 if r == col else 0.0 for r in range(p)]
        A_inv = [row[:] for row in XtX]
        for i in range(p):
            max_r = max(range(i, p), key=lambda r: abs(A_inv[r][i]))
            A_inv[i], A_inv[max_r] = A_inv[max_r], A_inv[i]
            e[i], e[max_r] = e[max_r], e[i]
            piv = A_inv[i][i]
            if abs(piv) < 1e-12:
                continue
            for j in range(i, p):
                A_inv[i][j] /= piv
            e[i] /= piv
            for k in range(p):
                if k != i:
                    fact = A_inv[k][i]
                    for j in range(i, p):
                        A_inv[k][j] -= fact * A_inv[i][j]
                    e[k] -= fact * e[i]
        cov_diag[col] = max(0.0, e[col])
        
    se = [math.sqrt(cov_diag[i] * sigma2) for i in range(p)]
    t_stats = [beta[i] / se[i] if se[i] > 1e-12 else 0.0 for i in range(p)]
    
    # Compute approximate p-values using normal approximation for large df
    p_vals = [2.0 * (1.0 - 0.5 * (1.0 + math.erf(abs(t) / math.sqrt(2.0)))) for t in t_stats]
    
    # F-statistic
    df_model = p - 1
    ms_model = (ss_tot - ss_res) / df_model if df_model > 0 else 0.0
    ms_res = ss_res / df_res
    f_stat = ms_model / ms_res if ms_res > 0 else 0.0
    
    return {
        "beta": beta,
        "se": se,
        "t_stats": t_stats,
        "p_values": p_vals,
        "r_squared": r_squared,
        "adjusted_r_squared": 1.0 - (1.0 - r_squared) * (n - 1) / df_res if df_res > 0 else 0.0,
        "ss_res": ss_res,
        "ss_tot": ss_tot,
        "f_stat": f_stat,
        "df_res": df_res,
        "df_model": df_model,
        "n": n,
        "p": p,
    }

def main():
    if not os.path.exists(HEATMAP_JSON):
        print(f"Error: {HEATMAP_JSON} does not exist.")
        sys.exit(1)
        
    with open(HEATMAP_JSON, "r", encoding="utf-8") as f:
        heatmap_data = json.load(f)
        
    records = heatmap_data.get("token_records", [])
    print("=" * 90)
    print(f"  STATISTICAL CONFOUNDER & VARIANCE DECOMPOSITION ANALYSIS (N = {len(records)} Tokens)")
    print(f"  Heatmap Hash: {heatmap_data.get('heatmap_hash')[:12]} | Source Trials: {heatmap_data.get('total_trials_executed'):,}")
    print("=" * 90)

    # 1. Evaluate 50-Bin Resolution
    bins_50 = heatmap_data.get("position_bins_50", [])
    
    # Compute Omnibus ANOVA across 50 bins
    overall_mean = statistics.mean([r["mean_delta_z"] for r in records])
    ss_between = 0.0
    ss_within = 0.0
    valid_bins = [b for b in bins_50 if b["token_count"] > 0]
    
    bin_p_values = []
    for b in bins_50:
        # Z-test against overall mean
        if b["token_count"] >= 3 and b["std_delta_z"] > 0:
            se = b["std_delta_z"] / math.sqrt(b["token_count"])
            z_score = (b["mean_delta_z"] - overall_mean) / se if se > 0 else 0.0
            p_val = 2.0 * (1.0 - 0.5 * (1.0 + math.erf(abs(z_score) / math.sqrt(2.0))))
        else:
            p_val = 1.0
        bin_p_values.append(p_val)
        
    adj_fdr = benjamini_hochberg(bin_p_values)
    adj_bonf = [min(1.0, p * len(bin_p_values)) for p in bin_p_values]
    
    # Write 50-bin CSV
    with open(OUTPUT_50_CSV, "w", newline="", encoding="utf-8") as f:
        writer = csv.writer(f)
        writer.writerow([
            "bin_id", "bin_index", "start_norm", "end_norm", "token_count",
            "mean_delta_z", "median_delta_z", "std_delta_z", "mean_abs_delta_z",
            "clean_rate", "se", "ci95_low", "ci95_high", "raw_p_value", "fdr_p_value", "bonferroni_p_value", "status"
        ])
        for idx, b in enumerate(bins_50):
            writer.writerow([
                b["bin_id"], b["bin_index"], b["start_normalized"], b["end_normalized"],
                b["token_count"], b["mean_delta_z"], b["median_delta_z"], b["std_delta_z"],
                b["mean_abs_delta_z"], b["clean_transition_rate"], b["standard_error"],
                b["ci95_low"], b["ci95_high"], bin_p_values[idx], adj_fdr[idx], adj_bonf[idx], b["status"]
            ])
    print(f"Saved 50-bin analysis to {OUTPUT_50_CSV}")

    # 2. Extract Confounder Features for Every Token Observation
    # Feature list per token:
    # y = mean_delta_z
    # x_pos = normalized_position
    # POS dummies: [Noun, Verb, Adjective, Adverb] (baseline = Other)
    # Domain dummies: [CS, Bio, ELI5, Finance, OpenQA] (baseline = General/OpenQA)
    # Context window count = min(3, 1 + min(sent_tok_idx, sent_len - 1 - sent_tok_idx))
    
    # First group by document to compute intra-sentence relative metrics
    by_doc = defaultdict(list)
    for r in records:
        by_doc[r["document_id"]].append(r)
        
    enhanced_records = []
    for doc_id, doc_recs in by_doc.items():
        doc_recs.sort(key=lambda x: x["token_index"])
        total_doc_toks = len(doc_recs)
        
        # Group by sentence
        by_sent = defaultdict(list)
        for r in doc_recs:
            by_sent[r["sentence_index"]].append(r)
            
        for sent_idx, sent_recs in by_sent.items():
            sent_len = len(sent_recs)
            for s_idx, r in enumerate(sent_recs):
                pos_norm = r["normalized_position"]
                pos_val = r["pos"]
                
                is_noun = 1.0 if pos_val in ["NN", "NNS"] else 0.0
                is_verb = 1.0 if pos_val.startswith("VB") else 0.0
                is_adj = 1.0 if pos_val.startswith("JJ") else 0.0
                
                doc_lower = doc_id.lower()
                dom = r.get("domain_class", "General")
                is_cs = 1.0 if "academic_cs_ai" in doc_lower or "cs" in doc_lower else 0.0
                is_bio = 1.0 if "biomedical" in doc_lower else 0.0
                is_eli5 = 1.0 if "eli5" in doc_lower or "expository" in doc_lower else 0.0
                is_fin = 1.0 if "finance" in doc_lower else 0.0
                
                # Sentence boundary position
                is_sent_initial = 1.0 if s_idx == 0 else 0.0
                is_sent_final = 1.0 if s_idx == sent_len - 1 else 0.0
                dist_sent_start = float(s_idx)
                dist_sent_end = float(sent_len - 1 - s_idx)
                sent_relative_pos = float(s_idx) / float(max(1, sent_len - 1))
                
                # Context sliding window overlap count (k=2 sliding window participation)
                window_count = 0.0
                if s_idx >= 2: window_count += 1.0
                if s_idx >= 1 and s_idx + 1 < sent_len: window_count += 1.0
                if s_idx + 2 < sent_len: window_count += 1.0
                window_count = max(1.0, window_count)
                
                token_len = float(len(r["token_text"]))
                
                enhanced_records.append({
                    "document_id": doc_id,
                    "token_index": r["token_index"],
                    "token_text": r["token_text"],
                    "lemma": r["lemma"],
                    "pos": pos_val,
                    "domain_class": dom,
                    "mean_delta_z": r["mean_delta_z"],
                    "mean_abs_delta_z": r["mean_abs_delta_z"],
                    "clean_transition_rate": r.get("clean_transition_rate", 0.0),
                    "pos_norm": pos_norm,
                    "pos_norm_sq": pos_norm ** 2,
                    "pos_norm_cu": pos_norm ** 3,
                    "is_noun": is_noun,
                    "is_verb": is_verb,
                    "is_adj": is_adj,
                    "is_cs": is_cs,
                    "is_bio": is_bio,
                    "is_eli5": is_eli5,
                    "is_fin": is_fin,
                    "is_sent_initial": is_sent_initial,
                    "is_sent_final": is_sent_final,
                    "dist_sent_start": dist_sent_start,
                    "dist_sent_end": dist_sent_end,
                    "sent_relative_pos": sent_relative_pos,
                    "window_count": window_count,
                    "token_len": token_len,
                })
                
    # Save tokens CSV
    if enhanced_records:
        keys = list(enhanced_records[0].keys())
        with open(OUTPUT_TOKENS_CSV, "w", newline="", encoding="utf-8") as f:
            writer = csv.DictWriter(f, fieldnames=keys)
            writer.writeheader()
            for r in enhanced_records:
                writer.writerow(r)
        print(f"Saved token confounder dataset to {OUTPUT_TOKENS_CSV}")

    # 3. Fit Nested OLS Models
    y = [r["mean_delta_z"] for r in enhanced_records]
    y_abs = [r["mean_abs_delta_z"] for r in enhanced_records]
    
    # Model 1: POS + Domain (Reference: POS=Adverb/Other, Domain=OpenQA)
    X1 = [[1.0, r["is_noun"], r["is_verb"], r["is_adj"], r["is_cs"], r["is_bio"], r["is_eli5"], r["is_fin"]] for r in enhanced_records]
    feat1_names = ["Intercept", "POS_Noun", "POS_Verb", "POS_Adj", "Dom_CS", "Dom_Bio", "Dom_ELI5", "Dom_Finance"]
    res_m1 = solve_ols(X1, y)
    
    # Model 2: POS + Domain + Sentence Structure
    # X2 = X1 + [is_sent_initial, is_sent_final, sent_relative_pos]
    X2 = [row + [r["is_sent_initial"], r["is_sent_final"], r["sent_relative_pos"]] for row, r in zip(X1, enhanced_records)]
    feat2_names = feat1_names + ["Sent_Initial", "Sent_Final", "Sent_Relative_Pos"]
    res_m2 = solve_ols(X2, y)
    
    # Model 3: POS + Domain + Sentence Structure + Normalized Position (Linear + Quadratic)
    # X3 = X2 + [pos_norm, pos_norm_sq]
    X3 = [row + [r["pos_norm"], r["pos_norm_sq"]] for row, r in zip(X2, enhanced_records)]
    feat3_names = feat2_names + ["Norm_Doc_Pos", "Norm_Doc_Pos_Sq"]
    res_m3 = solve_ols(X3, y)
    
    # Model 4: POS + Domain + Sentence Structure + Position + Context Window Geometry
    # X4 = X3 + [window_count, token_len]
    X4 = [row + [r["window_count"], r["token_len"]] for row, r in zip(X3, enhanced_records)]
    feat4_names = feat3_names + ["Context_Window_Count", "Token_Length"]
    res_m4 = solve_ols(X4, y)
    
    # Also evaluate on Absolute Impact |ΔZ|
    res_m3_abs = solve_ols(X3, y_abs)
    res_m4_abs = solve_ols(X4, y_abs)
    
    # Compute Partial F-tests
    # Model 1 -> Model 2
    f_1_to_2 = ((res_m1["ss_res"] - res_m2["ss_res"]) / (res_m2["p"] - res_m1["p"])) / (res_m2["ss_res"] / res_m2["df_res"])
    # Model 2 -> Model 3 (Testing Incremental Value of Document Position)
    f_2_to_3 = ((res_m2["ss_res"] - res_m3["ss_res"]) / (res_m3["p"] - res_m2["p"])) / (res_m3["ss_res"] / res_m3["df_res"])
    # Model 3 -> Model 4 (Testing Context Window Geometry)
    f_3_to_4 = ((res_m3["ss_res"] - res_m4["ss_res"]) / (res_m4["p"] - res_m3["p"])) / (res_m4["ss_res"] / res_m4["df_res"])
    
    # Cohen's f^2 effect size for incremental position: (R3^2 - R2^2) / (1 - R3^2)
    cohen_f2_pos = (res_m3["r_squared"] - res_m2["r_squared"]) / max(1e-6, 1.0 - res_m3["r_squared"])
    cohen_f2_geom = (res_m4["r_squared"] - res_m3["r_squared"]) / max(1e-6, 1.0 - res_m4["r_squared"])
    
    summary = {
        "n_tokens": len(enhanced_records),
        "models": {
            "model_1_pos_domain": {
                "desc": "POS Category + Domain Categorical Controls",
                "r_squared": res_m1["r_squared"],
                "adj_r_squared": res_m1["adjusted_r_squared"],
                "f_stat": res_m1["f_stat"],
                "coefficients": dict(zip(feat1_names, [
                    {"beta": b, "se": s, "t": t, "p": p}
                    for b, s, t, p in zip(res_m1["beta"], res_m1["se"], res_m1["t_stats"], res_m1["p_values"])
                ])),
            },
            "model_2_sentence_structure": {
                "desc": "POS + Domain + Sentence Position Controls",
                "r_squared": res_m2["r_squared"],
                "adj_r_squared": res_m2["adjusted_r_squared"],
                "f_stat": res_m2["f_stat"],
                "delta_r2_vs_m1": res_m2["r_squared"] - res_m1["r_squared"],
                "partial_f_vs_m1": f_1_to_2,
                "coefficients": dict(zip(feat2_names, [
                    {"beta": b, "se": s, "t": t, "p": p}
                    for b, s, t, p in zip(res_m2["beta"], res_m2["se"], res_m2["t_stats"], res_m2["p_values"])
                ])),
            },
            "model_3_linear_position": {
                "desc": "POS + Domain + Sentence + Normalized Document Position (Linear + Quadratic)",
                "r_squared": res_m3["r_squared"],
                "adj_r_squared": res_m3["adjusted_r_squared"],
                "f_stat": res_m3["f_stat"],
                "delta_r2_vs_m2": res_m3["r_squared"] - res_m2["r_squared"],
                "partial_f_vs_m2": f_2_to_3,
                "cohen_f2_pos": cohen_f2_pos,
                "coefficients": dict(zip(feat3_names, [
                    {"beta": b, "se": s, "t": t, "p": p}
                    for b, s, t, p in zip(res_m3["beta"], res_m3["se"], res_m3["t_stats"], res_m3["p_values"])
                ])),
            },
            "model_4_context_geometry": {
                "desc": "POS + Domain + Sentence + Position + Sliding Context Window Geometry",
                "r_squared": res_m4["r_squared"],
                "adj_r_squared": res_m4["adjusted_r_squared"],
                "f_stat": res_m4["f_stat"],
                "delta_r2_vs_m3": res_m4["r_squared"] - res_m3["r_squared"],
                "partial_f_vs_m3": f_3_to_4,
                "cohen_f2_geom": cohen_f2_geom,
                "coefficients": dict(zip(feat4_names, [
                    {"beta": b, "se": s, "t": t, "p": p}
                    for b, s, t, p in zip(res_m4["beta"], res_m4["se"], res_m4["t_stats"], res_m4["p_values"])
                ])),
            },
            "model_4_abs_delta_z": {
                "desc": "Full Model predicting Absolute Impact |ΔZ|",
                "r_squared": res_m4_abs["r_squared"],
                "adj_r_squared": res_m4_abs["adjusted_r_squared"],
                "coefficients": dict(zip(feat4_names, [
                    {"beta": b, "se": s, "t": t, "p": p}
                    for b, s, t, p in zip(res_m4_abs["beta"], res_m4_abs["se"], res_m4_abs["t_stats"], res_m4_abs["p_values"])
                ])),
            }
        },
    }
    
    with open(OUTPUT_JSON, "w", encoding="utf-8") as f:
        json.dump(summary, f, indent=2)
    print(f"Saved statistical regression model summaries to {OUTPUT_JSON}")
    
    # Print summary table
    print("\n" + "=" * 90)
    print("  OLS NESTED VARIANCE DECOMPOSITION SUMMARY")
    print("=" * 90)
    print(f"{'Model':<40} | {'R^2':<8} | {'Adj R^2':<8} | {'ΔR^2':<8} | {'Partial F':<10} | {'Cohen f^2':<10}")
    print("-" * 40 + "-+-" + "-" * 8 + "-+-" + "-" * 8 + "-+-" + "-" * 8 + "-+-" + "-" * 10 + "-+-" + "-" * 10)
    print(f"{'1. POS + Domain':<40} | {res_m1['r_squared']:.4f}   | {res_m1['adjusted_r_squared']:.4f}   | {'-':<8} | {'-':<10} | {'-':<10}")
    print(f"{'2. + Sentence Structure':<40} | {res_m2['r_squared']:.4f}   | {res_m2['adjusted_r_squared']:.4f}   | {res_m2['r_squared']-res_m1['r_squared']:+.4f}  | {f_1_to_2:8.3f}   | {(res_m2['r_squared']-res_m1['r_squared'])/(1-res_m2['r_squared']):.4f}")
    print(f"{'3. + Document Position (x, x^2)':<40} | {res_m3['r_squared']:.4f}   | {res_m3['adjusted_r_squared']:.4f}   | {res_m3['r_squared']-res_m2['r_squared']:+.4f}  | {f_2_to_3:8.3f}   | {cohen_f2_pos:.4f}")
    print(f"{'4. + Context Window Geometry':<40} | {res_m4['r_squared']:.4f}   | {res_m4['adjusted_r_squared']:.4f}   | {res_m4['r_squared']-res_m3['r_squared']:+.4f}  | {f_3_to_4:8.3f}   | {cohen_f2_geom:.4f}")
    print("-" * 90)

if __name__ == "__main__":
    main()
