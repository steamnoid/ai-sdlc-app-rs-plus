//! What a discovery agent concluded, in the fields the specification lists.
//!
//! **Eight fields, and they are the specification's** — transcribed from the glossary of the
//! mother project, **because a name a system chose for itself would be a name nothing could be held
//! to.**
//!
//! **A report that says nothing at all is a fault and not an artifact.** Silence that reached a
//! human at a gate would carry the authority of a finished analysis, **and a person cannot tell an
//! empty report from a real one by looking at it.**
//!
//! **An evidence address is derived and not asked for.** The model contributes the path and the
//! address follows from the repository the run is about — **an address a model may invent is worse
//! than none, and one it may leave out is an address nobody can rely on clicking.**

/// One file of the repository, and the address a reviewer opens to see it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceFile {
    /// Where it is, inside the repository.
    pub path: String,
    /// Where a reviewer opens it, **derived and never asked for.**
    pub link: String,
}

/// A report that said nothing at all about the repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADiscoveryReportThatSaysNothing;

/// A path that is not a path inside the repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnEvidenceOutsideTheRepository(pub String);

/// What a discovery agent concluded about a repository, and nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhatDiscoveryConcluded {
    /// What languages are in it.
    pub languages: Vec<String>,
    /// What frameworks are in it.
    pub frameworks: Vec<String>,
    /// What builds it, and `None` when it does not say.
    pub build_system: Option<String>,
    /// What checks it, and `None` when it does not say — **because a repository with no CI has
    /// nothing to say about CI and must be able to say so.**
    pub ci_system: Option<String>,
    /// How it is put together, in words.
    pub architecture_summary: String,
    /// What it is made of.
    pub key_components: Vec<String>,
    /// Which files it concluded that from.
    pub evidence_files: Vec<EvidenceFile>,
    /// What it would change, and it may say nothing here either.
    pub suggested_changes: Vec<String>,
}

impl WhatDiscoveryConcluded {
    /// A report with nothing in it, which is what an agent that learned nothing produces.
    pub fn everything_empty() -> Self {
        WhatDiscoveryConcluded {
            languages: Vec::new(),
            frameworks: Vec::new(),
            build_system: None,
            ci_system: None,
            architecture_summary: String::new(),
            key_components: Vec::new(),
            evidence_files: Vec::new(),
            suggested_changes: Vec::new(),
        }
    }

    /// One file of the repository, with the address a reviewer opens.
    ///
    /// **One and not many, because an offer checks each path against a listing separately** — and a
    /// batch helper next to a single-path one would be two ways to build the same thing.
    pub fn one_file(
        of_which_repository: &aisdlc_domain::repository::Repository,
        the_path: &str,
    ) -> Result<EvidenceFile, AnEvidenceOutsideTheRepository> {
        evidence_for(of_which_repository, the_path)
    }

    /// Whether the gate may be shown this at all.
    ///
    /// **At least one field and not all of them**, because a rule demanding every field would only
    /// ever be satisfied by a report that guesses.
    pub fn what_the_gate_may_be_shown(
        what_was_concluded: &WhatDiscoveryConcluded,
    ) -> Result<(), ADiscoveryReportThatSaysNothing> {
        let it_says_something = !what_was_concluded.languages.is_empty()
            || !what_was_concluded.frameworks.is_empty()
            || what_was_concluded.build_system.is_some()
            || what_was_concluded.ci_system.is_some()
            || !what_was_concluded.architecture_summary.trim().is_empty()
            || !what_was_concluded.key_components.is_empty()
            || !what_was_concluded.evidence_files.is_empty()
            || !what_was_concluded.suggested_changes.is_empty();
        if it_says_something {
            return Ok(());
        }
        Err(ADiscoveryReportThatSaysNothing)
    }

    /// What it concluded from, each with the address a reviewer opens.
    ///
    /// **Every path is checked before any address is built**, so a set containing one bad path
    /// produces no evidence at all rather than a report that is partly clickable.
    pub fn what_it_concluded_from(
        &self,
        of_which_repository: &aisdlc_domain::repository::Repository,
        the_paths: &[&str],
    ) -> Result<Vec<EvidenceFile>, AnEvidenceOutsideTheRepository> {
        the_paths
            .iter()
            .map(|one_path| evidence_for(of_which_repository, one_path))
            .collect()
    }
}

/// One path inside a repository, with the address a reviewer opens to see it.
fn evidence_for(
    of_which_repository: &aisdlc_domain::repository::Repository,
    the_path: &str,
) -> Result<EvidenceFile, AnEvidenceOutsideTheRepository> {
    if !is_a_path_inside_a_repository(the_path) {
        return Err(AnEvidenceOutsideTheRepository(the_path.to_string()));
    }
    Ok(EvidenceFile {
        path: the_path.to_string(),
        link: format!(
            "https://github.com/{}/{}/blob/main/{the_path}",
            of_which_repository.owner, of_which_repository.name
        ),
    })
}

/// Whether this string is a path in a repository, rather than a way out of one.
fn is_a_path_inside_a_repository(the_path: &str) -> bool {
    if the_path.trim().is_empty() || the_path.contains("://") || the_path.starts_with('/') {
        return false;
    }
    !the_path.split('/').any(|one_half| one_half == "..")
}
