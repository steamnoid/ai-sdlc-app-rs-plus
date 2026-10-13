//! What a creative run files from what it read, **joining the copy to the filings.**
//!
//! **Cycle 136 files what the copy saw.** The run reads its own checkout and the run files what
//! a model offers — **and nothing joined the two**, so the offers a run filed were grounded in a
//! listing somebody else showed it, or in none at all.
//!
//! **The listing filed about is the listing read off the copy**, because an offer grounded in
//! another directory's names files issues about files nobody holds.

use aisdlc_run::what_a_creative_run_reads::what_a_creative_run_reads_from_its_own_checkout;

#[test]
fn a_creative_run_files_one_filing_for_what_it_read() {
    let mut the_deployment = ADeploymentAndItsCheckout::one_ready_to_be_asked();
    let what_it_read = the_deployment.what_it_reads();

    let what_it_will_work_on = the_deployment.what_it_files(
        &what_it_read,
        aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers::Yes,
    );

    let works = what_it_will_work_on.expect("the flag is on and the filing was made");
    assert_eq!(
        (
            works.len(),
            the_deployment.where_it_files.what_it_was_asked.len(),
            *the_deployment.the_model.was_it_asked.borrow()
        ),
        (1, 1, true),
        "**because one offer grounded in what was read is one filing and one filing is one work** \
         — and the model was asked once, which is the fact this test is really about"
    );
}

#[test]
fn the_work_answers_the_filing_read_back_and_not_what_the_model_wrote() {
    let mut the_deployment = ADeploymentAndItsCheckout::one_ready_to_be_asked();
    let what_it_read = the_deployment.what_it_reads();

    let works = the_deployment
        .what_it_files(
            &what_it_read,
            aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers::Yes,
        )
        .expect("the flag is on and the filing was made");

    let what_the_work_answers = works[0].what_it_answers();
    let where_it_was_filed = what_the_work_answers
        .where_it_was_filed()
        .expect("a filing was filed somewhere")
        .as_str();
    assert_eq!(
        (what_the_work_answers.what_was_asked(), where_it_was_filed),
        (
            "Stop E2E runs leaving WAL state",
            "https://github.com/steamnoid/wellbeing-tracker-public/issues/11"
        ),
        "**because the filing was read back** — and what came back is a *different title* from \
         the one the model wrote, **which is the whole point**: a run that remembered its own \
         words would put the model's title in the work"
    );
}

#[test]
fn a_deployment_that_did_not_ask_files_nothing_and_asks_no_model() {
    let mut the_deployment = ADeploymentAndItsCheckout::one_ready_to_be_asked();
    let what_it_read = the_deployment.what_it_reads();

    let how_it_went = the_deployment.what_it_files(
        &what_it_read,
        aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers::No,
    );

    assert!(
        how_it_went.is_err()
            && !*the_deployment.the_model.was_it_asked.borrow()
            && the_deployment.where_it_files.what_it_was_asked.is_empty(),
        "**because the flag stands before the model and before GitHub** — a model asked costs \
         money and a filing made lands in a real repository"
    );
}

#[test]
fn an_agent_with_nothing_worth_filing_files_nothing() {
    let mut the_deployment = ADeploymentAndItsCheckout::one_answering_with_nothing();
    let what_it_read = the_deployment.what_it_reads();

    let works = the_deployment
        .what_it_files(
            &what_it_read,
            aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers::Yes,
        )
        .expect("no offers at all is not a fault");

    assert!(
        works.is_empty() && the_deployment.where_it_files.what_it_was_asked.is_empty(),
        "**because an agent that concluded there is nothing worth filing told the truth and \
         files nothing** — and an empty offer list is not a run that was never asked"
    );
}

/// The request to clone into a directory, **for the creative run's own checkout.**
fn the_request_to_clone_into(
    where_it_lands: &std::path::Path,
) -> aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo {
    let the_clone_of = aisdlc_run::the_repository_to_clone_into::ARepositoryToClone::of(
        aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
            .expect("two halves"),
        aisdlc_domain::work_item_id::WorkItemId::try_from("CREATIVE-1").expect("a name"),
        where_it_lands.to_path_buf(),
    );
    aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo::clone_the_repository(
        the_clone_of,
    )
}

/// A destination nothing stands at, **of this test's own and no other.**
fn the_destination_nothing_stands_at(a_name: &str) -> std::path::PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-creative-files-{}-{:?}-{a_name}",
        std::process::id(),
        std::thread::current().id()
    ));
    let _ = std::fs::remove_dir_all(&where_it_is);
    where_it_is
}

/// A project holding a listing at a place, **and answering about the real disk.**
struct AProjectHoldingAListing {
    where_it_lives: std::path::PathBuf,
    what_is_in_it: Vec<String>,
}

