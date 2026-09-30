//! A comment, really left on GitHub, by `gh`, against a real account.
//!
//! **Two halves, and the cheap one is first.** A comment on a filing that does not exist proves the
//! adapter runs, that `gh` accepts what it was handed, and — the half that matters — **that a
//! failure comes back as a failure.** A caller told a comment was written when it was not is a
//! reader of a record that does not exist, and this is the only tier that can see that.
//!
//! **The success half is somebody else's decision.** A real comment needs a real filing, and this
//! repository has no issues at all, so the address is read from `E2E_FILING` and the test refuses
//! to run without it rather than picking an address for itself. **A proof that comments on a
//! repository this system has no business commenting on is not a proof, it is a side effect.**

use aisdlc_domain::commissioned_request::AWhereItWasFiled;
use aisdlc_outside::a_comment_reaching_github::ACommentReachingGitHub;
use aisdlc_outside::the_environment::TheEnvironment;
use aisdlc_outside::who_signed_this_deployments_comments::who_signed_these_comments;
use aisdlc_run::a_comment_is_left::the_run_leaves_a_comment;
use aisdlc_run::a_git_hub_we_write_to::AGitHubWeWriteTo;
use aisdlc_run::who_signs_a_comment::WhoSignsAComment;

/// A request filed at the address given, which is somebody else's decision and not ours.
fn a_request_filed_at(
    the_address: &str,
) -> aisdlc_domain::commissioned_request::CommissionedRequest {
    aisdlc_domain::commissioned_request::CommissionedRequest::asked_for_and_filed_at(
        "a run was exercised on this".to_string(),
        "this filing exists so a real comment can be left on it".to_string(),
        AWhereItWasFiled::the_address_the_filing_returned(the_address)
            .expect("an address GitHub returned is an address"),
    )
    .expect("a request somebody filed is a request")
}

#[test]
#[ignore = "writes to GitHub for real, and needs gh authenticated"]
fn a_comment_on_a_filing_that_does_not_exist_is_a_refusal_and_not_a_quiet_success() {
    let who_signed =
        WhoSignsAComment::the_name_a_deployment_used("aisdlc-dev").expect("a name is a name");
    let an_address_no_filing_has =
        "https://github.com/steamnoid/ai-sdlc-app-rs-plus/issues/99999999";
    let mut the_adapter = ACommentReachingGitHub;

    let how_it_went = the_run_leaves_a_comment(
        &mut the_adapter,
        &a_request_filed_at(an_address_no_filing_has),
        "this run was exercised and expected a refusal",
        &who_signed,
    );

    assert!(
        how_it_went.is_err(),
        "**because a caller told a comment was written when it was not is a reader of a record that \
         does not exist** — and GitHub accepting a comment on an issue that is not there would be \
         the far end lying to us: {how_it_went:?}"
    );
    eprintln!("gh refused, and said: {how_it_went:?}");
}

#[test]
#[ignore = "writes to GitHub for real, and needs E2E_FILING to name a filing that exists"]
fn a_comment_on_a_filing_that_exists_is_left_under_the_names_a_deployment_chose() {
    let Ok(the_address) = std::env::var("E2E_FILING") else {
        eprintln!(
            "SKIPPED: E2E_FILING is not set, so no real filing was named and this system did not \
             choose one for itself — a proof that comments on a repository this system has no \
             business commenting on is not a proof, it is a side effect"
        );
        return;
    };
    let what_the_process_was_told = TheEnvironment::of(vec![(
        "AISDLC_WHO_SIGNS_A_COMMENT".to_string(),
        "aisdlc-dev".to_string(),
    )]);
    let who_signed = who_signed_these_comments(&what_the_process_was_told).expect("a name");
    let mut the_adapter = ACommentReachingGitHub;

    let how_it_went = the_run_leaves_a_comment(
        &mut the_adapter,
        &a_request_filed_at(&the_address),
        "This comment was left by `aisdlc-dev`, a run of this system, to prove the path from a \
         filed request to a comment on GitHub works against the real thing rather than a double. \
         No work was requested and none is claimed.",
        &who_signed,
    );

    eprintln!("left a comment on {the_address}: {how_it_went:?}");
    assert!(
        how_it_went.is_ok(),
        "**because the whole point of this tier is that the thing is real** — and a run that cannot \
         leave one comment has not proved anything about commenting: {how_it_went:?}"
    );
}
