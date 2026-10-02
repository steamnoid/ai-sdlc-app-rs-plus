//! What a planner concluded, in the five lists the specification gives a plan, and the
//! refusal for a plan that names no file.
//!
//! **Five lists and not one string, because the five are checked differently.** A file to change
//! must exist, a file to be added must not, and a test has to land in a directory the runner
//! already collects — **and a plan that carried them as prose could be checked for none of those
//! without parsing an argument out of a sentence.**
//!
//! **`dependencies` and `migrations` are not files, and that is the rule the refusal turns on.**
//! A plan naming a crate to add and no file to change changes nothing, **and a gate that approved
//! it would be approving a delivery with no change in it.**

/// The five lists, **in the order the specification lists them.**
const THE_LISTS_A_PLAN_IS_MADE_OF: [&str; 5] = [
    "files_to_change",
    "files_to_add",
    "tests_to_add",
    "dependencies",
    "migrations",
];

/// The three lists that name a file, **and the two that do not.**
const THE_LISTS_THAT_NAME_A_FILE: [&str; 3] = ["files_to_change", "files_to_add", "tests_to_add"];

/// What a planner concluded, **and nothing a model may add to it.**
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WhatAPlannerPlanned {
    /// Files this plan changes in place, and each of them must exist.
    pub files_to_change: Vec<String>,
    /// Files this plan brings, and none of them may exist.
    pub files_to_add: Vec<String>,
    /// Tests this plan brings, and each one's directory must be one the runner collects.
    pub tests_to_add: Vec<String>,
    /// What this plan brings with it, **and none of which is a file.**
    pub dependencies: Vec<String>,
    /// What this plan changes about the shape of the data, **and none of which is a file.**
    pub migrations: Vec<String>,
}

/// A plan that named no file, **with the lists that were empty and the words that came back.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct APlanThatSaysNothing {
    which_lists_were_empty: Vec<String>,
    what_came_back: String,
}

impl APlanThatSaysNothing {
    /// The refusal, **naming the lists that were empty.**
    ///
    /// **The lists and not a sentence saying the plan said nothing**, because a gate refusing with
    /// a sentence sends a person looking for a paragraph — **and the five lists are the fact a
    /// person deciding whether to approve needs.**
    pub fn of(what_was_empty: &[&str]) -> Self {
        Self {
            which_lists_were_empty: what_was_empty
                .iter()
                .map(|one| (*one).to_string())
                .collect(),
            what_came_back: String::new(),
        }
    }

    /// The refusal for an answer that was not a plan, **quoted whole.**
    ///
    /// **A model asked for a plan and answering with a sentence is the case this repository has been
    /// burned by three times**, and the words that came back are what tells a person whether to
    /// retry — **so they are quoted and not summarised.**
    pub fn quoting(what_came_back: &str) -> Self {
        Self {
            which_lists_were_empty: THE_LISTS_A_PLAN_IS_MADE_OF
                .iter()
                .map(|one| (*one).to_string())
                .collect(),
            what_came_back: what_came_back.to_string(),
        }
    }

    /// Which lists were empty, **so a reader is told the fact and not the conclusion.**
    pub fn which_lists_were_empty(&self) -> &[String] {
        &self.which_lists_were_empty
    }

    /// What the model said, word for word, **and empty when it said nothing a plan could be read
    /// out of.**
    pub fn what_came_back(&self) -> &str {
        &self.what_came_back
    }
}

impl std::fmt::Display for APlanThatSaysNothing {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what_was_empty = self.which_lists_were_empty.join(", ");
        write!(
            formatter,
            "this plan names no file, and the empty lists were: {what_was_empty}. A gate approving \
             it would be approving a delivery with no change in it"
        )
    }
}

impl std::error::Error for APlanThatSaysNothing {}

