# phiano — architecture

## The crates it stands on

```mermaid
graph LR
    C["phiano<br/>the instrument: agents, personas, readers, servers, the binaries"]
    X_phical["phical<br/>used by 1 module"]
    C --> X_phical
    X_phiclm["phiclm<br/>used by 2 modules"]
    C --> X_phiclm
    X_phiecs["phiecs<br/>used by 11 modules"]
    C --> X_phiecs
    X_phient["phient<br/>used by 12 modules"]
    C --> X_phient
    X_phistd["phistd<br/>used by 2 modules"]
    C --> X_phistd
    X_phiton["phiton<br/>used by 2 modules"]
    C --> X_phiton
```

## The modules in layers (a module sits above all it uses)

```mermaid
graph BT
    subgraph L0["layer 0"]
        m_config["config"]
        m_sys["sys"]
        m_tokenizer["tokenizer"]
    end
    subgraph L1["layer 1"]
        m_arena["arena"]
        m_core["core"]
        m_frames["frames"]
        m_lab["lab"]
        m_persona["persona"]
        m_wiki["wiki"]
    end
    subgraph L2["layer 2"]
        m_instruction["instruction"]
        m_seam["seam"]
    end
    subgraph L3["layer 3"]
        CYC3["ONE CYCLE, 2 modules use each other:<br/>command, drivers"]
    end
    subgraph L4["layer 4"]
        m_model["model"]
    end
    subgraph L5["layer 5"]
        m_server["server"]
    end
    L1 -->|"uses what is below"| L0
    L2 -->|"uses what is below"| L1
    L3 -->|"uses what is below"| L2
    L4 -->|"uses what is below"| L3
    L5 -->|"uses what is below"| L4
```

## The hubs (most used inside the crate)

```mermaid
graph LR
    ALL["the other modules of phiano"]
    m_tokenizer["tokenizer"]
    ALL -->|"9 modules use it"| m_tokenizer
    m_config["config"]
    ALL -->|"5 modules use it"| m_config
    m_core["core"]
    ALL -->|"4 modules use it"| m_core
    m_command["command"]
    ALL -->|"3 modules use it"| m_command
    m_instruction["instruction"]
    ALL -->|"2 modules use it"| m_instruction
    m_persona["persona"]
    ALL -->|"2 modules use it"| m_persona
    m_arena["arena"]
    ALL -->|"1 modules use it"| m_arena
    m_drivers["drivers"]
    ALL -->|"1 modules use it"| m_drivers
```

## Modules, one diagram each

- [arena](../../docs/phiano/arena.md)
- [command](../../docs/phiano/command.md)
- [config](../../docs/phiano/config.md)
- [core](../../docs/phiano/core.md)
- [drivers](../../docs/phiano/drivers.md)
- [frames](../../docs/phiano/frames.md)
- [instruction](../../docs/phiano/instruction.md)
- [lab](../../docs/phiano/lab.md)
- [model](../../docs/phiano/model.md)
- [persona](../../docs/phiano/persona.md)
- [seam](../../docs/phiano/seam.md)
- [server](../../docs/phiano/server.md)
- [sys](../../docs/phiano/sys.md)
- [tokenizer](../../docs/phiano/tokenizer.md)
- [wiki](../../docs/phiano/wiki.md)

What is left to do: [TODO.md](TODO.md)
