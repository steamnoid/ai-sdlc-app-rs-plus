//! What a creative run files from what it read, joining the copy to the filings.
//!
//! **The listing filed about is the listing read off the copy**, because an offer grounded in
//! another directory's names files issues about files nobody holds. The flag still stands before
//! the model and before GitHub, because the copy cost neither a question nor an issue.

use aisdlc_domain::repository::Repository;

use crate::AReasoningModel;
use crate::a_creative_run_files_what_it_would_file::{
    AFlagThatIsOff, a_creative_run_files_what_it_would_file,
};
use crate::a_filing_is_read::AFilingIsReadFrom;
use crate::a_git_hub_we_write_to::AGitHubWeWriteTo;
use crate::the_work_each_filing_becomes::WhatTheRunWillWorkOn;
use crate::what_a_creative_run_reads::WhatACreativeRunRead;
use crate::whether_an_agent_offers::WhetherAnAgentOffers;

/// What a creative run files from what it read, **or the flag that says it files nothing.**
pub fn a_creative_run_files_what_it_read(
    what_it_read: &WhatACreativeRunRead,
    the_model: &mut impl AReasoningModel,
    somewhere: &mut impl AGitHubWeWriteTo,
    somewhere_else: &mut impl AFilingIsReadFrom,
    of_which_repository: &Repository,
    may_an_agent_offer: WhetherAnAgentOffers,
) -> Result<Vec<WhatTheRunWillWorkOn>, AFlagThatIsOff> {
    a_creative_run_files_what_it_would_file(
        the_model,
        somewhere,
        somewhere_else,
        of_which_repository,
        what_it_read.what_it_saw(),
        may_an_agent_offer,
    )
}
