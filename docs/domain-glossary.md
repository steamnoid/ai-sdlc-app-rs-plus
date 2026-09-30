# Domain glossary

The source of truth for naming in this repository. A concept that is not here does not
get used yet, and a term in the code that is not here is a violation. The second
direction is checked by a test; the first is a reviewer's job, because this file
deliberately names things that have not been built.

A test reads this file rather than restating it, so that the guards follow the document
instead of drifting from it. Every guard is also exercised on a name it must catch and on
names it must leave alone, because a guard that has never been seen to fail is not
evidence of anything.

This file is the **first** thing written, before the first type. The reason is a rule
rather than a preference: a name decided while writing the code that uses it is a name
nobody looked at twice.

---

## How this file is read

| Where a name is written | What it does |
|---|---|
| a **bolded** entry | a blessed name — every type the domain defines must be one of them |
| a **code span** in ordinary prose | a blessed name, for the same reason |
| a code span in a table column headed **`Not`** | **refused, and not blessed.** Read by `a_synonym_is_never_used`, which also reads the next section and knows which of these are a whole name only. |
| a code span in prose naming a refused word | **refused, and not blessed** — which is why a refused word is written in plain prose here |
| anything under **`## Banned vocabulary`** | **refused, and not blessed** |

**Two exclusions, and neither is optional.** A prohibition read as a term is exactly
backwards, and this file has now made that mistake twice in two different shapes.

The first was the banned section: it explained its own rule in backticks, the explanation
blessed the example, and the guard passed the very name it existed to catch. A guard whose
documentation can disable it is one careless edit away from being off, so the whole section
is removed before the reading begins.

The second was every `Not` column, and it was found by a worker rather than by a failing
test: Task, Ticket, Job, Entity, Status, State, Phase, Step, Owner, Assignee, Team, move_to,
set_stage, advance, change_state, handle, check, verify and assert_valid were all in the blessed set, because they are written in code
spans inside a table. The reader decides by the **table's header row**: a cell in a column
headed `Not` is not a name, whatever else the column is.

The second shape was the same mistake one level deeper, and the reader cannot fix it: a
refused word written in backticks in **ordinary prose** is a blessed name, because prose
and a cell are read the same way. The reader can only empty cells, so the **document**
carries the obligation: **a refused word appears once, in a `Not` column, and nowhere else
in backticks.** Every other mention of one in this file is plain prose, on purpose, and a
sentence explaining the rule that backticked its own example has brought the ban down again.

---

## The core nouns

**`WorkItem`** — one tracked unit of work against a repository. The first-class domain
object. Its stage is never changed by assignment; `transition` is the only door.

**`WorkItemId`** — the name a work item is tracked by. A work item cannot be approved,
persisted or pointed at by an approval without one, so a blank one is refused where it
enters.

**A name this document lists is one concept and the code has one type for it.** A newtype
over `String` with a constructor that refuses, a `TryFrom<&str>` and a `Display` — and
**`Display` writes the name as written and nothing else**: no lowercasing, no trimming, no
reformatting. A name a person typed and a name a run reads back are the same name, and a
gate that compares them as strings would refuse a work item for a space somebody left in it
at the end.

**A blank means blank.** `""`, `"   "` and anything else that trims to nothing is refused,
and **the name is refused, not the trimmed name** — a caller that sent `"  x  "` gets `"  x  "`
back, because a name this system silently tidies is a name a lookup will miss.

**`WhoCommissioned`** — the party that asked for the work. **Who asked** and **which
discipline answers** are two questions in two fields, and the name is kept verbatim.

**The `aisdlc-` namespace belongs to agents and to nothing else.** An agent identifier
starts `aisdlc-` — `aisdlc-po-analyze`, `aisdlc-dev-impl` — and a gate **refuses a decision
made by one**, because an agent may ask for work and may not rule on the result. That is
why the namespace is fixed and readable rather than free text.

**A policy is `autopilot:<policy>`, and that is a different namespace on purpose.** A policy
may rule at a gate, on somebody's behalf, and the record says so verbatim rather than naming
a person: a record naming a person for a decision a policy made would be a forged audit
trail, and it would look exactly like the real thing at the moment somebody needs to tell
them apart. **One prefix cannot mean both "an agent" and "a policy"**, because the gate's
whole rule is that an `aisdlc-` name is refused — a policy sharing that namespace would be
refused along with the agents it is defined in contrast to.

This line previously said that an `aisdlc-` identifier meant a policy, which contradicted
the gate rule that refuses every one of them. Both were in this repository: the sentence here
and the check in `docs` and, in the earlier implementation, in the gate itself.

**The prefix is matched exactly, and the exactness is a decision rather than an oversight.**
`AISDLC-PO` and `"  aisdlc-dev-impl"` are **not** agents, because the namespace is spelled in
lower case and a name is compared as it was written. This is the same rule that keeps
`AgentIdentifier` from refusing `NONE` when it refuses `none`, and it is stated here for the
same reason: **a rule nobody has stated is a rule nobody may rely on, and the next person
to read a failing test would "fix" it by adding `to_lowercase()`.**

**It is a rule about names, not a control against an adversary.** Somebody who meant to
evade it could write `aisdlc_` or choose another prefix, so tightening the match would buy
nothing and would cost a legitimate name. What this refuses is a **model** that answers with
an identifier it invented, which is the threat this system actually has.

**`AgentIdentifier`** — an agent, named. Absence is `None` and never a string that says
"absent": `"none"`, `""` and `"   "` are all refused, because the state machine decides
whether an agent is present by asking whether there is one.

**`Repository`** — a public GitHub repository, named by its owner and its name. A half of
that boundary that is blank names nothing, and a half carrying a slash is both halves
pasted into one field.

**`CommissionedRequest`** — what somebody asked for, in the words they used. **Text, not
an address.** The words arrive with the work, the work item keeps them, and a stage that
reads them nine reads later reads what the first one read.

**`WhichArtifact`** — which of the eight artifacts a decision was about. **A name and not
a closed set of eight, and the reason is worth stating because the opposite looks tidier.**
The eight artifacts are produced by agents that do not exist yet, and naming their
identifiers is their business. A closed set written before them would bake a list into the
enum, the gate and this document, and every agent arriving later would have to match a name
it did not choose. **So the type refuses a blank and nothing else, and the first artifact
whose name is wrong is found by the router refusing to send a gate there.**

This is the one place in the domain where a `str` was nearly right, and it is still not a
`str`: a blank is refused at the door, so a decision can never be recorded about nothing.

**`WhichOrganisationReasons`** — who does the reasoning: `openai`, `ollama`,
`openai_compatible`, `openai_responses`. A closed set of four. Distinct from
`AReasoningModel`, which is the thing that reasons; this is the company it works for.

**`AnOrganisationThatDoesNotReason`** — a name that is not one of the four, so it is not an
organisation that reasons. **The refusal names what was asked for and what there is**, because
a refusal that only says "no" leaves the reader to guess which name was wrong when there are
exactly four right ones. A blank is refused here too, and it is not a blank name for one of the
four — it is nothing at all, and a run must not record who reasoned about a piece of work using
an answer to a question nobody asked.

**`EVERY_STAGE`**, **`EVERY_ROLE`** — all six stages and all five disciplines, so a caller
crossing one thing with every stage does not write a second list to keep in step. **Rust
writes a constant in capitals and a `#[allow(non_upper_case_globals)]` is refused by a gate
here, so these are the glossary's `EveryStage` and `EveryRole` in the language's own case.**
The vocabulary guard normalises a name to letters and digits, so the two spellings are one
name to it and one entry here blesses both.

**A variant is the reading of a name and the capitals are the writing.** `Stage::Ready` is
`READY`; `Role::Product` is `PO`. The two are one concept with two spellings, joined by one
`Display` and one `TryFrom`, and the capitals exist in exactly one place. **The variant is
the discipline rather than the acronym, and that is a decision rather than a coincidence:**
clippy accepts `PO` and `QA` and refuses `DEV`, so writing two of the five one way on the
strength of a letter count is a spelling a reader has to look up. All five are written the
same way because the reader should not have to know which is which.

**`AHalfThatIdentifiesNothing`** — a repository that names an owner and no name, or a name
and no owner. **A half carrying a slash is the other refusal**, and the two are one type
because a caller told "that is not a repository" has the same question whichever half it
left out.

**`AWorkItemThatNamesNothing`** — a work item whose name is blank. A work item cannot be
approved, persisted or pointed at by an approval without one.

**`AWorkItemThatNamesNobody`** — a work item that says nobody asked for it. Work with no
origin cannot be traced back to the request that started it.

**`AnAgentThatIsNotAnIdentity`** — a string that only means "absent": `"none"`, `""` and
`"   "`. **All three are refused, and the reason is the state machine**: it decides whether
an agent is present by asking whether there is one, so a work item in `IN_PROGRESS_BY_AGENT`
holding a blank agent would pass the biconditional while nobody was working on it.

**`ARequestThatSaysNothing`** — a commissioned request with no title and no body. A request
that says nothing was asked for.

**`AStageThatNobodyNamed`**, **`ARoleThatNobodyNamed`** — a name that is not one of the
closed set. They exist because a document names its stages and roles in capitals and the
documentation guard has to read them. They are also the only ways into a `Stage` or a
`Role` from text, so a name nobody wrote down never becomes one.

**`AnArtifactVersion`** — which version of an artifact somebody ruled on. Counts from one;
there is no zeroth version of anything.

**`AnArtifactVersionBelowOne`** — a version below one, which is not a version. It is a
separate type and not a message on the version because a caller told "zero is not a version"
has one question to ask, and a type of its own lets the compiler ask it at the door.

**`AnApprovalThatNamesNothing`** — an approval whose artifact name is blank. A decision can
never be recorded about nothing, so the blank is refused where the name enters.

**`ADecisionThatNamesNobody`** — a ruling whose decider is blank. Distinct from the approval's
refusal because the two are different doors: one is about what was ruled on and one is about
who ruled. **A record naming nobody is not a record that is merely incomplete**, and the
same refusal for both would have hidden which of the two was missing.

---

## Rules that a gate reads rather than a paragraph

**`a_body_nests_no_deeper_than_two` counts every brace and not only the control flow.**
`AGENTS.md` §1.6 is about control flow, and a struct literal is not control flow — so the
strictest honest reading of the document and the mechanical reading the gate implements are
not the same rule. The mechanical one was kept, and the difference is paid for in the code:
`StateMachineError::for_a_move_the_table_refuses` and `AnUncheckedWorkItem::because` exist
because `return Err(SomeError { why: format!(…) })` is three levels deep, and **a named
constructor is better code whether or not a gate is watching.**

