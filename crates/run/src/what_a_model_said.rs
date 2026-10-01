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
    #[must_use]
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
    #[must_use]
    pub fn answered_after(self, how_long_it_took: HowLongItTook) -> Self {
        Self {
            how_it_ended: HowItEnded::Answered(how_long_it_took),
            ..self
        }
    }

    /// This same call, refused, **naming the field that could not be read.**
    #[must_use]
    pub fn refused_because_the_field(self, which_field: &str) -> Self {
        Self {
            how_it_ended: HowItEnded::Refused(which_field.to_string()),
            ..self
        }
    }

    /// The line a reader sees, **and it carries the elapsed time and never the question.**
    ///
    /// **Two shapes and not one**, because `planning` and `planned in 41s` are different truths about
    /// the world and a template with an empty duration in it would print a number nobody measured.
    pub fn as_a_line(&self) -> String {
        let which = self.which_artifact.to_string();
        match &self.how_it_ended {
            HowItEnded::Outstanding => format!(
                "planning {which} for {} — {} characters asked",
                self.the_model, self.how_long_the_question_was
            ),
            HowItEnded::Answered(how_long) => {
                let the_seconds = the_seconds_in(*how_long);
                format!("planned {which} in {the_seconds}s for {}", self.the_model)
            }
            HowItEnded::Refused(which_field) => format!(
                "refused {which} in field {which_field} for {}",
                self.the_model
            ),
        }
    }
}

/// How long a call took, **in seconds with one digit, and never rounded to nothing.**
///
/// **A tenth of a second and not a whole one**, because a model that answered in 0.4s and a model that
/// answered in 0.0s are different facts **and a whole second would print both as `0s`** — which is a
/// line that reports a measurement it did not take.
pub fn the_seconds_in(how_long_it_took: HowLongItTook) -> String {
    let the_milliseconds = u32::try_from(how_long_it_took.0).unwrap_or(u32::MAX);
    format!("{:.1}", f64::from(the_milliseconds) / 1000.0)
}