impl AProjectHoldingAListing {
    /// A project holding the one file the model may cite.
    fn holding_readme(where_it_lives: std::path::PathBuf) -> Self {
        Self {
            where_it_lives,
            what_is_in_it: vec![String::from("README.md")],
        }
    }
}

impl aisdlc_run::TheProjectOnDisk for AProjectHoldingAListing {
    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        self.what_is_in_it.clone()
    }

    fn the_text_of(&mut self, _a_path: &aisdlc_run::ThePathInsideTheProject) -> Option<String> {
        None
    }

    fn a_file_is_written(
        &mut self,
        a_path: &aisdlc_run::ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite> {
        Err(
            aisdlc_run::the_project_on_disk::TheProjectRefusedToWrite::there_was_no_room_for_it(
                a_path,
                what_it_says,
            ),
        )
    }

    fn does_anything_stand_at(&self, a_path: &std::path::Path) -> bool {
        a_path.exists()
    }
}

/// A program that copies, **and answers what the copy stands on.**
struct AProgramThatCopies;

impl aisdlc_run::a_program_we_run::AProgramWeRun for AProgramThatCopies {
    fn run(
        &self,
        what_it_is_asked_to_do: &aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo,
    ) -> aisdlc_run::a_program_we_run::WhatTheProgramSaid {
        use aisdlc_run::a_program_we_run::WhatTheProgramSaid as Said;
        use aisdlc_run::what_we_tell_a_program_to_do::WhatWeTellAProgramToDo as Asked;
        match what_it_is_asked_to_do {
            Asked::CloneTheRepository { .. } => Said::of(true, ""),
            Asked::TheCommitItStandsOn { .. } => Said::of(true, THE_COMMIT),
            Asked::TheRefItStandsOn { .. } => Said::of(true, THE_REF),
            Asked::NameWhereTheCodeWasWrittenIn { .. } => Said::of(true, THE_ADDRESS),
            _ => Said::of(false, "not a question about a clone"),
        }
    }
}

/// The commit the copy stands on, as git wrote it.
const THE_COMMIT: &str = "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8";

/// The ref the copy stands on.
const THE_REF: &str = "main";

/// The address the code was written in, which may only be read.
const THE_ADDRESS: &str = "https://github.com/steamnoid/wellbeing-tracker-public";

/// One offer about the listing, **and the three judgements the model was asked for.**
fn one_offer_the_model_would_write() -> String {
    String::from(
        r#"{"offers":[{"title":"the listing: why the readme says nothing about the sidecar","what_it_asks_for":"say what the sqlite sidecar files are for","paths":["README.md"]}],"what_it_found":"the readme says nothing about the sidecar","what_it_could_not_settle":"whether the sidecar is still written","what_it_expects_next":"file the question"}"#,
    )
}

/// No offers at all, **and the judgements that say so in words.**
fn nothing_the_model_would_write() -> String {
    String::from(
        r#"{"offers":[],"what_it_found":"nothing in the listing is worth filing","what_it_could_not_settle":"whether an empty listing would change that","what_it_expects_next":"file nothing"}"#,
    )
}

/// What GitHub answered for one filing at one address, **with a title the model did not write.**
fn what_github_answered_at(its_address: &str, its_number: i64) -> String {
    format!(
        "{{\"number\":{its_number},\"title\":\"Stop E2E runs leaving WAL state\",\"body\":\
         \"the ask\",\"html_url\":\"{its_address}\",\"state\":\"open\"}}"
    )
}

/// **What GitHub holds, shared by two programs** — because `gh issue create` and `gh api` are two
/// programs against one place, **and a run that filed and reads back goes to the same place.**
#[derive(Clone, Default)]
struct WhatGitHubHolds(std::rc::Rc<std::cell::RefCell<String>>);

/// Somewhere that files and hands back a real address, **which is `gh issue create`.**
struct AGitHubThatFiles {
    what_it_was_asked: Vec<aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor>,
    what_github_holds: WhatGitHubHolds,
}

/// Somewhere that reads a filing back, **which is `gh api` and a different program.**
struct AGitHubThatReadsBack {
    what_github_holds: WhatGitHubHolds,
}

impl aisdlc_run::a_filing_is_read::AFilingIsReadFrom for AGitHubThatReadsBack {
    fn the_filing_at(
        &mut self,
        _where_it_answers: &aisdlc_domain::commissioned_request::AWhereItWasFiled,
    ) -> Result<aisdlc_run::an_issue_read::AnIssueRead, String> {
        aisdlc_run::an_issue_read::AnIssueRead::the_issue_read(&self.what_github_holds.0.borrow())
            .map_err(|why_not| why_not.to_string())
    }