**The cost is stated so the next reader can disagree with it.** Counting every brace refuses
a struct literal written inside a branch, and a reader who finds that too strict has one
place to change: this paragraph, and the two lines of the gate that count braces.

---

## The stages

The closed set. `Stage` is an enum; a stage is never a bare string, and it is written
with its own name in capitals.

| Value | Meaning | The agent must be | Not |
|---|---|---|---|
| `IDLE` | Nothing is happening. | absent | `Status`, `State`, `Phase`, `Step` |
| `AWAITING_AGENT_PICKUP` | An agent may claim this item. | present | |
| `IN_PROGRESS_BY_AGENT` | An agent is acting on this item. | present | |
| `AWAITING_HUMAN_APPROVAL` | A human must rule before anything continues. | absent | |
| `READY` | The entry state. A person queued this, and nobody holds it yet. | absent | |
| `DONE` | Finished. Terminal, no outgoing transition. | absent | |

A table rather than prose, because a test reads it to check the code against it.

Nothing in the transition table targets `READY`, and that is a property rather than a gap:
the framework cannot put work in its own queue, and a stage added next year is bounded by
the same table.

**Status and State are in a Not column on purpose.** Three different words have been
used for the same fact across the two implementations, and a reader who has seen two of
them has to stop and work out which one this is. One word: `Stage`.

---

## The roles

**`Role`** — the discipline accountable for a work item. The closed set. A table for the
same reason the stages are a table.

| Role | Meaning | Not |
|---|---|---|
| `PO` | product, accountable for what should be built | |
| `DEV` | development, accountable for code | |
| `QA` | quality, accountable for test strategy and verification | |
| `SEC` | security, accountable for risk | |
| `ARCH` | architecture, accountable for structure | `Owner`, `Assignee`, `Team` |

Every one of them is legitimate at every stage. The role is a discipline, and nothing ties
it to where the work item happens to be. There is no `AI`: a handoff is not a discipline,
and a set whose own definition excludes one of its members is not a set of disciplines.

---

## The two verbs of the domain

The specification states exactly two operations on state. These are the two, and a third
is a bug.

| Verb | What it is | Not |
|---|---|---|
| `transition` | move a work item to a legal next stage and hand back the moved work item | `move_to`, `set_stage`, `advance`, `change_state`, `handle` |
| `validate_state` | assert that a work item satisfies the invariants of its own state | `check`, `verify`, `assert_valid` |

A birth is not an operation. `a_work_item_commissioned_for_an_agent` is the only way a
work item enters the lifecycle and it adds no third verb.

**`state_machine` is the module those two verbs live in, and it is blessed in full.** It
carries `State`, which the stage table refuses, and a gate asks the blessing before it asks
the words — **because the module is named after the two verbs it holds, and renaming it would
be worse than carrying a refused word.** The alternative was `the_lifecycle`, and that is a
word about a process rather than about the thing that refuses an illegal move.

---

## The three doors into a work item

**This is where a clean domain earns its keep.** A work item has private fields, so the
only ways to build one are the doors below, and **every one of them validates**. An
earlier shape had a constructor that did not, which meant a work item could be built at a
stage that needs an agent and hold nobody — and a checkpoint carrying exactly that was
read back as a real work item.

| Door | What it is | Refuses |
|---|---|---|
| `standing_at` | a work item at a stage, held by nobody | a stage that needs an agent |
| `held_by` | the same work item, held by an agent | holding a stage where nobody may hold |
| `answering` | the same work item, carrying the request | nothing; a request is a fact, not a position |

**Serialization is not a fourth door.** The domain has no `serde`, no wire types and no
knowledge that a disk exists. Every type that crosses a boundary is written by hand in
`aisdlc-outside`, and it builds its work item by calling the three doors above. A
checkpoint is therefore validated by the same code that validates a fresh one, which is
the whole point: a derived `Deserialize` never calls a constructor, and a boundary that
does not call one is a boundary nobody checked.

**`AnUncheckedWorkItem`** — the door said no. One refusal type for all three, because a
caller that has just been told "this cannot exist" has one question, and three types would
make it choose which one to ask.

---

## The ports

**A port is a question the domain cannot answer by itself.** It is declared in
`aisdlc-run` and implemented in `aisdlc-outside`, and the direction is one-way: the
adapters know the ports, the ports know nothing about the adapters. There are ten, and
two of the splits are not inventions — they are lines this domain had already drawn.

| Port | Asks | Implemented by |
|---|---|---|
| `AReasoningModel` | to answer with a validated artifact | four organisations, one at a time |
| `AGitHubWeReadFrom` | for a checkout, and for a file at a ref | HTTPS, no credential |
| `AGitHubWeWriteTo` | for a branch, and for a pull request | `git` over SSH, always a credential |
| `TheProjectOnDisk` | for a listing, and for a file's text | the local filesystem |
| `AProgramWeRun` | for our own programs, on the host | `git` and this project's tooling |
| `AnUntrustedTestSuite` | for a repository's own tests, in a container | Docker, no host credential |
| `TheClock` | how long it has been, and to wait | the operating system, or a test's zero |
| `ARecordOfARun` | to remember a run, and to read it back | a SQLite table |
| `APersonToDecide` | to rule on an exact artifact | a person, or a policy on their behalf |
| `WhetherAnythingStandsThere` | whether anything stands at a directory, before anything is written there | the real filesystem |

**The third column says what would answer each port, and only three of them answer anything
today**: `TheProjectAsItIsOnDisk`, `TheProgramAsItRuns` and `WhetherAnythingStandsOnDisk`. The
other seven have a name for the thing that answers them and nothing behind the name, and **a
column that reads as a fact while seven of its ten rows are intentions is a table that has
rotted in the way the rule table next to it is checked not to.**

**"There are ten" was "There are nine" a moment ago, in the sentence introducing a table of ten
rows.** A count claimed in prose beside a table is a second copy of a fact the table already
holds, and a second copy is a copy that drifts — this one did. **No gate is built for it, and the
number says why**: this sentence is the only one in this document that claims a count, and it is
the one being fixed, so a gate would have one caller and no caller after the cycle. The fix is the
sentence; the gate would have been the thing #48 removed, rebuilt.

### Why reading and writing GitHub are two ports

Because **reading a public repository needs no credential and writing always needs one.**
When both were one port, a delivery had to be made by hand: the clone's credentials
belonged to some other account, and the reason appeared nowhere. Two ports means the type
says which half needs a secret, and the half that must not need one cannot be given one.

### Why our programs and a repository's tests are two ports

Because **executing a repository is the dangerous operation in this system.** Reading one
is not. An implementation agent writes code and a verification agent then runs that code,
with the developer's credentials and filesystem in reach. So the boundary goes around
*execution*, not around the run, and it goes around exactly the half that is untrusted.

**`AnUntrustedTestSuite`** — a test suite whose code this system did not write and cannot
veto. The word is a security fact, not a slur, and it is in the glossary because the
system has the fact and until now had no word for it.

**`ARepositoryToClone`** — which repository, for which work item, and the directory derived
from the two. **The directory is a field and not an argument**, because a derived place is
recorded once and then depended upon; a caller that passed it could pass a different one twice
and two runs of one work item would stop finding each other.

**`WhatWeTellAProgramToDo`** — what a program is asked to do, named after what the caller wants
and not as a command line. **A formatted line is where a value read out of a model becomes part
of a command**, so there is nothing here to format. **The whole history and not a depth**: the
request has no depth field, so there is nothing to pass one and nothing for a later change to
have forgotten.

**`WhichProgramWeRun`** and **`WhatTheProgramSaid`** — which of our own programs, and what it
wrote and whether it worked. **`did_it_work` and not whether it looked happy**: a program that
wrote a warning and returned zero did work, and a caller reading a return code as an outcome
learns about the wrong thing.

**`TheProgramAsItRuns`** — one of our own programs, really started on this host, and the one
implementation of `AProgramWeRun`. **A name and not `AProgramOnHost`**, which would say the host
rather than the thing that acts, and which invites reading the host as a collaborator rather than
as a place with our own tools on it.

**No shell, anywhere, ever.** The arguments are one thing each because **a formatted command line
is where a value read out of a model becomes part of a command** — and a shell metacharacter in an
argument is a second program in a disguise. **The program is named by the port's closed set and
never by the arguments**, so the same name is never in both places: two records of one decision
is one more thing to keep in step, and a reader cannot tell which one the process was started
with.

**A program that could not be started is a refusal and not a crash**, and it says so in the same
words a failure would — because the caller has to tell *git is not installed* from *there is no
such repository*, and that telling is not this system's to make up.

**`AProgramWeRun`** — a program we run ourselves, on the host, with our own arguments and
our own inputs. Its `working_directory` is a convenience, not a promise that anything may
be run there.

### What a port never carries

A port's arguments are named types, never a `String` and never a tuple. A port never
carries a credential, a token or a path that has not been through the project boundary
rules. **And a port with no implementation behind it is not a port** — it is a trait
nobody calls, so one arrives with the first caller that needs it and not before.

### `TheClock`, and why it is a port

The domain asks the clock two questions and never reads a system time itself. A bound that
depends on a caller is not a bound, and a test that has to sleep to prove a timeout works
is a test nobody runs. `TheClock`'s test implementation is an instant zero, so a
thirty-second bound is proven in no time at all.

### `APersonToDecide`, and why it is a port

Human approval is a hard architectural boundary, and a boundary that is a function call
inside one process still has to be an interface, because that is what stops the domain from
deciding for itself. **The thing behind the port may be a person or a policy, and the
record keeps the difference** — a record naming a person for a decision a policy made is a
forged audit trail, and it would look exactly like the real thing at the moment somebody
needs to tell them apart. An `aisdlc-` identifier is a policy; anything else is a person.

---

## The addresses of a repository

One repository is known by **five different strings**, and none of them is interchangeable.
They are five functions rather than one with a flag, because the choice has to be visible
at the call site.

**`the_address_of`** — what a person reads. `https://github.com/owner/name`, **with no
`.git`**, because this string ends up in a summary and the suffix is a thing `git` needs
and a person does not want.

**`the_clone_url_of`** — what `git` fetches from. The same string with `.git` on the end.
**No credential in it, ever, and the signature is what guarantees that**: the function is
handed a `&Repository` and returns a `String`, so there is nothing in scope that could
carry a token.

**`the_push_address_of`** — where a repository is **written**, which is a different place
from where it is read. Over SSH, and separately from every read address.

**`the_read_address_of`** — where one file of one repository at one **ref** lives. The ref
is a parameter rather than the repository's default branch, because reading a file at a
pull request's head is the question this address exists to answer.

