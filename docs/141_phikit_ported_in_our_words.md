# 141 · phikit ported: the body, the space and the device in our words, told as rows, watched by a system

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

## 1. The correction, and what changed

docs/140 read Apple's headers at run time and kept Apple's symbols as the things in the tree (`VNHumanBodyPoseObservationJointNameLeftShoulder` within `Vision/VNDetectHumanBodyPoseRequest/…`). The user's correction: we are not going to use the `apple-*` names; port the code into our code space and name it accordingly, with a README in each crate, and port it whole, so Apple's interfaces are interpreted rather than referenced. That is what this round did. The header reader (`objc.rs`, `rows.rs`) is gone from the crate — it stays in git at `326c637` as the tool that read the vocabularies once — and phikit is now code: thirty-one modules under three domains, every type named the Rust way in our words, every enumeration and struct told as rows by one mechanism.

**The port.** `rust-ios-ar/src/arkit` (3,502 lines, fifteen modules) is `phikit::track` (fifteen modules, each under 200 lines): `anchor` (the base anchor, `Tracking`, `Reason`), `camera` (with a projection that actually projects — the binding's `project_point` returned zero), `frame` (`Frame`, `Tracked` for every kind of anchor), `session` (events in order: added, updated, removed, frame, failed, interrupted), `config` (the eight modes and their options), `surface` (planes and images), `mesh` (geometry, mesh anchors, reconstruction, the scene), `depth` (the captured image, depth and confidence maps, the point cloud, segmentation), `light` (estimates, the directional light, probes), `raycast` (a query from a point or a pixel, results, a tracked raycast — with a floor it can meet), `geo` (places on the earth, with a distance), `map` (a world map that survives its bytes — the binding's `deserialize` was a stub), `share` (participants, collaboration, coaching), `room` and `capture` (the room-plan scan). The Vision and ARKit vocabularies are `phikit::body`: `Shape` (52, in 7 `Region`s read from the first word), the pose's `Joint` (23, in 7 `Group`s read from the words), the tracked `SkeletonJoint` (67, the binding's two skeletons), the hand's `HandJoint` (21) and `HandSkeletonJoint` (27, with `parent` read along the finger). IOKit's registry is `phikit::device`: `Registry` with `Entry`, six `Plane`s and `Value`s; `Services` with `State` bits, `Personality` matching by provider class and name and probe score; `Key` (81) with the string a registry reports, so a dump is read into our names at the edge and nowhere else.

**One mechanism tells it all.** `named! { pub enum Shape { BrowDownLeft, … } }` makes a `Named` type: `KIND`, `ALL`, `ident`, `name()` (`shape:mouth_smile_left`), `words()` (`mouth smile left`), `by_name`. `tell::cases::<T>(path)` tells every case as one scope under `path/<kind words>/<case words>`, the kind's scope saying its words, the thing `is-a` its kind, `has-word` and `says` its words and their pairs; `tell::fields(path, "Camera", &[…])` tells a struct's fields the same way with `has`; `tell::scope_says(path)` lets the module scopes say their own word, once. Every module has `told(path)`; the domains gather them; `Kit::new()` is all of it — **11,471 rows**, roots `body`, `track`, `device`, depth uniform: domain → module → kind → thing.

**A frame is a sentence; a system watches.** `Face::sentence(0.3)` is `mouth smile left, mouth smile right, cheek squint left`; `Pose::sentence`, `Hand::sentence`, `Room::sentence` likewise; `face::rows(path, frames, tau)` tells a sequence as a paragraph. New: `phikit::Watch`, a phiecs `System` — each step takes one `Frame`, relates the shapes said together `near` and each shape said to each the next frame says `before`; nothing is stored but the relations, and what the face does is read back as the agreement of two phasors on a channel, or as what a word `calls_for` under `before`. This is the first `System` run over a kit world (docs/140 §4 said none was).

## 2. Measured

**The cast** (`cargo run -p phikit --example ask -- …`, the cast of docs/126–129 unchanged):

| asked | drawn into | note |
|---|---|---|
| jaw open | body · face · shape · **jaw open** | |
| eye blink | face · shape · **eye blink right** / left | the two sides tie |
| excessive motion | track · anchor · reason · **excessive motion** | |
| limited tracking | track · anchor · tracking · **limited** | |
| world tracking | session error · world tracking failed; geo reason · world tracking unstable; config · mode · **world tracking** | three things say both words |
| floor | surface · plane classification · **floor**; room · surface type · floor; mesh · classification · floor | a floor is three kinds of thing |
| bedroom | room · category · **bedroom** | |
| device tree | device · registry · plane · **device tree** | |
| provider class | service · personality · provider class; keys · key · **provider class** | a key and a field, both rightly |
| registry entry | keys · key · registry entry property keys / id / … | |
| pose left elbow | pose · joint · **left elbow** | `left elbow` alone ties with the skeleton's |
| thumb mp | hand · hand joint · **thumb mp** | `thumb tip` alone: the skeleton's hands have one too |
| index finger knuckle | hand · hand skeleton joint · **index finger knuckle** | |
| left hand thumb tip | skeleton · skeleton joint · **left hand thumb tip** | |

