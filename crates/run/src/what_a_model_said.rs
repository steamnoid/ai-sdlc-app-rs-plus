//! What a run says while it is doing it, one line at a time.
//!
//! **Not to be confused with the durable record of a whole run**, which is written afterwards; **this
//! is one call, alive while it is happening.**
//!
//! **The shell for now.** Every value is refused, because a run that says nothing is better than one
//! that says something wrong — **and the tests are what demand a single word.**

/// Something a run is telling while it happens.
pub trait AWhomARunTellsWhatItIsDoing {
    /// One more thing the run is doing.
    fn it_is_told(&mut self, what_the_run_said: &WhatAModelSaid);
}

/// How long a call took, **in milliseconds, and never measured here.**
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HowLongItTook(pub u64);

/// A call a run is making, and what came of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatAModelSaid {
    which_artifact: aisdlc_domain::approval::WhichArtifact,
    the_model: String,
    how_long_the_question_was: usize,
    how_it_ended: HowItEnded,
}

/// How one call ended, **and the three shapes a reader has to tell apart.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HowItEnded {
    /// The model has not answered yet, **and a line saying only a label says nothing.**
    Outstanding,
    /// The model answered, and this is how long it took.
    Answered(HowLongItTook),
    /// The answer could not be read, **and this is the field that could not.**
    Refused(String),
}

impl WhatAModelSaid {
    /// A call being made, **and a run that is calling a model can always say that it is.**
    pub fn a_call_is_outstanding(
        which_artifact: aisdlc_domain::approval::WhichArtifact,
        how_long_the_question_was: usize,
        the_model: String,
    ) -> Self {
        Self {
            which_artifact,
            the_model,
            how_long_the_question_was,
            how_it_ended: HowItEnded::Outstanding,
        }
    }

    /// This same call, answered.
    pub fn answered_after(self, how_long_it_took: HowLongItTook) -> Self {
        Self {
            how_it_ended: HowItEnded::Answered(how_long_it_took),
            ..self
        }
    }

    /// This same call, refused, **naming the field that could not be read.**
    pub fn refused_because_the_field(self, which_field: &str) -> Self {
        Self {
            how_it_ended: HowItEnded::Refused(which_field.to_string()),
            ..self
        }
    }

    /// The line a reader sees, **and it carries the elapsed time and never the question.**
    pub fn as_a_line(&self) -> String {
        String::new()
    }
}

/// How long took a second and a bit, **said in seconds because that is what a reader counts in.**
pub fn the_seconds_in(how_long_it_took: HowLongItTook) -> String {
    let _ = how_long_it_took;
    String::new()
}
