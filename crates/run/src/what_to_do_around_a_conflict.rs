//! Three conflicts, three closed sets of answers, and a model choosing among them.
//!
//! **Free text is not a decision.** A conflict is the one point in a delivery where a person is
//! genuinely wanted — two branches cannot be joined, a proposal restates a filed issue, two runs
//! are touching one file — **and each of those has a small number of sensible answers that are the
//! same every time.** A model asked to "write what to do" will write a paragraph, **and a paragraph
//! is something a run cannot check, cannot record, and cannot tell from one day to the next whether
//! it did the same thing.**
//!
//! So the model **chooses a name and this module holds the set of names.** An answer outside the set
//! is refused rather than guessed, **because guessing here would go on to a rebase, a push or a
//! filing on somebody's board that nobody asked for.**

use std::fmt;

/// What to do when a pull request has a conflict between its branch and the one it joins.
///
/// **Two answers, and the second is not a failure.** "Rebase and try again" rewrites the branch and
/// asks the same question of the suite; "leave the pull request and say why" ends the run with a
/// pull request a person can read. **A run that offered only the first would rebase for ever
/// against a conflict that a rebase cannot fix.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatToDoWhenAPullRequestConflicts {
    /// Rebase the branch onto the one it joins, and let the suite run again.
    RebaseTheBranchAndTryTheMergeAgain,
    /// Leave the pull request open and record why it could not be merged.
    LeaveThePullRequestAndRecordWhy,
}

/// What to do when a proposed piece of work restates something somebody already filed.
///
/// **Two answers and the second is not a workaround.** Filing anyway is legitimate — **this run may
/// have read something the other did not** — and it costs nothing but a second issue in a queue.
/// Refusing would be deciding on a model's judgement of what counts as the same work, **and an issue
/// that landed and turned out to be a duplicate sits in somebody else's queue until a person closes
/// it.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatToDoWhenAProposalIsAlreadyFiled {
    /// Do not file it, and say which filed issue it restates.
    DoNotFileIt,
    /// File it, naming the filed one in the body so a reader can tell them apart.
    FileItNamingTheFiledOne,
}

/// What to do when two runs of this system are touching one file at the same time.
///
/// **Two answers and neither is a default.** Pushing over the other run's work destroys a delivery
/// somebody approved; not pushing leaves a branch that is not on the remote and a delivery that
/// reported a push. **Neither is safe on its own, which is why a model rules with both in front of
/// it rather than this module choosing.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatToDoWhenTwoRunsTouchOneFile {
    /// Push this run's change over the other.
    PushTheChange,
    /// Leave the change unpushed, and say so.
    LeaveTheChangeUnpushed,
}

/// What reading one answer may give, **and the name is short because a signature carrying the full
/// type is one rustfmt splits in two — and a split signature makes this repository's own nesting
/// guard count a brace that has nothing to do with the body.**
type TheDecision = Option<WhatToDoAroundAConflict>;

/// Which of the three situations an answer is about, **so one question and one answer carry all
/// three without a caller reading the answer's shape to find out.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AConflict {
    /// A pull request has a conflict between its branch and the one it joins.
    APullRequestConflicts,
    /// A proposed piece of work restates something somebody already filed.
    AProposalIsAlreadyFiled,
    /// Two runs of this system are touching one file at the same time.
    TwoRunsTouchOneFile,
}

/// A decision about one of the three conflicts, **and the three are told apart by the type rather
/// than by a tag a caller has to match on.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhatToDoAroundAConflict {
    /// A pull request has a conflict.
    AboutAPullRequestConflict(WhatToDoWhenAPullRequestConflicts),
    /// A proposed piece of work restates a filed one.
    AboutAProposalThatIsAlreadyFiled(WhatToDoWhenAProposalIsAlreadyFiled),
    /// Two runs are touching one file.
    AboutTwoRunsTouchingOneFile(WhatToDoWhenTwoRunsTouchOneFile),
}

/// A model answered with something that is not one of the named answers.
///
/// **And the answer is quoted, and so are the answers it could have given.** A model told *"that is
/// not one of the options"* and not told what the options are **will send the same sentence again** —
/// this repository has been burned by exactly that, on an invalid JSON body, three times running.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AConflictNobodyCouldRuleOn {
    /// Which of the three situations the question was about.
    pub about: AConflict,
    /// What the model wrote, verbatim.
    pub what_the_model_wrote: String,
    /// Every answer that was on offer, **so a correction can repeat them.**
    pub which_answers_were_offered: Vec<String>,
}

impl fmt::Display for AConflictNobodyCouldRuleOn {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} is not one of the answers this system can carry out about {}. Answer with exactly \
             one of: {}. **This is refused rather than guessed at**, because a decision a run cannot \
             check is not a decision, and guessing here would go on to a rebase, a push or a filing \
             on somebody's board that nobody asked for.",
            self.what_the_model_wrote,
            what_the_situation_is_called(self.about),
            self.which_answers_were_offered.join(", ")
        )
    }
}

impl std::error::Error for AConflictNobodyCouldRuleOn {}

/// What this situation is called, **and it goes into a sentence a person reads.**
fn what_the_situation_is_called(about: AConflict) -> &'static str {
    match about {
        AConflict::APullRequestConflicts => "a pull request that conflicts",
        AConflict::AProposalIsAlreadyFiled => "a proposal that restates a filing",
        AConflict::TwoRunsTouchOneFile => "two runs touching one file",
    }
}

