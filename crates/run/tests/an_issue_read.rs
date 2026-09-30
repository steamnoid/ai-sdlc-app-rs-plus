//! A filed request as its author wrote it, and where it actually lives.
//!
//! **`address` is the one GitHub returned, and not one derived from the repository and the
//! number.** An issue that was transferred answers at an address that does not carry the number
//! asked for, and a composed address cannot express that — **it can only be right or wrong, and it
//! was silently wrong on the only repository this system had been pointed at.**
//!
//! **`state` is carried, because a closed request is a fact a person reads before ruling.** A
//! stage deciding whether to work on something that was closed should see that without asking
//! GitHub a second time.
//!
//! **`body` may be empty and that is a real request.** Somebody filed it with a title and nothing
//! else, and refusing that would refuse the issue a person wrote in the fewest seconds.

use aisdlc_run::an_issue_read::AnIssueRead;

/// What GitHub answers with, for one issue, as the far end really sent it.
fn what_github_answered(with_the_answer: &str) -> String {
    format!(
        "{{\"number\":8,\"title\":\"{with_the_answer}\",\"body\":\"the root tsconfig covers the \
         tests\",\"html_url\":\"https://github.com/steamnoid/wellbeing-tracker-public/issues/8\",\
         \"state\":\"open\"}}"
    )
}

#[test]
fn the_address_is_the_one_github_returned() {
    let what_it_answered = what_github_answered("a title");

    let what_was_read = AnIssueRead::the_issue_read(&what_it_answered);

    assert_eq!(
        what_was_read
            .as_ref()
            .map(|what_it_says| what_it_says.where_it_answers()),
        Some("https://github.com/steamnoid/wellbeing-tracker-public/issues/8"),
        "**because the address is the far end's and not ours** — an issue that was transferred \
         answers somewhere the number asked for does not appear, and a composed address can only be \
         right or wrong: {what_was_read:?}"
    );
}

#[test]
fn an_answer_carrying_no_address_is_refused_rather_than_composed() {
    let without_an_address =
        "{\"number\":8,\"title\":\"a title\",\"body\":\"b\",\"state\":\"open\"}";

    let what_was_read = AnIssueRead::the_issue_read(without_an_address);

    assert!(
        what_was_read.is_err(),
        "**because an address this system composes is a link to a page that may not be this one** — \
         and a filing whose record points somewhere that was never asked for is a false reference \
         in the one record a reader follows: {what_was_read:?}"
    );
}

#[test]
fn a_closed_request_says_so_because_a_person_reads_it_before_ruling() {
    let closed =
        what_github_answered("a title").replace("\"state\":\"open\"", "\"state\":\"closed\"");

    let what_was_read = AnIssueRead::the_issue_read(&closed);

    assert_eq!(
        what_was_read
            .as_ref()
            .map(|what_it_says| what_it_says.the_state_it_was_in()),
        Some("closed"),
        "**because a closed request is a fact a person reads before ruling** — and a stage that has \
         to ask GitHub a second time to learn it is a stage that can rule on it wrongly: \
         {what_was_read:?}"
    );
}

#[test]
fn a_request_with_only_a_title_is_a_request_and_not_a_refusal() {
    let only_a_title = "{\"number\":9,\"title\":\"a title\",\"body\":\"\",\"html_url\":\"https://github.com/\
         steamnoid/wellbeing-tracker-public/issues/9\",\"state\":\"open\"}";

    let what_was_read = AnIssueRead::the_issue_read(only_a_title);

    assert_eq!(
        what_was_read
            .as_ref()
            .map(|what_it_says| what_it_says.what_was_asked()),
        Some("a title"),
        "**because somebody filed it with a title and nothing else** — and refusing that would \
         refuse the issue a person wrote in the fewest seconds, which is the one they wrote first: \
         {what_was_read:?}"
    );
}
