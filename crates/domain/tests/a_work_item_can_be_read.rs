//! A work item can say which one it is.
//!
//! **Every field of `WorkItem` is `pub(crate)` and there is an accessor for the stage, the
//! agent, the request and who asked — and none for the name the work item is tracked by.** So a
//! gate that writes `Approval { work_item_id, … }` had nothing to put there except a name it
//! had invented, **and a record whose subject is supplied by its writer is a record about a
//! guess.**
//!
//! **A reader and not a writer**, which is why this is a method and not a public field: asking
//! which work item it is is something anything may do, and changing the name is something only
//! the state machine does.

use aisdlc_domain::commissioned_request::CommissionedRequest;
use aisdlc_domain::repository::Repository;
use aisdlc_domain::role::Role;
use aisdlc_domain::stage::Stage;
use aisdlc_domain::who_commissioned::WhoCommissioned;
use aisdlc_domain::work_item::a_work_item_commissioned_for_an_agent;
use aisdlc_domain::work_item_id::WorkItemId;

fn a_work_item() -> aisdlc_domain::work_item::WorkItem {
    a_work_item_commissioned_for_an_agent(
        WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
        WhoCommissioned::try_from("paligakrzychu").expect("a name is not blank"),
        Repository::try_from("steamnoid/ai-sdlc-app-rs-plus").expect("two halves"),
        Role::Product,
        Some(
            CommissionedRequest::asked_for(
                "Name the stages".to_string(),
                "A reader should not have to read the code.".to_string(),
            )
            .expect("a request with a title and a body is a request"),
        ),
    )
}

#[test]
fn a_work_item_says_which_one_it_is() {
    let the_work_item = a_work_item();

    assert_eq!(
        the_work_item.the_work_item_id(),
        WorkItemId::try_from("WELLBEINGT-5").expect("a name is not blank"),
        "the name it is tracked by, read from the work item itself and not supplied by whoever is writing a record about it"
    );
}

#[test]
fn a_stage_says_what_it_is_called_without_a_second_place_writing_the_names() {
    assert_eq!(
        Stage::AwaitingHumanApproval.to_string(),
        "AWAITING_HUMAN_APPROVAL",
        "**`Stage` already knows how to name itself, through `Display`**, and this file once added a second method doing the same thing — six names written a second time in the one file `the_specification_and_the_code_have_not_drifted_apart` reads the names from. **The gate that catches a name written twice caught me, and the gate about one implementation is not even looking at this crate.**"
    );
}
