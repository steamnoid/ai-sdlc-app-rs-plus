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
| [`docs/domain-glossary.md`](docs/domain-glossary.md) | every name, **and the table of every rule with the module that holds it** |
| [`crates/domain`](crates/domain) | the state machine and every type, with **no `[dependencies]` section at all** |
| [`crates/run`](crates/run) | the use cases, and the ports — **and nothing that names a filesystem, a process, a clock or a network** |
| [`crates/outside`](crates/outside) | the real implementations, and **the only crate a gate lets reach the world** |
| [`src/`](src) | the walk every gate reads the source with |
| [`tests/`](tests) | the gates, in two files named for what they are about |
| [`scripts/gate`](scripts/gate) | the one command, and the check `main` requires |

**568 tests.** The domain is complete: the six stages, the five roles, the exhaustive
transition table, the work item and its three doors, and the approval record. `crates/run`
carries four moves of a run, the gate that asks a person, and **a run that really clones a
repository with `git` and reads the commit and the ref back out of the clone it made**.

**A filing goes in, a run reads it, and a comment comes back out — all three proved against
the real GitHub, in one run.**

`gh` reads a number a person typed, and the address comes from GitHub rather than being
composed from the number — **an issue that was transferred answers somewhere the number does not
appear, and a composed address can only be right or wrong.** A pull request at that address is
refused, because GitHub answers both from one path and a pull request's body describes a change
rather than asking for one. A number with nothing behind it is refused as *no such filing*,
which is not the same fact as *an answer this system cannot read* — **and a person told the
wrong one goes to look at this system instead of at their own number.** The comment is then
written at the address that was read, signed with the name a deployment chose for itself, and
says only what is true of it.

**A model has ruled, once, and it ruled `REFUSED`** — a decision rather than a failure to read
one, after three refusals that were exactly that failure and a request body that had never been
valid JSON.

**A whole run, on a real filing, with a real model ruling on it — proved in one run.**

    ruling on WhichArtifact("DiscoveryReport")
    ruled false; recorded None; the work now stands at Some(AwaitingAgentPickup)

A filing is read from GitHub, a work item is named and disciplined by whatever a run is told, the
work is picked up, worked on and presented at a gate with **the artifact name and its version**,
a real model is asked, its answer is read out of the envelope its endpoint sent, the ruling is
recorded **under the name of whoever answered**, and the work moves where the transition table says
a ruling moves it.

**The ruling was a refusal, and that is the correct outcome.** The model was asked about a
`DiscoveryReport` this system has never produced, and it refused; nothing was recorded, because a
refusal leaves nothing, and the work went back for another pass — **because a work whose ruling was
a refusal is not delivered.** A run that had approved would have been a run recording a delivery
nobody made.

**There is still no pipeline.** Every step above exists, and the steps run in order against the
real thing, **and no agent produces the artifact the gate is asked about.** `aisdlc-discover` picks
up the work; it does not discover anything yet, so there is no `DiscoveryReport` to approve and the
ruling can only be a refusal. **What is proved is a run that reads, presents, asks, records and
moves — and not one that does the work.**

**Nothing in this repository reads this file, so every claim in it is a promise rather than a
check.** The numbers that used to sit beside the crates were the most perishable facts here — two
of them had gone stale inside a single working day — **so they are gone, and what each crate is
instead of how long it is.** The one count left is the test count, because it moves only when a
test is added and because it is the one number a reader wants. The claim that matters is the gate
table in the glossary, because a gate holds that one.

## The three rules

1. **RULE 0 — strict TDD.** Every behaviour change is RED → GREEN, as two commits.
2. **RULE 1 — readability is the highest priority.** The code is read far more often than it
   is written, and it is read by a human. **There are no comments in the code.** `///` and
   `//!` are documentation; `//` and `/* */` do not exist. A function body is at most 20
   lines and nests no deeper than two levels.
3. **RULE 2 — the code mirrors the domain.** One concept, one name, everywhere — in
   production and in tests — and the name is one the glossary blesses.

**A rule with no gate is a wish.** Every one is listed in the glossary's table of rules
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
and implemented in `aisdlc-outside`. The glossary names each one and says why it is separate, and the same table says which of them
answer anything today. **A port with no implementation behind it is not a port** — it is a trait
nobody calls, so each arrives with the first use case that needs it, and the ones still waiting
are named in the glossary rather than counted here.

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

**There is no pipeline, and this is the one number in the file that matters: sixteen use cases
and not one of them is called from outside its own tests.** Every part is proved — the state
machine, the gate, the clone, the real `git`, the real disk — and the thing that joins them does
not exist, so **a reader who counts sixteen use cases and a green suite will assume a run, and
there is no run.** It is not a defect and it is not finished: the pipeline needs the first agent,
the first agent needs the first artifact, and the first artifact's name belongs to the agent that
will produce it — **so the chain is blocked at a name a person has not chosen yet, and building
the artifact first would be writing an identifier before the thing that owns it.**

What is not here yet: the pipeline itself, the container, the pull request, and the five ports
with no caller.

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
