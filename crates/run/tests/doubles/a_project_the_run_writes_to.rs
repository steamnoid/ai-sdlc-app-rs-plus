//! A project a run writes real files into.
//!
//! **One double per file, and not one file of doubles**, because `mod` compiles a copy into every
//! test binary that names it — **so a file holding four doubles is compiled four times and the three
//! a given binary does not use are reported dead**, and the only ways to stop that are an
//! `#[allow(dead_code)]`, which a gate in this repository refuses, or a copy.
//!
//! **It writes real files, and that is the whole of what it is for.** A double that kept its
//! artifacts in memory would make every reading test pass while nothing a person could open existed.

use aisdlc_run::the_path_inside_the_project::ThePathInsideTheProject;
use aisdlc_run::the_project_on_disk::{TheProjectOnDisk, TheProjectRefusedToWrite};

/// A project that writes a real file, **in a directory of its own.**
///
/// **The directory is named per caller**, because several tests share one process and would
/// otherwise find each other's artifacts.
pub struct AProjectThatWrites {
    where_it_lives: std::path::PathBuf,
}

impl AProjectThatWrites {
    /// A project that writes everything, **in a directory of its own.**
    pub fn writing_everything(in_a_directory_named: &str, a_work_item_named: &str) -> Self {
        let where_the_run_writes =
            where_the_projects_root_of_this_process_are(in_a_directory_named);
        let _ = std::fs::remove_dir_all(&where_the_run_writes);
        let the_work_items_directory = where_the_run_writes
            .join("steamnoid/wellbeing-tracker-public")
            .join(a_work_item_named);
        std::fs::create_dir_all(&the_work_items_directory)
            .expect("a directory this machine will hold");
        Self {
            where_it_lives: where_the_run_writes,
        }
    }
}

/// Where a run's projects live, **and it is named for one caller so two tests cannot collide.**
pub fn where_the_projects_root_of_this_process_are(
    in_a_directory_named: &str,
) -> std::path::PathBuf {
    let where_it_is = std::env::temp_dir().join(format!(
        "aisdlc-{}-{in_a_directory_named}",
        std::process::id()
    ));
    std::fs::create_dir_all(&where_it_is).expect("a directory this machine will hold");
    where_it_is
}

impl TheProjectOnDisk for AProjectThatWrites {
    fn where_it_lives(&self) -> &std::path::Path {
        self.where_it_lives.as_path()
    }

    fn what_is_in_it(&mut self) -> Vec<String> {
        Vec::new()
    }

    fn the_text_of(&mut self, _a_path: &ThePathInsideTheProject) -> Option<String> {
        None
    }

    fn a_file_is_written(
        &mut self,
        a_path: &ThePathInsideTheProject,
        what_it_says: &str,
    ) -> Result<(), TheProjectRefusedToWrite> {
        let the_path = std::path::PathBuf::from(a_path.as_str());
        std::fs::write(&the_path, what_it_says).expect("a file this machine will hold");
        Ok(())
    }
}
