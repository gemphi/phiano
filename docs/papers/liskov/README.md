# Barbara Liskov, read in Phient's terms — sixteen papers, January to September 2026

In Phient's world, a word (w) is nothing but what it relates to, under what conditions (c). The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

Barbara Liskov (born Barbara Huberman) took her Stanford PhD in 1968 under John McCarthy with a program that played chess endgames using a forcing tree and two comparison functions, **better** and **worse**, and no evaluation number. That is where Phient's three operators come from: `better`, `worse` and `loss` in `phinet::liskov` (docs/86). These sixteen papers read her work in order, from that thesis through the Venus operating system, abstract data types, CLU, Argus, promises, viewstamped replication, the substitution principle and Byzantine fault tolerance to her Turing lecture. Each claim about her work was fetched from the work or a primary record; each paper's section 16 says what was reached and what was not. Every Phient number comes from `PHIENT_FACTS.md`. She worked at MITRE, Harvard and MIT; she was never at IBM.

| # | dated | our paper | on her work | words |
|---|---|---|---|---|
| 1 | 2026-01-12 | [Better and worse before the number: the forcing tree as a comparison a pair makes](01_chess_end_games.md) | A Program to Play Chess End Games | 7,358 |
| 2 | 2026-01-26 | [A level is a condition: the Venus machine and the layered word](02_venus_operating_system.md) | The Design of the Venus Operating System | 7,339 |
| 3 | 2026-02-09 | [Reliability by structure: the partition as a word that owns its resources](03_design_methodology.md) | A Design Methodology for Reliable Software Systems | 7,224 |
| 4 | 2026-02-23 | [A type is its operations: the cluster read as a word](04_abstract_data_types.md) | Programming with Abstract Data Types | 7,167 |
| 5 | 2026-03-09 | [The word specified by its operations: six criteria for what a relation may say](05_specification_techniques.md) | Specification Techniques for Data Abstractions | 6,852 |
| 6 | 2026-03-23 | [A type is its operations: clusters, iterators and where clauses, read as words](06_clu_abstraction_mechanisms.md) | Abstraction Mechanisms in CLU | 7,173 |
| 7 | 2026-04-06 | [The named exit: a procedure that ends in one of several conditions](07_exception_handling.md) | Exception Handling in CLU | 7,314 |
| 8 | 2026-04-20 | [The hidden rep: a word as a cluster, a channel as a type generator, decoding as an iterator](08_clu_reference_manual.md) | CLU Reference Manual | 7,320 |
| 9 | 2026-05-04 | [The stable byte: a word as a guardian, a comparison as an action that commits or aborts](09_guardians_and_actions.md) | Guardians and Actions: Linguistic Support for Robust, Distributed Programs | 7,256 |
| 10 | 2026-05-18 | [The word as a guardian: stable relations, handler calls that happen once or not at all, and the nudge as a nested action](10_argus.md) | Distributed Programming in Argus | 7,183 |
| 11 | 2026-06-01 | [The typed placeholder: a call that returns before it answers](11_promises.md) | Promises: Linguistic Support for Efficient Asynchronous Procedure Calls in Distributed Systems | 7,469 |
| 12 | 2026-06-15 | [The viewstamp as a relation: a view as a channel, a majority as a condition, a lost event as loss](12_viewstamped_replication.md) | Viewstamped Replication: A New Primary Copy Method to Support Highly-Available Distributed Systems | 7,293 |
| 13 | 2026-06-29 | [Substitution is the test: is-a read as a claim about programs](13_data_abstraction_and_hierarchy.md) | Data Abstraction and Hierarchy | 7,292 |
| 14 | 2026-07-13 | [A stand-in is what still holds: substitution read as a claim about words](14_behavioral_subtyping.md) | A Behavioral Notion of Subtyping | 7,260 |
| 15 | 2026-08-10 | [Agreement among liars: a word as 3f+1 replicas, a channel as a view, meaning as a quorum](15_practical_byzantine_fault_tolerance.md) | Practical Byzantine Fault Tolerance | 7,194 |
| 16 | 2026-09-14 | [The only way to use the objects: what a word owes to an interface](16_power_of_abstraction.md) | The Power of Abstraction | 7,257 |

Total: 115,951 words. The brief the writers followed is `BRIEF.md`.
