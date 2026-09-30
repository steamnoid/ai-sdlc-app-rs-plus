//! A run leaves a comment on the filing its work answers, and says it is writing a person did not.
//!
//! **A comment is this system's handwriting and a reader cannot tell it from a person's.** So
//! every comment names who wrote it, and the name is one this system was told — **because a
//! comment that claims a person wrote it is the same forgery as a record naming a person for a
//! decision no person made.**
//!
//! **A request nobody filed has nowhere to be commented on, and that is a refusal and not a
//! comment on a made-up place.** A comment on a derived address is a comment on a page that does
//! not exist, and a link to nothing in an audit trail is worse than no link.

use aisdlc_domain::commissioned_request::{AWhereItWasFiled, CommissionedRequest};
use aisdlc_run::a_comment_is_left::the_run_leaves_a_comment;
use aisdlc_run::a_git_hub_we_write_to::{AGitHubWeWriteTo, WhatGitHubWasAskedFor};

/// Somewhere to write, that remembers what it was asked.
struct SomewhereThatAccepts {
    what_it_was_asked: std::cell::RefCell<Vec<WhatGitHubWasAskedFor>>,
}

impl SomewhereThatAccepts {
    fn a_new_one() -> Self {
        Self {
            what_it_was_asked: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl AGitHubWeWriteTo for SomewhereThatAccepts {
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &WhatGitHubWasAskedFor,
    ) -> Result<(), String> {
        self.what_it_was_asked
            .borrow_mut()
            .push(what_it_is_asked_for.clone());
        Ok(())
    }
}

/// The name a deployment writes its own comments under, and never a person's.
fn the_deployment_that_wrote_it() -> aisdlc_run::who_signs_a_comment::WhoSignsAComment {
    aisdlc_run::who_signs_a_comment::WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev")
        .expect("a deployment named one")
}

fn a_request_filed_at(the_address: &str) -> CommissionedRequest {
    CommissionedRequest::asked_for_and_filed_at(
        "a thing should change".to_string(),
        String::new(),
        AWhereItWasFiled::the_address_the_filing_returned(the_address).expect("an address"),
    )
    .expect("a title is a request")
}

#[test]
fn a_comment_names_where_it_was_left_and_who_wrote_it() {
    let mut somewhere = SomewhereThatAccepts::a_new_one();
    let the_address = "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/41";

    the_run_leaves_a_comment(
        &mut somewhere,
        &a_request_filed_at(the_address),
        "a run looked at this and here is what it found",
        &the_deployment_that_wrote_it(),
    )
    .expect("a request that was filed has somewhere to be commented on");

    let what_it_was_asked = somewhere.what_it_was_asked.borrow();
    assert_eq!(
        what_it_was_asked[0].where_it_is_going().as_str(),
        the_address,
        "**and it is the address the filing returned, word for word** — a comment on a composed address is a comment on a page that does not exist"
    );
    assert_eq!(
        what_it_was_asked[0].what_is_written(),
        "a run looked at this and here is what it found",
        "**and the words are the words** — a run rewriting them would be a run editing the request it was asked to answer"
    );
}

#[test]
fn a_comment_carries_who_wrote_it_and_nobody_may_name_a_person_for_it() {
    let mut somewhere = SomewhereThatAccepts::a_new_one();
    let the_address = "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/41";

    the_run_leaves_a_comment(
        &mut somewhere,
        &a_request_filed_at(the_address),
        "a run looked at this",
        &the_deployment_that_wrote_it(),
    )
    .expect("the comment is written");

    assert!(
        somewhere.what_it_was_asked.borrow()[0].who_wrote_this() == "aisdlc-dev",
        "**and the name is the one a deployment used** — a comment with no author is a comment a \
         reader cannot weigh, and **a reader who cannot weigh it will treat it as a person**: this \
         test used to assert that the name 'Alice' reached GitHub, which is this system signing its \
         own writing with a person, and the test called it correct"
    );
}

#[test]
fn a_request_nobody_filed_has_nowhere_to_be_commented_on() {
    let mut somewhere = SomewhereThatAccepts::a_new_one();
    let typed_by_hand =
        CommissionedRequest::asked_for("a thing should change".to_string(), String::new())
            .expect("a title is a request");

    let how_it_went = the_run_leaves_a_comment(
        &mut somewhere,
        &typed_by_hand,
        "a run looked at this",
        &the_deployment_that_wrote_it(),
    );

    assert!(
        how_it_went.is_err(),
        "**because a person who typed a sentence into this system filed it nowhere** — and a comment on a derived address is a comment on a page that does not exist, which is a false reference in the one record a reader follows"
    );
    assert!(
        somewhere.what_it_was_asked.borrow().is_empty(),
        "**and nothing was written at all** — a refusal that arrives after the write is a comment nobody can take back"
    );
}
