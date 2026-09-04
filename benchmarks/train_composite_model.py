#!/usr/bin/env python3
"""
Composite Token-Context Sensitivity Model Training, Cross-Validation & Feature Ablation
1. Constructs high-dimensional token-context feature vectors strictly from Development partition.
2. Trains Models A through E using deterministic 5-fold cross-validation.
3. Evaluates feature group ablations (spatial, POS, domain, context, window geometry, candidate features).
4. Freezes data/composite_model.json, data/token_sensitivity_records.json, and data/token_sensitivity_records.csv.
"""

import os
import sys
import json
import csv
import math
import hashlib
import statistics
import random
from collections import defaultdict

if sys.stdout.encoding != "utf-8":
    try:
        sys.stdout.reconfigure(encoding="utf-8")
    except Exception:
        pass

BASE_DIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
HEATMAP_JSON = os.path.join(BASE_DIR, "data", "sensitivity_heatmap.json")
TOKENS_JSON = os.path.join(BASE_DIR, "data", "token_sensitivity_records.json")
TOKENS_CSV = os.path.join(BASE_DIR, "data", "token_sensitivity_records.csv")
MODEL_JSON = os.path.join(BASE_DIR, "data", "composite_model.json")

def solve_ridge(X, y, alpha=1.0):
    """
    Solves Ridge Regression: beta = (X^T X + alpha * I)^(-1) X^T y
    """
    n = len(y)
    p = len(X[0])
    
    XtX = [[0.0 for _ in range(p)] for _ in range(p)]
    for i in range(p):
        for j in range(p):
            XtX[i][j] = sum(X[k][i] * X[k][j] for k in range(n))
            
    Xty = [sum(X[k][i] * y[k] for k in range(n)) for i in range(p)]
    
    # Add ridge penalty to all coefficients except intercept (column 0)
    for i in range(p):
        if i > 0:
            XtX[i][i] += alpha
        else:
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
                
    return b

def predict(X, beta):
    return [sum(row[i] * beta[i] for i in range(len(beta))) for row in X]

def compute_metrics(y_true, y_pred):
    n = len(y_true)
    if n == 0:
        return {"r2": 0.0, "mae": 0.0, "rmse": 0.0, "spearman": 0.0}
    
    mae = statistics.mean([abs(yt - yp) for yt, yp in zip(y_true, y_pred)])
    rmse = math.sqrt(statistics.mean([(yt - yp)**2 for yt, yp in zip(y_true, y_pred)]))
    
    y_mean = statistics.mean(y_true)
    ss_tot = sum((yt - y_mean)**2 for yt in y_true)
    ss_res = sum((yt - yp)**2 for yt, yp in zip(y_true, y_pred))
    r2 = 1.0 - (ss_res / ss_tot) if ss_tot > 0 else 0.0
    
    # Spearman rank correlation
    rank_true = get_ranks(y_true)
    rank_pred = get_ranks(y_pred)
    spearman = pearson_corr(rank_true, rank_pred)
    
    return {"r2": r2, "mae": mae, "rmse": rmse, "spearman": spearman}

def get_ranks(vals):
    indexed = sorted(enumerate(vals), key=lambda x: x[1])
    ranks = [0.0] * len(vals)
    for rank_idx, (orig_idx, _) in enumerate(indexed):
        ranks[orig_idx] = float(rank_idx + 1)
    return ranks

def pearson_corr(x, y):
    n = len(x)
    if n < 2: return 0.0
    mx = statistics.mean(x)
    my = statistics.mean(y)
    cov = sum((x[i] - mx) * (y[i] - my) for i in range(n))
    sx = math.sqrt(sum((x[i] - mx)**2 for i in range(n)))
    sy = math.sqrt(sum((y[i] - my)**2 for i in range(n)))
    return (cov / (sx * sy)) if (sx * sy) > 0 else 0.0

def k_fold_cross_validation(X, y, k=5, alpha=1.0):
    n = len(y)
    indices = list(range(n))
    rng = random.Random(42)
    rng.shuffle(indices)
    
    fold_size = n // k
    all_y_true = []
    all_y_pred = []
    
    for fold in range(k):
        val_idx = set(indices[fold * fold_size : (fold + 1) * fold_size if fold != k - 1 else n])
        train_X = [X[i] for i in range(n) if i not in val_idx]
        train_y = [y[i] for i in range(n) if i not in val_idx]
        val_X = [X[i] for i in val_idx]
        val_y = [y[i] for i in val_idx]
        
        beta = solve_ridge(train_X, train_y, alpha=alpha)
        preds = predict(val_X, beta)
        
        all_y_true.extend(val_y)
        all_y_pred.extend(preds)
        
    return compute_metrics(all_y_true, all_y_pred)

