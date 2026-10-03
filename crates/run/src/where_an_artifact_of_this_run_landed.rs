//! Where one artifact of one run landed, **and that is a fact about the run rather than about the
//! file.**
//!
//! **Cycle 125 made a whole run write each artifact and then threw the path away.** Seven files
//! landed beside the clone and nothing anywhere held that they were there, **so the summary this
//! repository owes a person could not have named one of them** — and a summary that cannot say where
//! its artifacts are is a page about claims.
//!
//! **One entry is one write, and it carries the three things a reader would look for**: which
//! artifact, at which version, and where it landed. **The version is not decoration** — an approval
//! names an exact artifact *and version*, and an entry without it could not be matched to the
//! approval that ruled on it.
//!
//! **The place is a plain path rather than a derived one.** It is what the writer was given and what
//! the file then existed at, **and a second derivation would be a second ordering nobody chose.**

use aisdlc_domain::approval::{AnArtifactVersion, WhichArtifact};

use crate::the_path_inside_the_project::ThePathInsideTheProject;

/// One artifact of one run, **and where it was written.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereAnArtifactOfThisRunLanded {
    /// Which artifact this was, **by the name the gate will be asked about.**
    pub which_artifact: WhichArtifact,
    /// The version it was written at, **and the approval names the same one.**
    pub the_version: AnArtifactVersion,
    /// Where it is on disk, **as the writer was given it and as the file then existed.**
    pub where_it_landed: ThePathInsideTheProject,
}

impl WhereAnArtifactOfThisRunLanded {
    /// One artifact written, **and it is a constructor because the three fields travel together and a
    /// reader should not have to know their order.**
    pub fn an_artifact_landed(
        which_artifact: WhichArtifact,
        the_version: AnArtifactVersion,
        where_it_landed: ThePathInsideTheProject,
    ) -> Self {
        Self {
            which_artifact,
            the_version,
            where_it_landed,
        }
    }
}
