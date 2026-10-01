//! Reading a filing, which is the other direction of the same boundary.
//!
//! **Writing and reading GitHub are two ports and not one**, because writing always needs a
//! credential and reading sometimes does not — **and one port that needed a credential would make
//! every run that only reads a filing ask for a key it does not use.**
//!
//! **Asked by address and not by number**, because a run that filed an issue holds an address and not
//! a number — **and asking for the number again would be this system guessing at a fact GitHub
//! already composed.**

use aisdlc_domain::commissioned_request::AWhereItWasFiled;

use crate::an_issue_read::AnIssueRead;

/// Reading a filing, and nothing else.
pub trait AFilingIsReadFrom {
    /// The filing at one address, **or the reason there was not one to read.**
    fn the_filing_at(&mut self, where_it_answers: &AWhereItWasFiled)
    -> Result<AnIssueRead, String>;

    /// Every filing already open on one repository, **or the reason there were none to read.**
    ///
    /// **This port and not a second one**, because both are *reading filings from GitHub*, **and a
    /// second `gh` seam is a second place for GitHub to have changed its output without us noticing.**
    ///
    /// **A run asks a model what to file, so it must know what is already filed** — otherwise the
    /// model can only ever find things for the first time, **and five real runs of it filed one
    /// SQLite observation three times because the tracker was never opened.**
    fn the_filings_already_on_file_of(
        &mut self,
        of_which_repository: &aisdlc_domain::repository::Repository,
    ) -> Result<crate::the_filings_already_on_file::TheFilingsAlreadyOnFile, String>;
}

/// The filing read back at the address GitHub gave for it, **in the words GitHub has and not the
/// ones the run remembers writing.**
///
/// **A refusal and not a filing**, because a work item built from what a run thinks it wrote carries
/// text GitHub never had, **and the two part company the moment a person edits one word.**
pub fn the_filing_a_filing_was_read_back(
    somewhere: &mut impl AFilingIsReadFrom,
    where_it_answers: &str,
) -> Result<AnIssueRead, String> {
    let the_address = AWhereItWasFiled::the_address_the_filing_returned(where_it_answers)
        .map_err(|named_nothing| named_nothing.to_string())?;
    somewhere.the_filing_at(&the_address)
}
