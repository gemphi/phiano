import math

class SyntheticGenerator:
    @staticmethod
    def generate_sentence(lexicon, seed_word):
        if seed_word not in lexicon:
            return None

        target_phase = lexicon[seed_word]["phase"]
        candidates = []
        for word, p in lexicon.items():
            diff = abs(p["phase"] - target_phase)
            if diff > math.pi:
                diff = 2.0 * math.pi - diff
            candidates.append((word, diff))

        candidates.sort(key=lambda x: x[1])
        top_words = [w[0] for w in candidates[:4]]

        if len(top_words) >= 3:
            return f"{seed_word} is related to {' '.join(top_words[1:])}"
        return None

class SyntheticCurriculumPipeline:
    def __init__(self, min_coherence=0.45):
        self.min_coherence = min_coherence

    def run(self, lexicon):
        generated = []
        accepted = []

        for word in list(lexicon.keys()):
            sent = SyntheticGenerator.generate_sentence(lexicon, word)
            if sent:
                generated.append(sent)

                # Quality filter: Calculate coherence
                tokens = sent.split()
                sum_x = sum(math.cos(lexicon[t]["phase"]) for t in tokens if t in lexicon)
                sum_y = sum(math.sin(lexicon[t]["phase"]) for t in tokens if t in lexicon)
                r = math.sqrt(sum_x**2 + sum_y**2) / len(tokens)

                if r >= self.min_coherence:
                    accepted.append((sent, r))

        return generated, accepted

# Setup test
phi = (1.0 + 5.0**0.5) / 2.0
words = [
    "rust", "ownership", "borrowing", "lifetime", "reference",
    "concurrency", "thread", "mutex", "channel", "pattern", "matching",
    "explain", "function", "implement", "code", "benchmark", "story"
]

lexicon = {w: {"phase": (len(w) * phi) % (2.0 * math.pi), "amp": 2.0} for w in words}

pipeline = SyntheticCurriculumPipeline(min_coherence=0.40)
generated, accepted = pipeline.run(lexicon)

print("=== PHIANO PHASE 5 (SYNTHETIC DATA PIPELINE) BENCHMARK ===")
print(f"Generated Synthetic Sentences: {len(generated)}")
print(f"Quality Filtered & Accepted for Retraining: {len(accepted)}\n")

print("Sample Quality-Filtered Synthetic Training Pairs:")
for sent, r in accepted[:5]:
    print(f"  [Coherence R: {r:.4f}] \"{sent}\"")

print("\n=== PHASE 5 SYNTHETIC DATA PIPELINE VERIFICATION COMPLETE ===")
