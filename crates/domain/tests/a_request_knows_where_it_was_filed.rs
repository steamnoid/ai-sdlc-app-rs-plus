//! A request that was filed somewhere remembers where, and one that was not, does not.
//!
//! **The address is the one the filing returned and never one composed from a number.** A derived
//! `…/issues/N` resolves to a page that does not exist in a repository with no issues, and **a
//! false reference in the audit trail is the one thing a traceability chain exists to prevent.**
//!
//! **The filing is beside the words and not instead of them.** The words are the request; the
//! filing is where those words were typed. A type that carried a number instead of a sentence made
//! a stage read the repository while the sentence skipped four stages.

use aisdlc_domain::commissioned_request::{AWhereItWasFiled, CommissionedRequest};

#[test]
fn a_request_somebody_typed_has_no_filing() {
    let the_request =
        CommissionedRequest::asked_for("a thing should change".to_string(), String::new())
            .expect("a title is a request");

    assert_eq!(
        the_request.where_it_was_filed(),
        None,
        "**because a person who typed a sentence into this system did not file it anywhere**, and a request that claims an address it was never given is a false reference in the first record a run writes"
    );
}

#[test]
fn a_request_that_came_from_a_filing_carries_the_address_that_filing_returned() {
    let an_address = "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/41";
    let the_request = CommissionedRequest::asked_for_and_filed_at(
        "a thing should change".to_string(),
        String::new(),
        AWhereItWasFiled::the_address_the_filing_returned(an_address).expect("an address"),
    )
    .expect("a title is a request");

    assert_eq!(
        the_request
            .where_it_was_filed()
            .expect("it was filed")
            .as_str(),
        an_address,
        "**and it is that address, word for word** — an address this system composed would be a link to a page it had not checked, which is the difference between a reference and a guess"
    );
}

#[test]
fn an_address_that_is_only_whitespace_is_not_an_address() {
    let the_address = AWhereItWasFiled::the_address_the_filing_returned("   ");

    assert!(
        the_address.is_err(),
        "**because an address of spaces is a link to nowhere and a link to nowhere in an audit trail is worse than no link at all** — a reader follows it, finds nothing, and learns that this system's references cannot be trusted"
    );
}

#[test]
fn the_words_are_still_the_request_and_the_filing_is_only_where_they_were_typed() {
    let the_request = CommissionedRequest::asked_for_and_filed_at(
        "a thing should change".to_string(),
        "in the place it says".to_string(),
        AWhereItWasFiled::the_address_the_filing_returned("https://example.invalid/i/1")
            .expect("an address"),
    )
    .expect("a title is a request");

    assert_eq!(
        (the_request.title.as_str(), the_request.body.as_str()),
        ("a thing should change", "in the place it says"),
        "**because the words are what somebody asked for and a stage reads them nine times** — the filing beside them must not become a reason to read something else, and a type that carried a number instead of a sentence made a product analyst read the repository while the sentence went to the planner"
    );
}
