//! An agent asks what is in the project, and reads a file out of it.
//!
//! **This is the first thing an agent needs and the first thing it did not have.** Every other
//! slice in this repository moved a work item or asked a model a question, and not one of them
//! read a file — so the two path rules were guarding a door nothing had walked through.
//!
//! **A file's text is asked for by a path that already landed inside the project.** The port
//! takes `&ThePathInsideTheProject` and not a `String`, so a caller cannot ask to read outside
//! the project without the type having already refused, and **the refusal happens once, at the
//! door, rather than at every call site that forgot.**

use aisdlc_run::a_path_in_the_project::a_path_in_the_project;
use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_path_inside_the_project::the_path_inside_the_project;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};
use aisdlc_run::the_project_this_run_is_in;

/// A project holding two named files, and a record of what it was asked.
struct AProjectWithTwoFilesInIt {
    where_it_lives: std::path::PathBuf,
    the_files: Vec<(String, String)>,
    was_asked_for: Vec<String>,
}

impl AProjectWithTwoFilesInIt {
    fn holding(these: &[(&str, &str)]) -> Self {
        Self {
            where_it_lives: std::path::PathBuf::from("a-project-holding-two-files"),
            the_files: these
                .iter()
                .map(|(a_name, its_text)| (a_name.to_string(), its_text.to_string()))
                .collect(),
            was_asked_for: Vec::new(),
        }
    }
}

impl TheProjectOnDisk for AProjectWithTwoFilesInIt {
    /// Whether anything stands at a path, **and this double answers about the real disk** — it
    /// exists so a test reads real files, **and a double that answered about its own memory would be
    /// a double that could not be wrong about the disk.**
    fn does_anything_stand_at(&self, a_path: &std::path::Path) -> bool {
        a_path.exists()
    }

    fn where_it_lives(&self) -> &std::path::Path {
        &self.where_it_lives
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        self.the_files
            .iter()
            .map(|(a_name, _)| a_name.clone())
            .collect()
    }

    fn the_text_of(&mut self, a_path: &ThePathInsideTheProject) -> Option<String> {
        self.was_asked_for.push(a_path.as_str().to_string());
        let the_name = a_path.as_str();
        self.the_files
            .iter()
            .find(|(a_name, _)| the_name.ends_with(a_name.as_str()))
            .map(|(_, its_text)| its_text.clone())
    }
    /// Nothing is ever written here, **and the method refuses rather than pretending** — a double
    /// that answered `Ok(())` would let a test pass on a write nothing did, **and these two files
    /// are about reading.**
    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        _what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        Err(TheProjectRefusedToWrite::there_was_no_room_for_it(
            a_path,
            "this project is a double that only ever reads",
        ))
    }
}

/// A path that is inside the checkout, asked for by name.
fn a_path_named(a_name_of_a_real_file: &str) -> ThePathInsideTheProject {
    let a_name = a_path_in_the_project(a_name_of_a_real_file).expect("an ordinary name is a path");
    the_path_inside_the_project(&the_project_this_run_is_in(), &a_name)
        .expect("a file of the checkout is inside the checkout")
}

fn a_path_inside_the_project() -> ThePathInsideTheProject {
    a_path_named("README.md")
}

#[test]
fn an_agent_asks_what_is_in_the_project_and_gets_a_listing_of_names() {
    let mut the_project = AProjectWithTwoFilesInIt::holding(&[
        ("README.md", "# a readme"),
        ("src/lib.rs", "// the library"),
    ]);

    let listing = aisdlc_run::the_agent_asks_what_is_in_the_project(&mut the_project);

    assert_eq!(
        listing,
        vec!["README.md".to_string(), "src/lib.rs".to_string()],
        "**names and not paths**, because an agent reading a repository needs to know what is there before it can ask about any of it"
    );
}

#[test]
fn an_agent_reads_a_file_by_a_path_that_already_landed_inside() {
    let mut the_project = AProjectWithTwoFilesInIt::holding(&[("README.md", "# a readme")]);
    let inside = a_path_inside_the_project();

    let the_text = aisdlc_run::the_agent_reads(&mut the_project, &inside)
        .expect("a file the project holds is there");

    assert_eq!(
        the_text, "# a readme",
        "and the text is what the project said rather than what the agent hoped for"
    );
}

#[test]
fn the_port_is_asked_with_the_resolved_spelling_and_not_the_one_written() {
    let mut the_project = AProjectWithTwoFilesInIt::holding(&[("README.md", "# a readme")]);
    let inside = a_path_inside_the_project();

    aisdlc_run::the_agent_reads(&mut the_project, &inside).expect("the readme is there");

    let asked_for = &the_project.was_asked_for[0];
    assert!(
        !asked_for.contains(".."),
        "**because the type carries the resolved path and the type is what the port is given**, so a port cannot be handed a spelling it would have to resolve to be safe"
    );
}

#[test]
fn an_agent_asking_for_a_file_the_project_does_not_hold_is_refused() {
    let mut the_project = AProjectWithTwoFilesInIt::holding(&[("README.md", "# a readme")]);
    let inside = a_path_named("Cargo.toml");

    let refusal = aisdlc_run::the_agent_reads(&mut the_project, &inside);

    assert!(
        refusal.is_err(),
        "**a file naming nothing is not an empty file** — a caller handed an empty string could not tell an empty file from a missing one, and every answer built on it would be a guess"
    );
}

#[test]
fn a_refusal_says_which_file_was_missing() {
    let mut the_project = AProjectWithTwoFilesInIt::holding(&[("README.md", "# a readme")]);
    let inside = a_path_named("Cargo.toml");

    let refusal =
        aisdlc_run::the_agent_reads(&mut the_project, &inside).expect_err("it is not there");

    assert!(
        refusal.what_was_asked_for().ends_with("Cargo.toml"),
        "**because a refusal that does not say what was asked for leaves the caller to keep a second list of what it has already asked for**"
    );
}