/// Every answer one situation is offered, **as the strings it is offered — which is the whole of
/// the wire, because a model is offered a fixed set of strings and answers with one of them.**
pub fn the_options_for(about: AConflict) -> Vec<String> {
    let every_one: Vec<&'static str> = match about {
        AConflict::APullRequestConflicts => vec![
            "rebase_the_branch_and_try_again",
            "leave_the_pull_request_and_record_why",
        ],
        AConflict::AProposalIsAlreadyFiled => {
            vec!["do_not_file_it", "file_it_naming_the_filed_one"]
        }
        AConflict::TwoRunsTouchOneFile => vec!["push_the_change", "leave_the_change_unpushed"],
    };
    every_one.iter().map(|one| (*one).to_string()).collect()
}

/// The decision a model made, read out of what it wrote.
///
/// **This is the one door, and there is no other.** Three situations and three answer sets, and
/// every one of them is refused for anything it does not recognise — **which becomes one refusal
/// carrying the answers. A caller that could reach a decision some other way could put a run into a
/// state its own rules never described.**
pub fn what_to_do_around_a_conflict(
    about: AConflict,
    what_the_model_wrote: &str,
) -> Result<WhatToDoAroundAConflict, AConflictNobodyCouldRuleOn> {
    let the_answer = what_the_model_wrote.trim();
    if let Some(the_decision) = the_answer_out_of(about, the_answer) {
        return Ok(the_decision);
    }
    Err(AConflictNobodyCouldRuleOn {
        about,
        what_the_model_wrote: what_the_model_wrote.to_string(),
        which_answers_were_offered: the_options_for(about),
    })
}

/// The answer this situation's names make, **and `None` for every name that is not one of
/// them — including a name that belongs to a different situation.**
fn the_answer_out_of(about: AConflict, what_the_model_wrote: &str) -> TheDecision {
    match about {
        AConflict::APullRequestConflicts => the_answer_about_a_pull_request(what_the_model_wrote),
        AConflict::AProposalIsAlreadyFiled => the_answer_about_a_proposal(what_the_model_wrote),
        AConflict::TwoRunsTouchOneFile => the_answer_about_two_runs(what_the_model_wrote),
    }
}

fn the_answer_about_a_pull_request(what_the_model_wrote: &str) -> TheDecision {
    let the_answer = match what_the_model_wrote {
        "rebase_the_branch_and_try_again" => {
            WhatToDoWhenAPullRequestConflicts::RebaseTheBranchAndTryTheMergeAgain
        }
        "leave_the_pull_request_and_record_why" => {
            WhatToDoWhenAPullRequestConflicts::LeaveThePullRequestAndRecordWhy
        }
        _ => return None,
    };
    Some(WhatToDoAroundAConflict::AboutAPullRequestConflict(
        the_answer,
    ))
}

fn the_answer_about_a_proposal(what_the_model_wrote: &str) -> TheDecision {
    let the_answer = match what_the_model_wrote {
        "do_not_file_it" => WhatToDoWhenAProposalIsAlreadyFiled::DoNotFileIt,
        "file_it_naming_the_filed_one" => {
            WhatToDoWhenAProposalIsAlreadyFiled::FileItNamingTheFiledOne
        }
        _ => return None,
    };
    Some(WhatToDoAroundAConflict::AboutAProposalThatIsAlreadyFiled(
        the_answer,
    ))
}

fn the_answer_about_two_runs(what_the_model_wrote: &str) -> TheDecision {
    let the_answer = match what_the_model_wrote {
        "push_the_change" => WhatToDoWhenTwoRunsTouchOneFile::PushTheChange,
        "leave_the_change_unpushed" => WhatToDoWhenTwoRunsTouchOneFile::LeaveTheChangeUnpushed,
        _ => return None,
    };
    Some(WhatToDoAroundAConflict::AboutTwoRunsTouchingOneFile(
        the_answer,
    ))
}

impl WhatToDoAroundAConflict {
    /// Whether this answer reaches somebody else, **which is the distinction a run needs.**
    ///
    /// **Three of the six reach somebody and three do not.** Filing an issue, pushing a branch and
    /// retrying a merge all reach out; leaving a pull request, not filing and not pushing do not.
    /// **A caller that could not ask would have to enumerate all six at every call site.**
    pub fn does_it_reach_the_world(&self) -> bool {
        !matches!(
            self,
            WhatToDoAroundAConflict::AboutAPullRequestConflict(
                WhatToDoWhenAPullRequestConflicts::LeaveThePullRequestAndRecordWhy
            ) | WhatToDoAroundAConflict::AboutAProposalThatIsAlreadyFiled(
                WhatToDoWhenAProposalIsAlreadyFiled::DoNotFileIt
            ) | WhatToDoAroundAConflict::AboutTwoRunsTouchingOneFile(
                WhatToDoWhenTwoRunsTouchOneFile::LeaveTheChangeUnpushed
            )
        )
    }
}

/// The question put to a model about one conflict, **naming every answer in full.**
///
/// **The answers are in the prompt and not only in the shape, and that is this module's own rule.**
/// A model shown a fixed set of strings in a tool's shape answers with something outside it, **and a
/// prompt that does not repeat the answers does not help: the answer is refused, the correction is
/// sent, and the same sentence comes back.**
pub fn the_question_about(about: AConflict, what_the_conflict_is: &str) -> String {
    let situation = what_the_situation_is_called(about);
    let every_answer = the_options_for(about).join(" or ");
    format!(
        "{situation}: {what_the_conflict_is}. Answer with exactly one of: {every_answer}. \
         **Answer with one of those names and nothing else** — a sentence cannot be carried out, \
         and **do not answer with a name from a different kind of conflict**, because each one \
         means something else."
    )
}
