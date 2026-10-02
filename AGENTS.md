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
A GREEN with no preceding RED on the branch is a RULE 0 violation, and **nothing checks the
history**: `every_change_is_a_red_then_a_green` was in the gate table and is not a rule any more,
because a rule that reads the commit log to police the process that writes the log is a rule about
the shape of the work and not about the work. **The pairing is checked by the reviewer reading
the two commits, and by a branch that does not compile between them.**

Stage the files the cycle is about, not everything. `git add -A` with a cycle still open
sweeps the *next* cycle's files into this one.

**A commit is an index, not an essay.** The subject says what changed; the body says what now
holds and what the rule found, and nothing else. **This repository ran 38 commits and 22 072 words
of commit message over 9 126 lines of code — 2.4 words of prose per line of code**, written by a
process that had already stopped measuring whether the work existed. Anything longer than that
belongs in the glossary, where a reader goes looking for the reason, or in a `///` above the code
it explains, where a reader going *to* that code will actually meet it.

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

## 0.3a A gate and the breaches it finds are two pull requests

**Every gate in this repository found real breaches in code written hours earlier**, and one
found seven in a single run. Six for six is not a coincidence and it is not the gate's fault
either: the code was written to satisfy the previous rule and nothing had yet asked about the
next one. A rule nobody has run finds nothing.

**So the gate lands alone, and the breaches it reports land in the next pull request.** Both
pass their own `gate` check, because the second one's work is what makes the first one's test
green — and the reader of the first one sees a gate added and nothing else, which is the diff
that says what it says. A pull request that adds a gate *and* rewrites five files cannot be
read as either thing.

The cost is stated because it is real: for a day, `main` is a repository whose gate does not
hold. That is the honest state and it is visible in the check that failed, which is worth
more than a green tick that arrived in a diff nobody could read.

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

## 1.9 A file is at most 300 lines, and 200 is a signal to look

Counted from the first line to the last, with documentation and blank lines not counted. No
exceptions, because an exception is a hole the next person uses.

**A long file is not wrong because it is long. It is wrong because it has two reasons to
change, and a reader who opens it cannot tell which half they are in.** The number is a
symptom, and the question a refusal asks is *name the second reason*, not *you have four
hundred and twelve lines*. A gate that only counts teaches nothing.

**Where the number comes from, and it is not Uncle Bob.** Robert C. Martin gives no number
for the length of a file. In *Clean Code* a function is bounded by a page, and a file is
bounded by having one reason to change. The 500 lines that circulate under his name come
from Jeff Vincent citing him. His own numeric rule is the "Rule of Five" — about members,
dependencies and responsibilities rather than lines — and he withdrew it as dogmatic.

The number here is ours, and it comes from two other rules in this file and from the files
the previous version of this repository actually contained: **432 lines at the median, 838 at
the ninetieth, and 1394 at the worst.** A ceiling set above the median and below the worst is
a ceiling that bites on a real failure and leaves an ordinary file alone. The longest of those
files was a test of the twenty-line rule, three times longer than the rule it was checking.

The derivation is 1.5. A body of twenty lines and a file of three hundred is fifteen bodies,
which is a list of parts a reader can hold. A file of a thousand is fifty, which is a list of
parts and a second idea, and the second idea is what a reader cannot find.

**A table of data is a `const`, and a `const` has no body**, so a file is never made longer by
holding its own fixtures inline. That is the answer to this rule and it is the same answer
1.5 has for a long body.

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
aisdlc-gates   ──►  (nobody)     reads the crates; never calls them
```

**`aisdlc-domain` has no `[dependencies]` section at all.** Not "few", not "only serde".
Nothing. This is the one rule the compiler enforces without being asked, and every other
boundary rests on it.

`aisdlc-gates` is a package of the workspace and **not a part of the system**. It is the
only code here allowed to *read* the other crates rather than call them, which is why it
cannot sit inside a layer — a gate that shares a dependency direction with the thing it
checks is not a boundary. It is also the reason the workspace is not a virtual manifest:
**a virtual workspace has no integration-test target**, so without this package
`cargo test` is green while running nothing.

Its library is not empty: it holds the scanner the gates share, because **a test double is
production code** and five gates calling a copy of their own is five names waiting to
disagree. It is `pub` so nothing has to be silenced — a private helper in a test binary is
dead the moment the test that does not use it is compiled, and silencing that is
`#[allow(dead_code)]`, which 1.3 forbids.

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

