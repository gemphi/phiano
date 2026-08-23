# Phiano Test Suite (`tests/`)

Comprehensive test suite for the Phiano continuous phase manifold engine, HTTP API server, and cognitive reasoning pipelines.

---

## Test Inventory

| Test Module | Description |
|:---|:---|
| [`test_cognitive.py`](./test_cognitive.py) | Tests cognitive spaces, dimensional projections, and phase alignment |
| [`test_def_chain.py`](./test_def_chain.py) | Tests recursive definition expansion chains and semantic resonance |
| [`test_def_chain2.py`](./test_def_chain2.py) | Multi-hop definition chaining and convergence analysis |
| [`test_dictionary_definitions.py`](./test_dictionary_definitions.py) | Tests Webster's dictionary ingestion and definition vectorization |
| [`test_dynamic_learning.py`](./test_dynamic_learning.py) | Real-time dynamic Kuramoto coupling and online weight adaptation |
| [`test_endpoints.py`](./test_endpoints.py) | REST API endpoints for `/v1/eval`, `/v1/learn`, `/v1/stats`, etc. |
| [`test_generate_phase.py`](./test_generate_phase.py) | Complex phasor wave superposition and harmonic token generation |
| [`test_instruction_execution.py`](./test_instruction_execution.py) | Command parsing and instruction execution via the CLI/REPL engine |
| [`test_interactive.py`](./test_interactive.py) | Interactive REPL session mocking and persona conversation tests |
| [`test_layers_phase.py`](./test_layers_phase.py) | 16-layer memory hierarchy (Surface, Pattern, Semantic, Deep) |
| [`test_phiano_rust.py`](./test_phiano_rust.py) | Rust binary integration, CLI execution, and subprocess communication |
| [`test_reason_chain.py`](./test_reason_chain.py) | Multi-step reasoning chains and destructive interference evaluation |
| [`test_reasoning_phase.py`](./test_reasoning_phase.py) | Phase-based deductive logic and truth value computation |
| [`test_synthetic_pipeline.py`](./test_synthetic_pipeline.py) | Synthetic training data generation and self-tuning cycles |
| [`test_wiki.py`](./test_wiki.py) | Wikipedia article ingestion and corpus extraction |
| [`test_wiki_api.py`](./test_wiki_api.py) | Wikipedia API integration and live streaming definition tests |

---

## Running the Tests

```bash
# Run all Python tests
pytest tests

# Run specific integration test
pytest tests/test_endpoints.py
```