def main():
    if not os.path.exists(HEATMAP_JSON):
        print(f"Error: {HEATMAP_JSON} does not exist.")
        sys.exit(1)
        
    with open(HEATMAP_JSON, "r", encoding="utf-8") as f:
        heatmap_data = json.load(f)
        
    records = heatmap_data.get("token_records", [])
    print("=" * 90)
    print(f"  COMPOSITE TOKEN-CONTEXT SENSITIVITY MODEL TRAINING (N = {len(records)} Tokens)")
    print(f"  Heatmap Hash: {heatmap_data.get('heatmap_hash')[:12]} | Total Profile Trials: {heatmap_data.get('total_trials_executed'):,}")
    print("=" * 90)

    # 1. Feature Extraction & Engineering for All Development Tokens
    by_doc = defaultdict(list)
    for r in records:
        by_doc[r["document_id"]].append(r)
        
    feature_records = []
    for doc_id, doc_recs in by_doc.items():
        doc_recs.sort(key=lambda x: x["token_index"])
        total_doc_toks = len(doc_recs)
        
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
                is_adv = 1.0 if pos_val.startswith("RB") else 0.0
                
                doc_lower = doc_id.lower()
                is_cs = 1.0 if "academic_cs_ai" in doc_lower or "cs" in doc_lower else 0.0
                is_bio = 1.0 if "biomedical" in doc_lower else 0.0
                is_eli5 = 1.0 if "eli5" in doc_lower or "expository" in doc_lower else 0.0
                is_fin = 1.0 if "finance" in doc_lower else 0.0
                
                is_sent_initial = 1.0 if s_idx == 0 else 0.0
                is_sent_final = 1.0 if s_idx == sent_len - 1 else 0.0
                dist_sent_start = float(s_idx)
                dist_sent_end = float(sent_len - 1 - s_idx)
                sent_relative_pos = float(s_idx) / float(max(1, sent_len - 1))
                
                # Context window overlap count
                window_count = 0.0
                if s_idx >= 2: window_count += 1.0
                if s_idx >= 1 and s_idx + 1 < sent_len: window_count += 1.0
                if s_idx + 2 < sent_len: window_count += 1.0
                window_count = max(1.0, window_count)
                
                tok_text = r["token_text"]
                tok_len = float(len(tok_text))
                vowels = sum(1 for c in tok_text.lower() if c in "aeiouy")
                syllable_est = float(max(1, vowels))
                
                # Heatmap bin features
                bin_10_score = pos_norm
                bin_20_score = pos_norm ** 2
                
                # Candidate quality features
                clean_rate = r.get("clean_transition_rate", 0.5)
                n_trials = float(r.get("n_trials", 1))
                std_dz = float(r.get("std_delta_z", 0.1))
                
                rec = {
                    "document_id": doc_id,
                    "token_index": r["token_index"],
                    "sentence_index": r["sentence_index"],
                    "token_text": tok_text,
                    "lemma": r["lemma"],
                    "pos": pos_val,
                    "domain_class": r.get("domain_class", "General"),
                    "target_abs_delta_z": r["mean_abs_delta_z"],
                    "target_delta_z": r["mean_delta_z"],
                    # Features
                    "pos_norm": pos_norm,
                    "pos_norm_sq": pos_norm ** 2,
                    "pos_norm_cu": pos_norm ** 3,
                    "sent_relative_pos": sent_relative_pos,
                    "is_sent_initial": is_sent_initial,
                    "is_sent_final": is_sent_final,
                    "dist_sent_start": dist_sent_start,
                    "dist_sent_end": dist_sent_end,
                    "is_noun": is_noun,
                    "is_verb": is_verb,
                    "is_adj": is_adj,
                    "is_adv": is_adv,
                    "is_cs": is_cs,
                    "is_bio": is_bio,
                    "is_eli5": is_eli5,
                    "is_fin": is_fin,
                    "window_count": window_count,
                    "token_len": tok_len,
                    "syllable_est": syllable_est,
                    "clean_rate": clean_rate,
                    "n_trials": n_trials,
                }
                feature_records.append(rec)

    # Save token sensitivity records
    with open(TOKENS_JSON, "w", encoding="utf-8") as f:
        json.dump(feature_records, f, indent=2)
    print(f"Saved complete token sensitivity records to {TOKENS_JSON}")
    
    if feature_records:
        keys = list(feature_records[0].keys())
        with open(TOKENS_CSV, "w", newline="", encoding="utf-8") as f:
            writer = csv.DictWriter(f, fieldnames=keys)
            writer.writeheader()
            for r in feature_records:
                writer.writerow(r)
        print(f"Saved token sensitivity records to {TOKENS_CSV}")

    # 2. Model Development & Cross-Validation
    y_target = [r["target_abs_delta_z"] for r in feature_records]
    
    # Model A: Mean baseline
    y_mean = statistics.mean(y_target)
    preds_a = [y_mean] * len(y_target)
    metrics_a = compute_metrics(y_target, preds_a)
    
    # Feature sets for Models B, C, D, E
    features_b = ["pos_norm", "pos_norm_sq", "sent_relative_pos"]
    features_c = ["is_noun", "is_verb", "is_adj", "is_cs", "is_bio", "is_eli5", "is_fin"]
    features_d = features_b + features_c + ["is_sent_initial", "is_sent_final", "window_count"]
    features_e = features_d + ["pos_norm_cu", "token_len", "syllable_est", "dist_sent_start", "dist_sent_end"]
    
    def build_matrix(fnames):
        mat = []
        for r in feature_records:
            row = [1.0] + [r[fn] for fn in fnames]
            mat.append(row)
        return mat

    X_b = build_matrix(features_b)
    X_c = build_matrix(features_c)
    X_d = build_matrix(features_d)
    X_e = build_matrix(features_e)
    
    # 5-fold cross validation for each model
    cv_b = k_fold_cross_validation(X_b, y_target, k=5, alpha=2.0)
    cv_c = k_fold_cross_validation(X_c, y_target, k=5, alpha=2.0)
    cv_d = k_fold_cross_validation(X_d, y_target, k=5, alpha=2.0)
    cv_e = k_fold_cross_validation(X_e, y_target, k=5, alpha=2.0)
    
    # 3. Systematic Feature Group Ablations on Model E
    abl_spatial = [fn for fn in features_e if not fn.startswith("pos_norm") and fn != "sent_relative_pos"]
    abl_pos = [fn for fn in features_e if not fn.startswith("is_noun") and not fn.startswith("is_verb") and not fn.startswith("is_adj")]
    abl_domain = [fn for fn in features_e if not fn.startswith("is_cs") and not fn.startswith("is_bio") and not fn.startswith("is_eli5") and not fn.startswith("is_fin")]
    abl_context = [fn for fn in features_e if not fn.startswith("is_sent_") and not fn.startswith("dist_sent")]
    abl_window = [fn for fn in features_e if fn != "window_count"]
    
    cv_abl_spatial = k_fold_cross_validation(build_matrix(abl_spatial), y_target, k=5, alpha=2.0)
    cv_abl_pos = k_fold_cross_validation(build_matrix(abl_pos), y_target, k=5, alpha=2.0)
    cv_abl_domain = k_fold_cross_validation(build_matrix(abl_domain), y_target, k=5, alpha=2.0)
    cv_abl_context = k_fold_cross_validation(build_matrix(abl_context), y_target, k=5, alpha=2.0)
    cv_abl_window = k_fold_cross_validation(build_matrix(abl_window), y_target, k=5, alpha=2.0)
    
    # 4. Fit Final Model E on full Dev Partition & Compute Standardization Params
    # Standardize features (mean=0, std=1)
    f_means = []
    f_stds = []
    for fn in features_e:
        vals = [r[fn] for r in feature_records]
        m = statistics.mean(vals)
        s = statistics.stdev(vals) if len(vals) > 1 else 1.0
        f_means.append(m)
        f_stds.append(s if s > 1e-6 else 1.0)
        
    X_e_std = []
    for r in feature_records:
        row = [1.0]
        for idx, fn in enumerate(features_e):
            row.append((r[fn] - f_means[idx]) / f_stds[idx])
        X_e_std.append(row)
        
    final_beta = solve_ridge(X_e_std, y_target, alpha=2.0)
    intercept = final_beta[0]
    weights = final_beta[1:]
    
    # Model Hash
    model_obj = {
        "model_name": "Composite_Token_Context_Ridge_Predictor",
        "version": "1.0.0",
        "target": "mean_abs_delta_z",
        "n_samples": len(feature_records),
        "source_corpus_hash": heatmap_data.get("source_corpus_hash"),
        "random_seed": 42,
        "ridge_alpha": 2.0,
        "intercept": intercept,
        "feature_names": features_e,
        "weights": weights,
        "feature_means": f_means,
        "feature_stds": f_stds,
        "cv_performance": {
            "model_a_mean": metrics_a,
            "model_b_position": cv_b,
            "model_c_pos_domain": cv_c,
            "model_d_pos_domain_context": cv_d,
            "model_e_full_composite": cv_e,
        },
        "feature_group_ablations": {
            "full_model": cv_e,
            "spatial_removed": cv_abl_spatial,
            "pos_removed": cv_abl_pos,
            "domain_removed": cv_abl_domain,
            "context_removed": cv_abl_context,
            "window_geometry_removed": cv_abl_window,
        },
        "model_hash": "",
    }
    
    m_bytes = json.dumps(model_obj, sort_keys=True).encode("utf-8")
    model_hash = hashlib.sha256(m_bytes).hexdigest()
    model_obj["model_hash"] = model_hash
    
    with open(MODEL_JSON, "w", encoding="utf-8") as f:
        json.dump(model_obj, f, indent=2)
    print(f"Saved frozen composite model to {MODEL_JSON} (Hash: {model_hash[:12]})")
    
    # Print CV Summary Table
    print("\n" + "=" * 90)
    print("  5-FOLD CROSS-VALIDATION MODEL COMPARISON (Dev Partition, N = 797)")
    print("=" * 90)
    print(f"{'Model':<35} | {'CV R^2':<10} | {'MAE':<10} | {'RMSE':<10} | {'Spearman rho':<12}")
    print("-" * 35 + "-+-" + "-" * 10 + "-+-" + "-" * 10 + "-+-" + "-" * 10 + "-+-" + "-" * 12)
    print(f"{'Model A: Mean Baseline':<35} | {metrics_a['r2']:+.4f}     | {metrics_a['mae']:.4f}     | {metrics_a['rmse']:.4f}     | {metrics_a['spearman']:+.4f}")
    print(f"{'Model B: Position-Only':<35} | {cv_b['r2']:+.4f}     | {cv_b['mae']:.4f}     | {cv_b['rmse']:.4f}     | {cv_b['spearman']:+.4f}")
    print(f"{'Model C: POS + Domain':<35} | {cv_c['r2']:+.4f}     | {cv_c['mae']:.4f}     | {cv_c['rmse']:.4f}     | {cv_c['spearman']:+.4f}")
    print(f"{'Model D: Pos + POS + Context':<35} | {cv_d['r2']:+.4f}     | {cv_d['mae']:.4f}     | {cv_d['rmse']:.4f}     | {cv_d['spearman']:+.4f}")
    print(f"{'Model E: Full Composite':<35} | {cv_e['r2']:+.4f}     | {cv_e['mae']:.4f}     | {cv_e['rmse']:.4f}     | {cv_e['spearman']:+.4f}")
    print("-" * 90)
    
    print("\n" + "=" * 90)
    print("  FEATURE GROUP ABLATION ANALYSIS (Impact on Model E Performance)")
    print("=" * 90)
    print(f"{'Ablated Group':<35} | {'CV R^2':<10} | {'Delta R^2':<10} | {'Spearman rho':<12}")
    print("-" * 35 + "-+-" + "-" * 10 + "-+-" + "-" * 10 + "-+-" + "-" * 12)
    print(f"{'Full Model E (Reference)':<35} | {cv_e['r2']:+.4f}     | {'-':<10} | {cv_e['spearman']:+.4f}")
    print(f"{'Spatial Features Removed':<35} | {cv_abl_spatial['r2']:+.4f}     | {cv_abl_spatial['r2'] - cv_e['r2']:+.4f}    | {cv_abl_spatial['spearman']:+.4f}")
    print(f"{'POS Category Removed':<35} | {cv_abl_pos['r2']:+.4f}     | {cv_abl_pos['r2'] - cv_e['r2']:+.4f}    | {cv_abl_pos['spearman']:+.4f}")
    print(f"{'Domain Category Removed':<35} | {cv_abl_domain['r2']:+.4f}     | {cv_abl_domain['r2'] - cv_e['r2']:+.4f}    | {cv_abl_domain['spearman']:+.4f}")
    print(f"{'Sentence Context Removed':<35} | {cv_abl_context['r2']:+.4f}     | {cv_abl_context['r2'] - cv_e['r2']:+.4f}    | {cv_abl_context['spearman']:+.4f}")
    print(f"{'Window Geometry Removed':<35} | {cv_abl_window['r2']:+.4f}     | {cv_abl_window['r2'] - cv_e['r2']:+.4f}    | {cv_abl_window['spearman']:+.4f}")
    print("-" * 90)

if __name__ == "__main__":
    main()