**`Addresses`** — the two addresses of a clone: `fetch_from` is where this system's copy
lives and may be pushed to, `origin_of_the_code` is where the code was written and may
only be read. `git clone` sets `origin` and nothing else, so without the second a reader
running `git remote -v` months later cannot tell a fork from a mystery.

**A path and a ref are escaped, or they are not.** A path holding `?` or `#` changes which ref
is fetched, so the address would name a different thing than the one it was built from — and the
address is what `git` is handed. **The boundary in `a_path_in_the_project` is about project paths
and does not apply to repository paths**, so `://` is not the only thing to refuse: a rule drawn at
a scheme lets every one of these through, because none of them carries one.

**`AStringThatCannotGoInAnAddress`** — a string that would change what an address fetches. The
refusal names **which of the two things was being built**, because a path and a ref are refused
alike, and **quotes the character back**, so a caller can match against what it built.

**`AStringThatCanGoInAnAddress`** — a string that can go in, and says so. It is a type and not a
`String` **because a plain `String` in the signature is the thing that has to be checked**, and a
check that the caller may skip is a check that will be skipped. A `path` holding `?` or `#` changes
which ref is fetched. This is recorded here as an obligation, not as a gap that is
acceptable: the boundary in `a_path_in_the_project` is about *project* paths and does not
apply to *repository* paths, and `://` is not the only thing to refuse.

---

## Reading the project

A repository is cloned onto local disk before any agent reads it, and every agent reads
that clone. These are the words for the two rules that keep a string from becoming a path
it must not become. **Whether a string can be a path at all, and whether the path that
results lands inside the project, are two questions with two answers**, and only the second
was ever about confinement.

**`a_path_in_the_project`** — the shape rule, asked before anything touches the disk.
Refuses a string that is only whitespace, a character no filesystem holds, an empty
segment, and any segment over `THE_LONGEST_A_COMPONENT_MAY_BE`. The bound is per segment
rather than per path, because a repository has paths longer than 255 bytes and they are
ordinary. A model wrote a 433-byte sentence into a path field once, and the operating
system raised about that rather than answering, which is why this rule comes first.

**`the_path_inside_the_project`** — the confinement rule. Decides by the **resolved
location**, not by the spelling, because `..` is a way out however it is written and a
symlink is a way out however innocent it looks. Resolution returns a *different spelling*
from the one it was given, and a caller that compares strings without resolving will call
a correct read a wrong one.

**`AStringThatIsNotAPath`** — a string that cannot be one, named for the rule and quoting
the input back. A refusal the model that wrote the input cannot match against what it
produced spends a correction and teaches it nothing.

**`APathOutsideTheProject`** — the path asked for lands outside. **A refusal on
principle**, and it carries *why* in a separate field, because the two reasons want
different answers: an address (`://`) is a request to fetch something, which a filesystem
has no business doing, and a path that resolves elsewhere is an escape.

**`APathThatIsNotInTheProject`** — it is inside the project and simply not there. **A fact
about the clone, not a refusal**, which is why it is not the previous noun: the only
sensible response to it is to ask for a different path.

**`ProjectFile`** — a file and the text it holds. **`ProjectListing`** — a directory's
entries told apart as directories and files, because a listing of bare names forces every
caller to stat each entry to learn whether it may descend into it.

**`ReadingTheProjectFailed`** — the ways reading failed: it leaves the project, it is not
there, it is a file where a directory was asked for, or it is there and could not be read.
**Four, because the fourth was the third doing two jobs.** "There and could not be read"
answered *a directory was expected* and *a file was expected* with the same words.

**`a_project_directory`** — the directory a run's artifacts and clone live under, and the
boundary `TheProjectOnDisk` is confined to. Derived rather than chosen, so two runs of the
same work item find the same place and a resumed run does not re-clone underneath itself.

**`TheProjectAsItIsOnDisk`** — what a project really holds, read by walking the filesystem, and
the one implementation of `TheProjectOnDisk`. **A name and not `AProjectOnDisk`**, which would
say the port rather than the thing answering it, and a port and its implementation sharing a
name is how a reader comes to depend on the port's promises from the implementation's code.

**`AFileTheProjectDoesNotHave`** — the refusal when a path inside the project names nothing.
**One refusal and not an empty answer**, because "I found nothing" and "here is what I found"
are the same value to a caller that treats absence as a listing, and only one of them is an
error.

**`AnUnanswerableRequest`** — a work item whose request a model cannot be asked about, naming the
artifact and the reason. **The refusal is a value and not an empty answer**, and it is built
before the call: **a refusal that has already spent a call has paid for it.**

**`NotTheRunItsOwnCheckout`** — a checkout a run made, handed a project living somewhere else,
naming **both** places. **A refusal quoting one of two facts leaves the reader guessing which was
wrong**, and the run cannot tell either — it was handed both.

**A listing is a listing, and that is why this refusal exists.** Every name in a project listing
is a real name of some project, so a run that listed a project rooted elsewhere hands an agent the
wrong repository's `README` and the agent has no way to know. **`TheProjectOnDisk` therefore has
`where_it_lives`, and a path rather than nothing**, because a project that cannot say where it is
cannot be told it is the right one — and a door is also what forces every double to name a real
place instead of an empty one.

**`NotStandingAtTheGate`** — a work item put to somebody who is not there, naming the stage it is
standing at. **`DONE` is the case that matters**: a finished work item has nothing left to
approve, and the record would be evidence that a human ruled on work that was already over.


---

## Cloning a repository

**`ARepositoryToClone`** — the request: which repository, from where it is fetched, and
where it lands. The destination is a field and not a convention because it is **derived,
not chosen**.

**`ACloneOnDisk`** — a checkout on disk, and **the commit and the ref it is standing
on, both read from the clone and never remembered.** A value kept from an earlier run is a
value about the wrong commit.

**`ref_name`, and not `branch`.** A checkout is not always on a branch: cloning a detached
ref leaves none behind, and the honest answer for a checkout that is on none is `HEAD`.
Naming the field `branch` would be a lie in precisely the case where the value matters.

**`TheCommit`** and **`TheRef`** — what a clone is standing on, both read from the clone and
**never remembered**, because a value kept from an earlier run is a value about the wrong commit and
a run that resumed believing it knows which commit it has would report its changes against one it
stopped at days ago. **`HEAD` is a ref and not a branch**, because a checkout cloned detached leaves
no branch behind, and the honest answer for one on none is `HEAD`.

**`ACommitThatNamesNothing`** and **`ARefThatNamesNothing`** — a blank in either, refused and
quoting the input back, because a name this system silently tidies is a name a lookup will miss.
**The same sentence keeps `WorkItemId` from trimming**, and one sentence serving two types is a
sentence that was worth writing once.

**`the_run_makes_a_checkout`** and not `the_run_clones_a_repository`, **because a name ending on
`Repository` is a persistence abstraction wearing a domain word** — and this system already calls a
clone a checkout in `ACloneOnDisk`, so the name that was refused is not the one that describes
it. **Three questions in order, and each can stop the next**: whether anything stands at the
destination, whether git copied the repository, what the copy stands on.

**`WhetherAModelRulesHere`** — whether a model may rule at the gate on this deployment, as a
value and **not as a setting read inside a use case**. A use case that reads its own configuration
has moved a decision out of a type and into a line of code, and the line is somewhere a reader has
to find. **Two values and not a bool**, so a call site says which thing is being set.

**`where_a_deployment_put_its_model`** — where a deployment said its model is, **or the variable
it left unset.** The first real run fell back to `http://localhost:11434` because nobody had read
`LLM_ENDPOINT`, and the message that came back was *Failed to connect to localhost port 11434* — **a
fallback is a lie with a position**, the same rule that stopped the board from storing a column, and
this is the place that used to have one. A guess that is wrong costs the same time as a refusal and
says less.

**An endpoint a deployment names is the whole address, and not a base this system completes.** The
second real run posted to exactly what `LLM_ENDPOINT` said and a web page came back, because the
variable held a base. **A base plus a path this system invents is a fallback with a different
address** — the same grudge as the `localhost` before it, and one step further from the truth,
because the invented part looks like a standard rather than like a guess. `LLM_ENDPOINT` must end at
`/chat/completions`, and the test says so by refusing a base by name.

**`TheEnvironment` reads four names, and the two that were missing are how a model gets asked at
all** — `LLM_ENDPOINT` and `LLM_MODEL` joined the flag and the key. A reader closed to two names
described a deployment that had not finished being configured.

**`curl` is told `-s`, because a program that narrates is a program whose narration can be mistaken
for what it found.** The first real run came back with a transfer table where the model's words
should have been, and the refusal quoted a percentage.

**`the_ruling_a_model_gave`** — the ruling, out of the answer its endpoint wrapped it in. **The
envelope is the endpoint's and the ruling is not**: a model asked to say one word answers with JSON
around that word, and a reader that wants the whole answer to *be* the word decides no model can
ever rule — which is what happened here, and what the green suite said nothing about because the
double answered with a bare word that no endpoint returns.

**Every way of failing here is a refusal and never an approval**, because that is the direction the
failure has to point: a broken answer read as an approval puts a model at a human's gate on the
strength of a bug. The one word is still checked afterwards, so **an envelope can approve nothing
on its own** — and a model that added a sentence has not ruled.

**`TheEnvironment`** — what a process was told before it started, **as a value and never as a
reading.** Everything else in this system takes what a process was told as a value it was handed,
**because a function that reads its own configuration has moved a decision out of a type and into a
line of code** — and a reader that goes looking cannot be handed a test. `std::env` appears in
`reading_what_the_process_was_told` and nowhere else, so the one place a test can point at is the
one place it happens.

**A variable nobody set is `None`, and never a default.** A reader that invents a value for a key it
did not find describes a deployment that does not exist. **The two names are written down in
`how_this_deployment_is_set_up` and not taken as arguments**, because a caller that could name a
variable could read a different one — and what a deployment *is* would then be that caller's
opinion rather than a fact about the process. `AISDLC_A_MODEL_RULES_HERE` and `OPENAI_API_KEY` are
the only two this system reads.

**`the_flag_as_this_deployment_sets_it`** — the flag as this deployment set it, **given what the
environment held or nothing at all**, and the reader lives in the adapter because it is the only
crate allowed to reach one. **A flag that is absent is off and is not a failure**: a deployment
that has never heard of the flag must still run, and a run that refuses to start without a switch
is not a run that fails safely, it is a run that does not run.

**One word opens the gate and no other does.** `yes` opens it; `YES`, `y`, `true`, `1`, `on`,
`enabled` and the empty string all leave it shut, **because a word this system does not know is a
word it will not act on** — and guessing that `true` means yes would put a model at a gate on a
deployment whose operator never asked for one. The test proves this by making the reader
accept those spellings and watching it fail.

