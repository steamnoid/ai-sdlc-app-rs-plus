# AGENTS.md — ai-sdlc, in Rust

A desktop application that onboards a public GitHub repository, reasons about it with
language models inside explicit contracts, a state machine and human approval gates, and
produces a traceable Pull Request.

The point of the project is not the demo. It is a deterministic software system in which
LLM agents perform bounded reasoning inside explicit contracts, state machines, tools and
human approval gates. **Making the boundary between deterministic software and
probabilistic AI obvious is a core feature.**

This file is short on purpose. Three rules, the gates that enforce them, the layout and
the commands. Anything longer belongs in [`docs/domain-glossary.md`](docs/domain-glossary.md),
which is the source of truth for naming, or in a commit body, which is where the reasoning
about a particular change lives.

---

# RULE 0 — Strict TDD is the only way to change this codebase

Every behaviour change follows **RED → GREEN → REFACTOR → GATE**, in that order, without
exception.

## 0.1 The cycle

1. **RED** — write the failing test first. It must fail for the *right* reason: an
   assertion about behaviour. A test that does not compile is a legitimate RED **only when
   the property under test is a type-level property** — `legal_transitions_from` is an
   exhaustive `match`, so a seventh `Stage` that fails to compile *is* the assertion
   failing. Anything a runtime can check is checked by running the runtime, because
   deserialization bypasses every constructor and a hand-built value can be built that no
   call site would accept.
2. **GREEN** — the smallest implementation that passes. No speculative parameters, no
   unrequested dependencies, no abstraction without a caller.