/// What a model said is read as a plan, **or the refusal saying which lists were empty.**
///
/// **The list a file is named in, and not the one it is checked in.** `dependencies` and
/// `migrations` are read as faithfully as the rest — **and they do not make a plan a plan**,
/// which is the whole of what the last check is.
pub fn the_plan_a_model_proposed(
    what_the_model_said: &str,
) -> Result<WhatAPlannerPlanned, APlanThatSaysNothing> {
    let what_it_came_back_as: serde_json::Value = serde_json::from_str(what_the_model_said)
        .map_err(|_| APlanThatSaysNothing::quoting(what_the_model_said))?;
    let what_a_planner_concluded = WhatAPlannerPlanned {
        files_to_change: the_named_paths_in(&what_it_came_back_as, "files_to_change"),
        files_to_add: the_named_paths_in(&what_it_came_back_as, "files_to_add"),
        tests_to_add: the_named_paths_in(&what_it_came_back_as, "tests_to_add"),
        dependencies: the_named_paths_in(&what_it_came_back_as, "dependencies"),
        migrations: the_named_paths_in(&what_it_came_back_as, "migrations"),
    };
    if !what_a_planner_concluded.names_a_file() {
        return Err(APlanThatSaysNothing::of(&the_lists_that_were_empty(
            &what_a_planner_concluded,
        )));
    }
    Ok(what_a_planner_concluded)
}

impl WhatAPlannerPlanned {
    /// Every path this plan names, **from the three lists that name files and not from the two
    /// that do not.**
    ///
    /// **The one list an implementer may write from, and it is built here rather than asked for**,
    /// because a plan naming a crate to add is not a file to write — **and a comparison against
    /// the wrong list is a run writing a file a person never approved.**
    pub fn the_paths_it_named(&self) -> Vec<String> {
        let mut every_path = Vec::new();
        for one_list in [
            &self.files_to_change,
            &self.files_to_add,
            &self.tests_to_add,
        ] {
            for one_path in one_list {
                every_path.push(one_path.clone());
            }
        }
        every_path
    }

    /// Whether this plan names at least one file, **and `dependencies` do not count.**
    fn names_a_file(&self) -> bool {
        !self.files_to_change.is_empty()
            || !self.files_to_add.is_empty()
            || !self.tests_to_add.is_empty()
    }
}

/// Which of the three file-lists were empty, **and the two that name no file are not named.**
///
/// **A reader told a crate is missing is sent looking for a file that is not there**, and the two
/// lists that do not name files are not part of the answer to "why is this plan empty".
fn the_lists_that_were_empty(what_a_planner_concluded: &WhatAPlannerPlanned) -> Vec<&'static str> {
    let every_list = [
        (&what_a_planner_concluded.files_to_change, true),
        (&what_a_planner_concluded.files_to_add, true),
        (&what_a_planner_concluded.tests_to_add, true),
        (&what_a_planner_concluded.dependencies, false),
        (&what_a_planner_concluded.migrations, false),
    ];
    let mut found = Vec::new();
    for (the_list, it_names_a_file) in every_list {
        if the_list.is_empty() && it_names_a_file {
            found.push(THE_LISTS_A_PLAN_IS_MADE_OF[found.len()]);
        }
    }
    found
}

/// One list of names, **read as a list and with a name written twice kept once.**
///
/// **A `Set` and not a `Vec` of duplicates** because the same file written twice is one file to
/// change, **and a downstream list of writes carrying one path twice is a run that would write it
/// twice and report two changes.**
fn the_named_paths_in(what_it_came_back_as: &serde_json::Value, which_list: &str) -> Vec<String> {
    let Some(every_name) = what_it_came_back_as
        .get(which_list)
        .and_then(serde_json::Value::as_array)
    else {
        return Vec::new();
    };
    let mut found: Vec<String> = Vec::new();
    for one_name in every_name {
        let Some(the_name) = one_name.as_str() else {
            continue;
        };
        if found.iter().any(|already| already == the_name) {
            continue;
        }
        found.push(String::from(the_name));
    }
    found
}

/// The three lists that name a file, **exposed because a caller checks those and not the others.**
pub fn the_lists_that_name_a_file() -> [&'static str; 3] {
    THE_LISTS_THAT_NAME_A_FILE
}