**`AMayNotRule`** — a gate nobody switched a model on for, naming the artifact **and the flag**.
**The flag and the artifact, and not a bare "no"**: a human turning the flag on needs to know it
was the thing that was missing.

**The flag is read before the model, and that order is the rule.** A gate that reaches the model
and then ignores the flag has already paid for a ruling nobody authorised. **The refusal is the
honest answer until a person turns it on**, rather than a decision this system made on a human's
behalf — and the whole of what the flag buys is a model that may be asked, nothing more.

**`HowThisDeploymentIsSetUp`** — the flag and the key of one deployment, read together. **Two
readers are two places to be consistent, and what they must agree about is one decision**: a model
asked for and not reachable is a run that cannot answer anybody. So a deployment that typed `yes`
and supplied nothing is **refused naming both halves at once**, rather than failing at the far end
in words this system did not write — **because a caller reading only *it is off* goes and sets the
flag again, and a caller reading only *there is no key* sets a key on a deployment that never
asked for a model.**

**It is a thing runs print, log and put in a commit message**, and it holds a credential behind a
value that prints as `a credential` and has no `Debug`. The test proves the summary carries no key
**and still says what this deployment can do**, because a summary that says nothing is the same as
no summary.

**`ASetupThatAsksForAModelItCannotReach`** — that refusal, naming the word the operator typed.

**`ACredential`** — a key a deployment supplied, **which prints as the words "a credential" and
never as itself, and has no `Debug` at all.** A derived `Debug` is one `{:?}` away from a log and a
derived `Display` is one `{}` away from an error message, and **adding either was tried twice in one
day and the suite said nothing either time** — because **no test can hold a fact about a type, only
code that would stop compiling can.** So the type carries no `Debug`, the one `Display` is
hand-written, and the reason is written where whoever adds the derive is already reading.

**The key lives on the program and not on the request**, because a request is the thing this system
renders, records, quotes into a commit message and shows to a reader — **so a secret on a request
is a secret in four places, and a secret on a program is a secret in one.** The third test proves
the consequence: a question a reader could not read would be a question nobody could check.

**`ACredentialThatIsNothing`** — a blank key, refused, **because a credential that is blank is a
request the far end refuses** and the far end's refusal is this system's problem to read.

**`WhatGitHubWasAskedFor`** — one thing asked of GitHub, carrying the address word for word, the
words unaltered, and **who wrote it**. **A comment is this system's handwriting and a reader cannot
tell it from a person's**, so the author travels with it, and the name is one this system was told
— because a comment claiming a person wrote it is the same forgery as a record naming a person for
a decision no person made. **That is the one thing a comment shares with a gate ruling**, and the
gate already refuses it by name.

**`the_run_leaves_a_comment`** — leaves a comment on the filing its work answers, **refusing a
request nobody filed before it writes anything.** The order is the destination guard's rule again:
**a refusal that arrives after the write is a comment nobody can take back.**

**`ANowhereToLeaveAComment`** — and **two reasons land here that are not the same thing**: a
request nobody filed, and a write that failed. The reason is a string either way, **because a
caller reading one message can act on it and a caller reading a category has to ask which.**

**`AWhereItWasFiled`** — where a request was filed, **as the address the filing itself returned**
and as nothing at all when nobody filed it. **The only door takes a whole address, and nothing
watches for a second door**: adding `an_address_composed_of(what, 41)` to that module was tried
and every test still passed. **A claim about a door that does not exist is the one claim a test
cannot make from the outside, because it has nothing to name** — and a reader deserves to be told
that the thing holding it is the code, not the suite.

**`AWhereItWasFiledThatNamesNothing`** — an address that is only whitespace, so a link to nowhere.
**Worse than no link at all**, because a reader follows it, finds nothing, and learns that this
system's references cannot be trusted.

**`asked_for_and_filed_at`** — the words **and** the filing, as a second door rather than a
settable field, so a caller cannot fill an address in afterwards from a number it happened to have.
**The words first, and the filing beside them rather than instead of them**: a type that carried a
number instead of a sentence made a product analyst read the repository while the sentence went
to the planner.

**`who_signed_these_comments`** — who signs the comments this deployment writes, or the variable
it left unset. **A port that answers is not a thing anybody calls**: `ACommentReachingGitHub` existed
and `WhoSignsAComment` existed, and until this reader existed **there was no way for a deployment to
put itself on a comment** — so the only way to reach the adapter was to invent a name, and inventing
a name is the forgery `WhoSignsAComment` was written to remove.

**An absent variable is a refusal and not a default, and the refusal comes before the write**,
because a comment with an invented author is worse than no comment: it looks like a record.

**The author is in the body and not in an argument, because `gh` has no author argument.** A real run
proved it: the program answered `unknown flag: --author-name`. `gh issue comment` posts as whoever
the token belongs to and offers nothing to change that, **so an author this system sets is an author
this system has invented** — and a test that asserted `--author-name` was in the arguments was
asserting a flag that does not exist. The name goes in the words, where a reader can see it, and the
record keeps it for the audit trail. The account is the author and this system does not get to
pretend otherwise.

**`a_filing_is_read_ruled_on_recorded_and_moved_all_in_one_run`** — the only test that walks
every part at once, and **it exists because every part was true on its own and nothing proved they
compose.** A read and a write that did not meet, a work item that stood at `IDLE`, a gate that took
only a person's decision, and a model that ruled with nowhere for the ruling to go — **four
half-systems, each proved against the real thing, and no proof that they run.**

**A run that dies at the eleventh step has still told the truth about where the eleventh step is**,
which is why the assertions sit at the end and each stage prints what it has before the next begins.
On issue `#8` the model refused, nothing was recorded, and the work went back to
`AWAITING_AGENT_Pickup` — **which is the correct outcome, because there is no `DiscoveryReport`
yet for it to approve.**

**`the_ruling_moves_the_work`** — the work moved where its ruling says it goes, and nowhere else.
**An approval is the only move that ends the work**, and `DONE` is the table's own destination for
it — **not one this file chose.** A refused work goes back to `AWAITING_AGENT_PICKUP`, because **a
work whose ruling was a refusal is not delivered**, and a refusal that ended the work would record a
delivery nobody made. That asymmetry was written down in the domain and nothing had yet performed it.

**A refused work goes back to the agent the specification names, and the work could not tell this
file which agent that was.** `AWAITING_HUMAN_APPROVAL` holds no agent — presenting drops it — **so a
refused work has nobody recorded to return to.** That is a real gap in the work item, and it is done
about here without changing the invariant: **the name comes from §10, so it is transcribed rather than
chosen here**, and the gap is written down rather than papered over with a second agent field.

**Both moves leave no agent holding it where the stage holds none**, and a move that carried one along
would be refused by the invariant — **which is the point of an invariant.**

**`the_gate_records_the_ruling_it_was_given`** — the record a ruling left behind, and `None` when
it refused. **A model's ruling is a decision and never a record** until this exists: the gate's own
recorder took what a person decided, **so a model could rule at a gate and the ruling went nowhere**,
and a run carrying a decision nobody can point at is a run whose next step has no evidence behind it.

**The record names whoever answered, and that is the only difference between a person and a model.**
A record that said "approved" and left the name off would be evidence with no author, **and a record
that put a person's name on a model's ruling would be a forged audit trail** — the same forgery the
comment path closed a cycle ago, on the other side of the gate.

**`None` is the whole shape of a refusal**, because a refusal is a decision and not an approval, and
a record of a refusal would be a record of something that was approved — **which is the one thing a
record must not be.**

**`an_agent_presents_what_it_produced_and_the_question_is_put`** — what an agent produced, put in
front of the gate, and the question that asks about it. **Presenting and asking are one act**, and
they used to be two functions that took the artifact twice: once to throw it away and once to ask
about it — **so the thing approved and the thing ruled on were two arguments somebody had to keep
equal by hand, and nothing checked that they ever were.** A gate that can be asked about something
other than what was put in front of it is a gate whose approval is about an artifact nobody saw.
**So there is nothing to keep equal: there is one value.**

**The work must already be `IN_PROGRESS_BY_AGENT`, and the table is what says so.** There is no
edge out of `AWAITING_AGENT_PICKUP` straight to the gate, and a work standing at pickup has not been
worked on at all. **A third RED in this repository asserted an edge the table does not have, and the
table was right** — which is the second time a test of mine was corrected by a machine rather than by
a reader.

**`WhatDiscoveryConcluded`** — what a discovery agent concluded, in **the eight fields the
specification lists**, transcribed from the glossary of the mother project: languages, frameworks,
build system, CI system, architecture summary, key components, evidence files, suggested changes.
**A name a system chose for itself would be a name nothing could be held to**, which is the only
reason this can be written at all.

**A report that says nothing at all is a fault and not an artifact**, named
**`ADiscoveryReportThatSaysNothing`**: **silence that reached a human at a gate would carry the
authority of a finished analysis, and a person cannot tell an empty report from a real one by looking
at it.** **At least one field and not all of them**, because a repository with no CI has nothing to
say about CI and must be able to say so — **absence is representable**, and a rule demanding every
field would only ever be satisfied by a report that guesses.

**`EvidenceFile`** — one path and the address a reviewer opens, **and the address is derived and
never asked for**: the model contributes the path and the address follows from the repository the run
is about. **An address a model may invent is worse than none, and one it may leave out is an address
nobody can rely on clicking.** **`AnEvidenceOutsideTheRepository`** is the refusal, and every path is
checked before any address is built, **so a set containing one bad path produces no evidence at all
rather than a report that is partly clickable.**

**The first version built the address out of a repository written into the file**, and `grep`
found it there — **the same grudge as `A-1`, committed two cycles earlier and in a different shape.**
The repository is now an argument.

**`WhatAnAgentHasProduced`** — the name and the version of one thing an agent produced, both
checked. **`WhichArtifact` is an open `String` and it accepts anything**, so this is the door that is
closed: **a gate that approves an artifact nobody here has ever heard of is a gate that approved a
stranger's paperwork, and the record names it as approved, which is the one thing a record must
never do.**

**The seven names are the specification's, transcribed from the mother project's own router** — and
not composed here, for the same reason the agent's name was. `DiscoveryReport`, `Requirements`,
`QaPlan`, `ArchitectureReport`, `SecurityReport`, `ImplementationPlan`, `VerificationReport`. **A
test refuses every one of the seven being refused**, because a run naming one and being turned away
would mean this repository is narrower than the pipeline it is supposed to run.