Where the asked words are one thing's, the cast lands on it. Where two things rightly have the words — the pose and the skeleton both have a left elbow; the hand and the skeleton's hands both have a thumb tip; a floor is a plane's classification, a room's surface and a mesh's class — the cast ties, and the module's word (`pose`, `skeleton`) or a word only one has settles it. One finding worth keeping: for `hand thumb tip` the skeleton's `left hand thumb tip` wins over the hand's `thumb tip`, because it says all three asked words itself where the hand's gets `hand` from its module and kind — and the locality rule (D3) then discounts `hand` at the kind level, where most siblings say it. A longer name that says the asked words outdraws a shorter one that inherits them. That is the cast behaving as designed (said beyond the parent), and it is the reason the body's trees were given module words to say.

**The watching world** (`cargo run -p phikit --example watch -- <repetitions>`; a smile followed by a blink, a yawn followed by the tongue, each shown `n` times with a rest between):

| repetitions | smile L ~ smile R (`near`) | smile L → blink L (`before`) | smile L → tongue (`before`) | jaw → tongue | jaw → blink L | `follows(smile L)` |
|---|---|---|---|---|---|---|
| 1 | −0.577 | 1.000 | −0.601 | −0.613 | −0.235 | blink L 1, blink R 1 |
| 3 | 0.029 | 0.990 | −0.765 | −0.003 | −0.523 | blink L 3, blink R 3 |
| 12 | 0.949 | 0.997 | −0.992 | 0.947 | −0.960 | blink L 12, blink R 12 |
| 48 | 1.000 | 1.000 | −0.998 | 1.000 | −0.998 | blink L 48, blink R 48 |

Read honestly: the phasor agreement at one showing is the seed (smile→blink reads 1.000 at 1 because the two seeded angles on that channel happen to lie 0.1 apart; jaw→tongue, equally shown, reads −0.613), and it takes about twelve showings at the coupling rate (0.84¹² ≈ 0.12 of the gap left) before the related pairs stand above 0.94 and the unrelated below −0.9. The edge counts (`calls_for`, the `follows` column) are right from the first showing. This is docs/136's finding again, in the body: the count learns from one pointing, the phasor is a slower, superposable summary of it. A pair never shown (`smile ~ brow down`) is `None` at every row — silence is not agreement.

**Housekeeping.** phikit: 31 modules in 3 domains, every file under 200 lines (longest `session.rs`, 198), 26 tests and a doctest, two examples (`ask`, `watch`), a README in the crate and one in each domain; READMEs added to phiplm and phiano (every crate now has one). `anatomy` takes no root any more. Workspace: all tests pass, zero warnings. The `regex` dependency is gone.

## 3. What was not done, honestly

- No device frames exist here; `Watch` was shown frames made in a test. The learning-speed test on a body (docs/140 §4) still waits for a device.
- The registry has no dump to read; `Registry::rows` was tested on three entries. `ioreg -l` on a Mac is the input it wants.
- The binding's room-plan view controller (`ARRoomCaptureViewController`, 200 lines of UI state) was reduced to `Guidance`; its `instruction_images` were not kept. The binding's `ARSceneMeshManager` delegate trait and `ARCollaborationSessionDelegate` were replaced by events and plain state — a trait with no second implementation is a dangling function.
- Skeleton parents are not declared: the binding does not set them and the headers do not list them; `Bone::parent` is what a device reports. The hand skeleton's `parent` is read along the finger from the names, which is the one place a hierarchy was inferred from words, and it is tested.
- The approach documents the user asked for alongside this — `docs/training.md`, `docs/guard.md`, `docs/ecs_vs_jepa.md` and `docs/world_model.md` (phiecs and phinet drawn; the learning steps; each level a model of its own) at the workspace root — are written in this round and are the next files to read.

`cargo test -p phikit` · `cargo run -p phikit --example ask -- "jaw open"` · `cargo run -p phikit --example watch -- 12` · `cargo run --release --bin anatomy -- --ask "left elbow"`
