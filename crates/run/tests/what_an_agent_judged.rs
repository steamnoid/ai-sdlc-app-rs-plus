//! The three things an agent is asked about its own work, and the text it leaves behind.
//!
//! **A blank is refused rather than stored, and that is the whole point of these three fields.**
//! An agent is asked what it found, what it could not settle and what it expects next — **and an
//! empty field reads as *the agent had nothing to say*, which is not a thing anybody can check.**
//! A model that fills a shape and says nothing is exactly what this repository already knows how
//! to get, and it is why these three are required rather than three more optional lists.
//!
//! **The middle one is the one worth having.** A real run's implementer once wrote that a plan named
//! one path and no intent, so nobody could tell a rule for a file that already exists from a whole
//! new file — a true sentence and not a reason to stop, **and the single most useful thing on the
//! card.**

use aisdlc_run::what_an_agent_judged::{
    WHAT_MAY_BE_SAID_INSTEAD_OF_A_JUDGEMENT, WhatAnAgentJudged,
};

#[test]
fn an_agent_that_says_nothing_in_one_of_three_is_refused() {
    let said_nothing_about_what_it_found = WhatAnAgentJudged::what_an_agent_said(
        "  ",
        "the plan names one path and no intent",
        "ask the author which",
    );

    assert!(
        said_nothing_about_what_it_found.is_err(),
        "**because an empty field reads as *the agent had nothing to say*** — and that is not a thing \
         anybody can check, which is the same reason an ignored test must say why: a silent field and \
         an unasked one are indistinguishable afterwards. What an agent with nothing may say \
         instead is in the refusal: {WHAT_MAY_BE_SAID_INSTEAD_OF_A_JUDGEMENT:?}"
    );
}

#[test]
fn an_agent_says_all_three_and_they_are_kept_verbatim() {
    let what_it_said = WhatAnAgentJudged::what_an_agent_said(
        "an e2e directory the suite never collects",
        "the plan names one path and no intent",
        "ask the author which",
    );

    let what_it_is = what_it_said.expect("all three were said");
    assert_eq!(
        (
            what_it_is.what_it_found.as_str(),
            what_it_is.what_it_could_not_settle.as_str(),
            what_it_is.what_it_expects_next.as_str()
        ),
        (
            "an e2e directory the suite never collects",
            "the plan names one path and no intent",
            "ask the author which"
        ),
        "**kept verbatim and not summarised** — because a conclusion this system did not ask for is \
         one no human was ever shown, and a paraphrase of what an agent said is a second thing said \
         in its name"
    );
}

#[test]
fn the_middle_one_is_the_widest_doubt_the_run_has() {
    let what_it_said = WhatAnAgentJudged::what_an_agent_said(
        "some findings",
        "the plan names one path and no intent",
        "ask the author",
    );

    let what_it_is = what_it_said.expect("all three were said");
    assert_eq!(
        what_it_is.what_it_could_not_settle.as_str(),
        "the plan names one path and no intent",
        "**because a discovery agent is the only one that sees the whole repository** — and a doubt \
         it writes down is a doubt nothing else in the run is in a position to have, which is why \
         its note belongs on the card rather than inside the report"
    );
}
