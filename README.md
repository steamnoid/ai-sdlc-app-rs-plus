# ai-sdlc-app-rs-plus

A desktop application that onboards a public GitHub repository, reasons about it with
language models inside explicit contracts, a state machine and human approval gates, and
produces a traceable pull request.

**The domain has no dependencies.** Not "few", not "only serde" — `crates/domain` has no
`[dependencies]` section at all, and every other boundary in the repository rests on that
one line. Everything the domain cannot answer by itself is a **port**: a trait declared in
`crates/run`, implemented in `crates/outside`, and named concretely in exactly one place.

```text
aisdlc-domain  ──►  (nobody)
aisdlc-run     ──►  domain
aisdlc-outside ──►  run, domain
aisdlc-desk    ──►  outside          the composition root, the only place naming a concrete type
aisdlc-gates   ──►  (nobody)         reads the crates; never calls them
```

## What is different here

**No comments in the code.** `///` and `//!` are documentation. `//` and `/* */` do not
exist outside them, and a gate reads every `.rs` file to keep it that way. This is a
forcing function, not a style: a comment that would have been written is a place where the
code failed to say something in the only language it can be read in.

**Twenty gates, and no rule without one.** A rule with no gate is a wish, and a wish is
worse than nothing because a reader stops checking. Every gate is exercised on input it
must catch *and* on input it must leave alone, because a guard that has never been seen to
fail is not evidence of anything.

**The glossary is the first thing written.** Names are settled in
[`docs/domain-glossary.md`](docs/domain-glossary.md) before the first type exists, and the
gates read that file rather than restating it, so they follow the document instead of
drifting from it.

**A gate is a package, not a directory.** `aisdlc-gates` is the only code here permitted to
*read* the other crates rather than call them, which is why it cannot sit inside a layer. Its
library holds the code the gates share, because a test double is production code and five
gates each carrying a copy is five names waiting to disagree.

## Where it is

**The workspace, the glossary, the domain glossary's gates, and the start of phase 0.**

| | |
|---|---|
| the workspace and four packages | `aisdlc-domain`, `aisdlc-run`, `aisdlc-outside`, `aisdlc-gates` |
| `Stage`, `Role`, `WorkItem`, `Approval` | named in the glossary, **not yet built** |
| two gates | the domain's manifest and every edge in the workspace |
| the other fifteen gates | not built |
| the domain, the run, the adapters, the desk | not built |

**A green suite is not a working system.** Nothing here has run against a real model, a
real repository or a real pull request, and a claim that this ships anything would be a
claim about a program that does not exist.

## Running it

```bash
cargo test --workspace --no-fail-fast   # no network, no credentials, no Docker, no model
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
cargo doc --workspace --no-deps
```

`--no-fail-fast` is not optional politeness: `cargo test` stops after the first failing
binary, so a count read out of its output is a count of the suites that happened to run
first.

## The rules

Everything about how this repository is built is in [`AGENTS.md`](AGENTS.md).

## Licence

All rights reserved. See [`LICENSE`](LICENSE).
