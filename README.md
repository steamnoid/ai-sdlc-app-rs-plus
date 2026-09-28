# ai-sdlc-app-rs-plus

A desktop application that onboards a public GitHub repository, reasons about it with
language models inside explicit contracts, a state machine and human approval gates, and
produces a traceable pull request.

**There is no code here yet.** What this repository holds is the specification of how the
code is to be written, and the specification is worth more than code written before it.

## What is here

| | |
|---|---|
| [`AGENTS.md`](AGENTS.md) | three rules, the gates that enforce them, the layout, the commands |
| [`docs/domain-glossary.md`](docs/domain-glossary.md) | every name, and the rule each one protects |
| [`LICENSE`](LICENSE) | all rights reserved |

Four files, and the fourth one is a licence.

## The three rules

1. **RULE 0 — strict TDD.** Every behaviour change is RED → GREEN → REFACTOR → GATE, in
   that order, as two commits: `test(scope): RED — the specific expectation that fails` and
   `feat(scope): the behaviour that now holds`.
2. **RULE 1 — readability is the highest priority.** The code is read far more often than it
   is written, and it is read by a human. **There are no comments in the code.** `///` and
   `//!` are documentation; `//` and `/* */` do not exist, because a comment is a place where
   the code failed to say something in the only language it can be read in. A function body
   is at most 20 lines and nests no deeper than two levels.
3. **RULE 2 — the code mirrors the domain.** One concept, one name, everywhere — in
   production and in tests — and the name is one the glossary blesses.

**A rule with no gate is a wish.** Seventeen of them are named in `AGENTS.md` with the test
that will refuse to let each be broken, and every one of them arrives with the code it
judges, because a gate written before the code it judges is a gate nobody has ever seen fail.

## The architecture, in one picture

```text
aisdlc-domain  ──►  (nobody)
aisdlc-run     ──►  domain
aisdlc-outside ──►  run, domain
aisdlc-desk    ──►  outside        the composition root, the only place naming a concrete type
aisdlc-gates   ──►  (nobody)       reads the crates; never calls them
```

**`aisdlc-domain` has no `[dependencies]` section at all.** Not "few", not "only serde".
Nothing. That one line in one `Cargo.toml` is the only rule in this repository the compiler
enforces without being asked, and every other boundary rests on it.

Everything the domain cannot answer by itself is a **port**: a trait declared in `aisdlc-run`
and implemented in `aisdlc-outside`. There are nine, and the glossary names each one and says
why it is separate.

## What happened here

This repository was built once, at length, and then **wiped and started again**. Seventy-one
commits are preserved at the tag `przed-rewrite` and the branch of the same name, and the
reason is written down in `AGENTS.md` under *Recorded departures* — a history is not deleted
because it is embarrassing, but because continuing to push into it makes the rewrite
impossible to see.

The rules survived the wipe. The glossary survived the wipe. The code did not, and the order
was wrong the first time: the gates were built before the domain, so six of them were green
because there was nothing to judge. The order is reversed now — the domain first, and each
gate with the code it judges.

## Running it

There is nothing to run yet. The commands `AGENTS.md` names are the gate, and they become
runnable when the first `Cargo.toml` exists.

## Licence

All rights reserved. See [`LICENSE`](LICENSE).