**`AnArtifactThisSystemDoesNotKnow`** — that refusal, and **it says the name is a gap in this
pipeline and not a fault of the run**, because an artifact with no rule written for it is a gap in
the pipeline — **and blaming a run for a name this repository has not declared sends somebody to fix
the run.** **`WhatWasNotAThingThisSystemHolds`** is the closed refusal, with
**`AnArtifactVersionBelowOne`** for the version.

**Version zero is never a version**, because it is the state before there was one, **and a gate
presented with version zero is being shown nothing and asked to rule on it.**

**`WhatThisFilingIsAbout`** — which work item this run is and which discipline answers for it,
**both given and neither guessed.** `A-1` and `PO` sat inside the function that builds the work, so
**every run in history was a work item called `A-1` accountable to the product analyst, whatever was
filed** — and a record that says the same thing every time is not a record of this run.

**The role is the discipline that answers and not the one that notices.** An issue nobody had looked
at yet has a discipline waiting, and only the work has one answering — which is why it is something a
run is told and not something a file can guess.

**`TheWorkItemsId`** — the name a work item will carry, **as the run wrote it and not as a type
re-spells it**, and refused when it is blank: **a blank id is a work nothing can be found by**, and a
run that went ahead would leave a record nothing points at. **`TheRoleThatAnswers`** is the same
shape for a discipline, and **`AWorkItemsIdThatIsNotOne`** and **`ADisciplineThatIsNotOne`** are the
refusals, named after what is wrong rather than after what happened.

**`a_filing_is_answered`** — a filing that was read, written back on as a comment. **Two working
ends and no use joining them are two proofs and not a system**: the read exists and the comment
exists, and a run that never joins them has demonstrated a repository and a filing rather than a
thing that works on a filing.

**The comment goes to the address that was read and never to one composed from the number**, and the
filing in the test is **transferred** — it answers at `…/issues/301` and was asked for as `8`. **An
address composed from a number is a comment on a page that has nothing to do with the one the run
read.** This is the same trap the domain closed once and `no_body_is_built_by_a_hand`'s sibling rule
now holds, arriving at the last possible place to be caught.

**`TheWork`** — the work a read filing becomes. **The words a person filed are the words the work
carries**, because a work item that paraphrases a filing is a record of a different request, and the
person who reads it later is the only one who can tell the difference.

**`AFilingIsRead`** — a filing, read out of GitHub by `gh`. **This is the half that was missing
while the writing half was finished**: a comment could go back to a filing that nothing could bring
in, and a system with one end closed is a system that cannot be run, only demonstrated.

**Both streams, and not one**, because a refusal from the far end arrives on either and a reader
that is told only about the stream it did not expect is told nothing. **One argument each and no
shell**, like every other program here.

**`AnIssueRead`** — what a person filed, word for word, and where it answers. **`address` is the
one GitHub returned and not one derived from the repository and the number**: an issue that was
transferred answers at an address that does not carry the number asked for, and a composed address
cannot express that — **it can only be right or wrong, and it was silently wrong on the only
repository this system had been pointed at.**

**An answer carrying no address is refused rather than composed**, because a link to nothing in an
audit trail is worse than no link at all.

**Whether it is still open is carried, because a closed request is a fact a person reads before
ruling** — a stage that has to ask GitHub a second time to learn it is a stage that can rule on it
wrongly. **It is a property of the filing and not a `Stage`**: `Stage` is where a work item stands in
this system, and a filing does not stand anywhere.

**The body may be empty and that is a real request.** Somebody filed it with a title and nothing
else, and refusing that would refuse the issue a person wrote in the fewest seconds.

**A pull request at the address of a filing is refused rather than read as a request.** GitHub
answers issues and pull requests from one path, so a number handed for an issue may name a pull
request — **and in the repository this system is pointed at that is the likely reading and not a
corner**, because six of the seven entries GitHub answers with there are pull requests. Its body
describes a change rather than asking for one, and **reading it as a request hands the pipeline a
description to work from, and every artifact after it is about work nobody asked for.**

**A move the destination refuses on the agent says which way it was wrong, because a refusal that
says the opposite of what happened sends a reader to fix the thing that was already right.** A stage
that holds an agent and was given none has been given no agent where one is required; a stage that
holds no agent and was given one has been handed an agent it cannot have. The old sentence said
`AWAITING_AGENT_PICKUP does not take the agent it was given` — **and the stage requires one, so the
one case a reader meets first was the sentence that told them the opposite of what happened.**

**`the_agent_that_picks_up_work`** — `aisdlc-discover`, **and the name is the specification's and
not this file's.** §10 lists the first agent as the Repository Discovery agent and the mother
project writes that name out. **Transcribing it is not inventing it**, which is the whole reason this
could be written at all: **a name a system chose for itself would be a name nothing could be held
to**, and the attempt to write `aisdlc-something` is what the refused move above was about.

**`a_run_picks_up_the_work_it_was_given`** — a run taking up the work it was given, along the one
edge the table allows. **The work stands at `IDLE` and a work item there has not begun**, however
true everything in it is, and `IDLE -> AWAITING_AGENT_PICKUP` is what makes the record a piece of
work rather than a note about one. **The refusal is the domain's and not this file's**, because a
move composed here would be a second transition table — and a second table is one nobody reads when
the first one is the specification.

**`WhyAnAnswerIsNotAFiling`** — why an answer is not a filing, **named rather than a paragraph.**
**"There is no filing" and "this system cannot read this" are different facts**, and one is worth
retrying while the other is worth reading the number again. **A far end having a bad minute is not a
filing not existing** — and telling a person their filing is gone when the far end answered nonsense
is worse than saying nothing.

**`ANoSuchFiling`** — there is nothing at the number that was asked for. GitHub answers a missing
filing with `{"message":"Not Found","status":"404"}` on one stream and `gh: Not Found (HTTP 404)` on
the other, and **the two streams are not concatenated**, because joining them produces something
that is not an answer at all — so a reader that only knew how to parse would report that it cannot
read a filing that is merely not there, **and a person told the wrong one goes to look at this
system instead of at their own number.**

**`AnAnswerThatIsNotAFiledRequest`** — that refusal, quoting the answer back **and not summarising
it**, because a caller deciding whether to retry needs to know what GitHub actually said.

**`AnIssueToRead`** — which issue of which repository, **as a person named it.** A number and not
an address, because a number is what a person types; the address is GitHub's to answer with, not
ours to compose.

**Refused before it is sent, not answered as a 404**, because a mistyped number and a missing issue
answer identically from the far end — **and a run that cannot tell them apart reports "no such
issue" for somebody who typed `7 ` with a space in it**, having gone on to write eight artifacts
about work nobody commissioned.

**`ANumberThatIsNotAnIssueNumber`** — that refusal, its own type and not a string, **because a
caller that has to tell a typo from a missing filing has nothing to act on when both arrive as
text.** The number is in the message and not only in the field: a person reading a refusal is told
what they typed, while a caller reading a field has to know which field to look at.

**`the_body_a_model_is_given`** — the body a model is given, **built by the JSON library and not
by a hand.** This body was once written by hand, with the question put where a quoted string belongs
and no quotes around it — **so every request this system ever sent a model was invalid JSON**, the
far end answered `400` with **zero bytes**, and the refusal that reached us quoted an empty string
while the same curl with a hand-written body answered `APPROVED` in half a second.

**Three real runs, the same refusal, and the cause was eleven characters of format string that no
test ever looked at.** A hand-rolled escape is a lossy channel in both directions, and the question
is a work item's name and an artifact's name, which are strings people type — **so a quote in one of
them must not be able to break the request it is in.**

**`with_what_a_deployment_named`** — what a deployment said a model must be asked besides the
body, one header each. **A deployment fact and not a gateway's name written into this system**: a
proxy in front of a model can require anything, so the deployment says what it requires — **and no
name of any particular host appears in `aisdlc-run`.**

**A header is a name and a value and not two arguments**, because a name split from its value is a
header sent to a host it was not addressed to. **A line that is not a header is not sent**, and a
deployment that named none sends none — sending an empty one is a header with an empty name, which is
a thing no host can be addressed by.

**A header is not a credential and cannot stand in for one**: the program with no credential is asked
for nothing at all even when the deployment named three headers.

**A comment is left on a filing somebody named, and never on one this system chose.** This repository
has no issues at all, so the address comes from `E2E_FILING` and the test refuses to run without it —
**a proof that comments on a repository this system has no business commenting on is not a proof, it
is a side effect.**

**`ACommentReachingGitHub`** — a comment turned into what `gh` is run with, and the answer
`AGitHubWeWriteTo` was missing. **A port with one implementor that was a test double is a port
nothing could reach**, so a comment had never been written anywhere and the use case was proved
against a place that accepts anything.

**No shell, one argument each, and the address is one argument.** A comment body is prose, and prose
has dashes and colons and backticks in it — **the first comment this system writes should be the
shape that makes re-splitting impossible rather than the shape that gets away with it.** A failure
comes back as a refusal and not as a quiet success, because a caller told a comment was written when
it was not is a reader of a record that does not exist.

**A third assertion could not be written, and its not existing is the property.** A comment with
nowhere to go cannot be built at all: the port's request holds an address and not an absent one,
because the use case refuses a request nobody filed *before* a request exists. **A type that could
hold a comment with nowhere to put it would be a type asking to be used without the guard** — so the
guard is not a check somebody can skip, it is the absence of a value.

**`WhoSignsAComment`** — the name a deployment writes its own comments under, **and never a person
and never chosen by a caller.** A comment is not a decision, so a person did not decide it and did
not write it. **A type and not a `WhoDecided` reused**, because a type both a person's ruling and an
automated comment can be built from is a type that cannot tell a person from this system — and that
is the whole question.

This system held the way: the use case took the name as an argument, `WhoDecided` accepts any string
that is not empty, and a test asserted that **"Alice"** reached GitHub. **A forgery primitive is
removed by having no door rather than by a check on the way through it** — so the use case takes no
name, and a test proves it by reading the signature.

**A deployment that named nobody is refused before anything is written**, because a comment with
nobody on it is a comment nobody can be asked about, and an invented name is the forgery this type
exists to remove.

**`ADeploymentThatNamedNobody`** — a deployment that named nothing, or named only spaces. **Named
after what is wrong and not after what happened**, because a refusal whose name says "error" tells a
reader nothing they did not already know.

**`WhereAModelIs`** — where a model answers and which one, **both deployment facts and neither a
secret**. **And that is the point of the type existing**: the address and the model name are the
two facts a deployment configures and can say out loud, **and the credential is deliberately not
one of them.**

