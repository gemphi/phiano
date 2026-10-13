"""Checks phiano::bpe against Hugging Face `tokenizers` (ByteLevel BPE on the
same vocab.json + merges.txt) — the probe sentences from bin/bpe_check plus
every sentence of Emma."""
import json, sys
from tokenizers import Tokenizer, models, pre_tokenizers, decoders
tok = Tokenizer(models.BPE.from_file("data/tokenizers/phi4/vocab.json", "data/tokenizers/phi4/merges.txt"))
tok.pre_tokenizer = pre_tokenizers.ByteLevel(add_prefix_space=False, use_regex=True)
tok.decoder = decoders.ByteLevel()
probe = json.load(open("data/tokenizers/phi4/parity_rust.json"))
bad = 0
for p in probe:
    hf = tok.encode(p["text"]).ids
    ok = hf == p["ids"]; bad += not ok
    print("ok " if ok else "FAIL", repr(p["text"][:50]), "" if ok else (hf[:20], p["ids"][:20]))
print("probe:", "ALL MATCH" if bad == 0 else f"{bad} differ")

emma = json.load(open("data/tokenizers/phi4/parity_rust_emma.json"))
bad = sum(tok.encode(p["text"]).ids != p["ids"] for p in emma)
print(f"Emma: {len(emma)} sentences, {len(emma)-bad} identical, {bad} differ")
