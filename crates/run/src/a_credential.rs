//! A credential a deployment supplied, which prints as a credential and never as itself.
//!
//! **A credential that prints itself is a credential one `{}` away from a log.** Every way of
//! turning this value into text prints that there is a credential and not what it is, so the
//! mistake that leaks it produces a sentence with nothing in it — **which is the only safe shape
//! for a mistake to have.**

use std::fmt;

/// A key a deployment supplied, which cannot be read by printing it.
#[derive(Clone, PartialEq, Eq)]
pub struct ACredential(String);

impl ACredential {
    /// The key a deployment supplied, or a refusal when there is none.
    pub fn the_key_a_deployment_supplied(the_key: &str) -> Result<Self, ACredentialThatIsNothing> {
        if the_key.trim().is_empty() {
            return Err(ACredentialThatIsNothing(the_key.to_string()));
        }
        Ok(ACredential(the_key.to_string()))
    }

    /// The key, for the one caller that has to put it into a request.
    ///
    /// **The only door, and it exists because a program that cannot read its credential cannot
    /// authenticate with it** — and the alternative, a credential that hands its own text to
    /// everything that touches it, is why this type carries no `Debug` in the first place.
    /// `Display` stays `a credential`, so a `Display` anywhere in this system still cannot leak it.
    pub fn the_key_to_authenticate_with(&self) -> &str {
        &self.0
    }
}

/// **Deliberately not derived, and the compiler is what holds that.** A derived `Debug` is one
/// `{:?}` away from a log and a derived `Display` is one `{}` away from an error message, and
/// **adding either was tried twice in one day and the suite said nothing either time** — because
/// no test can hold a fact about a type, only code that would stop compiling can. So the type
/// carries no `Debug` at all, the one `Display` below is hand-written, and the reason is written
/// down where whoever adds the derive is already reading.
impl fmt::Display for ACredential {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("a credential")
    }
}

/// A key that is blank, so the far end refuses the request and this system reads the refusal.
#[derive(Clone, PartialEq, Eq)]
pub struct ACredentialThatIsNothing(pub String);

impl fmt::Debug for ACredentialThatIsNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "a credential that is nothing")
    }
}

impl fmt::Display for ACredentialThatIsNothing {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} is not a credential", self.0)
    }
}

impl std::error::Error for ACredentialThatIsNothing {}
