# ai-sdlc-app-rs-plus

## What is here

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

**There is still no pipeline at the gate.** Every step above exists, and the steps run in order
against the real thing, **and no agent produces the artifact the gate is asked about.**
`aisdlc-discover` picks up the work; it does not discover anything yet, so there is no
`DiscoveryReport` to approve and the ruling can only be a refusal. **What is proved is a run that
reads, presents, asks, records and moves — and not one that does the work.**

## An agent that files work of its own

**`AISDLC_AN_AGENT_OFFERS=yes` opens a second thing this repository does**, and it is off unless a
deployment says so:

    clone → the files the checkout holds → a model asked what is worth filing
      → a filing made with `gh issue create` → the filing read back → a work the product
        analyst answers, commissioned by `llm agent`

**A whole creative run, on a real repository, with a real model — proved in one run.**

    filed and worked on: https://github.com/steamnoid/wellbeing-tracker-public/issues/15
    test result: ok. 1 passed; 0 failed

**The filing carries the offer and the addresses the run derived from the files it actually saw** —
because the person reading it is the one who has to check it, **and a confident claim with no way to
check it is worse than no claim at all.** The work is then built **from the filing read back and not
from what the run remembers writing**, and the run's own copy of the words is not in it: a person who
edits one word of an issue must not have that word appear in the work.

**Three things stop it, and all three are refusals rather than warnings.**

- **The flag is read before the model and before GitHub.** A model asked costs money and a filing
  made lands in a real repository, **and a flag read afterwards is a flag that was not there when it
  mattered.**
- **An offer that names no file the run showed the model is refused**, because a model that invents
  `src/main.rs` in a project with none **produces an offer that reads exactly like a real one.**
- **A filing read at one address and paired with another is refused**, because both halves are real
  and nothing downstream could tell.

**Every run of the tier files a real issue.** It is `#[ignore]`d for that reason and not for speed,
**and five runs filed six issues in a fork this deployment controls** — two of them the same
observation twice, closed as duplicates, the rest distinct and open.

## What the real tier found, and what it could not have found with a stub

**Three real runs produced three different answers to the same question**, and every one of them was
a defect this repository had been calling finished:

| what a real model did | what it proved about the code |
|---|---|
| answered with `offers` only | **the question had never asked for the three judgements its parser demanded** — a prompt that does not name a field and a parser that insists on it look identical from inside the type system |
| answered with all four keys and called the paths `paths_it_concluded_from` | **a field named in prose is a field the model names**, because the question said *the paths it concluded from* and the model named the field after the sentence |
| put all three judgements inside one offer's `what_it_asks_for` as prose | **three failures of one kind is a pattern, and the pattern says a prompt was being treated as a contract** — the check belongs in the parser and the question only says what to write |

**And a real run found two defects of its own, both invisible from a test double:**

- **`AGitHubWeWriteTo` had no implementation for a filing.** Seven cycles of the creative path were
  proved entirely on a double, **which is exactly what happened with a comment**: the port had one
  implementor and it was a test double, so the use case was proved against a place that accepts
  anything.
- **The address parser was written for the wrong form.** GitHub's `html_url` carries a scheme and
  this system stores the bare `owner/name/issues/N`, **so a real answer refused to parse in a run
  that had already filed the issue.**

**None of the four was findable with a stub**, and one of them was found by a model refusing to
answer — **which is the whole argument for a tier that talks to the real thing.**

**Nothing in this repository reads this file, so every claim in it is a promise rather than a
check.** The numbers that used to sit beside the crates were the most perishable facts here — two
of them had gone stale inside a single working day — **so they are gone, and what each crate is
instead of how long it is.** The one count left is the test count, because it moves only when a
test is added and because it is the one number a reader wants. The claim that matters is the gate
table in the glossary, because a gate holds that one.

## The three rules

1. **RULE 0 — strict TDD.** Every behaviour change is RED → GREEN, as two commits. The RED must
   fail on an assertion about behaviour and not on a missing import.
2. **RULE 1 — readability is the highest priority.** The code is read far more often than it
   is written, and it is read by a human. **There are no comments in the code.** `///` and `//!`
   are documentation and are allowed; `//` and `/* */` are refused by
   `the_code_carries_no_comments`, which reads every `.rs` file and names the file and the line.
   A function body is at most twenty lines, nests no deeper than two, and a file is at most three
   hundred lines of code.
3. **RULE 2 — the code mirrors the domain.** One concept has one name everywhere, the names come
   from [`docs/domain-glossary.md`](docs/domain-glossary.md), and `aisdlc-domain` has no
   `[dependencies]` section at all — **which is the one boundary rule the compiler enforces
   without being asked**, and every other boundary rests on it.