# Phiano Utilities & Scripts (`scripts/`)

Command-line utilities, model downloaders, corpus extractors, and end-to-end demonstration scripts.

---

## Script Inventory

| Script | Description |
|:---|:---|
| [`demo.py`](./demo.py) | Interactive showcase of Phiano's phase manifold, semantic similarity, and Kuramoto coupling |
| [`cli_walkthrough.py`](./cli_walkthrough.py) | Automated walkthrough of CLI commands and REPL features |
| [`ask_rust.py`](./ask_rust.py) | Interactive Q&A interface querying the Rust Book trained manifold |
| [`fetch_rust_book.py`](./fetch_rust_book.py) | Downloads and processes the official Rust Book into sentence training chunks |
| [`download_phi4.py`](./download_phi4.py) | Downloads Phi-4 GGUF quantized models for hybrid cognitive reasoning |
| [`download_phi4_vision.py`](./download_phi4_vision.py) | Downloads Phi-4 MultiModal / Vision weights |
| [`train_and_compose_story.py`](./train_and_compose_story.py) | Trains a custom literary corpus and generates poetic compositions |

---

## Execution

```bash
# Run the interactive demo
python scripts/demo.py

# Download the Rust Book corpus
python scripts/fetch_rust_book.py

# Download quantized model weights
python scripts/download_phi4.py
```
