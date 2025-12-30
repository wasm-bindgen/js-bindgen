## FAQ

### How Does This Improve on [`wasm-bindgen`]?

The main improvement is the lack of an interpreter and code-level post-processing. While this
document has a small description on both, we encourage anybody to look at [`wasm-bindgen`s issue
tracker] and judge for themselves how many issues these have caused.

It should also be noted that much effort has gone into evaluating if many of these improvements
could have been made transparently in `wasm-bindgen` without a breaking change or a complete
rewrite. Some of them could have! Ultimately we decided against it because the cost was already so
high, having to again make many compromises to keep the existing API or behavior would have in many
ways defeated the purpose of these improvements.

In conclusion `js-bindgen` is much leaner and significantly less complex than `wasm-bindgen`.

[`wasm-bindgen`]: https://github.com/wasm-bindgen/wasm-bindgen
[`wasm-bindgen`s issue tracker]: https://github.com/wasm-bindgen/wasm-bindgen/issues

#### Breaking Changes

Another major point was splitting up the crate further to more easily support breaking changes.
Specifically making a breaking change in `wasm-bindgen` had such a high cost it was not considered
worthwhile at any point in time. This specifically comes down to `wasm-bindgen` not being compatible
with other versions of itself, which had many reasons, but mostly being about compatibility with the
interpreter and post-processor. This makes upgrading the ecosystem entirely unfeasible: a single
crate in a dependency tree using a different version breaks the build. This means that every library
depending on `wasm-bindgen` has to make a breaking change and users have to wait for every single
crate in their dependency tree to update before being able to make the switch.

`web-sys` had a different issue: it was so expensive to compile, that having multiple versions in
your dependency tree would be quite a bad experience for our users. Over time `web-sys` has
accumulated more and more APIs that were not up-to-date with the changing Web API and we felt quite
unable to make the jump without collecting all those changes, making a breaking change quite costly.

#### Interpreter

`wasm-bindgen` encoded some information, like traits, in run-time code. Extracting that information
required running an interpreter during post-processing.

This has several drawbacks:

- We have to maintain a small but functional Wasm interpreter. This interpreter had to account for
  many code paths that it can't and shouldn't execute, e.g. [`__wasm_call_ctors`], making it
  sometimes very difficult to understand errors and debugging them.
- Running Rust code in the interpreter means that the compiler ultimately had to store some
  Rust-related information in the resulting Wasm module. While we diligently removed the functions
  we interpreted, some things couldn't be removed. Like corresponding data in data sections.
  Removing many of these would either mean we rely on unstable Rust/LLVM behavior or implementing a
  highly sophisticated analyzer that can correctly track down the correct parts in these data
  sections and adjust all corresponding pointers.
- Running the interpreter also added overhead to the overall post-processing, while not significant,
  it can be a non-trivial amount in very large projects.

In `js-bindgen` we rely on embedding information via custom sections instead. These can easily be
parsed and removed in post-processing without touching any Wasm instructions or relying on run-time
behavior.

[`__wasm_call_ctors`]:
	https://github.com/WebAssembly/tool-conventions/blob/8e3191e4992b7e96369ebcfec3af86610464ec27/Linking.md#start-section

#### Code-Level Post-Processing

Because Rust, at the time of writing, doesn't not support inline assembly, `wasm-bindgen` had to
insert many instructions via post-processing. `wasm-bindgen` did this very successfully over a very
large period of time.

However, over time we have identified many issues with this approach:

- While simply inserting instructions would have been quite simple, we had many features that
  required actually modifying functions. Over time this has surfaced many unforeseen bugs that were
  difficult to debug and sometimes even more difficult to fix. Especially because our
  post-processing happens post-linker, depending on the optimization level, we loose a lot of symbol
  information which can make it quite hard to track down what we needed to transform, sometimes we
  even had to rely on simple heuristic guesses.
- Dead-code elimination turned out to be quite difficult and caused many bugs over time. It is also
  not perfect. We often tried to increase the coverage just to run into user reports that expose
  subtle behavior with various configurations like LTO.
- We had to maintain [`walrus`], a library to perform Wasm transformations. It is already very
  functional but keeping up with new Wasm proposals requires significant implementation investments.
  We were hoping to switch to a third-party library, unfortunately none meeting our needs have
  materialized so far.

