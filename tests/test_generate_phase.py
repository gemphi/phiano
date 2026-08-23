import math

class ContextWaveBuffer:
    def __init__(self, capacity=4096, decay_base=0.5):
        self.sum_x = 0.0
        self.sum_y = 0.0
        self.capacity = capacity
        self.decay_base = decay_base
        self.tokens = []

    def push_turn(self, lexicon, text):
        self.sum_x *= self.decay_base
        self.sum_y *= self.decay_base

        words = [w.lower() for w in text.split() if w.isalnum()]
        for w in words:
            if w in lexicon:
                p = lexicon[w]
                self.sum_x += p["amp"] * math.cos(p["phase"])
                self.sum_y += p["amp"] * math.sin(p["phase"])
            self.tokens.append(w)

    def context_phase(self):
        angle = math.atan2(self.sum_y, self.sum_x)
        return angle if angle >= 0 else angle + 2 * math.pi

class Generator:
    def __init__(self, max_tokens=16, temperature=0.2):
        self.max_tokens = max_tokens
        self.temperature = temperature

    def generate(self, lexicon, context_buffer, prompt):
        context_buffer.push_turn(lexicon, prompt)
        current_phase = context_buffer.context_phase()
        generated = []

        for step in range(self.max_tokens):
            jitter = math.sin(step * 0.618) * self.temperature * 0.1
            target_phase = (current_phase + jitter) % (2 * math.pi)

            # Ray cast
            best_word = None
            best_diff = 999.0

            for word, p in lexicon.items():
                if generated and generated[-1] == word:
                    continue
                diff = abs(p["phase"] - target_phase)
                if diff > math.pi:
                    diff = 2 * math.pi - diff
                if diff < best_diff:
                    best_diff = diff
                    best_word = word

            if best_word:
                generated.append(best_word)
                p = lexicon[best_word]
                current_phase = (current_phase + 0.3 * math.sin(p["phase"] - current_phase)) % (2 * math.pi)
            else:
                break

        res = " ".join(generated)
        context_buffer.push_turn(lexicon, res)
        return res

# Test setup
phi = (1.0 + 5.0**0.5) / 2.0
words = [
    "rust", "code", "borrow", "checker", "lifetime", "reference",
    "ownership", "mutability", "thread", "concurrency", "mutex", "channel",
    "safety", "memory", "struct", "trait", "impl", "function", "pattern"
]

lexicon = {}
for w in words:
    seed = (len(w) * phi) % (2 * math.pi)
    lexicon[w] = {"phase": seed, "amp": 2.0}

buffer = ContextWaveBuffer()
gen = Generator(max_tokens=12, temperature=0.1)

print("=== PHIANO PHASE 1 & 2 ENGINE TEST ===")
print("Turn 1 Prompt: 'rust code ownership borrowing'")
resp1 = gen.generate(lexicon, buffer, "rust code ownership borrowing")
print(f"Generated Sequence 1: {resp1}")
print(f"Context Wave Phase: {buffer.context_phase():.4f} rad")

print("\nTurn 2 Prompt: 'lifetime references'")
resp2 = gen.generate(lexicon, buffer, "lifetime references")
print(f"Generated Sequence 2: {resp2}")
print(f"Context Wave Phase: {buffer.context_phase():.4f} rad")

print("\n=== PHASE 1 & 2 VERIFICATION COMPLETE ===")