**Every rule in this file has a gate, and the table of them is in
[`docs/domain-glossary.md`](docs/domain-glossary.md) with the module that holds each one.** A rule
with no gate is not a rule, it is a wish**, and a wish is worse than nothing because a reader stops
checking.

So is a gate that was never seen to refuse anything. **Each column says what a gate arrives
with, and the answer is always the code it judges** — a gate written before the code it
polls is a gate nobody has watched catch a mistake, and the earlier version of this
repository had eight of them, all green, all empty.

**There is one table of rules and it is in
[`docs/domain-glossary.md`](docs/domain-glossary.md), with the module that holds each one.** This
file used to carry a four-row table of its own, with the two gate files named by bare filename and
no path to them, and prose about the rows marked *not built* — **of which it had none, every row
saying it was built.** Both halves were copies of a table the glossary holds and a gate checks, and
a second copy of a checked fact is a copy that drifts: this one carried two dead sentences before
anybody looked.

**So the copy is gone rather than refreshed**, which is the third time in this repository that the
answer to *this document disagrees with a checked one* has been to delete the copy instead of
correcting it. A summary is a second place to be wrong, and the glossary's table is readable.

**The transition table and the biconditional are compiler-enforced, not read from this
file.** `legal_transitions_from` is an exhaustive `match` on `Stage`, so a seventh stage
does not compile until the table is completed. The table below is here for a reader, and
`the_documentation_says_what_the_code_does` is the gate that will hold it to the code.

| From | To |
|---|---|
| `IDLE` | `AWAITING_AGENT_PICKUP` |
| `AWAITING_AGENT_PICKUP` | `IN_PROGRESS_BY_AGENT` |
| `IN_PROGRESS_BY_AGENT` | `AWAITING_HUMAN_APPROVAL`, `AWAITING_AGENT_PICKUP` |
| `AWAITING_HUMAN_APPROVAL` | `AWAITING_AGENT_PICKUP`, `DONE` |
| `READY` | `AWAITING_AGENT_PICKUP` |
| `DONE` | — terminal |

**Two things every gate has in common.** It is exercised on input it must catch *and* on
input it must leave alone, because a guard that has never been seen to fail is not evidence
of anything. And it names the file and the line, because a gate that says "something is
wrong somewhere" is a gate nobody can act on.

**What a gate does not do.** It stops drift and makes a breach visible. It does not stop
somebody who has decided to breach it, and it does not judge the prose in a `///`. Claiming
more than that would be a claim this repository is not built to make.

---

# Layout

**Seven of the nine things below are here and two are not**, and which is which is a fact about
the checkout rather than an intention. A crate or a module appears when the first test that needs
it is written, not before, **so the two that are not are written as what they would be, and
`a_document_does_not_deny_what_this_checkout_holds` is what holds the difference.**

```text
crates/
  domain/     Stage, Role, WorkItem, Approval, the transition table and the two verbs
    tests/    the domain's own tests, because a test importing the domain cannot live in
              the root package and that package depends on nothing on purpose
  run/        the use cases, the ports as traits, the router, suspend and resume
  outside/    github, the project on disk, programs, the container, the model, sqlite
  desk/       TARGET, not here: the composition root, the only place a concrete type is named
src/          the gates' library: the scanner, and the helpers the gates share
tests/        the gates. Two files, and the file states the concern rather than the rule
real/         TARGET, not here: a real model, a real repository, a real container
docs/         domain-glossary.md, the source of truth for naming
```

**A gate lives in the package whose rules it judges.** The rules about the gates' own code
live in `tests/` at the root, which is the `aisdlc-gates` package; the rules about the
domain live in `crates/domain/tests/`. The alternative — making the root package depend on
the domain so that one `tests/` directory could hold everything — would have broken the one
rule every other boundary rests on.

---

# Commands

**These five are the gate**, and their exit code is the verdict.
`--no-fail-fast` is not optional politeness: `cargo test` stops after
the first failing binary, so a count read out of its output is a count of the suites that
happened to run first. **One of these five used to read that this repository has no
`Cargo.toml`, and `./scripts/gate` is the command it was talking about.**

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

# How a change reaches main

**`main` is protected, and nothing reaches it except a pull request whose `gate` is green.**
The protection also covers administrators, which is the whole point: the hand that merges
is the hand that skipped the gate three times, and a guard that does not apply to the guard
is not a guard.