In `js-bindgen` we have instead opted to build our own stable inline assembly feature that works by
compiling assembly and passing it directly to the linker. This makes it very simple, maintenance
free and produces perfect dead-code elimination.

Our post-processing is limited to reading our own custom sections and removing them afterwards.

`wasm-bindgen` had an optimization pass making use of the multivalue proposal, which we do not
implement. As it was using no information from `wasm-bindgen` or Rust, this is better left to
`wasm-opt`, a separate Wasm optimizer or even better: finally bringing stable multivalue support to
LLVM and Rust to enable the compiler to make this optimization.

[`walrus`]: https://github.com/wasm-bindgen/walrus

### Why Don't We Compile the Assembly Code on the Proc-Macro Level?

This was one of the initial experiments of this project. However, it turned out to have some major
issues:

- Most of our assembly code is put together by a proc-macro by accessing various traits. Proc-macros
  can't read values off traits to actually extract the assembly code to compile.
- We only need to compile assembly code when producing the final module, proc-macros however are
  running much more often. Especially Rust Analyzer would cause major issues with race-conditions
  trying to constantly re-generate files. While we have attempted various workarounds to detect Rust
  Analyzer and other race-conditions, ultimately there was no fool-proof way to solve this. It also
  hurts incremental compilation, where we want to cause as little overhead as possible.
- Because we build our JS code via associated trait variables, which we can't evaluate in a
  proc-macro, we are stuck doing post-processing anyway.

### Why Don't We Compile the Assembly in a Build Script?

Crucially the assembly code is put together by a proc-macro by accessing various traits and creating
a long string that can be embedded into a custom section.

While there has been some past work on evaluating single Rust files from build scripts, this would
largely duplicate the compilers work and in many cases is just not feasible because of all the
interactions with traits that often live in a separate crate.

### Why Don't We Remove Our Custom Sections Before Passing Them into the Linker?

This was also an early experiment in the development of this project. While this works in some
cases, like with object files, it doesn't work easily with `*.rlib` archives. Crucially the `*.rlib`
format is not stable and it is unclear what we could break by modifying the containing archive
members.

It should also be noted that this wouldn't get rid of post-linker processing as we require reading
the resulting imports to determine dead-code elimination for the JS code.

As this will likely be a performance improvement, even if just slightly, it would still be a welcome
improvement. We invite any expert on the topic to chime in on what we can and can't depend on about
Rust's archive format and how we could modify and much more importantly parse its contents more
reliably.

### Why Do We Use WAT Instead of X?

We have a couple of constraints to work under:

- The compiler needs to be shipped to users.
- Low maintenance.
- Ability to transform the input.
- Forward-compatibility with Rust's `asm!`.

Ultimately we have decided to go with WAT and a `wasm-tools` toolchain. Below you will find a
detailed evaluation of every scenario that has come up.

#### GAS

Rust's `asm!` uses GAS so it would be perfectly forward-compatible.

It would also be very low maintenance because we can rely on LLVM to maintain it for us. But notably
GAS is seriously behind on new proposals, which could turn this into very high-maintenance if it
ends up requiring us contributing to LLVM.

##### With `llvm-mc`

`llvm-mc` would require compiling LLVM on the users machine during installation. Considering the
build time requirements this is completely off the tables. We might consider shipping binaries,
which is often very undesirable by users.

We are already shipping LLVM and therefor can use their parser to transform the input.

##### Rust's `asm!`

At the time of writing it is unstable and therefor can't be shipped to users.

We can't transform the input unless we build our own GAS parser.

#### WAT

We have high-quality parsers and code generators from `wasm-tools` that would help us easily
transform the input.

However, it will not be forward-compatible with Rust's `asm!` unless Rust decides to switch from GAS
to WAT. We would have to write our own WAT to GAS compiler, which should be quite straightforward
but nonetheless a big chunk of work.

##### With `wat2wasm`

Similarly to `llvm-mc` shipping this to users would require compiling WABT, the project `wat2wasm`
lives in, on the users machine during installation. While the build time is orders of magnitude
lower than LLVM, its still significant. Again we might consider shipping binaries, which is often
very undesirable by users.

Maintenance is currently an issue seeing that there are new proposals that are not yet supported by
WABT.

##### With `wasm-tools`

Shipping this to users would be ideal because they are just Rust libraries.

These tools are very well maintained, even though at the time of writing they are missing compiling
relocatable object files which we would have to contribute.