**`AskAModelHowItRules`** — a request carrying the endpoint, the model and the body, and **no
credential at all.** `curl` builds no arguments for it today, and the program run says so in words
rather than fetching the question as an address. **A translation guessed at is a translation that
looks right until somebody runs it**, and the fact this one is missing is the fact a reader
cannot see: **where a secret lives in a process is a decision a person has to make, and this
system has not made it.**

**`AnAnswerThatIsNotARuling`** — a model's answer that was not `APPROVED` or `REFUSED` and
nothing else, quoting what came back. **A contract a model can miss is not a contract**, so an
answer in another shape is a refusal rather than a guess at what was meant.

**`the_question_a_model_is_asked`** — the question, **built here and not by the model**, so a model
cannot ask itself a different one and answer it. It says `Do not name who decided; that is not
yours to say` — and the reason is not politeness. **`decided_by` comes from this system's own
configuration and never from the answer**, however confidently the answer offers a name.

**A forged audit trail looks exactly like a real one at the moment somebody needs to tell them
apart**, which is the only moment the difference matters. So an answer carrying a name is **refused
rather than obeyed**: reading the first word would take the yes and drop the forgery in the same
step. A model that rules is recorded as a model, whether the answer was yes or no.

**`WhetherAnythingStandsOnDisk`** — the real disk answering whether anything stands somewhere,
and the one implementation of `WhetherAnythingStandsThere`. **The directory is read and not
checked for existing, because existing is a different question**: a place can be there and hold
nothing at all, and a run that made its own destination is in exactly that state. **A run refused
by `exists()` is a run that can never start**, which is the one failure a check written for
convenience produces.

**`WhetherAnythingStandsThere`** — whether anything at all stands at a directory, asked
**before** anything is written there. **A question and not a thing returned**, because the answer
is a yes or a no. **A separate port rather than a check inside `AProgramWeRun`**, so the decision
to clone at all is made by the use case and not by the adapter that copies files: an adapter that
could refuse on its own is an adapter holding a decision it has no business holding.

**`origin_of_the_code`** — the address a clone was written from, which may only be read, and
**`upstream`** as the remote carrying it. **`git clone` sets `origin` and nothing else**, so
without the second a reader running `git remote -v` months later cannot tell a fork from a
mystery.

**The two addresses are one address today, and the code says so rather than hiding it.** The
deployment's own copy and the public original are the same place until the deployment forks
something, and a reader being told that by accident is exactly the mystery this is here to
remove. **The second is asked after the copy exists** — a run asked to name it before the clone
has nothing to name.

**`HowCloningWent`** — the clone and the reason, one of which is always nothing. **`DONE` and not
`Result` at the boundary**, because a caller asks *which way it failed* as often as it asks *the
clone*, and a `Result` read the second way would throw the reason away.

**`WhichWayItFailed`** and **`WhyCloningTheRepositoryFailed`** — the four ways, and the
refusal in one of them carrying whatever git said. **Four and not three**, and the fourth was
named here only inside the second one's description, as the thing a caller has to tell apart
from *no such repository* and from *git is not installed*: **`GitIsUnhappy`**, which is git
refusing for a reason this system does not name. **A closed set of four and not a free
string**, because a refusal whose name is a `String` is a refusal the caller cannot match
against anything.

**`WhyCloningTheRepositoryFailed`** — four ways, and the last two are this system's own
findings. *The destination already holds something* is a refusal on principle: a run
resumes, the directory is derived, and **a refusal that arrives after the damage is not a
refusal**. *The repository is not there* quotes git's own words inside ours, because the
caller has to tell *no such repository* from *git is unhappy* from *git is not installed*.
*The repository has nothing in it yet*: somebody created a repository and has not committed
to it, and **a value read out of a command that failed is that command's complaint wearing
a field's name.**

**A full clone, and not a shallow one.** The run does things with the history — a diff
against a base, `git log` for how a file got the way it is — and a shallow clone answers
none of them. The cheapest clone is a false economy paid at the first review.

**`upstream`, set after the clone.** `git clone` sets `origin` and nothing else, so a clone
of this system's own copy would carry no memory of what it is a copy of.

---

## Reading one file out of a repository

**`AFileToRead`** — which file, of which repository, at which ref. **`ARepositoryFile`** — a
file and what was in it, and **the path kept exactly as it was asked for.**

**`ARequestToGitHub`** — where it goes, what it says about who is asking, and how long it
may take. **The bound is a field and not a convention**, so a request that carries no
bound is a value that cannot be built.

**No `Authorization` header when the deployment stated no credential**, which is the
ordinary case. An `Authorization: Bearer ` with an empty value is a claim about the caller
that is false, and a host may answer it differently from a request that makes no claim at
all. **A credential is never in the address**, because the address is what a run records
and quotes.

**`ThereIsNoSuchFile` and `WhyThereIsNoFile` — and not PathNotInRepository, which is the
name the earlier implementation used.** That name covers one of the two reasons, and a
name covering half a rule is trusted for the other half. A `403` answers with a body that
parses and says nothing about a file, so the caller's next question — *was my path wrong,
or was my request refused* — has a different answer for each.

**The refusal may say the repository might not be public, because that is what an operator
has to check. It must not say it knows**, and it must not talk about permissions: a
private repository is answered exactly as an absent path is.

**Every status this tool cannot interpret is a refusal, and only `200` is a file.** A `500`
whose body happens to carry a `content` field must not be handed back as the file a run
asked for: a run that quotes an error page as a file is a run whose report is a lie with a
path on it.

**The three facts a caller is asked — path, repository, ref — are fields and not a
sentence**, so the message cannot forget one.

---

## Putting a branch on a remote

**`ABranchToPush`** — the branch a run wants and the commit it points at. **The commit is
taken from the clone and not asked for**: a caller that could name the commit could push a
commit it has not read, and every later claim about what was delivered would be about
something else.

**`APushedBranch`** — a branch that is on the remote, with the address it went to, so a
caller can go and look at it rather than take the tool's word for it.

**`TheBranchIsAlreadyOnTheRemote`** — and **not** NothingToPushError, which is what the
earlier implementation calls it. "Nothing to push" describes only the case where the
branch already holds this run's own commit. It says nothing about **a branch somebody else
is using**, which is the case with teeth.

**One rule produces both cases, so one refusal covers both.** This run does not move a
branch it did not create.

**`TheRemoteRefusedThePush`** — separate from the above, because the two need different
answers from a person: a branch that already existed is this run's own doing, and a remote
that is not there is not.

**`ls-remote`, asked before anything is pushed.** It reads the remote without changing it,
which is what makes the refusal possible *before* rather than after — **and a push that has
already landed is not something to undo.** A branch name is quoted into a command whose
shape is fixed: nothing a caller supplies reaches the program as an option.

**`HEAD:refs/heads/<name>` rather than a local branch name.** Pushing the branch the
checkout is standing on would move *that* branch on the remote.

**A delivery is a git operation, so it goes through `AProgramWeRun` and not the GitHub
API.** The clone is here, the commit is in it, and pushing it is what `git` is for.

---

## Approval

**`Approval`** — a record that somebody approved an exact artifact at an exact version of
one work item. It stops covering that the moment any of the three changes.

**The record carries no instant, and that is the domain's edge rather than an omission.**
This definition names three things — an artifact, a version, a work item — and a clock is
not one of them. The domain has no dependencies and therefore no way to know what time it
is, and the only honest way to hold one would be to be handed it, which would make the
domain a thing that believes what it is told about the world. **The instant is recorded
beside the record, by the boundary that has a clock**, and the two are written together so
a reader can find one from the other.

An earlier implementation of this repository carried `decided_at` on the decision itself.
That was possible because that domain had a `chrono` dependency, and it is the same
dependency the rewrite exists to remove — so the field is not lost, it is moved to the only
layer entitled to it.

**`WhoDecided`** — the party that ruled, recorded verbatim and never replaced. A person's
name and a policy's name are both valid and the record keeps the difference.

**`ApprovalQuestion`** — the exact artifact and its version, presented for a ruling.
Presenting the version is not politeness: an approval that names no version still covers
the artifact after it has been rewritten, which makes the record a claim rather than a
decision.

**`ApprovalDecision`** — the ruling itself, and it carries the `WhoDecided` verbatim.

**`AgentDecidedError`** — an agent tried to rule. The prohibition is not a convention here;
it is an assertion, and this error exists to be raised. An agent may ask for work and may
not rule on the result, and those are two halves enforced in two places.

**The gate refuses a work item that is not standing at it**, in any stage but
`AWAITING_HUMAN_APPROVAL`. `DONE` is the case that matters: a finished work item has
nothing left to approve, and the record would be evidence that a human ruled on work that
was already over.

---

## Asking a model for an artifact

**`AnAnswerIsNotYetAnArtifact`** — what a model said when it was asked for an artifact,
before anything has checked it. **The name is the point.** Nothing guarantees a model
honoured the request, so the answer is data until something validates it, and a type called
`QaPlan` on the way out of a model call would be a reader treating a dictionary as a plan.

**The tool is forced, not offered.** A gateway that does not enforce the JSON-schema method
a structured-output call defaults to answers a structured request three ways across three
runs — prose, then YAML, then JSON carrying one of four fields — and each becomes a
validation failure. **The model was never the difficulty.** So the request forces a tool
call, because only a tool call can be forced: `json_mode` asks for JSON inside a message,
and **a request this system cannot compel is a request whose failure it pays for.**

**The artifact's own name is the tool name**, so the forcing names the artifact rather
than a label invented beside it.

**The call collects rather than streams.** A gateway that parses a tool call's arguments
incrementally cannot complete a forced call until the fragments are all there, so
streaming and forcing are the same problem seen from two sides. Worse, a streaming failure
happens *inside* that parse, so the answer never reaches the correction loop — **and the
correction loop is the only route to a plan on a model that does not do structured output.**

**`TheModelRefused`** — one refusal naming the artifact and the field. A run asks eight
agents for eight different artifacts, so *"the shape was wrong"* without saying which is a
question rather than a diagnosis.

**Every type a model is asked for refuses unknown fields.** Otherwise a model invents a
field and the system silently ignores it, and an artifact that says something the
specification does not have is an artifact nobody checked.

---

## What a run says while it is doing it

**`WhatAModelSaid`** — what a run asked a model, what it answered and how long it took,
said as it happens. **Not to be confused with `ARecordOfARun`**, which is the durable
record of a whole run written afterwards; this is one call, alive while it is happening.

**The bare word is not a name.** The banned list covers `log`, `trace`, `record`,
`telemetry`, `metrics`, `instrumentation` and `span` for this entry — while calling the
entry *a record of what a model said*. The ban is scoped to the bare word, not to the word
inside a longer name: each of those seven says nothing, and `WhatAModelSaid` says what
happened.