```bash
git worktree add ../wt-<name> -b <scope>/<name> main
# ... the work, in its own worktree, on its own branch ...
git push -u origin <scope>/<name>
gh pr create --fill
gh pr merge --squash --delete-branch     # only once the gate is green; the title carries the RED
```

A worker branch is deleted after its merge, and a **squash discards its commits** — so only the
name goes and the commits do not: `main` carries one line per pull request, and a branch's two
commits are not among them. Measured on the seventy-eight pull requests so far: 160 commits on
branches, 69 on `main`, and **zero of the 69 contain a RED**. Without that, the repository
accumulates one short-lived branch per gate and stops looking like a line of work.

**Squash, always, and the squash is given a subject of its own** — because which of the two
GitHub uses depends on how many commits there are, and that was measured on five pull requests
rather than assumed:

| commits on the branch | the subject `main` gets |
|---|---|
| one | **the commit's own message** |
| two or more | **the pull request's title** |

**A behaviour cycle is two commits, so a cycle's subject is its title** — and the title was never
the pair, so for twenty-four paired cycles `main` held the GREEN alone and the RED nowhere. The
title is what a reader of the pull request sees; **the subject is what `main` keeps**, so the pair
goes in the subject, given explicitly rather than inherited from either:

```bash
gh pr merge --squash --delete-branch \
  --subject "GREEN — <the behaviour that now holds> (pairs with RED — <what failed>)" \
  --body "$(git log --format=%B --reverse -1)"
```

**This is not a gate and it is deliberately not one.** The rule that read the commit history was
removed from the table on the grounds that **it is a rule about the shape of the work and not
about the work** — a position taken deliberately and written down. What was not predicted is the
consequence of holding it *together with* squashing: the one record that something was broken
first went away twice over. **A gate is not brought back for this, because a gate is the thing
that was decided against; the record comes back through the title instead**, which is where a
squash already looks.

**`required_approving_review_count` is zero, on purpose, and it is a decision somebody
should know about.** A human review requirement would have been safer, and it would also
have meant that twelve hours of delegated work sat on branches that could not land, which
parks changes rather than policing them. The gate is the authority here: it cannot be
bypassed, it runs on every pull request, and it is the thing that refused the three merges
that were wrong. Raising the count to one is one command and turns this from a
gate-policed repository into a team one.

---

# Recorded departures

**This repository is no longer synchronised with the implementation it was read from, and
that is a decision rather than a drift.** There is no `SOURCES.lock` here and there is no
test that re-hashes a file in a sibling checkout.

The reasons, in the order they mattered:

1. **The specification is this repository's own.** The stages, the roles, the transition
   table and the approval record were carried across, but the *implementation* was written
   from scratch in a language with a different compiler, a different naming convention and a
   different type system. A pin would have recorded which commit a rule was read from, and
   then every rule would have had two homes: the glossary and the file it came from. The
   glossary is the one home.
2. **The two diverged seven times in the first day**, and every divergence was settled by
   moving a rule *into* this repository and writing down why. A pin would have made each of
   those look like a missed update rather than a decision.
3. **The rules here are not a translation of the rules there.** A domain with no
   dependencies, a port per boundary and twenty gates is a different architecture, not the
   same one in another language. Carrying a hash of a file that argued for the opposite would
   be worse than carrying nothing.

The cost, written down because this repository writes costs down: a reader who wants to know
where a rule came from has to read the glossary's reasoning rather than diff two files. That
is one file instead of two, and the one file is the one a rule has to survive in.

**The history was wiped on 2026-09-28 and the rules were not.** Seventy-one commits are
preserved at the tag `przed-rewrite` and the branch of the same name; the `main` that carried
them is an orphan with four files on it.

The reason is worth stating precisely, because "it went badly" is not one. The code was built
by eleven delegated workers working in parallel, in separate worktrees, and three things
happened that do not happen when one person works in one tree: two workers wrote the same
file and collided, three workers died without writing anything and left no way to tell that
from a refusal, and one worker built a file in the **wrong repository** and the branch had to
be untangled from a project that had nothing to do with this one. Fourteen gates came out of
it, and eight of those gates were green because there was nothing for them to judge — the
gates were built before the domain they were meant to police, so a gate that had never been
seen to refuse anything looked exactly like a gate that worked.

So: **one worktree, one branch, one cycle at a time, and the domain before its gates.** The
glossary survived because it was written before any code. The rules survived because nothing
in them depended on the code. Everything else was an artefact of an approach that is not
being repeated, and keeping it would only have made the second attempt harder to see.

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
