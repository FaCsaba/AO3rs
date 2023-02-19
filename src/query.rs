trait QueryValue: std::fmt::Display {
    type Output;

    fn to_query_value(&self) -> Self::Output;

    fn is_included(&self) -> bool;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Period {
    Years,
    Weeks,
    Months,
    Days,
    Hours,
}

impl std::fmt::Display for Period {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Period::Years => write!(f, "years"),
            Period::Weeks => write!(f, "weeks"),
            Period::Months => write!(f, "months"),
            Period::Days => write!(f, "days"),
            Period::Hours => write!(f, "hours"),
        }
    }
}

/// Create a range of time
///
/// AO3 allows you to create a range of time
///
/// Examples form AO3 (taking Wednesday 25th April 2012 as the current day):
/// ```rust
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum DateRange {
    #[default]
    None,
    Exactly(usize, Period),
    MoreThan(usize, Period),
    LessThan(usize, Period),
    Between(usize, usize, Period),
}

impl QueryValue for DateRange {
    type Output = String;
    /// Create a query value used for the date field in [QueryBuilder](QueryBuilder)
    fn to_query_value(&self) -> String {
        match self {
            DateRange::None => String::new(),
            DateRange::Exactly(time, period) => format!("{} {} ago", time, period),
            DateRange::MoreThan(time, period) => format!("> {} {} ago", time, period),
            DateRange::LessThan(time, period) => format!("< {} {} ago", time, period),
            DateRange::Between(from_time, to_time, period) => {
                format!("{}-{} {}", from_time, to_time, period.to_string())
            }
        }
    }

    fn is_included(&self) -> bool {
        self != &Self::None
    }
}

impl std::fmt::Display for DateRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DateRange::None => write!(f, "None"),
            DateRange::Exactly(time, period) => write!(f, "Exactly {} {} ago", time, period),
            DateRange::MoreThan(time, period) => write!(f, "More than {} {} ago", time, period),
            DateRange::LessThan(time, period) => write!(f, "Less than {} {} ago", time, period),
            DateRange::Between(from_time, to_time, period) => {
                write!(f, "Between {} and {} {} ago", from_time, to_time, period)
            }
        }
    }
}

/// Completion Status
///
/// Wether a fan fiction has been completed or not
/// ```rust
/// use ao3rs::query::AO3QueryBuilder;
///
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum CompletionStatus {
    /// Ignore whether work was completed or not
    /// query value: empty string
    /// default
    #[default]
    Ignore,

    /// A work has been completed,
    /// unless the author was an asshole and put completed but really they just abandoned it
    ///
    /// query value: T
    OnlyCompleted,

    /// A work has yet to be completed
    ///
    /// query value: F
    OnlyIncomplete,
}

impl QueryValue for CompletionStatus {
    type Output = String;
    /// Create a query value used
    fn to_query_value(&self) -> String {
        match self {
            CompletionStatus::Ignore => String::from(""),
            CompletionStatus::OnlyCompleted => String::from("T"),
            CompletionStatus::OnlyIncomplete => String::from("F"),
        }
    }

    fn is_included(&self) -> bool {
        self != &Self::Ignore
    }
}

impl std::fmt::Display for CompletionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompletionStatus::Ignore => write!(f, "Don't care"),
            CompletionStatus::OnlyCompleted => write!(f, "Only allow completed"),
            CompletionStatus::OnlyIncomplete => write!(f, "Only allow incomplete"),
        }
    }
}

/// Crossover
///
/// Wether a fan fiction is a crossover or not
/// ```rust
/// use ao3rs::query::QueryBuilder;
/// ```
#[derive(Debug, Default, PartialEq, Eq, Clone)]
enum CrossoverStatus {
    /// Don't care if there are crossovers
    ///
    /// query value: empty string
    #[default]
    Ignore,

    /// Only works that feature crossovers
    ///
    /// query value: T
    OnlyCrossover,

    /// Only works which do not have crossovers
    ///
    /// query value: F
    OnlyNonCrossover,
}

impl QueryValue for CrossoverStatus {
    type Output = String;

