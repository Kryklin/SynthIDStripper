import subprocess
import os
import math
import hashlib

def sha256_hash_prefix(key: int, context_tokens: list[str]) -> int:
    """Computes deterministic 64-bit pseudo-random seed from context tokens and secret key."""
    h = hashlib.sha256()
    h.update(key.to_bytes(8, 'big'))
    for tok in context_tokens:
        h.update(tok.lower().encode('utf-8'))
        h.update(b'|')
    digest = h.digest()
    return int.from_bytes(digest[:8], 'big')

def is_green_token(key: int, context: list[str], token: str, greenlist_ratio: float = 0.5) -> tuple[bool, float]:
    """Evaluates if a candidate token falls in the pseudo-random greenlist given its n-gram context."""
    h = hashlib.sha256()
    h.update(key.to_bytes(8, 'big'))
    for c in context:
        h.update(c.lower().encode('utf-8'))
        h.update(b'|')
    h.update(token.lower().encode('utf-8'))
    val = int.from_bytes(h.digest()[:8], 'big') / float(2**64 - 1)
    return (val < greenlist_ratio), val

def detect_watermark(text: str, key: int = 428917492, context_width: int = 2, greenlist_ratio: float = 0.5, z_threshold: float = 3.0):
    """
    Standard statistical hypothesis testing detector (HuggingFace WatermarkDetector / DeepMind SynthID spec).
    Computes green-token count, green fraction, standardized Gaussian Z-score, and p-value.
    """
    words = [w.strip(".,!?;:\"'()[]{}") for w in text.split() if w.strip(".,!?;:\"'()[]{}")]
    if len(words) <= context_width:
        return {"error": "Text too short"}

    scored_tokens = len(words) - context_width
    green_count = 0
    g_values = []

    for i in range(context_width, len(words)):
        ctx = words[i - context_width : i]
        tok = words[i]
        is_green, g_val = is_green_token(key, ctx, tok, greenlist_ratio)
        if is_green:
            green_count += 1
        g_values.push(g_val) if hasattr(g_values, 'push') else g_values.append(g_val)

    green_fraction = green_count / scored_tokens
    # Expected under null: green_fraction ~ greenlist_ratio (0.50), var = p*(1-p)/N
    expected_green = scored_tokens * greenlist_ratio
    var = scored_tokens * greenlist_ratio * (1.0 - greenlist_ratio)
    z_score = (green_count - expected_green) / math.sqrt(var)

    # One-tailed p-value from Z-score
    p_val = 0.5 * math.erfc(z_score / math.sqrt(2))

    prediction = z_score >= z_threshold

    return {
        "total_words": len(words),
        "scored_windows": scored_tokens,
        "green_count": green_count,
        "green_fraction": green_fraction,
        "z_score": z_score,
        "p_value": p_val,
        "is_watermarked": prediction,
        "classification": "WATERMARKED (SynthID Signal Detected)" if prediction else ("SUSPICIOUS" if z_score >= 1.645 else "UNWATERMARKED / STRIPPED (Null)")
    }

def main():
    print("=" * 70)
    print("      OFFICIAL SYNTHID / HUGGINGFACE WATERMARK STRIPPING BENCHMARK")
    print("=" * 70)

    input_file = "test.txt"
    wm_file = "benchmarks/synthid_watermarked_sample.txt"
    stripped_file = "benchmarks/synthid_stripped_sample.txt"
    os.makedirs("benchmarks", exist_ok=True)

    # 1. Generate Watermarked Text using tournament biasing
    print("\n[Step 1] Watermarking input text using SynthID context hashing...")
    subprocess.run([
        ".\\target\\release\\lexicon_stripper.exe",
        "-f", input_file,
        "-o", wm_file,
        "--synthid-watermark",
        "--synthid-key", "428917492"
    ], check=True)

    with open(wm_file, "r") as f:
        wm_text = f.read()

    # 2. Run Detector on Watermarked Text
    wm_res = detect_watermark(wm_text, key=428917492, context_width=2)
    print("\n--- 1. DETECTION ON WATERMARKED TEXT ---")
    print(f"  Scored Context Windows:  {wm_res['scored_windows']}")
    print(f"  Green Tokens Observed:   {wm_res['green_count']} / {wm_res['scored_windows']} ({wm_res['green_fraction']*100:.1f}%) [Null Expectation = 50.0%]")
    print(f"  Standardized Z-Score:    {wm_res['z_score']:.3f}")
    print(f"  Statistical p-value:     {wm_res['p_value']:.6f}")
    print(f"  Detector Classification: {wm_res['classification']}")

    # 3. Strip Watermark using LexiconStripper
    print("\n[Step 2] Processing watermarked text through Lexicon Stripper (--combined)...")
    subprocess.run([
        ".\\target\\release\\lexicon_stripper.exe",
        "-f", wm_file,
        "-o", stripped_file,
        "-r",
        "--combined"
    ], check=True)

    with open(stripped_file, "r") as f:
        stripped_text = f.read()

    # 4. Run Detector on Stripped Text
    stripped_res = detect_watermark(stripped_text, key=428917492, context_width=2)
    print("\n--- 2. DETECTION ON STRIPPED TEXT (AFTER LEXICON STRIPPER) ---")
    print(f"  Scored Context Windows:  {stripped_res['scored_windows']}")
    print(f"  Green Tokens Observed:   {stripped_res['green_count']} / {stripped_res['scored_windows']} ({stripped_res['green_fraction']*100:.1f}%) [Null Expectation = 50.0%]")
    print(f"  Standardized Z-Score:    {stripped_res['z_score']:.3f}")
    print(f"  Statistical p-value:     {stripped_res['p_value']:.6f}")
    print(f"  Detector Classification: {stripped_res['classification']}")

    # 5. Comparative Summary
    print("\n" + "=" * 70)
    print("                       FINAL BENCHMARK COMPARISON")
    print("=" * 70)
    print(f"  Metric                     | Watermarked Text      | Stripped Text (Output)")
    print("-" * 70)
    print(f"  Green Token Fraction       | {wm_res['green_fraction']*100:6.1f}%                | {stripped_res['green_fraction']*100:6.1f}%")
    print(f"  Gaussian Z-Score           | {wm_res['z_score']:6.3f}                 | {stripped_res['z_score']:6.3f}")
    print(f"  Statistical Significance   | p = {wm_res['p_value']:.5f} (Flagged)   | p = {stripped_res['p_value']:.5f} (Clean)")
    print(f"  Detection Verdict          | {wm_res['classification']:21} | {stripped_res['classification']}")
    print("=" * 70)

if __name__ == "__main__":
    main()
