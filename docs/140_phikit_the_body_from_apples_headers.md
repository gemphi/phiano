# 140 · phikit: Apple's frameworks read as a world — the body, the hand and the face from the headers

In Phient's world, a word (w) is a seed: what it relates to, under what conditions (c), grows from it. The Phical paradigm, X ⇒ Y, drives and composes Phient's Phasor Language Models: Phinet, Phiano, PhiTon. Phical stores no point: a word is 32–64 relations, each a complex phasor on a closed circle with a count; meaning is a side effect of pairs of words moving under a condition.

"Intelligence is not what you know, it's what you do when you don't know" — the remark attributed to Piaget on the slide the user brought — and it should be measured by the speed of learning, not by a benchmark. That is the standard the counts have been held to since docs/113 (one pointing), and it is the standard for what follows: a persona that is to have a body should get it from one reading of what the body's makers named, not from a schema we type in.

## 1. The ask, and what was done

phiecs is to be a full world model that reads Apple's own source; phiano's personas are to learn the human anatomy from the Apple crates — the body pose, the hands, facial tracking; and the relevant parts of the Rust ARKit bindings are to come under phinet as a crate. The crate is `phikit`, in the workspace beside phiplm (`phiplm ← phikit ← phiano`), and it does three things, each tested on the real headers under `gemphi\phi\apple-source\ruskit`:

**It reads declarations as rows.** `phikit::objc::parse` takes a header's text — Objective-C, C or C++ as Apple writes them — and returns its declarations: exported constants with their declared type (`VN_EXPORT VNHumanBodyPoseObservationJointName const …LeftShoulder`), interfaces and C++ classes with their parents and protocols, properties with their owner and type, enum cases (`NS_ENUM`, `OS_ENUM`, `typedef enum`), string-enum typedefs, IOKit's `#define k…Key` registry keys, and `API_DEPRECATED_WITH_REPLACEMENT` as a `replaced-by`. It is a reader of the forms the frameworks use, not a compiler; comments are stripped first. `phikit::rows` turns each declaration into the quest's rows: `is-a` the type or parent, `has` for a property, `implements`, `replaced-by`, `names` for a key's string, and `within` the framework, the header, the type and the symbol's own scope; `has-word` and `says` for each word of the name and `says` for its pairs.

