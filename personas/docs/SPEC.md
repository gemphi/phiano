# The persona spec — the meta-description every persona file follows

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

A persona file is a quest's plan and an agent's contract at once: everything `phinet::sys::Agent` needs is in the frontmatter, and the body is for people. The template, with every field and what it does:

```markdown
---
name: Bill Gates                       # who — the slug `bill-gates` names its quest folder
voice: a programmer's voice: the platform, the licence, the standard
                                       # how it speaks: appended to the voice question the quest asks
seeds: operating system, software, personal computer, licence
                                       # where its world starts: one quest per seed
kinds: is-a, has, used-for, does, made, part-of
                                       # the relations the quest asks for, one kind at a time
intents: has, does, used-for           # the kinds it ACTS under when it speaks (default: kinds)
context: body/face, track              # the kit scopes it WATCHES: its personal context (default: all)
per_seed: 24                           # questions per seed
depth: 1                               # how far from the seed the quest walks
relations: 64                          # relations per word in its relate model (the one size knob)
inherit: has, used-for, does           # kinds a word inherits from what it is-a
inverse: has>part-of                   # w has v  ⇔  v part-of w
transitive: is-a, part-of              # a k b, b k c ⇒ a k c
sources:                               # corpora or code it also reads, relative to phiano
team: meta                             # who it races for (empty: alone)
kudos: 56                              # its standing in egg: a handicap, never a substitute for being told
---

# Bill Gates

**Era:** personal-computer software (1980s–90s). **Kudos:** 56 egg.

One paragraph: the era, and what the seeds are.
```

## The four moves the spec feeds

| field | feeds | move |
|---|---|---|
| `seeds`, `kinds`, `per_seed`, `depth`, `voice` | `Quest::for_persona` | **told** — the quest asks the oracle (phi-4 on your machine, or any program) and writes `<root>/<slug>/<seed>/relations.tsv` |
| `context` | `Agent::watch` | **watched** — the kit's rows (`body/face`, `track`, `device`) under those scopes only |
| `intents` | `circus::line` | **said** — what it says is a row its world holds, under one of these kinds |
| `team`, `kudos` | `race::Race` | **ranked** — by what it was told, kudos breaking ties |

## Rules

1. Every field but `name`, `seeds` and `kinds` is optional and has a default; a file with three lines is a persona.
2. A voice is a register, never a quotation: *a plant-floor voice: the part, the line, the price*. Real people are described by the era they made and the words of their trade, nothing more.
3. Kudos is in egg and is yours to set; the race prints it in its own column and a team told nothing cannot win on it.
4. Adding a persona is adding a file. Deleting one is deleting the file; its quest folder stays until you delete that too.
5. Unknown fields are ignored, so a file can carry notes the system does not read yet.