    fn to_query_value(&self) -> String {
        match self {
            CrossoverStatus::Ignore => String::from(""),
            CrossoverStatus::OnlyCrossover => String::from("T"),
            CrossoverStatus::OnlyNonCrossover => String::from("F"),
        }
    }

    fn is_included(&self) -> bool {
        self != &Self::Ignore
    }
}

impl std::fmt::Display for CrossoverStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CrossoverStatus::Ignore => write!(f, "Don't care"),
            CrossoverStatus::OnlyCrossover => write!(f, "Only allow crossovers"),
            CrossoverStatus::OnlyNonCrossover => write!(f, "Only allow non crossovers"),
        }
    }
}

impl QueryValue for bool {
    type Output = String;

    fn to_query_value(&self) -> String {
        match self {
            true => String::from("1"),
            false => String::from("0"),
        }
    }

    fn is_included(&self) -> bool {
        true
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum NumericalValueRange {
    #[default]
    None,
    Exactly(usize),
    MoreThan(usize),
    LessThan(usize),
    Between(usize, usize),
}

impl QueryValue for NumericalValueRange {
    type Output = String;

    fn to_query_value(&self) -> String {
        match self {
            NumericalValueRange::None => format!(""),
            NumericalValueRange::Exactly(num) => format!("{num}"),
            NumericalValueRange::MoreThan(num) => format!("> {num}"),
            NumericalValueRange::LessThan(num) => format!("< {num}"),
            NumericalValueRange::Between(from_num, to_num) => format!("{from_num}-{to_num}"),
        }
    }

    fn is_included(&self) -> bool {
        self != &Self::None
    }
}

impl std::fmt::Display for NumericalValueRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericalValueRange::None => write!(f, "None"),
            NumericalValueRange::Exactly(num) => write!(f, "Exactly {num}"),
            NumericalValueRange::MoreThan(num) => write!(f, "More than {num}"),
            NumericalValueRange::LessThan(num) => write!(f, "Less than {num}"),
            NumericalValueRange::Between(from_num, to_num) => {
                write!(f, "Between {from_num} and {to_num}")
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Fandoms(Vec<String>);

impl QueryValue for Fandoms {
    type Output = String;

    fn to_query_value(&self) -> String {
        self.0.join(",")
    }

    fn is_included(&self) -> bool {
        self.0.len() > 0
    }

    
}

impl std::fmt::Display for Fandoms {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} ]", self.0.join(", "))
    }
}

impl QueryValue for Rating {
    type Output = String;

    fn to_query_value(&self) -> String {
        match self {
            Rating::None => format!(""),
            Rating::Mature => (Rating::Mature as usize).to_string(),
            Rating::Explicit => (Rating::Explicit as usize).to_string(),
            Rating::NotRated => (Rating::NotRated as usize).to_string(),
            Rating::TeenAndUp => (Rating::TeenAndUp as usize).to_string(),
            Rating::General => (Rating::General as usize).to_string(),
        }
    }

    fn is_included(&self) -> bool {
        self != &Self::None
    }
    
}

