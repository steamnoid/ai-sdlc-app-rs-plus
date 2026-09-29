# ai-sdlc-app-rs-plus

A desktop application that onboards a public GitHub repository, reasons about it with
language models inside explicit contracts, a state machine and human approval gates, and
produces a traceable pull request.

**The specification came first and it still does.** The glossary is what the code is written
against, and a name that is not in it does not enter the source.

## What is here

| | |
|---|---|
| [`AGENTS.md`](AGENTS.md) | three rules, the layout, the commands |
| [`docs/domain-glossary.md`](docs/domain-glossary.md) | every name, the rule each one protects, and the table of all nineteen |
| [`crates/domain`](crates/domain) | the state machine and every type: 3737 lines, no dependencies |
| [`crates/run`](crates/run) | the use cases, and the ports they are written against: 2549 lines |
| [`crates/outside`](crates/outside) | the implementations of the ports, and nothing else: 515 lines |
| [`src/`](src) | the walk every gate reads the source with: 1068 lines |
| [`tests/`](tests) | the nineteen gates, in two files named for what they are about |
| [`scripts/gate`](scripts/gate) | the one command, and the check `main` requires |

**324 tests.** The domain is complete: the six stages, the five roles, the exhaustive
transition table, the work item and its three doors, and the approval record. `crates/run`
carries four moves of a run, the gate that asks a person, the model that answers, and **a run
that really clones a repository with `git` and reads the commit and the ref back out of the
clone it made**.

**The counts above are counted, not checked.** Nothing in this repository reads this file, so a
number here is a number somebody wrote down on a day — **and the gates do not hold the README,
which makes every claim in it a promise rather than a check.** The one that matters is the gate
table in the glossary, because a gate holds that.

## The three rules

1. **RULE 0 — strict TDD.** Every behaviour change is RED → GREEN, as two commits.
2. **RULE 1 — readability is the highest priority.** The code is read far more often than it
   is written, and it is read by a human. **There are no comments in the code.** `///` and
   `//!` are documentation; `//` and `/* */` do not exist. A function body is at most 20
   lines and nests no deeper than two levels.
3. **RULE 2 — the code mirrors the domain.** One concept, one name, everywhere — in
   production and in tests — and the name is one the glossary blesses.

**A rule with no gate is a wish.** All nineteen are listed in the glossary's table of rules
with the file whose `mod` holds each one, and the gate that holds the table refuses a rule
nobody holds and a module no row names — **in both directions**, because a table checked only
for what it claims is a table a rule can be deleted from without anyone noticing. Three rules
were lost that way once, and the gate exists because of it.

## The architecture, in one picture

```text
aisdlc-domain  ──►  (nobody)
aisdlc-run     ──►  domain
aisdlc-outside ──►  run, domain
aisdlc-gates   ──►  (nobody)       reads the crates; never calls them
```

**`aisdlc-domain` has no `[dependencies]` section at all.** Not "few", not "only serde".
Nothing.

Everything the domain cannot answer by itself is a **port**: a trait declared in `aisdlc-run`
and implemented in `aisdlc-outside`. The glossary names ten and says why each is separate.
**Five of them have a caller and three have a real implementation behind them**: the project on
disk, the program we run, and the directory we ask about. The other five do not yet, and a port
with no implementation behind it is not a port — it is a trait nobody calls, so each arrives with
the first use case that needs it.

**The port table is not checked for completeness, and it was wrong until this file was read
against the code.** A port with a caller and a real implementation had been left out of it for
three pull requests. The rule table beside it *is* checked in both directions, which is the
difference between the two: **one had a gate and stayed true, the other had a reader and did
not.**

## The run, as far as it goes

```text
READY → AWAITING_AGENT_PICKUP → IN_PROGRESS_BY_AGENT → AWAITING_HUMAN_APPROVAL → DONE
```

Four moves, all of them through the domain's own `transition()`. `DONE` is terminal and
exactly one edge reaches it, and the edge is the gate: **the run closes a work item by holding
the record of somebody's ruling, and the record is an argument rather than a check**, so a
refusal — which leaves no record — has nothing to close.

**The clone is here**, and it is the only part of this system that executes anything: `git`
started with an argument vector and no shell, the destination asked about before it is written
to, the commit and the ref read back out of the clone, and the four ways a clone can fail
refused by name rather than as a boolean.

What is not here yet: the container, the pull request, and the five ports with no caller.

## Running it

```bash
cargo fetch        # dependencies
./scripts/gate     # the one command
```

`./scripts/gate` runs `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test` and
`cargo doc`, and stops at the first one that refuses. It is also the check `main` requires,
so a pull request cannot merge on a green tick from anything else.

There is nothing to *run* yet — this is a library and a set of gates, and the application
arrives after the domain it reasons about. **What does exist is real: `git` really is started,
a directory really is read, and a checkout really is made**, and the two tests that prove it say
in the source that they need the network, because `fetch_from` is always an https address.

## Licence

All rights reserved. See [`LICENSE`](LICENSE).