**A record installed in one place, and not a change to the agents.** Eight agents each
wrapping their own model call is eight chances to forget, and a run that reports no
duration for the one call that mattered.

**A refusal names the artifact and the field, and the model name is an argument.** The
model is the same on every call, so the model cannot stand in for which artifact was
refused. And a line naming the wrong model is worse than no line, because the question the
line exists to answer is *which model was that*.

**The fragments are kept though they are not printed.** On a failure they are the whole
diagnosis: *a model asked for JSON and answered in prose* is the difference between *the
gateway is broken* and *this model does not do structured output*, and nothing else in the
system can tell those apart.

**The size of a question, never its text.** A 400-character prompt and a 40 000-character
one are two different lengths of wait, and the number tells them apart without putting a
repository's contents into somebody's terminal.

**Every line carries the elapsed time, because a label alone says nothing while the thing
it labels is still happening.** `planning` says a call is outstanding; `planned in 41s`
says the model answered.

**A whole run writes its lines to a file; anything shorter prints.** Appended and flushed,
because a run that overwrites answers "where is it now" and loses the rest. A file is not
captured by anything and outlives the process.

---

## The three layers, and the one direction

| Layer | Crate | May name | Rule |
|---|---|---|---|
| the domain | `aisdlc-domain` | nothing outside itself | **no dependencies, no `async`, no `unsafe`, no I/O** |
| the run | `aisdlc-run` | the domain, and its own ports | no organisation, no filesystem, no runtime |
| the outside | `aisdlc-outside` | everything | implements the ports and nothing else |
| the desk | `aisdlc-desk` | everything | the only place a concrete type is named |
| **the gates** | `aisdlc-gates` | **nothing, at all** | **reads the other crates and never calls them** |

**`aisdlc-gates` is a package of the workspace and is not a part of the system.** Its
library holds the code the gates share, and the rest is `tests/`. It is the only code here
permitted to
**read** the other crates rather than call them, which is why it cannot live inside a
layer: a gate sharing a dependency direction with the thing it checks is not a boundary.
It is the one place a test helper may use a dev-dependency, because test code is allowed a
crate production code is not, and a helper production could also reach has stopped being
one.

**The shared code is public, and that is not a style choice.** A private helper in a test
binary is reported dead the moment the test that does not use it is compiled, and silencing
that is `#[allow(dead_code)]`. Five gates that each carried their own copy would need it
five times, and a ban on `#[allow]` with five exemptions in it is not a ban.

**A virtual workspace has no integration-test target.** Without this package `tests/` is a
directory cargo never looks at, and `cargo test` is green while running nothing — which is
the failure mode every gate in this repository exists to make impossible.

**`aisdlc-domain` has no `[dependencies]` section at all.** Not "few", not "only serde".
Nothing. This is the one rule that the compiler enforces without being asked, and it is the
rule every other boundary rests on.

**There is no `dyn` in `aisdlc-domain` or `aisdlc-run`.** Ports are generic, and the
composition root monomorphises. The cost is a longer type at each boundary and a louder
compile error when a test double drifts from the real thing; the gain is that no trait has
to be made object-safe, no `async` macro has to rewrite a signature, and a reader can
follow what a port returns without knowing a trait exists.

**No `thiserror` in the domain**, and the rule above is why: a derive that generates a
`Display` is a dependency, and the domain has none. `Display` is written by hand. It is
eight lines each and a reader can see the whole error message.

**A file belongs to exactly one layer, and an LLM is a Layer B fact.** Reading the
repository, writing files, running tests, keeping state and routing are all deterministic
and belong outside the reasoning. Nothing in `aisdlc-domain` may reach for a model.

---

## How a file is read by the gates

**A gate cannot use a regex crate.** The gates may have dev-dependencies and this is the
only thing that may, but a rule enforced by a pattern is a rule nobody can read when it
stops matching. The source is read the way a person reads it, one character at a time, and
that walk is this repository's own code.

**`ASuppression`** — an `#[allow(…)]` or an `#[expect(…)]` found in a source file, with
the line it stands on and the attribute exactly as written. **The bare word `suppression`
is a name; the attribute is the thing.** It is refused everywhere, and the reason is that
`cargo clippy -- -D warnings` already denies every warning — so an `#[allow]` in this
repository can only be silencing something the gate command would already have refused. It
is either a bug in the code or a loophole in the rule, and either way it is a place where
the two have stopped agreeing without anybody deciding anything.

**A `Cargo.toml` `[lints]` table is not one.** It configures clippy for a whole crate
rather than standing in a source file, and the root one carries five `allow`s on purpose —
`module_name_repetitions` among them — because `WorkItem` has a `work_item_id` and the
domain is right about that and clippy is not. Reading it as a suppression would be the
same error as reading a `.gitignore` line as a comment.

**`APlaceholder`** — an unfinished thing left in the code: `todo!`, `unimplemented!`,
`unreachable!`, a panic message saying the work was not done, and the words a person types
when leaving something unfinished. **Refused, with no exceptions and no allowlist** — an
allowlist is a list of the holes this repository has decided to live with, and this one has
decided not to. A placeholder is an unfinished cycle, and an unfinished cycle is already
visible in the history, so a placeholder gives it a second home in a line of code.

**The words, as a block and not as a paragraph, because the guard reads the block and this
paragraph says why it is short.** `later`, `for now` and `temporary` are ordinary English and
this repository's own prose uses two of them; a gate that refused them inside a panic message
would be refusing the language rather than the unfinished work.

```text
TODO   FIXME   XXX   HACK   not done
```

**A refused word in a string is accepted and in a panic message is refused, and that is the
whole difficulty.** A `panic!` is what a run does when it cannot go on, so a message saying
the work was not done is that run announcing an unfinished cycle from inside code that looks
finished — **the one place a placeholder can speak without passing either the comment gate
or the history.** A string that is not a message is a value, and a value holding the word is
an example, a fixture, or text shown to a person.

**`ARuntimeReadOfTheEnvironment`** — a test reaching into the process's environment at **run
time**: `var`, `var_os`, `vars`, `set_var`, `remove_var`, `temp_dir`. Refused, because the
suite must be green with no network, no API key, no Docker and no model, and a test that
needs a token means a boundary is wrong.

**The words, as a block and not as a paragraph, because the guard reads the block and this
paragraph says why it is a list of six.** Each one is a way a test can come to depend on
something the machine happens to have, and the list is drawn at the reach rather than at the
danger — so it grows only when somebody adds a way of reaching, which is the one thing they
have to know in order to add one.

```text
var   var_os   vars   set_var   remove_var   temp_dir
```

**`env!` and `option_env!` are not one of these, and the exemption is load-bearing rather
than theoretical.** They are macros the compiler expands at build time. `CARGO_MANIFEST_DIR`
is set by cargo, carries no credential, and **seven gates in this repository reach the
checkout through it** — so a rule drawn at the word `env` would refuse six of them through
one line in the shared scanner. `std::env::current_dir` *is* a run-time read and is **not**
refused: it carries no credential, and a list of every read in `std::env` is a list to keep
in step with a standard library.

**`temp_dir` is refused although it is about no credential at all.** The rule is drawn at the
**reach** rather than at the danger, because a rule drawn at the danger is a list of dangerous
things, and that list grows every time somebody remembers a new one.

**`ADeclaredSecret`** — a credential the gate workflow hands to a step, whether a stored one
referred to by a dot or a value pasted in as a literal. The rule is `secrets.` **with the
dot**, and that is not a detail: this repository's own workflow contains a sentence explaining
that the job has no stored credentials, and a rule drawn at the bare word would refuse the
sentence recording the rule.

**`HowTheFileIsWritten`** — a source file, taken apart into the stretches it is made of,
each one knowing what it is and where it starts. It is a value rather than a service,
because six gates ask the same two questions of it and the answer must not be six
answers.

**`WhatIsWrittenThere`** — the five kinds a stretch can be: `Code`, `AComment`, `AString`,
`ACharacter`, `TheSpaceBetween`. **A closed set**, because the gates' whole reliability
rests on the claim that a span of characters in a `.rs` file is one of five things and
nothing else. A sixth kind would be a change to what a gate is allowed to assume, so it is
a change here and nowhere else.

**`WhereTheReadingHasGotTo`** — how far along a source file the reading has come: the byte
it is standing on and the line it is standing in. **It is a type and not four arguments to
a function**, and the reason is the count: a walk that has to pass a position and a line and
a collection to every step is a walk whose steps cannot be read one at a time, and 1.2 says a
function that needs a comment explaining its second half is two functions.

**`AStretchOfTheFile`** — a run of characters that are all the same kind of thing: code, a
comment, a string, a character, or the space between. It carries the line it starts on,
because a gate that says "there is a comment somewhere" is a gate nobody can act on.

**The four kinds that are not code, and why each one has to be told apart.**

| What it is | Told from what | What it would be mistaken for otherwise |
|---|---|---|
| a `//` comment | two slashes outside everything else | a scheme in a URL inside a string |
| a `///` or `//!` comment | a third slash, or two then one | the ordinary comment, and then a rule is broken by its own documentation |
| a string | `"`, `r"`, `b"`, `br"` and every raw-string hash count | code, and the slashes in a URL become a comment |
| a character | `'x'`, and **not** `'a` | a lifetime, and half a file turns into a runaway character literal |

**A lifetime is not a character literal, and this is the one that bites.** `let binding =
'a;` is a binding; `'a'` is a character. A scanner that treats the quote as opening a
character swallows the rest of the file, and the guard that reports "a comment" on a file
with none is a guard nobody trusts afterwards. **A `'` that is followed by an identifier
and then a `'` is a character; a `'` followed by anything else, or by an identifier that
does not close, is a lifetime** — and a raw identifier's `r#'name#` is a third case that
exists.

**Block comments nest in Rust**, unlike in C. `/* /* */ */` is one comment, and a scanner
that stops at the first `*/` treats the rest as code — which would let a whole file hide
behind a comment the gate believed it had read.

**`AFileTheLengthGateDoesNotRead`** — a file the rule of three hundred lines of code does
not apply to, named in the table below and read from here rather than named in the gate. **A
gate that carries its own exceptions is a gate whose exceptions nobody reads**, so the
exemption is in the glossary with every other exemption and the gate asks.

**Two files, and both of them are the gates.** A gate file used to be one rule, and the rule
was the name of the file — which cost a cycle of work per rule and put two hundred and fifty
lines of explanation in front of a test that measures one thing. **The rules are now in two
files named for what they are about, the way the sibling repository names them**, and a file
named for a concern holds every rule of that concern however long it grows. **The twenty-line
body and the two-level nesting do still hold in both**, because a concern that cannot be read
is not a concern.