**A name is its words.** `words_of` splits a symbol at its case changes and underscores after stripping the spelling of its declared type — `LeftShoulder`, `index finger knuckle`, `eye blink left`, `thumb cmc` — and when a group of constants is declared as a type that is not its prefix (Vision's deprecated `VNBodyLandmarkKey…` are `VNRecognizedPointKey`), the words the group shares by name are read from the group and dropped. A type is a name too: its scope says its own words, so `human` tells the human body's joints from the animal's. Every symbol is one scope under its type, so a longer name draws no more than a shorter one; the tree is framework → header → type → symbol, which is how Apple declared the body.

**The rows are a world and a tree.** `Kit::read(root)` walks every `.h` under a directory of frameworks and names each by the directory above `Headers`; `Kit::knowing()` counts the rows into phiplm's `Counts` and `ScopeTree`; `Kit::world()` coins every symbol, scope and word in a phiecs `World`, declares a relation per kind and relates every row — pairs of words moving under a kind. phiano's `anatomy` binary runs it.

## 2. Measured on the headers

`anatomy C:\Users\phiac\Workspace\gemphi\phi\apple-source\ruskit` (ARKit and Vision under `Interfaces/MacOSX`, and five IOKit headers of the kernel framework: `IOService`, `IORegistryEntry`, `IOKitKeys`, `IOTypes`, `IOUserClient`):

106 headers in three frameworks, **27,785 rows**. What the frameworks name most, read, not listed by us: ARKit's 52 blend shapes (`brow down left … tongue out`), the 27 joints of visionOS's hand skeleton (`wrist, thumb knuckle, … forearm arm`), Vision's 21 hand-pose joints (`thumb cmc, thumb mp, thumb ip, thumb tip, index mcp, …`), its 19 body-pose joints (`nose, left eye, right eye, left ear, …, root`), its 18 three-dimensional ones, its 25 animal joints, and IOKit's 139 registry keys; 45 classes under `NSObject`, the `VNImageBasedRequest` family of 26 requests.

Cast into that tree with the cast of docs/126–129 unchanged (`--ask`):

| asked | drawn into |
|---|---|
| left elbow | Vision · VNHumanBodyPose3DObservationJointName · **left elbow** |
| left elbow human | the same, with `human` said by the type |
| right knee | Vision · VNHumanBodyPose3DObservationJointName · **right knee** |
| eye blink | ARKit · ARBlendShapeLocation · **eye blink right** / left |
| mouth smile | ARKit · ARBlendShapeLocation · **mouth smile right** / left |
| jaw open | ARKit · ARBlendShapeLocation · **jaw open** |
| thumb tip | Vision · VNHumanHandPoseObservationJointName · **thumb tip** |
| index finger tip | ARKit · ar_hand_skeleton_joint_name · **index finger tip** |
| registry entry | IOKit · IOKitKeys · **io registry entry property keys key** |

Nine of nine land on the symbol a reader would name, by what the scopes say; the context tests (10/10 · 20/20 · 20/20 at 400 and 2,000 turns, both seeds) and the story are unchanged, since nothing in the cast changed. Two things were tried on the cast and reverted, and are worth recording: counting an asked word once along a path (rather than once per level that says it) made every scope under a type equal and broke the update test (13–15 of 20); ranking by the leaf's own saying first kept the update test but stopped the cast at the parent of the right joint, because the locality rule (D3) rightly makes a word every sibling says count for none of them. What fixed the kit was the kit: one scope per symbol, so depth is uniform, as it is in prose.

## 3. The face as a language, for the personas

`phikit::face` is the hook between the kit and a persona. ARKit gives, every frame, fifty-two blend-shape coefficients in [0, 1] (`ARFaceAnchor`; in Rust, `rust-ios-ar`'s `ARFaceAnchor` with its `ARBlendShapeLocation` enum, under `gemphi\phi\crates_extracted\crates\rust-ios-ar\src\arkit\anchor.rs`). A frame is a sentence: the shapes at or above a threshold are the words it says, strongest first; a sequence of frames under a path (`faces/<persona>/<scene>`) is a paragraph, told with the same rows a story is. Then phiplm's `Knowing` reads a face as it reads a story and goes on from it. Tested: a smile that closes the eyes, told three times under one scene — asked what follows the smile, the tree says `eyeblinkleft` first. No frame is produced in this crate: frames come from a device through the binding, and the user's machine is not a Mac with a TrueDepth camera. What is here is the telling, which is what a persona needs to learn its own face the moment it is shown one.

The persona's body, then, is this: the joints Apple named, as scopes in a tree a cast can be drawn into; its face, the fifty-two shapes as words; its expressions, what it was told its face does, counted under the scenes they happened in. Nothing of it was declared here.

## 4. What was not done, honestly

- The `rust-ios-ar` bindings were read and are referenced; nothing of them was copied into phikit. The kit reads headers; a reader of Rust bindings (their enums are the same names) is the same `objc` reader with one more form and is not written.
- Of IOKit's 89 kernel headers five were read, the ones that are the registry (the device tree — itself a tree of conditions). The rest, and the `hid`, `usb`, `graphics` subdirectories, are a directory argument away.
- `Kit::world()` builds the phiecs world and relates every row; nothing yet runs a `System` over it, and the Phasors coupled by `relate` are not yet read back by anything but the test.
- No measurement of learning speed on the body exists yet, because no device data exists here. The test that would be Piaget's — a persona shown one new gesture once and asked for it after a session of others — is the context test of docs/129 with frames for turns, and `face::rows` is what makes those frames rows.

Housekeeping: `phikit` (five modules, each under 200 lines, 6 tests), `anatomy` in phiano, the workspace table in `phinet/Cargo.toml`. Workspace: 361 tests, zero warnings.

`anatomy <apple-source\ruskit> --ask "left elbow"` · `anatomy <root> --rows anatomy.tsv` · `cargo test -p phikit`
