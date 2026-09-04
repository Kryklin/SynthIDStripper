#!/usr/bin/env python3
"""
Downloads and prepares a stratified, multi-domain benchmark corpus of verified
AI-generated texts from the Human ChatGPT Comparison (HC3) and standard AI detection datasets.
"""

import os
import sys
import json
import urllib.request

DATASET_DOMAINS = {
    "expository_eli5": "https://huggingface.co/datasets/Hello-SimpleAI/HC3/resolve/main/reddit_eli5.jsonl",
    "academic_cs_ai": "https://huggingface.co/datasets/Hello-SimpleAI/HC3/resolve/main/wiki_csai.jsonl",
    "finance_business": "https://huggingface.co/datasets/Hello-SimpleAI/HC3/resolve/main/finance.jsonl",
    "biomedical_science": "https://huggingface.co/datasets/Hello-SimpleAI/HC3/resolve/main/medicine.jsonl",
    "open_domain_qa": "https://huggingface.co/datasets/Hello-SimpleAI/HC3/resolve/main/open_qa.jsonl",
}

OUTPUT_DIR = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "data", "corpus")

def download_domain_samples(domain_name, url, target_count=20, min_word_count=120):
    domain_dir = os.path.join(OUTPUT_DIR, domain_name)
    os.makedirs(domain_dir, exist_ok=True)
    
    print(f"[*] Downloading samples for domain: {domain_name} ...")
    req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64)"})
    
    saved_count = 0
    try:
        with urllib.request.urlopen(req, timeout=30) as resp:
            for line in resp:
                if saved_count >= target_count:
                    break
                line_str = line.decode("utf-8", errors="ignore").strip()
                if not line_str:
                    continue
                try:
                    data = json.loads(line_str)
                    chatgpt_answers = data.get("chatgpt_answers", [])
                    if not chatgpt_answers:
                        continue
                    
                    # Take the longest, most substantive answer
                    best_answer = max(chatgpt_answers, key=len).strip()
                    word_count = len(best_answer.split())
                    if word_count < min_word_count:
                        continue
                    
                    sample_id = f"sample_{saved_count+1:03d}"
                    file_path = os.path.join(domain_dir, f"{sample_id}.txt")
                    meta_path = os.path.join(domain_dir, f"{sample_id}.meta.json")
                    
                    with open(file_path, "w", encoding="utf-8") as f:
                        f.write(best_answer)
                        
                    meta = {
                        "id": sample_id,
                        "domain": domain_name,
                        "question": data.get("question", ""),
                        "word_count": word_count,
                        "source_dataset": "HC3",
                    }
                    with open(meta_path, "w", encoding="utf-8") as f:
                        json.dump(meta, f, indent=2)
                        
                    saved_count += 1
                except Exception as parse_err:
                    continue
    except Exception as fetch_err:
        print(f"    [!] Error downloading {domain_name}: {fetch_err}")
        return 0

    print(f"    [+] Saved {saved_count} verified long-form AI texts in {domain_dir}")
    return saved_count

def main():
    print("==================================================")
    print("      MULTI-DOMAIN AI CORPUS INGESTION TOOL       ")
    print("==================================================")
    
    total_saved = 0
    samples_per_domain = 20 # 20 per domain x 5 domains = 100 long-form texts (total ~20,000+ words)
    
    for domain, url in DATASET_DOMAINS.items():
        count = download_domain_samples(domain, url, target_count=samples_per_domain, min_word_count=120)
        total_saved += count
        
    print("==================================================")
    print(f"[✓] Corpus ingestion complete. Total samples: {total_saved}")
    print(f"[✓] Storage path: {OUTPUT_DIR}")
    print("==================================================")

if __name__ == "__main__":
    main()