    /// Nothing on file, **because this test is about what the run does with an answer.**
    fn the_filings_already_on_file_of(
        &mut self,
        _of_which_repository: &aisdlc_domain::repository::Repository,
    ) -> Result<aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile, String> {
        Ok(aisdlc_run::the_filings_already_on_file::TheFilingsAlreadyOnFile::none_yet())
    }
}

impl aisdlc_run::a_git_hub_we_write_to::AGitHubWeWriteTo for AGitHubThatFiles {
    fn it_is_asked_to(
        &mut self,
        what_it_is_asked_for: &aisdlc_run::a_git_hub_we_write_to::WhatGitHubWasAskedFor,
    ) -> Result<aisdlc_run::a_git_hub_we_write_to::WhereGitHubAnswered, String> {
        self.what_it_was_asked.push(what_it_is_asked_for.clone());
        let its_number = 10 + i64::try_from(self.what_it_was_asked.len()).expect("a few");
        let where_it_went =
            format!("https://github.com/steamnoid/wellbeing-tracker-public/issues/{its_number}");
        *self.what_github_holds.0.borrow_mut() =
            what_github_answered_at(&where_it_went, its_number);
        Ok(aisdlc_run::a_git_hub_we_write_to::WhereGitHubAnswered::an_address_at(&where_it_went))
    }
}

/// A model that says what it was told to say **and remembers whether it was asked at all.**
struct AModelThatWasMaybeAsked {
    what_it_said: String,
    was_it_asked: std::cell::RefCell<bool>,
}

impl aisdlc_run::AReasoningModel for AModelThatWasMaybeAsked {
    fn what_it_says_when_asked_for(
        &mut self,
        _which_artifact: &aisdlc_domain::approval::WhichArtifact,
        _which_organisation: aisdlc_domain::which_organisation_reasons::WhichOrganisationReasons,
        _the_question: &str,
    ) -> Result<String, aisdlc_run::the_words_inside_the_endpoints_envelope::AnAnswerNobodyCanRead>
    {
        *self.was_it_asked.borrow_mut() = true;
        Ok(self.what_it_said.clone())
    }
}

/// A deployment, its checkout and its GitHub, **everything one creative filing needs.**
struct ADeploymentAndItsCheckout {
    the_program: AProgramThatCopies,
    the_model: AModelThatWasMaybeAsked,
    where_it_files: AGitHubThatFiles,
    where_it_reads_back: AGitHubThatReadsBack,
}

impl ADeploymentAndItsCheckout {
    /// A deployment whose model would offer one grounded piece of work.
    fn one_ready_to_be_asked() -> Self {
        Self::one_answering(one_offer_the_model_would_write())
    }

    /// A deployment whose model answers with nothing worth filing.
    fn one_answering_with_nothing() -> Self {
        Self::one_answering(nothing_the_model_would_write())
    }

    /// A deployment whose model says what it was told to say.
    fn one_answering(what_the_model_says: String) -> Self {
        let what_github_holds = WhatGitHubHolds::default();
        Self {
            the_program: AProgramThatCopies,
            the_model: AModelThatWasMaybeAsked {
                what_it_said: what_the_model_says,
                was_it_asked: std::cell::RefCell::new(false),
            },
            where_it_files: AGitHubThatFiles {
                what_it_was_asked: Vec::new(),
                what_github_holds: what_github_holds.clone(),
            },
            where_it_reads_back: AGitHubThatReadsBack { what_github_holds },
        }
    }

    /// What this deployment read from its own checkout.
    fn what_it_reads(&mut self) -> aisdlc_run::what_a_creative_run_reads::WhatACreativeRunRead {
        let where_it_lands = the_destination_nothing_stands_at("files-what-it-read");
        let mut the_project = AProjectHoldingAListing::holding_readme(where_it_lands.clone());
        what_a_creative_run_reads_from_its_own_checkout(
            &mut self.the_program,
            &mut the_project,
            &the_request_to_clone_into(&where_it_lands),
        )
        .expect("nothing stood at the destination and git answered")
    }

    /// What this deployment files from what it read.
    fn what_it_files(
        &mut self,
        what_it_read: &aisdlc_run::what_a_creative_run_reads::WhatACreativeRunRead,
        flag: aisdlc_run::whether_an_agent_offers::WhetherAnAgentOffers,
    ) -> Result<
        Vec<aisdlc_run::the_work_each_filing_becomes::WhatTheRunWillWorkOn>,
        aisdlc_run::a_creative_run_files_what_it_would_file::AFlagThatIsOff,
    > {
        aisdlc_run::a_creative_run_files_what_it_read::a_creative_run_files_what_it_read(
            what_it_read,
            &mut self.the_model,
            &mut self.where_it_files,
            &mut self.where_it_reads_back,
            &aisdlc_domain::repository::Repository::try_from("steamnoid/wellbeing-tracker-public")
                .expect("two halves"),
            flag,
        )
    }
}
