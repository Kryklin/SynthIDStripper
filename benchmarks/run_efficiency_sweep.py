import subprocess
import os
import math
import hashlib
import json
import statistics

def is_green_token(key: int, context: list[str], token: str, greenlist_ratio: float = 0.5) -> tuple[bool, float]:
    h = hashlib.sha256()
    h.update(key.to_bytes(8, 'big'))
    for c in context:
        h.update(c.lower().encode('utf-8'))
        h.update(b'|')
    h.update(token.lower().encode('utf-8'))
    val = int.from_bytes(h.digest()[:8], 'big') / float(2**64 - 1)
    return (val < greenlist_ratio), val

def detect_watermark(text: str, key: int = 428917492, context_width: int = 2, greenlist_ratio: float = 0.5, z_threshold: float = 3.0):
    words = [w.strip(".,!?;:\"'()[]{}") for w in text.split() if w.strip(".,!?;:\"'()[]{}")]
    if len(words) <= context_width:
        return {"error": "Text too short"}

    scored_tokens = len(words) - context_width
    green_count = 0

    for i in range(context_width, len(words)):
        ctx = words[i - context_width : i]
        tok = words[i]
        is_green, _ = is_green_token(key, ctx, tok, greenlist_ratio)
        if is_green:
            green_count += 1

    green_fraction = green_count / scored_tokens
    expected_green = scored_tokens * greenlist_ratio
    var = scored_tokens * greenlist_ratio * (1.0 - greenlist_ratio)
    z_score = (green_count - expected_green) / math.sqrt(var)
    p_val = 0.5 * math.erfc(z_score / math.sqrt(2))
    prediction = z_score >= z_threshold

    return {
        "scored_windows": scored_tokens,
        "green_count": green_count,
        "green_fraction": green_fraction,
        "z_score": z_score,
        "abs_z": abs(z_score),
        "p_value": p_val,
        "is_watermarked": prediction
    }

def main():
    print("=" * 80)
    print("      SYNTHID EFFICIENCY SWEEP: DOSE-RESPONSE CURVE (TURNOVER -> |Z|)")
    print("=" * 80)

    wm_file = "synthid_watermarked.txt"
    if not os.path.exists(wm_file):
        subprocess.run([
            ".\\target\\release\\lexicon_stripper.exe",
            "-f", "test.txt",
            "-o", wm_file,
            "--synthid-watermark",
            "--synthid-key", "428917492"
        ], check=True)

    with open(wm_file, "r") as f:
        wm_text = f.read()

    base_eval = detect_watermark(wm_text)
    base_abs_z = base_eval["abs_z"]

    probabilities = [0.0, 0.15, 0.30, 0.45, 0.60, 0.75, 0.90, 1.00]
    seeds = [101, 202, 303, 404, 505, 606, 707]

    results = []

    for prob in probabilities:
        if prob == 0.0:
            results.append({
                "prob": 0.0,
                "mean_turnover": 0.0,
                "mean_abs_z": base_abs_z,
                "std_abs_z": 0.0,
                "mean_p_val": base_eval["p_value"],
                "clean_rate": 0.0
            })
            continue

        turnovers = []
        abs_zs = []
        p_vals = []
        clean_count = 0

        for seed in seeds:
            cmd = [
                ".\\target\\release\\lexicon_stripper.exe",
                "-f", wm_file,
                "-j",
                "--lexical",
                "-p", str(prob),
                "-s", str(seed)
            ]
            res = subprocess.run(cmd, capture_output=True, text=True, check=True)
            report_data = json.loads(res.stdout)
            text = report_data["transformed_text"]
            t_over = report_data["report"]["lexical_turnover"] * 100.0
            turnovers.append(t_over)

            eval_res = detect_watermark(text)
            abs_zs.append(eval_res["abs_z"])
            p_vals.append(eval_res["p_value"])
            if eval_res["abs_z"] < 1.645:
                clean_count += 1

        mean_t = statistics.mean(turnovers)
        mean_z = statistics.mean(abs_zs)
        std_z = statistics.stdev(abs_zs) if len(abs_zs) > 1 else 0.0
        mean_p = statistics.mean(p_vals)
        clean_rate = (clean_count / len(seeds)) * 100.0

        results.append({
            "prob": prob,
            "mean_turnover": mean_t,
            "mean_abs_z": mean_z,
            "std_abs_z": std_z,
            "mean_p_val": mean_p,
            "clean_rate": clean_rate
        })

    print("\n" + "-" * 85)
    print(f"{'Target Prob':12} | {'Turnover':10} | {'Mean |Z|':10} | {'Std Dev':9} | {'Mean p-value':14} | {'Clean Rate (% < 1.645)'}")
    print("-" * 85)
    for r in results:
        print(f"{r['prob']*100:6.0f}%      | {r['mean_turnover']:6.1f}%    | {r['mean_abs_z']:7.3f}    | ±{r['std_abs_z']:6.3f}   | {r['mean_p_val']:10.4f}     | {r['clean_rate']:6.0f}%")
    print("-" * 85)

    # ASCII Dose-Response Curve
    print("\n=== DOSE-RESPONSE CURVE: LEXICAL TURNOVER -> |Z| ===")
    print(" |Z| Score (Significance Threshold = 1.645 [---])")
    print(" 3.0 | * (Baseline: Z=2.85)")
    for r in results:
        bar_len = int(r["mean_abs_z"] * 12)
        bar = "#" * bar_len
        thresh_marker = "|" if bar_len >= int(1.645 * 12) else " "
        status = "FLAGGED" if r["mean_abs_z"] >= 1.645 else "CLEAN"
        print(f"{r['mean_turnover']:4.1f}% | {bar:<36} [{r['mean_abs_z']:.2f}] -> {status}")
    print("     +-------------------------------------")
    print("       0.0        1.0        2.0        3.0")

if __name__ == "__main__":
    main()