impl std::fmt::Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rating::None => write!(f, "None"),
            Rating::Mature => write!(f, "For Mature Audiences"),
            Rating::Explicit => write!(f, "Work is Explicit"),
            Rating::NotRated => write!(f, "Work is not rated"),
            Rating::TeenAndUp => write!(f, "For Teens And Up"),
            Rating::General => write!(f, "For General Audiences"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArchiveWarning {
    CreatureChoseNotToUseArchiveWarnings = 14,
    GraphicDepictionOfViolence = 17,
    MajorCharacterDeath = 18,
    NoArchiveWarningsApply = 16,
    RapeNonCon = 19,
    Underage = 20,
}
impl QueryValue for ArchiveWarning {
    type Output = String;

    fn to_query_value(&self) -> String {
        match self {
            ArchiveWarning::CreatureChoseNotToUseArchiveWarnings => {
                (ArchiveWarning::CreatureChoseNotToUseArchiveWarnings as usize).to_string()
            }
            ArchiveWarning::GraphicDepictionOfViolence => {
                (ArchiveWarning::GraphicDepictionOfViolence as usize).to_string()
            }
            ArchiveWarning::MajorCharacterDeath => {
                (ArchiveWarning::MajorCharacterDeath as usize).to_string()
            }
            ArchiveWarning::NoArchiveWarningsApply => {
                (ArchiveWarning::NoArchiveWarningsApply as usize).to_string()
            }
            ArchiveWarning::RapeNonCon => (ArchiveWarning::RapeNonCon as usize).to_string(),
            ArchiveWarning::Underage => (ArchiveWarning::Underage as usize).to_string(),
        }
    }

    fn is_included(&self) -> bool {
        true
    }
}

impl std::fmt::Display for ArchiveWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArchiveWarning::CreatureChoseNotToUseArchiveWarnings => {
                write!(f, "Creature Chose Not To Use Archive Warnings")
            }
            ArchiveWarning::GraphicDepictionOfViolence => {
                write!(f, "Graphic Depiction Of Violence")
            }
            ArchiveWarning::MajorCharacterDeath => write!(f, "Major Character Death"),
            ArchiveWarning::NoArchiveWarningsApply => write!(f, "No Archive Warnings Apply"),
            ArchiveWarning::RapeNonCon => write!(f, "Rape/Non-Con"),
            ArchiveWarning::Underage => write!(f, "Underage"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct ArchiveWarnings(Vec<ArchiveWarning>);

impl std::fmt::Display for ArchiveWarnings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // kinda wonky but seems like the most elegant solution for now
        write!(f, "[ {} ]", self.0.iter().map(|aw| aw.to_string()).collect::<Vec<String>>().join(", "))
    }
}

impl QueryValue for ArchiveWarnings {
    type Output = Vec<String>;

    fn to_query_value(&self) -> Self::Output {
        self.0.iter().map(|aw| aw.to_query_value()).collect()
    }

    fn is_included(&self) -> bool {
        self.0.len() > 0
    }
}

#[derive(Debug, Default)]
pub struct AO3QueryBuilder {
    /// Searches everything
    any_field: String,

    /// Title of the work
    title: String,

    /// Author or creator of the work
    author: String,

    /// Date on which it was last updated or (if not updated at all) posted,
    date: DateRange,

    /// Wether or not a work should be completed
    completion_status: CompletionStatus,

    /// Wether to include crossovers in the query
    crossover_status: CrossoverStatus,

    /// If turned on only show chapters with just one chapter
    is_single_chapter: bool,

    /// Word count
    word_count: NumericalValueRange,

    /// Fandoms
    fandoms: Fandoms,

    /// Rating
    rating: Rating,

    /// Archive warnings
    archive_warnings: ArchiveWarnings,
}

/// Rating given to a specific work
#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum Rating {
    /// We don't care what the rating is
    #[default]
    None,

    /// Not rated fan fiction works
    NotRated = 9,

    /// Fan fiction works for general audiences
    General = 10,

    /// Fan fiction works for teens and up audiences
    TeenAndUp = 11,

    /// Fan fiction works for mature audiences
    Mature = 12,

    /// Fan fiction containing explicit content
    Explicit = 13,
}

impl AO3QueryBuilder {
    pub fn new() -> Self {
        AO3QueryBuilder {
            ..Default::default()
        }
    }

    pub fn set_title(mut self, title: &dyn AsRef<str>) -> Self {
        self.title = title.as_ref().to_string();
        self
    }

