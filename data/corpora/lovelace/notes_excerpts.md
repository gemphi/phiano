# Ada Lovelace, *Notes* on L. F. Menabrea's "Sketch of the Analytical Engine Invented by Charles Babbage" (1843)

Verbatim excerpts, public domain. The whole text — Menabrea's Sketch and all seven Notes, A to G, with the Bernoulli table — is beside this file as `sketch_of_the_analytical_engine.txt` (27,284 words; John Walker's Fourmilab transcription, fetched 2026-11-08). These excerpts are the lines the code quotes; paper 27 reads all twelve of her ideas against phisor.

## Note A — what kind of machine it is

> "The Analytical Engine weaves algebraical patterns just as the Jacquard-loom weaves flowers and leaves."

> "While the Difference Engine can merely tabulate, and is incapable of developing, the Analytical Engine can either tabulate or develope."

> "The Analytical Engine is an embodying of the science of operations, constructed with peculiar reference to abstract number."

> "…the material expression of any indefinite function of any degree of generality and complexity, such as for instance, F(x, y, z, log x, sin y, x^p, &c.)"

> "The engine might compose elaborate and scientific pieces of music of any degree of complexity or extent."

> "…the peculiar and independent nature of the considerations which … belong to operations, as distinguished from the objects operated upon."

> "Many persons who are not conversant with mathematical studies imagine that because the business of [the Analytical Engine] is to give its results in numerical notation, the nature of its processes must consequently be arithmetical and numerical, rather than algebraical and analytical."

## Note D — arrangement

> "In almost every computation a great variety of arrangements for the succession of the processes is possible, and various considerations must influence the selections amongst them for the purposes of a calculating engine."

## Note G — the limit she set

> "The Analytical Engine has no pretensions whatever to originate anything. It can do whatever we know how to order it to perform."

> "It can follow analysis; but it has no power of anticipating any analytical relations or truths."

(Note G also contains the Bernoulli-number programme — the first published algorithm intended for a machine.)

## Why these matter to Phient

*Difference Engine* — a fixed train of wheel columns; the method of finite differences; each wheel timed, each turning the next through a carry; it can only tabulate the one polynomial it is set to. That is `Facet::relational_score`: fixed gears (the channels), each context wheel turning the candidate's wheel through the channel its condition selects, no averaging, no programme — and it can only *read* what the corpus has set into the wheels.

*Analytical Engine* — a **store** (the lexicon of two-sided phasors), a **mill** (the Fourier operator, `phinet`), and **cards** (the training objectives and the learned scorer's weights, `learn::Tuner`) that decide what operation the mill performs on what the store holds; conditional branching (the learned scorer choosing among candidates); "algebraical patterns" woven from relations rather than numbers — her own point that the objects operated upon need not be numbers. The relational reading is the Difference Engine inside the Analytical Engine: the exact, fixed, mechanical part that the programmable part learns to weight (+0.70, docs/73).

Her limit stands: the engine "has no pretensions whatever to originate anything". Every number in docs/64–73 is a measurement of how well the machine follows the analysis it was ordered to perform — and the fitted weights are the order.
