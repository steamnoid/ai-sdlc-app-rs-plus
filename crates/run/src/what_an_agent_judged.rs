//! The three things an agent is asked about its own work, and the text it leaves behind.
//!
//! **A blank is refused rather than stored, and that is the whole point of these three fields.**
//! An agent with nothing to report says so **in words** — **and an empty field reads as *the agent
//! had nothing to say*, which is not a thing anybody can check.** That is the same reason an
//! ignored test must say why it is ignored: a silent field and a field nobody asked for are
//! indistinguishable afterwards.

/// What an agent with nothing to report may say instead, which is an answer.
pub const WHAT_MAY_BE_SAID_INSTEAD_OF_A_JUDGEMENT: &str = "nothing was found, and nothing was \
left unsettled";

/// One of the three judgements, and it was blank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AJudgementNobodyMade(pub String);

/// What an agent found, what it could not settle, and what it expects next.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAnAgentJudged {
    /// What it found.
    pub what_it_found: String,
    /// **The widest doubt the run has** — a discovery agent is the only one that sees the whole
    /// repository, and a doubt it writes down is one nothing else is in a position to have.
    pub what_it_could_not_settle: String,
    /// What it expects next.
    pub what_it_expects_next: String,
}

impl WhatAnAgentJudged {
    /// What an agent said about its own work, **refused when one of the three is blank.**
    pub fn what_an_agent_said(
        what_it_found: &str,
        what_it_could_not_settle: &str,
        what_it_expects_next: &str,
    ) -> Result<Self, AJudgementNobodyMade> {
        WhatAnAgentJudged::nothing_blank(what_it_found)?;
        WhatAnAgentJudged::nothing_blank(what_it_could_not_settle)?;
        WhatAnAgentJudged::nothing_blank(what_it_expects_next)?;
        Ok(WhatAnAgentJudged {
            what_it_found: what_it_found.to_string(),
            what_it_could_not_settle: what_it_could_not_settle.to_string(),
            what_it_expects_next: what_it_expects_next.to_string(),
        })
    }

    /// Refuse one blank judgement, and say what an empty one would have meant.
    fn nothing_blank(what_was_said: &str) -> Result<(), AJudgementNobodyMade> {
        if what_was_said.trim().is_empty() {
            return Err(AJudgementNobodyMade(what_was_said.to_string()));
        }
        Ok(())
    }
}
