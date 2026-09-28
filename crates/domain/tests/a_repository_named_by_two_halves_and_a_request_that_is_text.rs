//! A repository named by two halves, and a request that is text and not an address.
//!
//! **These two are in one file because each of them is a boundary somebody gets wrong in the
//! same direction.** A repository is two halves, and pasting both into one field is the
//! mistake; a request is words, and turning words into an address is the mistake. Both are
//! refusals a caller feels before anything is built, rather than a defect found nine stages
//! later on a remote nobody can reach.
//!
//! No file is read here, because this is a test of the domain and the domain may not reach
//! for a filesystem. The specification's account of these is checked by a gate.

use aisdlc_domain::commissioned_request::{ARequestThatSaysNothing, CommissionedRequest};
use aisdlc_domain::repository::{AHalfThatIdentifiesNothing, Repository};

#[test]
fn a_repository_is_named_by_an_owner_and_a_name() {
    let repository = Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves");

    assert_eq!(
        repository.owner, "steamnoid",
        "the first half is who owns it"
    );
    assert_eq!(
        repository.name, "ai-sdlc-app-rs-plus",
        "the second is what it is called"
    );
}

#[test]
fn a_repository_is_written_the_way_it_is_named() {
    let repository = Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves");

    assert_eq!(
        repository.to_string(),
        "steamnoid/ai-sdlc-app-rs-plus",
        "one slash, in the middle, and nothing else added"
    );
}

#[test]
fn a_half_that_is_blank_names_nothing() {
    for address in [
        "",
        "/",
        "steamnoid/",
        "/ai-sdlc-app-rs-plus",
        "  /name",
        "owner/   ",
    ] {
        let result = Repository::try_from(address);

        assert_eq!(
            result,
            Err(AHalfThatIdentifiesNothing(address.to_string())),
            "a repository is an owner and a name, and {address:?} is not both of them"
        );
    }
}

#[test]
fn a_half_carrying_a_slash_is_both_halves_pasted_into_one_field() {
    for address in [
        "steamnoid",
        "steamnoid/ai-sdlc-app-rs-plus/extra",
        "steamnoid//ai-sdlc-app-rs-plus",
    ] {
        let result = Repository::try_from(address);

        assert_eq!(
            result,
            Err(AHalfThatIdentifiesNothing(address.to_string())),
            "{address:?} is not one owner and one name: a second slash means both halves were pasted into the first"
        );
    }
}

#[test]
fn the_address_that_was_refused_is_reported_back_whole() {
    let refused = Repository::try_from("steamnoid/ai-sdlc-app-rs-plus/extra")
        .expect_err("two slashes are three halves");

    assert_eq!(
        refused.0, "steamnoid/ai-sdlc-app-rs-plus/extra",
        "what was sent is what is reported, because a refusal that quotes a part of the address is a refusal a reader has to reconstruct"
    );
}

#[test]
fn a_request_is_the_words_and_not_an_address() {
    let request = CommissionedRequest::asked_for(
        "add a health endpoint".to_string(),
        "GET /health should return 200 and a body".to_string(),
    )
    .expect("a title and a body say something");

    assert_eq!(request.title, "add a health endpoint");
    assert_eq!(request.body, "GET /health should return 200 and a body");
}

#[test]
fn the_words_are_kept_exactly_as_they_were_used() {
    for (title, body) in [
        ("  spaced  ", "  and  body  "),
        ("HTTPS caps", "MiXeD CaSe"),
        ("tabs\there", "newlines\nthere"),
    ] {
        let request =
            CommissionedRequest::asked_for(title.to_string(), body.to_string()).expect("words");

        assert_eq!(request.title, title, "a title is kept as typed: {title:?}");
        assert_eq!(request.body, body, "and a body is kept as typed: {body:?}");
    }
}

#[test]
fn a_request_with_only_a_title_is_a_request() {
    let request =
        CommissionedRequest::asked_for("add a health endpoint".to_string(), String::new())
            .expect("a title alone says something was asked for");

    assert_eq!(request.title, "add a health endpoint", "the title is kept");
    assert!(
        request.body.is_empty(),
        "and the body is empty rather than refused"
    );
}

#[test]
fn a_request_with_only_a_body_is_a_request() {
    let request = CommissionedRequest::asked_for(String::new(), "please add /health".to_string())
        .expect("a body alone says something was asked for");

    assert!(
        request.title.is_empty(),
        "the title is empty rather than refused"
    );
    assert_eq!(request.body, "please add /health", "and the body is kept");
}

#[test]
fn a_request_with_no_title_and_no_body_says_nothing() {
    for (title, body) in [("", ""), ("   ", ""), ("", "\t"), ("  ", "  ")] {
        let result = CommissionedRequest::asked_for(title.to_string(), body.to_string());

        assert_eq!(
            result,
            Err(ARequestThatSaysNothing),
            "a request that says nothing was asked for, and {title:?} with {body:?} says nothing"
        );
    }
}

#[test]
fn a_request_that_says_nothing_says_so_where_a_reader_can_see_it() {
    let said = ARequestThatSaysNothing.to_string();

    assert!(
        said.contains("title") && said.contains("body"),
        "a refusal that does not name the two fields leaves the reader to guess which one was empty: {said}"
    );
}

#[test]
fn the_two_boundaries_refuse_different_things() {
    let a_repository = Repository::try_from("/name");
    let a_request = CommissionedRequest::asked_for("a title".to_string(), "   ".to_string());

    assert_eq!(
        a_repository,
        Err(AHalfThatIdentifiesNothing("/name".to_string())),
        "a repository needs both halves or neither"
    );
    assert_eq!(
        a_request.ok().map(|it| it.title),
        Some("a title".to_string()),
        "a request needs only one of the two, so the two boundaries cannot be one type"
    );
}
