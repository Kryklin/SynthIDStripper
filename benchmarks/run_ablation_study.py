import subprocess
import os
import math
import hashlib
import json

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
        "p_value": p_val,
        "is_watermarked": prediction,
        "classification": "WATERMARKED" if prediction else ("SUSPICIOUS" if z_score >= 1.645 else "CLEAN / NULL")
    }

def main():
    print("=" * 95)
    print("      SYNTHID WATERMARK ABLATION STUDY: MEASURING LAYER-BY-LAYER SIGNAL DECAY")
    print("=" * 95)

    wm_file = "synthid_watermarked.txt"
    if not os.path.exists(wm_file):
        print(f"Error: {wm_file} not found. Generating...")
        subprocess.run([
            ".\\target\\release\\lexicon_stripper.exe",
            "-f", "test.txt",
            "-o", wm_file,
            "--synthid-watermark",
            "--synthid-key", "428917492"
        ], check=True)

    with open(wm_file, "r") as f:
        wm_text = f.read()

    # Initial baseline watermark detection
    base_eval = detect_watermark(wm_text)
    base_z = base_eval["z_score"]

    runs = [
        ("A", "Watermarked Baseline (No Transforms)", [], False),
        ("B", "Lexical Only", ["--lexical"], True),
        ("C", "Syntax Restructuring Only", ["--syntax"], True),
        ("D", "Cadence Variation Only", ["--cadence"], True),
        ("E", "Function Words Only", ["--function-words"], True),
        ("F", "Combined (All Layers)", ["--combined"], True),
    ]

    results = []

    for label, desc, flags, needs_exec in runs:
        out_file = f"benchmarks/ablation_run_{label}.txt"
        os.makedirs("benchmarks", exist_ok=True)

        if not needs_exec:
            text = wm_text
            turnover = 0.0
            replaced = 0
        else:
            cmd = [
                ".\\target\\release\\lexicon_stripper.exe",
                "-f", wm_file,
                "-o", out_file,
                "-j"
            ] + flags
            res = subprocess.run(cmd, capture_output=True, text=True, check=True)
            report_data = json.loads(res.stdout)
            text = report_data["transformed_text"]
            turnover = report_data["report"]["lexical_turnover"] * 100.0
            replaced = report_data["report"]["replaced_words"]

        eval_res = detect_watermark(text)
        current_z = eval_res["z_score"]
        delta_z = ((base_z - current_z) / base_z) * 100.0 if base_z != 0 else 0.0

        results.append({
            "label": label,
            "desc": desc,
            "replaced": replaced,
            "turnover": turnover,
            "green_frac": eval_res["green_fraction"] * 100.0,
            "z_score": current_z,
            "delta_z": delta_z,
            "p_val": eval_res["p_value"],
            "verdict": eval_res["classification"]
        })

    print("\n" + "-" * 115)
    print(f"{'Run':4} | {'Configuration':34} | {'Turnover':8} | {'Replaced':8} | {'Green %':7} | {'Z-Score':8} | {'Signal Reduction':16} | {'p-value':8} | {'Verdict'}")
    print("-" * 115)
    for r in results:
        print(f"{r['label']:4} | {r['desc']:34} | {r['turnover']:6.1f}%  | {r['replaced']:8} | {r['green_frac']:5.1f}%  | {r['z_score']:7.3f} | {r['delta_z']:14.1f}%  | {r['p_val']:7.4f} | {r['verdict']}")
    print("-" * 115)

if __name__ == "__main__":
    main()
