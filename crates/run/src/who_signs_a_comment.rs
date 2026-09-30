//! The name on a comment this system wrote, and why it is not the name of a decision.
//!
//! **A comment is not a decision, so a person did not decide it and did not write it.** The name on
//! a comment is a record of who wrote it, and this system wrote it — **so the name is a fact about
//! the deployment and a caller may not choose it.** A type and not a `WhoDecided` reused, because a
//! type that both a person's ruling and an automated comment can be built from is a type that
//! cannot tell a person from this system, and that is the whole question.

/// A deployment named itself, or named nobody.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ADeploymentThatNamedNobody(pub String);

/// The name a deployment writes its own comments under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhoSignsAComment(String);

impl WhoSignsAComment {
    /// The name a deployment used, or the empty string it used.
    ///
    /// **A deployment that named nobody is refused, and not given a default.** A comment with
    /// nobody on it is a comment nobody can be asked about, and an invented name is the forgery
    /// this type exists to remove — so the refusal is here, before anything is written.
    pub fn the_name_a_deployment_used(
        what_it_named: &str,
    ) -> Result<Self, ADeploymentThatNamedNobody> {
        if what_it_named.trim().is_empty() {
            return Err(ADeploymentThatNamedNobody(what_it_named.to_string()));
        }
        Ok(WhoSignsAComment(what_it_named.trim().to_string()))
    }

    /// The name, as GitHub will carry it.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for WhoSignsAComment {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}
