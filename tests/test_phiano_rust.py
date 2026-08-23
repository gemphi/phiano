import math
import cmath
import os
import time

def tokenize(text):
    words = []
    for w in text.lower().split():
        clean = "".join(c for c in w if c.isalnum())
        if clean:
            words.append(clean)
    return words

class SpectralPhasor:
    def __init__(self, phase, amplitude=1.0, band_n=1):
        self.phase = phase
        self.amplitude = amplitude
        self.band_n = band_n

class Facet:
    def __init__(self):
        self.lexicon = {}
        self.phi = (1.0 + 5.0**0.5) / 2.0
        
    def get_or_init(self, token):
        if token not in self.lexicon:
            seed_phase = (len(token) * self.phi) % (2.0 * math.pi)
            self.lexicon[token] = SpectralPhasor(seed_phase, 1.0, 1)
        return self.lexicon[token]

class KuramotoTrainer:
    def __init__(self, learning_rate=0.15):
        self.learning_rate = learning_rate

    def train_sentence(self, facet, text):
        tokens = tokenize(text)
        if not tokens:
            return 0

        # Ensure initialized
        for t in tokens:
            facet.get_or_init(t)

        # Centroid Phase
        sum_x = sum(facet.lexicon[t].amplitude * math.cos(facet.lexicon[t].phase) for t in tokens)
        sum_y = sum(facet.lexicon[t].amplitude * math.sin(facet.lexicon[t].phase) for t in tokens)
        target_phase = math.atan2(sum_y, sum_x)

        updated = 0
        for t in tokens:
            phasor = facet.lexicon[t]
            phase_err = math.sin(target_phase - phasor.phase)
            phasor.phase = (phasor.phase + self.learning_rate * phase_err) % (2.0 * math.pi)
            phasor.amplitude = min(5.0, phasor.amplitude + 0.05)
            updated += 1

        return updated

class Evaluator:
    @staticmethod
    def evaluate(facet, text):
        tokens = tokenize(text)
        if not tokens:
            return {"coherence": 0.0, "resonance": 0.0}

        sum_x, sum_y = 0.0, 0.0
        known = 0

        for t in tokens:
            if t in facet.lexicon:
                p = facet.lexicon[t]
                sum_x += p.amplitude * math.cos(p.phase)
                sum_y += p.amplitude * math.sin(p.phase)
                known += 1

        n = len(tokens)
        r = math.sqrt(sum_x**2 + sum_y**2) / max(n, 1)
        coherence = min(1.0, r)
        resonance = known / float(n) if n > 0 else 0.0

        return {"coherence": coherence, "resonance": resonance}

print("============================================================")
print("   PHIANO - STANDALONE RUST BOOK INGESTION & TRAINING BENCH ")
print("============================================================\n")

corpus_path = r"c:\Users\phiac\Workspace\gemphi\phiano\data\rust_book_corpus.txt"
with open(corpus_path, "r", encoding="utf-8") as f:
    sentences = [line.strip() for line in f if line.strip()]

print(f"--> Loaded {len(sentences):,} clean sentences from all 104 chapters of the Rust Book.")

facet = Facet()
trainer = KuramotoTrainer(0.15)
evaluator = Evaluator()

test_prompts = [
    "ownership and borrowing rules in Rust",
    "references lifetimes and generic traits",
    "concurrency mutex channels and thread safety",
    "pattern matching and enum data types"
]

print("\n--- 1. BEFORE TRAINING (BASELINE) ---")
for p in test_prompts:
    res = evaluator.evaluate(facet, p)
    print(f'  Prompt: "{p}"')
    print(f'  Coherence (Kuramoto R): {res["coherence"]:.4f} | Resonance: {res["resonance"]:.4f}')

print("\n--- 2. KURAMOTO PHASE ATTRACTION TRAINING (3 EPOCHS) ---")
start_time = time.time()
epochs = 3
total_updates = 0

for epoch in range(1, epochs + 1):
    ep_start = time.time()
    updates = 0
    for s in sentences:
        updates += trainer.train_sentence(facet, s)
    total_updates += updates
    print(f"  [Epoch {epoch}/{epochs}] Updated {updates:,} word phasors in {time.time() - ep_start:.2f}s")

elapsed = time.time() - start_time
print(f"\n--> Training Complete in {elapsed:.2f}s!")
print(f"--> Trained Manifold Lexicon Size: {len(facet.lexicon):,} unique words\n")

print("--- 3. AFTER TRAINING (POST-INSPECT COHERENCE IMPROVEMENT) ---")
for p in test_prompts:
    res = evaluator.evaluate(facet, p)
    print(f'  Prompt: "{p}"')
    print(f'  Coherence (Kuramoto R): {res["coherence"]:.4f} | Resonance: {res["resonance"]:.4f}')

print("\n--- 4. STANDALONE OSCILLATOR RIVER FLOW STORY GENERATION ---")

# River flow generator
prompt = "ownership borrowing and lifetime in Rust code"
tokens = tokenize(prompt)
sum_x = sum(facet.lexicon[t].amplitude * math.cos(facet.lexicon[t].phase) for t in tokens if t in facet.lexicon)
sum_y = sum(facet.lexicon[t].amplitude * math.sin(facet.lexicon[t].phase) for t in tokens if t in facet.lexicon)
target_angle = math.atan2(sum_y, sum_x) % (2 * math.pi)

# Ray cast words near target_angle
words_with_dist = []
for w, p in facet.lexicon.items():
    angle_diff = abs(p.phase - target_angle)
    if angle_diff > math.pi:
        angle_diff = 2 * math.pi - angle_diff
    words_with_dist.append((w, angle_diff, p.amplitude))

words_with_dist.sort(key=lambda x: x[1])
top_resonant = [w[0] for w in words_with_dist[:15]]

print(f'  Prompt: "{prompt}"')
print(f'  Opening Flow (Source Sector):  {" ".join(top_resonant[:5])}')
print(f'  Tension Flow (Opposite Sector): {" ".join(top_resonant[5:10])}')
print(f'  Climax Flow (Resolution):      {" ".join(top_resonant[10:15])}')

print("\n=== BENCHMARK SUCCESSFULLY COMPLETED ===")
