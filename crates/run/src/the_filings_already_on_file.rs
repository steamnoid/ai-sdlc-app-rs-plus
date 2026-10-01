//! A filing already on file, which a run reads before it proposes anything.
//!
//! **A tracker is where a person already said the same thing.**

/// One filing already on file, **which is a number and a title and nothing else.**
///
/// **No body and no state.** A number and a title are what tells a model that something is already
/// being discussed; **a body would put somebody else's words in this system's prompt.**
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AFilingAlreadyOnFile {
    which: i64,
    what_it_is_called: String,
}

impl AFilingAlreadyOnFile {
    /// A filing that is open.
    pub fn that_is_open(which: i64, what_it_is_called: &str) -> Self {
        AFilingAlreadyOnFile {
            which,
            what_it_is_called: what_it_is_called.to_string(),
        }
    }

    /// Which number it is.
    pub fn which(&self) -> i64 {
        self.which
    }

    /// What it is called.
    pub fn what_it_is_called(&self) -> &str {
        &self.what_it_is_called
    }
}

/// Every filing already on file, **and the whole list rather than the closest match.**
///
/// **All of them and not the best one**, because a model asked about the closest match would repeat
/// the second-closest without ever hearing of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TheFilingsAlreadyOnFile {
    every_one: Vec<AFilingAlreadyOnFile>,
}

impl TheFilingsAlreadyOnFile {
    /// None of them, **and that is a real answer rather than a missing one.**
    pub fn none_yet() -> Self {
        Self {
            every_one: Vec::new(),
        }
    }

    /// Add one, **and the order they were read in is kept.**
    pub fn and_this_one_was_too(&mut self, one_more: AFilingAlreadyOnFile) {
        self.every_one.push(one_more);
    }

    /// Whether a filing with this number is on file, **and the number is compared and not the title.**
    pub fn whether_it_holds(&self, which: i64) -> bool {
        self.every_one.iter().any(|one| one.which == which)
    }

    /// How many are on file, **and this is what the question says before listing them.**
    pub fn how_many(&self) -> usize {
        self.every_one.len()
    }

    /// The filings on file as the question's own lines, **one per filing.**
    ///
    /// **Numbers and titles and nothing else**, because the words a person wrote inside their own
    /// filing are theirs — **and this system's prompt has no business carrying them anywhere.**
    pub fn as_lines_for_the_question(&self) -> String {
        self.every_one
            .iter()
            .map(|one| format!("#{} {}", one.which, one.what_it_is_called))
            .collect::<Vec<String>>()
            .join("\n")
    }
}