| File | Not | Why |
|---|---|---|
| `readability_guards.rs` | over three hundred lines of code | it holds every rule about how this repository's code is written, and splitting it to reach a number would put one rule in one file again — which is what the number was meant to stop |
| `documentation_guards.rs` | over three hundred lines of code | the same, for the rules about what the documents claim rather than what the code does |

---

## Words that are a whole name and not a word inside one

**Six of the words in the `Not` columns are ordinary English, and a gate that refused them
inside a name would refuse this repository for writing prose.**

The synonym gate refuses a name built out of a refused word, so that a compound carrying
Status is caught. Applied to Step that refuses `step_back_over_the_white_space_at_the_end`,
which is a real function in `src/scanning.rs` and is about whitespace. **A reading cannot
tell a synonym from an English word, so this document has to say which of its refusals are
which.** The six, as a block rather than as a table, because a table of them in backticks
would bless every one of them:

```text
Step   check   handle   Owner   Assignee   Team
```

A word in that block is refused when a name **is** that word and not when a name **has**
it. Every other word in a `Not` column is refused either way — and the compound carrying
State is **accepted** as a whole, because it is blessed and a gate asks the blessing before
it asks the words.

**Why the split falls where it does.** Status and State are the same fact as Stage, and a
compound carrying one of them is carrying the wrong word, so they are refused anywhere.
Step is not a word anybody reaches for when they mean Stage; it is a word people use, and
the only thing this repository wants to say about it is that a stage is not a step.

**Every word in this section is written without backticks, and that is the third time this
document has had to be told.** A backtick here is a blessing, whatever the sentence around
it intends: the first version of this section blessed every prohibition it named, the
second blessed the very compound it was refusing, and this paragraph is the third. The
rule is therefore no longer a sentence in a table at the top of the file — it is a section
of its own, and the reading has a test that fails the day a refused word is blessed.

---

## Banned vocabulary

Read by `a_name_is_never_generic_vocabulary`, which does not restate this list. A banned
word is a noun a name **ends** on, because the last word of an identifier is what the name
claims the thing *is*. A family is matched as a prefix or a suffix, so a name built on one
is caught while a word that merely contains it is not.

When one of these is genuinely the right idea, the honest question is which domain concept
is being obscured.

```text
Manager   Handler   Processor   Helper   Util   Utils   Common   Base<Domain>
Data      Info      Payload     Params   Result Item     Entity   Dto
Cfg       Meta      Misc        Temp     Temp0   Foo     Bar      Whatever
Adapter   Port      UseCase     Builder  Factory Runner   Executor Engine
Service   Context   State       Status
Repository
```

**Two lists, because the two exemptions mean opposite things, and the guard reads both.**

**Blessed as a whole name, refused as a compound's ending:** `Repository`. A name that is
exactly `Repository` is this system's public GitHub repository; a name that *ends* on it —
`AWorkRepository`, `GitHubRepository` — is a persistence abstraction wearing a domain word.

**Refused alone, blessed as a compound's ending:** `Item`. `Item` on its own says nothing
about what the thing is, and `WorkItem` is the first-class object this glossary names in §The
core nouns. **A list that refused both would have refused the domain's own central type on its
first line**, and a rule that cannot be run against its own author is a rule nobody checked.

**Both lines are what the guard reads**, so a word added to the list above needs no change in
code, and blessing one is a visible act rather than a comment.

**`Adapter`, `Port` and `UseCase` are on the list on purpose.** This repository is Clean
Architecture, and those three are its own words for the things this glossary names
properly: a port is `AReasoningModel` or `TheProjectOnDisk`, not `ModelPort`; the thing
that implements one is named after what it reaches, not after the fact that it implements
something. **A word borrowed from a pattern is the first step towards a codebase that
talks about the pattern instead of about the work.**

**`Repository` is refused as a suffix and blessed as a word.** `AWorkRepository` and
`GitHubRepository` are persistence abstractions wearing a domain word, and they are caught
because the guard judges the last word. `Repository` itself is exempt because the glossary
blesses it here: this system's repository is a public GitHub repository, and it is not the
same thing at all.

**Status and State were used for Stage in two implementations.** Both are refused
now, so the third one has to learn the word.

---

**`#[ignore = "…"]` and not a bare `#[ignore]`.** A skipped test is a test that is not
run, and a skipped test nobody can explain is a hole in the suite with a green tick over it. The
reason goes in the attribute rather than in a comment beside it, **because a comment saying why
a test is ignored would be refused by the rule that this repository leaves no comments** — so
the only place a reason can live is a place the compiler reads, and a reason in the source is a
reason a reader cannot lose by deleting the line above it. `#[test]` and `#[derive(…)]` are not
ignores, and a rule drawn at "any attribute without an `=`" refused both.

**`dyn` and not a dispatch.** A trait object is a promise that the concrete type behind a value
can be replaced, and in the core that promise buys nothing: a work item is a work item, a stage
is a stage, and a value that is "some stage" is a value nobody can say anything definite about.
The rule is drawn at the word rather than at the dispatch, **because a list of the dispatches
this repository does not need is a list somebody has to keep in step with a language**. The core
is the domain and the gates — both are where a decision has to be visible in the source — and
`dynamic` is a word rather than three letters inside one.

## The rules and what holds each

**One row per rule, and the middle column names the file whose `mod` holds it.** A rule with
no row is a rule nobody is looking for, and this repository lost three of them by rewriting a
table: the rules were named in it and nowhere else, and the row went when the table did.

**A rule that is unbuilt says so, and a rule that is unbuilt and does not say so is refused.**
A debt written down is a debt somebody can pick up; refusing the row would be refusing the only
record of it. The two compiler rows are not tests — they are `#![forbid(unsafe_code)]` and
`#![deny(missing_docs)]`, and the compiler is what holds them.

**Every row here is built, and the five that were not are gone rather than ticked off.** Two of
them were this rule twice: `the_stage_is_a_closed_set_of_six` and
`the_role_is_a_closed_set_of_five` are already held by
`the_specification_and_the_code_have_not_drifted_apart`, whose
`there_are_as_many_on_both_sides_as_the_specification_declares` proves both counts. Two more
guarded an empty set: this workspace has no third-party dependency to allow and no `#[ignore]`
to explain, so `every_third_party_crate_is_on_its_crate_allowlist` and
`an_ignored_test_says_why_it_is_ignored` hold over nothing.
`the_dependencies_point_inward` had nothing to point inward from — the domain depends on
nothing and there is one layer above it. The last, `every_change_is_a_red_then_a_green`, wanted
to read the commit history to police the process that writes it, **which is a rule about the
shape of the work and not about the work.**

**Removing a row is a decision and is written down here rather than made quietly.** The three
rules that were lost from the gate table in `AGENTS.md` were lost by an edit nobody recorded,
and `the_documentation_says_what_the_code_does` exists because that happened.

**And it stays removed, now with the history read to decide rather than assumed.** Seventy-eight
pull requests: 160 commits on branches, 74 on `main`, and **not one of the 74 named a RED** — the
subject is the commit's own message when a branch has one commit and the title when it has two or
more, and a behaviour cycle has two, so the subject was the title, and the title was never the
pair. Seventy-seven of the seventy-eight carried a test, and the one that did not was the
workspace harness, which has nothing to break first. **A gate reading the history would have had
nothing to catch that the record had not already lost**, and it would be the rule about the shape
of the work rebuilt as a rule about the shape of the work.

**The record comes back through the subject instead, given explicitly to `--subject`** — and that
was verified rather than assumed: the command that disproved the first version of this paragraph
is the one that confirms the second.

| Rule | Held in | Built |
|---|---|---|
| No `unsafe` | `#![forbid(unsafe_code)]` — the compiler | with the crates |
| Every public item documented | `#![deny(missing_docs)]` — the compiler | with the crates |
| `the_scanner_tells_a_comment_from_a_string` | `readability_guards.rs` | with the scanner |
| `a_file_is_no_longer_than_three_hundred_lines` | `readability_guards.rs` | with the first files |
| `a_function_body_is_no_longer_than_twenty_lines` | `readability_guards.rs` | with the scanner |
| `a_body_nests_no_deeper_than_two` | `readability_guards.rs` | with the scanner |
| `the_code_carries_no_comments` | `readability_guards.rs` | with the scanner |
| `no_allow_silences_a_lint` | `readability_guards.rs` | with the first `.rs` |
| `no_placeholder_is_left_in_the_code` | `readability_guards.rs` | with the first `.rs` |
| `a_name_is_never_generic_vocabulary` | `readability_guards.rs` | with the first types |
| `a_synonym_is_never_used` | `readability_guards.rs` | with the first types |
| `the_domain_depends_on_nothing` | `readability_guards.rs` | with the crates |
| `the_use_cases_may_not_reach_the_world` | `readability_guards.rs` | with the crates |
| `a_name_the_gates_share_has_one_implementation` | `readability_guards.rs` | with the gates |
| `no_test_reads_an_environment_variable` | `readability_guards.rs` | with the tests |
| `the_specification_and_the_code_have_not_drifted_apart` | `documentation_guards.rs` | with the transition table |
| `the_glossary_blesses_every_type_the_domain_defines` | `documentation_guards.rs` | with the first types |
| `the_documentation_says_what_the_code_does` | `documentation_guards.rs` | with the first gate |
| `the_gate_workflow_needs_no_secret` | `documentation_guards.rs` | with the workflow |
| `the_core_never_names_a_trait_object` | `readability_guards.rs` | with the domain |
| `an_ignored_test_says_why_it_is_ignored` | `readability_guards.rs` | with the tests |
| `a_declaration_is_held_by_what_it_lacks` | `readability_guards.rs` | with the declarations |
| `no_body_is_built_by_a_hand` | `readability_guards.rs` | with every body this system will send |
| `a_test_is_not_its_own_proof` | `readability_guards.rs` | with every test that describes a program |

---

## Naming rules at a glance

1. **Spell it out.** No abbreviations: `work_item`, not `wi`.
2. **One concept, one name, everywhere** — in production and in tests. If `WorkItem` is
   the term, Task, Ticket, Job and Entity are all wrong.
3. **The verb is `transition` or `validate_state`**, or it is not a state operation.
4. **A type is a domain type.** `Stage`, not `String`; `None`, not `"none"`.
5. **A boundary payload is a named type**, not a tuple and not a map.
6. **Errors are named after the rule they protect.** `StateMachineError`, not `AppError`.
7. **A port is named after what it reaches**, not after the fact that it is a port.
8. **A table is a `const`, not a function body**, which is why the twenty-line rule does
   not apply to it.
9. **If you cannot name it in domain words, you do not understand it yet.** Rename it or
   delete it. Never abbreviate to escape the question.