    pub fn get_title<'a>(&'a self) -> &'a str {
        &self.title
    }

    pub fn set_author(mut self, author: &dyn AsRef<str>) -> Self {
        self.author = author.as_ref().to_string();
        self
    }

    pub fn get_author<'a>(&'a self) -> &'a str {
        &self.author
    }

    pub fn set_date_range(mut self, date: DateRange) -> Self {
        self.date = date;
        self
    }

    pub fn get_date_range(&self) -> String {
        self.date.to_string()
    }

    pub fn only_completed(mut self) -> Self {
        self.completion_status = CompletionStatus::OnlyCompleted;
        self
    }

    /// Don't care whether or not a work is complete
    pub fn ignore_completion_status(mut self) -> Self {
        self.completion_status = CompletionStatus::Ignore;
        self
    }

    pub fn only_incomplete(mut self) -> Self {
        self.completion_status = CompletionStatus::OnlyIncomplete;
        self
    }

    pub fn get_completion_status(&self) -> String {
        self.completion_status.to_string()
    }

    pub fn only_crossover(mut self) -> Self {
        self.crossover_status = CrossoverStatus::OnlyCrossover;
        self
    }

    /// Don't care whether or not a work is a crossover or not
    pub fn ignore_crossover_status(mut self) -> Self {
        self.crossover_status = CrossoverStatus::Ignore;
        self
    }

    pub fn only_non_crossover(mut self) -> Self {
        self.crossover_status = CrossoverStatus::OnlyNonCrossover;
        self
    }

    pub fn get_crossover_status(&self) -> String {
        self.crossover_status.to_string()
    }

    pub fn single_chapter(mut self, is_single_chapter: bool) -> Self {
        self.is_single_chapter = is_single_chapter;
        self
    }

    pub fn is_single_chapter<'a>(&'a self) -> &'a bool {
        &self.is_single_chapter
    }

    pub fn set_word_count(mut self, word_count: NumericalValueRange) -> Self {
        self.word_count = word_count;
        self
    }

    pub fn get_word_count(&self) -> String {
        self.word_count.to_string()
    }

    pub fn set_fandoms(mut self, fandoms: Vec<String>) -> Self {
        self.fandoms = Fandoms(fandoms);
        self
    }

    pub fn add_fandom(mut self, fandom: &dyn AsRef<str>) -> Self {
        self.fandoms.0.push(fandom.as_ref().to_string());
        self
    }

    pub fn get_fandoms(&self) -> String {
        self.fandoms.to_string()
    }

    pub fn set_rating(mut self, rating: Rating) -> Self {
        self.rating = rating;
        self
    }

    pub fn set_archive_warnings(mut self, archive_warnings: Vec<ArchiveWarning>) -> Self {
        self.archive_warnings = ArchiveWarnings(archive_warnings);
        self
    }

    pub fn add_archive_warning(mut self, archive_warning: ArchiveWarning) -> Self {
        self.archive_warnings.0.push(archive_warning);
        self
    }

    /// Perform a simple search with a single query
    pub fn simple_search(mut self, query: &str) {
        self.any_field = query.to_string();
        self.send()
    }

    /// Send query
    pub fn send(self) {}
}

impl std::fmt::Display for AO3QueryBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Query:")?;
        if self.title != "" {
            writeln!(f, "\ttitle: {}", self.title)?
        }
        if self.author != "" {
            writeln!(f, "\tauthor: {}", self.author)?
        }
        if self.date.is_included() {
            writeln!(f, "\tdate: {}", self.date)?
        }
        if self.completion_status.is_included() {
            writeln!(f, "\tcompletion_status: {}", self.completion_status)?
        };
        writeln!(f, "\tis single chapter: {}", self.is_single_chapter())?;
        if self.word_count.is_included() {
            writeln!(f, "\tword count: {}", self.word_count)?
        }
        if self.fandoms.is_included() {
            writeln!(f, "\tfandoms: {}", self.fandoms)?
        }
        if self.rating.is_included() {
            writeln!(f, "\trating: {}", self.rating)?
        }
        if self.archive_warnings.is_included() {
            writeln!(f, "\tarchive warnings: {}", self.archive_warnings)?
        }
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_builder() {
        println!(
            "{}",
            AO3QueryBuilder::new()
                .set_date_range(DateRange::Exactly(5, Period::Years))
                .set_fandoms(vec![
                    String::from("Hello kitty island adventure"),
                    String::from("Monster Hunter world")
                ])
                .add_fandom(&"fandom")
                .set_archive_warnings(vec![ArchiveWarning::CreatureChoseNotToUseArchiveWarnings])
        );
    }
}
