//! A filed request as its author wrote it, and where it actually lives.
//!
//! **`address` is the one GitHub returned, and not one derived from the repository and the
//! number.** An issue that was transferred answers at an address that does not carry the number
//! asked for, and a composed address cannot express that — **it can only be right or wrong, and it
//! was silently wrong on the only repository this system had been pointed at.**
//!
//! **Whether it is still open is carried, because a closed request is a fact a person reads before
//! ruling.** A stage deciding whether to work on something that was closed should see that without
//! asking GitHub a second time. **It is a property of the filing and not a `Stage`** — `Stage` is
//! where a work item stands in this system, and a filing does not stand anywhere.
//!
//! **`body` may be empty and that is a real request.** Somebody filed it with a title and nothing
//! else, and refusing that would refuse the issue a person wrote in the fewest seconds.

/// An answer that is not a filed request, and why it is not one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnAnswerThatIsNotAFiledRequest {
    why_not: String,
}

impl AnAnswerThatIsNotAFiledRequest {
    /// The refusal, with the answer quoted back.
    ///
    /// **Quoted and not summarised**, because a caller deciding whether to retry needs to know
    /// what GitHub actually said, and a paraphrase of it is one more layer to be wrong in.
    pub fn because(what_it_said: &str, why_not: &str) -> Self {
        Self {
            why_not: format!("{why_not}. What came back was: {what_it_said}"),
        }
    }
}

impl std::fmt::Display for AnAnswerThatIsNotAFiledRequest {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.why_not)
    }
}

/// The refusal for a pull request, naming where it is and quoting what came back.
fn a_pull_request(
    where_it_answers: Option<&str>,
    what_github_answered: &str,
) -> AnAnswerThatIsNotAFiledRequest {
    let why_not = format!(
        "{where_it_answers:?} is a pull request and not a filed request: GitHub answers both from \
         one path, and a pull request's body describes a change rather than asking for one"
    );
    AnAnswerThatIsNotAFiledRequest::because(what_github_answered, &why_not)
}

/// What one field of an answer says, or nothing when it says nothing.
///
/// **Empty and not refused,** because somebody filed a request with a title and nothing else, and
/// a reader that refused that would refuse the issue a person wrote in the fewest seconds.
fn the_field_says(what_it_came_back_as: &serde_json::Value, which_field: &str) -> String {
    what_it_came_back_as
        .get(which_field)
        .and_then(|it| it.as_str())
        .unwrap_or_default()
        .to_string()
}

/// What a person filed, word for word, and where it answers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnIssueRead {
    what_was_asked: String,
    what_was_said_besides: String,
    where_it_answers: String,
    whether_it_is_open: bool,
}

impl AnIssueRead {
    /// What GitHub answered, or the reason it is not a filed request.
    pub fn the_issue_read(
        what_github_answered: &str,
    ) -> Result<Self, AnAnswerThatIsNotAFiledRequest> {
        let what_it_came_back_as: serde_json::Value = serde_json::from_str(what_github_answered)
            .map_err(|_| {
                AnAnswerThatIsNotAFiledRequest::because(
                    what_github_answered,
                    "that is not an answer this system can read",
                )
            })?;
        Self::what_it_says(what_github_answered, &what_it_came_back_as)
    }

    /// The parts of an answer that are a filed request, and the ones that are refused.
    fn what_it_says(
        what_github_answered: &str,
        what_it_came_back_as: &serde_json::Value,
    ) -> Result<Self, AnAnswerThatIsNotAFiledRequest> {
        let where_it_answers = what_it_came_back_as
            .get("html_url")
            .and_then(|it| it.as_str());
        if what_it_came_back_as.get("pull_request").is_some() {
            return Err(a_pull_request(where_it_answers, what_github_answered));
        }
        let Some(where_it_answers) = where_it_answers else {
            return Err(AnAnswerThatIsNotAFiledRequest::because(
                what_github_answered,
                "that answer carries no address, and an address this system would compose is a link \
                 to a page that may not be this one",
            ));
        };
        Ok(AnIssueRead {
            what_was_asked: the_field_says(what_it_came_back_as, "title"),
            what_was_said_besides: the_field_says(what_it_came_back_as, "body"),
            where_it_answers: where_it_answers.to_string(),
            whether_it_is_open: the_field_says(what_it_came_back_as, "state") == "open",
        })
    }

    /// The title, which is a request a person wrote in the fewest seconds.
    pub fn what_was_asked(&self) -> &str {
        &self.what_was_asked
    }

    /// Everything said besides the title, and an empty string is a real request.
    pub fn what_was_said_besides(&self) -> &str {
        &self.what_was_said_besides
    }

    /// The address GitHub returned, word for word.
    pub fn where_it_answers(&self) -> &str {
        &self.where_it_answers
    }

    /// Whether it was still open, because a person reads that before ruling.
    pub fn whether_it_is_open(&self) -> bool {
        self.whether_it_is_open
    }
}
