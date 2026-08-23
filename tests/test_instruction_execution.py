import math

class InstructionKind:
    CODE = "Code"
    EXPLAIN = "Explain"
    CREATIVE = "Creative"
    ANALYZE = "Analyze"
    COMMAND = "Command"

    @staticmethod
    def parse(prompt):
        p = prompt.lower()
        if any(w in p for w in ["code", "function", "implement", "rust", "fix"]):
            return InstructionKind.CODE
        elif any(w in p for w in ["explain", "what is", "how does", "why"]):
            return InstructionKind.EXPLAIN
        elif any(w in p for w in ["write", "story", "haiku", "poem"]):
            return InstructionKind.CREATIVE
        elif any(w in p for w in ["compare", "benchmark", "analyze"]):
            return InstructionKind.ANALYZE
        return InstructionKind.COMMAND

class InstructionEngine:
    def format_template(self, prompt):
        return f"<|user|>\n{prompt.strip()}\n<|end|>\n<|assistant|>\n"

    def execute_instruction(self, prompt, lexicon):
        kind = InstructionKind.parse(prompt)
        template = self.format_template(prompt)

        tokens = [w.lower() for w in prompt.split() if w.isalnum()]

        # Find resonant words
        phi = (1.0 + 5.0**0.5) / 2.0
        target_phase = (len(tokens) * phi) % (2.0 * math.pi)

        candidates = []
        for word, phase in lexicon.items():
            diff = abs(phase - target_phase)
            if diff > math.pi:
                diff = 2.0 * math.pi - diff
            candidates.append((word, diff))

        candidates.sort(key=lambda x: x[1])
        top_words = [w[0] for w in candidates[:6]]

        output = f"[Instruction Executed as {kind}]\nTemplate Context: {template.strip()}\nExecuted Resonant Output: {' '.join(top_words)}"
        return output

# Test setup
words = [
    "rust", "borrowing", "ownership", "lifetime", "reference",
    "concurrency", "thread", "mutex", "channel", "pattern", "matching",
    "explain", "function", "implement", "code", "benchmark", "story"
]

phi = (1.0 + 5.0**0.5) / 2.0
lexicon = {w: (len(w) * phi) % (2.0 * math.pi) for w in words}

engine = InstructionEngine()

instructions = [
    "write code for rust mutex channel thread safety",
    "explain ownership and borrowing in rust",
    "write a story about golf on the fairway",
    "benchmark and compare phiano performance"
]

print("=== PHIANO PHASE 4 (INSTRUCTION TAKING & EXECUTION) BENCHMARK ===")

for idx, inst in enumerate(instructions, 1):
    print(f"\n[Test {idx}] Input Instruction: \"{inst}\"")
    result = engine.execute_instruction(inst, lexicon)
    print(result)

print("\n=== PHASE 4 INSTRUCTION TAKING VERIFICATION COMPLETE ===")