3. **REFACTOR** — improve it with the suite green.
4. **GATE** — the four commands in [Commands](#commands) clean, or the work is not done.

A docstring, a commit message, or a report must not claim the compiler replaced a test. If
the test was deleted, the property was narrowed, and the report says so.

## 0.2 Commit discipline

Each cycle is exactly two commits:

```text
test(<scope>): RED — <the specific expectation that fails>
feat(<scope>): <the behaviour that now holds>
```

The RED commit is expected to leave the suite red and is paired with the very next commit.
A GREEN with no preceding RED on the branch is a RULE 0 violation, and
`every_change_is_a_red_then_a_green` reads the history to say so.

Stage the files the cycle is about, not everything. `git add -A` with a cycle still open
sweeps the *next* cycle's files into this one.

## 0.3 Forbidden

- Implementation written before a failing test demands it.
- A commented-out assert, a `#[allow]`, a `todo!` or a `# type: ignore` added to turn the
  suite green. All four are caught by gates, and a gate that reports its own count is
  there so a reader can see when one was needed.
- Adding a dependency to make something testable. Inject a seam instead.
- Widening scope mid-cycle.
- A test double that answers on nobody's behalf. A double that runs out of decisions
  raises, because one that decided by itself would let a test pass with nothing having been
  decided.

## 0.4 The hygiene check that matters most

`cargo test` must stay green with **no network, no API key, no Docker, no model.** If it
needs a token, a boundary is wrong. Real credentials, a real container and a real model
belong behind `#[ignore]` with the reason printed, which
`an_ignored_test_says_why_it_is_ignored` checks rather than trusts.

## 0.5 Precedence

RULE 0, 1 and 2 > this file > the glossary. For any external protocol, the **current
official documentation beats all three** — that ecosystem changes fast and model memory is
not a source.

---

# RULE 1 — Readability is the highest priority

The code is read far more often than it is written, and it is read by a **human**.

## 1.1 Read top to bottom

A module must tell its story in the order things happen. A reader who reads the file once
should be able to explain what it does without jumping.

## 1.2 No abbreviations. Ever

`work_item`, not `wi`. `legal_transitions`, not `trs`.

## 1.3 Types are explicit everywhere

A reader must never have to infer a type, and a boundary must never accept a `String` where
a domain type exists. This is what `enum` is for.

## 1.4 **There are no comments in the code**

`///` and `//!` are documentation. `//` and `/* */` do not exist outside them, anywhere,
for any reason, and `the_code_carries_no_comments` reads every `.rs` file to say so.

This is not a style preference. It is a forcing function. Every comment that would have
been written is a place where the code failed to say something in the only language it can
be read in, and the pressure is to fix **the code** rather than to describe it. When a
comment really is the honest answer, it belongs in the `///` above the item, which is where
a reader looks first and where it survives the code being moved.

`TODO`, `FIXME` and `SAFETY` are not exceptions. A placeholder is an unfinished cycle, and
`no_placeholder_is_left_in_the_code` reports every one of them by file and line.

## 1.5 **A function body is at most 20 lines**

Counted from `{` to `}` inclusive. Documentation and blank lines are not counted. There
are no exceptions, because an exception is a hole the next person uses.

10 lines is a signal to look, not a signal to split. **Splitting is not the answer to a
long body** — extraction adds names, and names cost the reader more than lines do. The
answer is that a body over 20 lines almost always holds **two** things, and the second one
gets a name from the glossary.

## 1.6 **A function body nests no deeper than two levels**

Three levels of `if` is always one of three things: a `match` that wants to be written, a
chain of guards, or two functions in one. The first two are rewrites; the third is 1.5.

## 1.7 No generic infrastructure vocabulary

The list lives in the glossary and is read from there rather than restated, so the guard
follows the document instead of drifting from it. A banned word is a noun a name **ends**
on, because the last word of an identifier is what the name claims the thing *is*.

`Adapter`, `Port` and `UseCase` are on that list on purpose. This repository is Clean
Architecture, and a codebase that names its own layers stops talking about the work.

## 1.8 No layer may hide control flow

No macro that generates control flow, no dynamic registration, no `Deref` that runs code.
A `derive` that generates a `Display` is a dependency, and the domain has none.

---

# RULE 2 — The code mirrors the domain

The domain is not something the code describes. The code **is** the domain, written down.

## 2.1 One concept, one name, everywhere

No synonyms, ever. If `WorkItem` is the term, `Task`, `Ticket`, `Job` and `Entity` are all
wrong, and `a_synonym_is_never_used` reads the glossary's `Not` columns to say which words
are wrong.

## 2.2 Names come from the glossary, and the glossary comes first

A concept that is not in [`docs/domain-glossary.md`](docs/domain-glossary.md) does not get
used yet. The glossary was written before the first type, and a term in the code that is
not in it is a violation — checked, in that direction, by
`the_glossary_blesses_every_type_the_domain_defines`.

The other direction is a reviewer's job, because the glossary deliberately names things
that have not been built. **`TheClock`, `ARecordOfARun` and `AnUntrustedTestSuite` are in
there and nothing implements them**, and that is the accepted cost of a rule that would
otherwise cost a whole cycle per new port.

## 2.3 The two verbs of the domain

`transition` and `validate_state`, and no third. `a_work_item_commissioned_for_an_agent`
is a birth, not an operation.

## 2.4 The direction of the dependency, and the one exception

```text
aisdlc-domain  ──►  (nobody)
aisdlc-run     ──►  domain
aisdlc-outside ──►  run, domain
aisdlc-desk    ──►  outside
```

**`aisdlc-domain` has no `[dependencies]` section at all.** Not "few", not "only serde".
Nothing. This is the one rule the compiler enforces without being asked, and every other
boundary rests on it.

Ports are traits declared in `aisdlc-run` and implemented in `aisdlc-outside`, so the
adapters know the ports and the ports know nothing about the adapters. **`aisdlc-desk` is
the one place a concrete type is named**, and it is the composition root.

**No `dyn` in the domain or the run.** Ports are generic and the composition root
monomorphises, so no trait has to be made object-safe and no `async` macro has to rewrite
a signature.

## 2.5 Data crossing a boundary is a named type

Not a tuple, not a map, not a bare `String`. And a **wire type is not a domain type**:
`aisdlc-domain` knows nothing about serialization, so every type that crosses a boundary is
written by hand in `aisdlc-outside` and builds its domain value by calling the same
validating doors a fresh one goes through.

## 2.6 Errors are named after the rule they protect

`StateMachineError`, `AnUncheckedWorkItem`, `AgentDecidedError`. Not `AppError`, not
`CustomError`, and not `anyhow` at a boundary that can name what went wrong.

---

# The domain in one page

The work item is a first-class domain object. **Never let a model mutate state directly** —
`transition` is the only door.

```text
STAGE  IDLE | AWAITING_AGENT_PICKUP | IN_PROGRESS_BY_AGENT
       | AWAITING_HUMAN_APPROVAL | READY | DONE
ROLE   PO | DEV | QA | SEC | ARCH          the discipline accountable for the work
AGENT  an explicit identifier such as "aisdlc-po-analyze" — or None
```

**Biconditional invariant**, enforced by `validate_state`:

```text
AGENT is None  <->  STAGE in { IDLE, AWAITING_HUMAN_APPROVAL, READY, DONE }
AGENT is set   <->  STAGE in { AWAITING_AGENT_PICKUP, IN_PROGRESS_BY_AGENT }
```

**The transition table is exhaustive.** No implicit transitions, no magic status changes,
no model-generated arbitrary states.

| From | To |
|---|---|
| `IDLE` | `AWAITING_AGENT_PICKUP` |
| `AWAITING_AGENT_PICKUP` | `IN_PROGRESS_BY_AGENT` |
| `IN_PROGRESS_BY_AGENT` | `AWAITING_HUMAN_APPROVAL`, `AWAITING_AGENT_PICKUP` |
| `AWAITING_HUMAN_APPROVAL` | `AWAITING_AGENT_PICKUP`, `DONE` |
| `READY` | `AWAITING_AGENT_PICKUP` |
| `DONE` | — terminal |

**Model output can never bypass transition validation.**

**The pipeline**, with a human approval gate after each artifact:

```text
repository -> discover -> [approval] -> PO -> [approval] -> QA -> [approval]
  -> architecture -> [approval] -> security -> [approval] -> implementation plan
  -> [approval] -> implementation -> QA verification -> [approval] -> pull request
```

**The three layers of every file.** A file belongs to exactly one.

| Layer | Responsibility | Rule |
|---|---|---|
| **A — deterministic** | state, invariants, transitions, persistence, GitHub, filesystem, test execution, validation | **must never reach for a model** |
| **B — agentic** | repository understanding, requirements, QA scenarios, architecture, security, planning, code generation, review | asks a model, through a port |
| **C — observability** | traces, latency, token usage, model comparison, golden datasets | observes the others |

**Human approval is a hard architectural boundary.** AI proposes, a human reviews and
approves, AI continues. An agent approving its own output is refused by name, not by
convention.

---

# The gates

**Every rule in this file has a gate. A rule with no gate is not a rule, it is a wish**,
and a wish is worse than nothing because a reader stops checking.

| Rule | Gate |
|---|---|
| No `unsafe` | `#![forbid(unsafe_code)]` — the compiler |
| Every public item documented | `#![deny(missing_docs)]` — the compiler |
| The transition table is exhaustive | an exhaustive `match` — the compiler |
| An inconsistent `WorkItem` cannot be built | private fields and three validating doors — the compiler |
| The domain depends on nothing | `the_domain_depends_on_nothing` |
| Dependencies point inward | `the_dependencies_point_inward` |
| No organisation or runtime in the run | `every_third_party_crate_is_on_its_crate_allowlist` |
| No comments | `the_code_carries_no_comments` |
| A body is at most 20 lines | `a_function_body_is_no_longer_than_twenty_lines` |
| A body nests two deep at most | `a_body_nests_no_deeper_than_two` |
| No generic vocabulary | `a_name_is_never_generic_vocabulary` |
| A synonym is never used | `a_synonym_is_never_used` |
| A type is a term the glossary blesses | `the_glossary_blesses_every_type_the_domain_defines` |
| The documents say what the code does | `the_documentation_says_what_the_code_does` |
| No `dyn` in the core | `the_core_never_names_a_trait_object` |
| Green with no credentials | `no_test_reads_an_environment_variable` and `the_gate_workflow_needs_no_secret` |
| An ignored test says why | `an_ignored_test_says_why_it_is_ignored` |
| No placeholder | `no_placeholder_is_left_in_the_code` |
| No `#[allow]` | `no_allow_silences_a_lint` |
| A cycle is a RED and a GREEN | `every_change_is_a_red_then_a_green` |

**Two things every gate has in common.** It is exercised on input it must catch *and* on
input it must leave alone, because a guard that has never been seen to fail is not evidence
of anything. And it names the file and the line, because a gate that says "something is
wrong somewhere" is a gate nobody can act on.

**What a gate does not do.** It stops drift and makes a breach visible. It does not stop
somebody who has decided to breach it, and it does not judge the prose in a `///`. Claiming
more than that would be a claim this repository is not built to make.

---

# Layout

Only what exists is listed. A crate or a module appears when the first test that needs it is
written, not before.

```text
crates/
  domain/     WorkItem, Stage, Role, Approval, LEGAL_TRANSITIONS, transition
  run/        the use cases, the ports as traits, the router, suspend and resume
  outside/    github, the project on disk, programs, the container, the model, sqlite
  desk/       the composition root: the only place a concrete type is named
tests/        the gates. One file per rule, and the file name states the rule.
real/         a real model, a real repository, a real container; ignored by default
docs/         domain-glossary.md, the source of truth for naming
```

---

# Commands

```bash
cargo test --workspace --no-fail-fast   # the default suite: no network, no credentials
cargo test --workspace -- --ignored      # real models, real repositories, Docker
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
cargo doc --workspace --no-deps
```

**GATE is all five.** `--no-fail-fast` is not optional politeness: `cargo test` stops after
the first failing binary, so a count read out of its output is a count of the suites that
happened to run first.

**The `e2e` tier is deliberately not in CI.** It needs a model and a credential, and a
repository that needs a key to be tested is one whose tests are not run by everybody.

---

# Backlog

One slice, one cycle set, one gate. Never build everything at once.

| Phase | Slice | Gate |
|---|---|---|
| 0 | the workspace, the glossary, and the gates themselves | every gate fails on its own fixtures first |
| 1 | the domain: entities, the state machine, approval | 36 stage/agent pairs crossed, and a seventh `Stage` does not compile |
| 2 | the ports in `aisdlc-run`, and the use cases | no `dyn`, no `async` macro, no runtime named |
| 3 | `aisdlc-outside`: the project on disk, programs, the container | a real container behind `#[ignore]`, with the reason printed |
| 4 | `aisdlc-outside`: the model, four organisations, a forced tool call, a correction loop | a real model answers with a validated artifact |
| 5 | `aisdlc-outside`: GitHub | a real push to a real fork |
| 6 | the lifecycle: eight gates, one router, suspend and resume | no route without a destination; a whole run counts its own container runs |
| 7 | `ARecordOfARun`: one SQLite table | a whole run is reconstructable from its own record |
| 8 | `aisdlc-desk`: Tauri and Svelte, five screens | a gate is decided in the interface and recorded as a person or a policy, never as an agent |
| 9 | the whole run, on a real model and a real repository | one end-to-end run produces a traceable pull request |

## Definition of Done

An item closes only when a test or a command proves it — never by a manual run.

State machine deterministic · invariants tested · human approval durable · agent ownership
enforced · a real model produces a validated artifact · a repository's own suite runs with
no credential reaching it · a whole run is reconstructable from its own record · **one
end-to-end run produces a traceable pull request** · `cargo test` needs no network and no
key · every gate is enforced rather than promised · the documentation matches the code.
