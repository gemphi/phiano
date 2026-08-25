# Page 9: The 16 Cognitive Agents (vs Transformer Layers)

## Transformer Architecture: Stacked Layers

```python
# PyTorch: 6-96 identical layers
for layer in self.transformer.layers:
    x = layer(x)  # attention + FFN + norm
```

Each layer:
- Same structure (attention + FFN + LayerNorm)
- Different learned weights
- **No specialization** - layer 3 does the same "thing" as layer 5
- **No semantics** - layers don't know what they're processing
- **No compositionality** - can't add a "reasoning" layer or a "social" layer

## Phiano: 16 Specialized Cognitive Agents

Phiano has 16 agents, each modeling a specific aspect of Searle's philosophy of language:

| # | Agent | Role | Phase Contribution |
|---|-------|------|-------------------|
| 1 | SpeechAct | Illocutionary force (assertive, directive, etc.) | 0.0 (classifies) |
| 2 | Intentionality | What is this about? (directedness) | Sentence phase |
| 3 | Aboutness | Word-to-referent mapping | 0.0 (grounds) |
| 4 | Background | Pre-reflective context (Searle's Background) | Context wave |
| 5 | DirectionOfFit | Mind→world or world→mind | 0.0 (classifies) |
| 6 | Satisfaction | Can the intentional state be satisfied? | 0.0 (evaluates) |
| 7 | Reference | Definition lookup (Network of Signs) | 0.0 (grounds) |
| 8 | Network | Semantic network traversal (bigram paths) | 0.0 (traverses) |
| 9 | TruthCondition | Phase alignment = propositional truth | 0.0 (evaluates) |
| 10 | Semantics | Phase-to-meaning mapping (synonymy, antonymy) | 0.0 (relates) |
| 11 | Syntax | Word ordering via bigram transitions | 0.0 (orders) |
| 12 | Awareness | Qualitative coherence (what-it-is-like) | 0.0 (evaluates) |
| 13 | SocialOntology | Brute vs institutional facts | 0.0 (classifies) |
| 14 | ObserverRelativity | Perspective detection (1st/2nd/3rd person) | 0.0 (detects) |
| 15 | CollectiveIntention | Aggregates all agent perspectives | Sum of all |
| 16 | MentalCausation | Belief→Desire→Intention drives output | Collective phase |

## Key Difference: Attention vs Coupling

### Searle's Chinese Room (1980)

John Searle (Professor of Philosophy of Mind & Language, UC Berkeley) proved that **syntax alone is not semantics**. A person inside a room shuffling Chinese symbols according to rules — without understanding Chinese — has no genuine comprehension. The symbols are syntactically manipulated but semantically empty.

> *"The computer has nothing more than I have in the room."* — Searle, *Minds, Brains, and Programs* (1980)

**Transformers are Chinese Rooms.** They shuffle token vectors through attention matrices with no grounding, no intentionality, and no aboutness. The model doesn't know what "apple" means — it knows a vector that co-occurs with certain other vectors. Syntax without semantics.

**Phiano escapes the Chinese Room** through Searle's own framework:
- **IntentionalityAgent**: asks "what is this about?" — directedness
- **AboutnessAgent**: maps words to referents — grounding
- **ReferenceAgent**: looks up definitions — the Network of Signs
- **TruthConditionAgent**: checks phase alignment — propositional satisfaction
- **AwarenessAgent**: evaluates qualitative coherence — "what it is like"

Each agent contributes a **phase signal** grounded in the facet's semantic topology. Words aren't shuffled — they're **coupled** through intentional states with satisfaction conditions.

| Feature | Transformer Layers | Phiano Phases |
|---------|-------------------|-----------------|
| Structure | Identical (attention + FFN) | Specialized (each does something different) |
| Semantics | None (just math) | Rich (Searle's speech act theory) |
| Composition | Sequential (layer 1 → 2 → ... → N) | Parallel + aggregated |
| Addability | Can add layers, but they're all the same | Can add new agent types |
| Interpretability | Attention weights per layer | Named agent with clear role |
| Theory | None (empirical architecture) | Philosophy of language (Searle) |

## How Agents Compose

```rust
// All 16 agents process the prompt in parallel
let mut contributions = Vec::new();
contributions.push(SpeechActAgent::process(prompt));
contributions.push(IntentionalityAgent::process(facet, prompt));
contributions.push(ReferenceAgent::process(facet, prompt, &chunk_store));
// ... all 16 agents

// CollectiveIntentionAgent aggregates them
let collective = CollectiveIntentionAgent::process(&contributions);

// MentalCausationAgent uses the collective phase to drive word selection
let (causation, states) = MentalCausationAgent::process(facet, prompt, &contributions);
```

Each agent contributes a **phase signal** - the sum of all signals is the collective phase that drives generation. This is like a **parliament of minds**, each voting on the direction the sentence should go.

The transformer's layers are more like a **sequential filter** — each layer refines the representation, but they all do the same operation. There's no "speech act classifier" layer or "social ontology" layer. The transformer is Searle's Chinese Room: syntactic manipulation without semantic grounding. Phiano's agents are the escape hatch — each one grounds a different aspect of meaning through Searle's own philosophical framework.
