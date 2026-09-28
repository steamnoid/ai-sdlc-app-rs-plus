//! A repository is a public GitHub repository, and it is two halves or it is nothing.
//!
//! A repository is named by an owner and by a name, and it is **two fields rather than
//! one `owner/name` string**, which is the whole reason the two are told apart here: a
//! string carrying a slash could be a repository that happened to be written down, and
//! nothing in it would say which half of the boundary a caller had got wrong.
//!
//! **The property that matters is a property about types, and this file asserts it in
//! every direction it can.** There is no single-string door into a `Repository` at all —
//! `new` takes two arguments and there is no `TryFrom<&str>` — so a caller holding a
//! string cannot get a repository out of it however it reads. A blank half is refused
//! because it names nothing, and a half carrying a slash is refused because it is both
//! halves pasted into one field, and **both refusals are one type**: a caller told "that
//! is not a repository" has the same question whichever half it left out, and three
//! types would make it choose which one to ask.
//!
//! The words a caller used come back in the refusal exactly as they were written, and
//! nothing in this crate tidies a name — **a value this system silently tidies is a value
//! a lookup will miss.**

use aisdlc_domain::{AHalfThatIdentifiesNothing, Repository};

/// Every pair of halves that does not name a repository, and why each one is here.
///
/// **A table is a `const` and a `const` has no body**, which is the eighth naming rule in
/// the glossary and the reason this list is not written inside a test: six of the cases
/// below are a fixture a reader can check against the rule by eye, and a list written one
/// per line inside a `for` would be twenty-eight counted lines, which the twenty-line rule
/// refuses.
///
/// The first two are the two halves missing, the next two are a half that is only
/// whitespace, and the last two are a half carrying a slash in it.
const THE_HALVES_THAT_IDENTIFY_NOTHING: [(&str, &str); 6] = [
    ("", ""),
    ("", "rails"),
    ("github", ""),
    ("   ", "rails"),
    ("github", "   "),
    ("github/rails", "rails"),
];

/// A well-formed pair, written the way a person writes it, and the whole of the fixture.
const A_WELL_FORMED_REFERENCE: (&str, &str) = ("github", "rails");

/// The refusal for a pair that is not a repository, and the pair itself if it is one.
///
/// **The return type is the assertion that there is one refusal and not three.** A caller
/// that has been told "that is not a repository" has one question whichever half it left
/// out, and a second type would make it choose which of two to ask before it knows
/// anything about the answer.
fn the_refusal_for(owner: &str, name: &str) -> AHalfThatIdentifiesNothing {
    match Repository::new(owner, name) {
        Ok(_) => panic!("{owner}/{name} names a repository, so there is no refusal to hand back"),
        Err(refusal) => refusal,
    }
}

#[test]
fn a_half_that_identifies_nothing_is_refused() {
    for (owner, name) in THE_HALVES_THAT_IDENTIFY_NOTHING {
        let refused = Repository::new(owner, name);
        assert!(
            refused.is_err(),
            "{owner:?} and {name:?} do not name a repository: an owner and no name, a name and \
             no owner, and a half carrying a slash are each half a repository rather than one"
        );
    }
}

#[test]
fn a_name_carrying_a_slash_cannot_be_built_as_a_half() {
    let refused = Repository::new("github", "rails/rails");

    assert!(
        refused.is_err(),
        "the two halves are two fields, and a name holding a slash is both of them pasted into \
         one — which is the whole reason the type is not a single string"
    );
}

#[test]
fn an_owner_and_a_name_name_a_repository() {
    let (owner, name) = A_WELL_FORMED_REFERENCE;

    let built = Repository::new(owner, name);

    assert!(
        built.is_ok(),
        "{owner}/{name} is an owner and a name, and the only thing this type refuses is half of one"
    );
}

#[test]
fn a_repository_is_written_the_way_a_person_reads_it() {
    let (owner, name) = A_WELL_FORMED_REFERENCE;

    let written = Repository::new(owner, name).expect("a well-formed pair names a repository");

    assert_eq!(
        written.to_string(),
        "github/rails",
        "the two halves written as one address and no `.git`, because this string is what a \
         person reads and a person does not want the suffix `git` needs"
    );
}

#[test]
fn a_refused_half_comes_back_exactly_as_it_was_written() {
    let refusal = the_refusal_for("github", "   ");

    assert_eq!(
        refusal.what_was_looked_for(),
        "   ",
        "the half is refused and the half is what was written: a value this system silently \
         tidies is a value a lookup will miss"
    );
}

#[test]
fn a_half_carrying_a_slash_is_refused_with_the_same_type_and_another_reason() {
    let from_a_blank_half = the_refusal_for("github", "   ").to_string();
    let from_a_slash = the_refusal_for("github/rails", "rails").to_string();

    assert_ne!(
        from_a_blank_half, from_a_slash,
        "one type covers both halves, and one type is not one sentence: a blank half needs a \
         different answer from a half that is two halves in one, and a reader debugging a run \
         reads the written refusal rather than the type of it"
    );
    assert!(
        from_a_slash.contains("github/rails"),
        "the written refusal quotes the half back, and the half is what a caller has to change: \
         {from_a_slash}"
    );
}
