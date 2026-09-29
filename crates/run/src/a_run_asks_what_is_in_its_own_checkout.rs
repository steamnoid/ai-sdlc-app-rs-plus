//! A run that has made a checkout asks what is in it, and only ever about that checkout.
//!
//! **A run makes a checkout and nothing has ever read one**, so two facts that must agree were
//! never asked to agree: the checkout knows where it landed and the project knew nothing.
//!
//! **The two places are compared before the project is asked anything.** A listing is a
//! listing — every name in it is a real name of some project — so a run that listed a project
//! rooted somewhere else would hand an agent the wrong repository and the agent could not tell.

use aisdlc_domain::a_clone::ACloneOnDisk;

use crate::the_project_on_disk::TheProjectOnDisk;

/// What a run asked for, and where the project it asked about is rooted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotTheRunItsOwnCheckout {
    where_the_checkout_lands: String,
    where_the_project_lives: String,
}

impl NotTheRunItsOwnCheckout {
    /// Why the project was refused, naming both places.
    ///
    /// **Both places, and not the one that looked wrong.** A refusal naming one of two facts
    /// this system holds leaves the reader to guess which was wrong, and the run cannot tell
    /// either — it was handed both.
    pub fn because(&self) -> String {
        format!(
            "the run cloned into {} and was handed a project living at {}",
            self.where_the_checkout_lands, self.where_the_project_lives
        )
    }
}

/// The two places, refused when they differ, and nothing when they agree.
fn the_places_the_run_refuses(
    the_checkout: &ACloneOnDisk,
    the_project: &impl TheProjectOnDisk,
) -> Option<NotTheRunItsOwnCheckout> {
    if the_project.where_it_lives() == the_checkout.where_it_lands() {
        return None;
    }
    Some(NotTheRunItsOwnCheckout {
        where_the_checkout_lands: the_checkout.where_it_lands().display().to_string(),
        where_the_project_lives: the_project.where_it_lives().display().to_string(),
    })
}

/// What the run's own checkout holds, or a refusal naming the two places that differ.
pub fn the_run_asks_what_is_in_its_own_checkout(
    the_checkout: &ACloneOnDisk,
    the_project: &mut impl TheProjectOnDisk,
) -> Result<Vec<String>, NotTheRunItsOwnCheckout> {
    if let Some(a_refusal) = the_places_the_run_refuses(the_checkout, the_project) {
        return Err(a_refusal);
    }
    Ok(the_project.what_is_in_it())
}
