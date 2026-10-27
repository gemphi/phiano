# phiano — architecture

## The crates it stands on

```mermaid
graph LR
    C["phiano<br/>the instrument: agents, personas, readers, servers, the binaries"]
    X_phical["phical<br/>used by 3 modules"]
    C --> X_phical
    X_phiclm["phiclm<br/>used by 3 modules"]
    C --> X_phiclm
    X_phiecs["phiecs<br/>used by 41 modules"]
    C --> X_phiecs
    X_phient["phient<br/>used by 5 modules"]
    C --> X_phient
    X_phifno["phifno<br/>used by 1 module"]
    C --> X_phifno
    X_phistd["phistd<br/>used by 18 modules"]
    C --> X_phistd
    X_phiton["phiton<br/>used by 9 modules"]
    C --> X_phiton
```

## The modules in layers (a module sits above all it uses)

```mermaid
graph BT
    subgraph L0["layer 0"]
        m_attention["attention"]
        m_bpe["bpe"]
        m_config["config"]
        m_layers["layers"]
        m_nonlinear["nonlinear"]
        m_scribe["scribe"]
        m_sys["sys"]
        m_tokenizer["tokenizer"]
    end
    subgraph L1["layer 1"]
        m_arena["arena"]
        m_data["data"]
        m_frames["frames"]
        m_lab["lab"]
        m_memory["memory"]
        m_roles["roles"]
    end
    subgraph L2["layer 2"]
        m_eval["eval"]
        m_kinds["kinds"]
        m_seam["seam"]
    end
    subgraph L3["layer 3"]
        CYC3["ONE CYCLE, 15 modules use each other:<br/>chunker, cognitive, compose, conception, flow<br/>generate, learn, metrics, persona, phinet<br/>relational, sources, storage, topdown, trainer"]
        m_oscillator["oscillator"]
    end
    subgraph L4["layer 4"]
        m_correction["correction"]
        m_curriculum["curriculum"]
        m_envision["envision"]
        m_helix["helix"]
        m_instruction["instruction"]
        m_reasoning["reasoning"]
        m_synthesis["synthesis"]
        m_synthetic["synthetic"]
        m_wiki["wiki"]
    end
    subgraph L5["layer 5"]
        m_lifelong["lifelong"]
    end
    subgraph L6["layer 6"]
        CYC6["ONE CYCLE, 2 modules use each other:<br/>command, drivers"]
    end
    subgraph L7["layer 7"]
        m_model["model"]
    end
    subgraph L8["layer 8"]
        m_server["server"]
    end
    L1 -->|"uses what is below"| L0
    L2 -->|"uses what is below"| L1
    L3 -->|"uses what is below"| L2
    L4 -->|"uses what is below"| L3
    L5 -->|"uses what is below"| L4
    L6 -->|"uses what is below"| L5
    L7 -->|"uses what is below"| L6
    L8 -->|"uses what is below"| L7
```

## The hubs (most used inside the crate)

```mermaid
graph LR
    ALL["the other modules of phiano"]
    m_tokenizer["tokenizer"]
    ALL -->|"28 modules use it"| m_tokenizer
    m_config["config"]
    ALL -->|"26 modules use it"| m_config
    m_trainer["trainer"]
    ALL -->|"20 modules use it"| m_trainer
    m_eval["eval"]
    ALL -->|"11 modules use it"| m_eval
    m_cognitive["cognitive"]
    ALL -->|"10 modules use it"| m_cognitive
    m_chunker["chunker"]
    ALL -->|"9 modules use it"| m_chunker
    m_generate["generate"]
    ALL -->|"9 modules use it"| m_generate
    m_memory["memory"]
    ALL -->|"5 modules use it"| m_memory
```

## Modules, one diagram each

- [arena](../../docs/phiano/arena.md)
- [attention](../../docs/phiano/attention.md)
- [bpe](../../docs/phiano/bpe.md)
- [chunker](../../docs/phiano/chunker.md)
- [cognitive](../../docs/phiano/cognitive.md)
- [command](../../docs/phiano/command.md)
- [compose](../../docs/phiano/compose.md)
- [conception](../../docs/phiano/conception.md)
- [config](../../docs/phiano/config.md)
- [correction](../../docs/phiano/correction.md)
- [curriculum](../../docs/phiano/curriculum.md)
- [data](../../docs/phiano/data.md)
- [drivers](../../docs/phiano/drivers.md)
- [envision](../../docs/phiano/envision.md)
- [eval](../../docs/phiano/eval.md)
- [flow](../../docs/phiano/flow.md)
- [frames](../../docs/phiano/frames.md)
- [generate](../../docs/phiano/generate.md)
- [helix](../../docs/phiano/helix.md)
- [instruction](../../docs/phiano/instruction.md)
- [kinds](../../docs/phiano/kinds.md)
- [lab](../../docs/phiano/lab.md)
- [layers](../../docs/phiano/layers.md)
- [learn](../../docs/phiano/learn.md)
- [lifelong](../../docs/phiano/lifelong.md)
- [memory](../../docs/phiano/memory.md)
- [metrics](../../docs/phiano/metrics.md)
- [model](../../docs/phiano/model.md)
- [nonlinear](../../docs/phiano/nonlinear.md)
- [oscillator](../../docs/phiano/oscillator.md)
- [persona](../../docs/phiano/persona.md)
- [phinet](../../docs/phiano/phinet.md)
- [reasoning](../../docs/phiano/reasoning.md)
- [relational](../../docs/phiano/relational.md)
- [roles](../../docs/phiano/roles.md)
- [scribe](../../docs/phiano/scribe.md)
- [seam](../../docs/phiano/seam.md)
- [server](../../docs/phiano/server.md)
- [sources](../../docs/phiano/sources.md)
- [storage](../../docs/phiano/storage.md)
- [synthesis](../../docs/phiano/synthesis.md)
- [synthetic](../../docs/phiano/synthetic.md)
- [sys](../../docs/phiano/sys.md)
- [tokenizer](../../docs/phiano/tokenizer.md)
- [topdown](../../docs/phiano/topdown.md)
- [trainer](../../docs/phiano/trainer.md)
- [wiki](../../docs/phiano/wiki.md)

What is left to do: [TODO.md](TODO.md)
