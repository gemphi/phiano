import math

class SpectralPhasor:
    def __init__(self, phase, amplitude=1.0):
        self.phase = phase
        self.amplitude = amplitude

class HierarchicalPhaseField:
    def __init__(self):
        self.layer_sectors = [64, 32, 16, 8]
        self.layers = [{} for _ in range(4)]

    def build_hierarchy(self, lexicon):
        # Layer 0: Surface Words
        self.layers[0] = lexicon

        # Layer 1: Concept Clusters (32 sectors)
        self.layers[1] = self._cluster_layer(self.layers[0], 32)

        # Layer 2: Domain Sectors (16 sectors)
        self.layers[2] = self._cluster_nodes(self.layers[1], 16)

        # Layer 3: Meta-Patterns (8 sectors)
        self.layers[3] = self._cluster_nodes(self.layers[2], 8)

    def _cluster_layer(self, lexicon, num_sectors):
        sector_sums_x = {}
        sector_sums_y = {}
        counts = {}
        sector_width = (2.0 * math.pi) / num_sectors

        for word, p in lexicon.items():
            sector = int(p.phase // sector_width) % num_sectors
            sector_sums_x[sector] = sector_sums_x.get(sector, 0.0) + p.amplitude * math.cos(p.phase)
            sector_sums_y[sector] = sector_sums_y.get(sector, 0.0) + p.amplitude * math.sin(p.phase)
            counts[sector] = counts.get(sector, 0) + 1

        clusters = {}
        for sec in range(num_sectors):
            if counts.get(sec, 0) > 0:
                sx = sector_sums_x[sec]
                sy = sector_sums_y[sec]
                phase = math.atan2(sy, sx) % (2.0 * math.pi)
                amp = math.sqrt(sx**2 + sy**2) / counts[sec]
                clusters[sec] = SpectralPhasor(phase, amp)

        return clusters

    def _cluster_nodes(self, prev_layer, num_sectors):
        sector_sums_x = {}
        sector_sums_y = {}
        counts = {}
        sector_width = (2.0 * math.pi) / num_sectors

        for sec_id, p in prev_layer.items():
            sector = int(p.phase // sector_width) % num_sectors
            sector_sums_x[sector] = sector_sums_x.get(sector, 0.0) + p.amplitude * math.cos(p.phase)
            sector_sums_y[sector] = sector_sums_y.get(sector, 0.0) + p.amplitude * math.sin(p.phase)
            counts[sector] = counts.get(sector, 0) + 1

        clusters = {}
        for sec in range(num_sectors):
            if counts.get(sec, 0) > 0:
                sx = sector_sums_x[sec]
                sy = sector_sums_y[sec]
                phase = math.atan2(sy, sx) % (2.0 * math.pi)
                amp = math.sqrt(sx**2 + sy**2) / counts[sec]
                clusters[sec] = SpectralPhasor(phase, amp)

        return clusters

    def resonate_depth(self, query_phase):
        results = []
        for level in range(1, 4):
            num_sectors = self.layer_sectors[level]
            sec_width = (2.0 * math.pi) / num_sectors
            sec_idx = int(query_phase // sec_width) % num_sectors
            if sec_idx in self.layers[level]:
                node = self.layers[level][sec_idx]
                diff = abs(node.phase - query_phase)
                if diff > math.pi:
                    diff = 2.0 * math.pi - diff
                results.append((level, num_sectors, sec_idx, diff, node.amplitude))
        return results

# Test execution
phi = (1.0 + 5.0**0.5) / 2.0
test_words = [
    "ownership", "borrowing", "lifetime", "reference", "mutability",
    "concurrency", "thread", "mutex", "channel", "future", "async",
    "struct", "enum", "trait", "impl", "pattern", "matching", "macro"
]

lexicon = {}
for w in test_words:
    phase = (len(w) * phi) % (2.0 * math.pi)
    lexicon[w] = SpectralPhasor(phase, 2.5)

field = HierarchicalPhaseField()
field.build_hierarchy(lexicon)

print("=== PHIANO PHASE 3 (MULTI-LAYER DEPTH) HIERARCHY TEST ===")
print(f"Layer 0 (Surface Words): {len(field.layers[0])} items")
print(f"Layer 1 (Concept Clusters - 32 Sectors): {len(field.layers[1])} active clusters")
print(f"Layer 2 (Domain Sectors - 16 Sectors): {len(field.layers[2])} active domains")
print(f"Layer 3 (Meta-Patterns - 8 Sectors): {len(field.layers[3])} active meta-patterns")

query_phase = (len("ownership") * phi) % (2.0 * math.pi)
depth_resonances = field.resonate_depth(query_phase)

print(f"\nQuerying Hierarchical Depth for 'ownership' (Phase: {query_phase:.4f} rad):")
for lvl, num_sec, sec_idx, diff, amp in depth_resonances:
    layer_names = ["Surface Words", "Concept Clusters", "Domain Sectors", "Meta-Patterns"]
    print(f"  Layer {lvl} [{layer_names[lvl]}] ({num_sec} sectors) -> Sector {sec_idx} | Angular Dist: {diff:.4f} rad | Amp: {amp:.4f}")

print("\n=== PHASE 3 MULTI-LAYER DEPTH VERIFICATION COMPLETE ===")
