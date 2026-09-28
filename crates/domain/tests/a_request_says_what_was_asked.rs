//! A commissioned request is what somebody asked for, in the words they used.
//!
//! **Text and not an address, and what this type does not have is most of what it is.** A
//! request derives nothing: no repository, no issue number, no URL, no `…/issues/N`. The
//! implementation this domain was read from derived that last one from an owner and a
//! number, **and the repository it was pointed at has no issues at all** — so every
//! summary it produced was full of links to pages that do not exist, and a summary full
//! of them is worse than a summary with none.
//!
//! **Two arguments and no single-string door, and nothing is trimmed, lowercased or
//! re-wrapped.** A title and a body are two pieces of text and no one string is both of
//! them, and the words are the words: a title a person typed and a title a run reads back
//! nine reads later are the same title, the space somebody left at the end of it included.
//!
//! ## The refusal is narrow on purpose, and here is the question it leaves open
//!
//! **A title with no body is a request, and a body with no title is a request.** A
//! request that says nothing was asked for is refused; a request that says one of the two
//! things somebody asked for is kept. Refusing a title on its own would be refusing a
//! question a person can answer — the analysis asks for the body, and a summary that
//! carried an empty body would be a summary carrying a gap.
//!
//! **The question being chosen not to answer is "is this enough to act on".** Whether a
//! body is long enough, specific enough or unambiguous enough to plan against is the
//! requirements analysis's question and not a type's, and a type that answered it would be
//! refusing work on grounds the person who asked for it would never see.

use aisdlc_domain::{ARequestThatSaysNothing, CommissionedRequest};

/// Every pair of words that asks for nothing, and why each one is here.
///
/// **A table is a `const` and a `const` has no body**, which is the eighth naming rule in
/// the glossary. The first is two empty halves, and the other three are halves that are
/// only whitespace: a run is asked what somebody meant, and whitespace is what somebody
/// meant when they typed nothing.
const THE_REQUESTS_THAT_SAY_NOTHING: [(&str, &str); 4] =
    [("", ""), ("   ", ""), ("", "\n\t"), (" \t ", "  ")];

/// A title written the way a person writes one, with the spaces they left in it.
const A_TITLE_AS_IT_WAS_TYPED: &str = "  Ship   the   Thing  ";

/// A body written across two lines, which is what a person writing prose produces.
const A_BODY_AS_IT_WAS_TYPED: &str = "it is slow\nand I would like it not to be";

/// The refusal for a pair of words that asks for nothing, and the pair itself if it asks
/// for something.
///
/// The return type is there so that the narrowness of the refusal is a compile-time fact
/// rather than a sentence: there is one type, and it is what a caller is handed whenever a
/// request says nothing, whichever of the two halves happened to be empty.
fn the_refusal_for(title: &str, body: &str) -> ARequestThatSaysNothing {
    match CommissionedRequest::new(title, body) {
        Ok(_) => panic!("{title:?} and {body:?} ask for something, so there is no refusal"),
        Err(refusal) => refusal,
    }
}

#[test]
fn a_request_that_says_nothing_is_refused() {
    for (title, body) in THE_REQUESTS_THAT_SAY_NOTHING {
        let refused = CommissionedRequest::new(title, body);
        assert!(
            refused.is_err(),
            "{title:?} and {body:?} ask for nothing, and a request that says nothing was asked \
             for cannot be traced back to anything"
        );
    }
}

#[test]
fn a_title_with_no_body_says_what_was_asked() {
    let asked = CommissionedRequest::new("Ship the thing", "");

    let request = asked.expect("a title names what was asked, and the body can be asked for later");
    assert_eq!(
        request.the_title_of(),
        "Ship the thing",
        "the title is the request as far as this type is concerned, and a body nobody wrote is \
         a question the analysis asks rather than a reason to refuse"
    );
    assert_eq!(
        request.the_body_of(),
        "",
        "and the body is absent rather than invented, because a summary full of detail nobody \
         wrote is a summary nobody can check"
    );
}

#[test]
fn a_body_with_no_title_says_what_was_asked() {
    let asked = CommissionedRequest::new("", "make the test suite run without a network");

    let request = asked.expect("a body in somebody's own words is somebody asking for something");
    assert_eq!(
        request.the_body_of(),
        "make the test suite run without a network",
        "the words arrive with the work and the work item keeps them, and a stage reading them \
         nine reads later reads what the first one read"
    );
}

#[test]
fn the_words_are_the_words() {
    let asked = CommissionedRequest::new(A_TITLE_AS_IT_WAS_TYPED, A_BODY_AS_IT_WAS_TYPED);

    let request = asked.expect("a title and a body of somebody's own words say what was asked");
    assert_eq!(
        request.the_title_of(),
        A_TITLE_AS_IT_WAS_TYPED,
        "no trimming, no lowercasing and no re-wrapping: a name a person typed and a name a run \
         reads back are the same name, or a gate comparing them as strings refuses a work item \
         for a space somebody left in it"
    );
    assert_eq!(
        request.the_body_of(),
        A_BODY_AS_IT_WAS_TYPED,
        "and a line ending in the middle of a body is a line ending somebody typed, not one to \
         be tidied away"
    );
}

#[test]
fn a_refused_request_comes_back_as_it_was_written() {
    let refusal = the_refusal_for("  ", "\t");

    assert_eq!(
        refusal.the_title_written(),
        "  ",
        "the refusal quotes the words back untrimmed, and a refusal that tidies what it is \
         refusing teaches the caller nothing about what they wrote"
    );
    assert_eq!(
        refusal.the_body_written(),
        "\t",
        "and it quotes both halves, because which half was empty is the whole of the answer"
    );
    let said = refusal.to_string();
    assert_eq!(
        said, "a request with the title \"  \" and the body \"\\t\" says nothing",
        "a reader debugging a run reads the written refusal and not the type of it, and the \
         `Debug` form is what tells two blank halves apart: with a tab written as a tab, a \
         title of spaces and a body of a tab are the same sentence"
    );
}
