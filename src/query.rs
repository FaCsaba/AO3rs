const BASE_AO3_SEARCH_URL: &'static str = "https://archiveofourown.org/works/search?";

trait QueryValue: std::fmt::Display {
    type Output;

    fn to_query_value(&self) -> Self::Output;

    fn is_included(&self) -> bool;
}

impl QueryValue for String {
    type Output = String;

    fn to_query_value(&self) -> Self::Output {
        self.to_string()
    }

    fn is_included(&self) -> bool {
        !self.is_empty()
    }
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
                format!("{}-{} {}", from_time, to_time, period)
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
            NumericalValueRange::None => String::new(),
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
struct MultiString(Vec<String>);

impl QueryValue for MultiString {
    type Output = String;

    fn to_query_value(&self) -> String {
        self.0.join(",")
    }

    fn is_included(&self) -> bool {
        !self.0.is_empty()
    }
}

impl std::fmt::Display for MultiString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[ {} ]", self.0.join(", "))
    }
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

impl QueryValue for Rating {
    type Output = String;

    fn to_query_value(&self) -> String {
        match self {
            Rating::None => String::new(),
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

#[derive(Debug, PartialEq, Eq, Clone)]
struct MultiSelect<T>(Vec<T>)
where
    T: QueryValue;

impl<T> std::fmt::Display for MultiSelect<T>
where
    T: QueryValue,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // kinda wonky but seems like the most elegant solution for now
        write!(
            f,
            "[ {} ]",
            self.0
                .iter()
                .map(|aw| aw.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl<T> QueryValue for MultiSelect<T>
where
    T: QueryValue,
    Vec<String>: FromIterator<<T as QueryValue>::Output>,
{
    type Output = Vec<String>;

    fn to_query_value(&self) -> Self::Output {
        self.0.iter().map(|aw| aw.to_query_value()).collect()
    }

    fn is_included(&self) -> bool {
        !self.0.is_empty()
    }
}

impl<T: QueryValue> Default for MultiSelect<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Category {
    /// Female / Female
    FF = 116,

    /// Female / Male
    FM = 22,

    /// General
    Gen = 21,

    /// Male / Male
    MM = 23,

    /// Multi
    Multi = 2246,

    /// Other
    Other = 24,
}

impl QueryValue for Category {
    type Output = String;

    fn to_query_value(&self) -> Self::Output {
        match self {
            Category::FF => (Category::FF as usize).to_string(),
            Category::FM => (Category::FM as usize).to_string(),
            Category::Gen => (Category::Gen as usize).to_string(),
            Category::MM => (Category::MM as usize).to_string(),
            Category::Multi => (Category::Multi as usize).to_string(),
            Category::Other => (Category::Other as usize).to_string(),
        }
    }

    fn is_included(&self) -> bool {
        true
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Category::FF => write!(f, "F/F"),
            Category::FM => write!(f, "F/M"),
            Category::Gen => write!(f, "Gen"),
            Category::MM => write!(f, "M/M"),
            Category::Multi => write!(f, "Multi"),
            Category::Other => write!(f, "Other"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum SortBy {
    #[default]
    BestMatch, // TODO: the rest of the sort bys
}

impl QueryValue for SortBy {
    type Output = String;

    fn to_query_value(&self) -> Self::Output {
        match self {
            SortBy::BestMatch => format!("_score"),
        }
    }

    fn is_included(&self) -> bool {
        true
    }
}

impl std::fmt::Display for SortBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortBy::BestMatch => write!(f, "Best Match"),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub enum SortDirection {
    #[default]
    Descending,
    Ascending,
}

impl QueryValue for SortDirection {
    type Output = String;

    fn to_query_value(&self) -> Self::Output {
        match self {
            SortDirection::Descending => format!("desc"),
            SortDirection::Ascending => format!("asc"),
        }
    }

    fn is_included(&self) -> bool {
        true
    }
}

impl std::fmt::Display for SortDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortDirection::Descending => write!(f, "Descending order"),
            SortDirection::Ascending => write!(f, "Ascending order"),
        }
    }
}

#[derive(Debug, Default)]
pub struct AO3QueryBuilder {
    /// Searches everything
    any_field: String,

    /// Title of the work
    title: String,

    /// Author or creator of the work
    authors: MultiString,

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
    fandoms: MultiString,

    /// Rating
    rating: Rating,

    /// Archive warnings
    archive_warnings: MultiSelect<ArchiveWarning>,

    /// Categories
    categories: MultiSelect<Category>,

    /// Characters
    characters: MultiString,

    /// Relationships
    relationships: MultiString,

    /// Additional Tags
    additional_tags: MultiString,

    /// Hits
    hits: NumericalValueRange,

    /// Kudos
    kudos: NumericalValueRange,

    /// Comments
    comments: NumericalValueRange,

    /// Bookmarks
    bookmarks: NumericalValueRange,

    /// Sort by
    sort_by: SortBy,

    /// Sort direction
    sort_direction: SortDirection,
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

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn set_authors(mut self, authors: Vec<String>) -> Self {
        self.authors = MultiString(authors);
        self
    }

    pub fn push_author(mut self, author: String) -> Self {
        self.authors.0.push(author);
        self
    }

    pub fn get_authors(&self) -> String {
        self.authors.to_string()
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

    pub fn is_single_chapter(&self) -> &bool {
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
        self.fandoms = MultiString(fandoms);
        self
    }

    pub fn push_fandom(mut self, fandom: &dyn AsRef<str>) -> Self {
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
        self.archive_warnings = MultiSelect(archive_warnings);
        self
    }

    pub fn add_archive_warning(mut self, archive_warning: ArchiveWarning) -> Self {
        self.archive_warnings.0.push(archive_warning);
        self
    }

    pub fn set_categories(mut self, categories: Vec<Category>) -> Self {
        self.categories = MultiSelect(categories);
        self
    }

    pub fn push_category(mut self, category: Category) -> Self {
        self.categories.0.push(category);
        self
    }

    pub fn set_characters(mut self, characters: Vec<String>) -> Self {
        self.characters = MultiString(characters);
        self
    }

    pub fn push_character(mut self, character: String) -> Self {
        self.characters.0.push(character);
        self
    }

    pub fn set_relationships(mut self, relationships: Vec<String>) -> Self {
        self.relationships = MultiString(relationships);
        self
    }

    pub fn push_relationship(mut self, relationship: String) -> Self {
        self.relationships.0.push(relationship);
        self
    }

    pub fn set_additional_tags(mut self, additional_tags: Vec<String>) -> Self {
        self.additional_tags = MultiString(additional_tags);
        self
    }

    pub fn push_additional_tag(mut self, additional_tag: String) -> Self {
        self.additional_tags.0.push(additional_tag);
        self
    }

    pub fn set_hits(mut self, hits: NumericalValueRange) -> Self {
        self.hits = hits;
        self
    }

    pub fn set_kudos(mut self, kudos: NumericalValueRange) -> Self {
        self.kudos = kudos;
        self
    }

    pub fn set_comments(mut self, comments: NumericalValueRange) -> Self {
        self.comments = comments;
        self
    }

    pub fn set_bookmarks(mut self, bookmarks: NumericalValueRange) -> Self {
        self.bookmarks = bookmarks;
        self
    }

    pub fn set_sort_by(mut self, sort_by: SortBy) -> Self {
        self.sort_by = sort_by;
        self
    }

    pub fn set_sort_direction(mut self, sort_direction: SortDirection) -> Self {
        self.sort_direction = sort_direction;
        self
    }

    /// Perform a simple search with a single query
    pub async fn simple_search(mut self, query: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.any_field = query.to_string();
        self.send().await?;
        Ok(())
    }

    fn create_url(&self) -> String {
        let mut is_first = true;
        let mut q = String::from(BASE_AO3_SEARCH_URL);
        fn add_delim(q: &mut String, is_first: &mut bool) {
            if !*is_first {
                q.push_str("&");
            }
            *is_first = false;
        }
        if self.any_field.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[query]={}",
                self.any_field.to_query_value()
            ))
        }
        if self.title.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[title]={}",
                self.title.to_query_value()
            ))
        }
        if self.authors.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[authors]={}",
                self.authors.to_query_value()
            ))
        }
        if self.date.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[revised_at]={}",
                self.date.to_query_value()
            ))
        }
        if self.completion_status.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[complete]={}",
                self.completion_status.to_query_value()
            ))
        };
        if self.crossover_status.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[crossover]={}",
                self.crossover_status.to_query_value()
            ))
        }
        if self.is_single_chapter.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!(
                "work_search[single_chapter]={}",
                self.is_single_chapter().to_query_value()
            ))
        }
        if self.word_count.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[word_count]={}", self.word_count.to_query_value()))
        }
        if self.fandoms.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[fandom_names]={}", self.fandoms.to_query_value()))
        }
        if self.rating.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[rating_ids]={}", self.rating.to_query_value()))
        }
        if self.archive_warnings.is_included() {
            self.archive_warnings.to_query_value().into_iter().for_each(|aw| {
                add_delim(&mut q, &mut is_first);
                q.push_str(&format!("work_search[archive_warning_ids][]={}", aw))
            });
        }
        if self.categories.is_included() {
            self.categories.to_query_value().into_iter().for_each(|cat| {
                add_delim(&mut q, &mut is_first);
                q.push_str(&format!("work_search[category_ids][]={}", cat))
            });
        }
        if self.characters.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[character_names]={}", self.characters.to_query_value()))
        }
        if self.relationships.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[relationship_name]={}", self.relationships.to_query_value()))
        }
        if self.additional_tags.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[freeform_names]={}", self.additional_tags.to_query_value()))
        }
        if self.hits.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[hits]={}", self.hits.to_query_value()))
        }
        if self.kudos.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[kudos_count]={}", self.kudos.to_query_value()))
        }
        if self.comments.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[commets_count]={}", self.comments.to_query_value()))
        }
        if self.bookmarks.is_included() {
            add_delim(&mut q, &mut is_first);
            q.push_str(&format!("work_search[bookmarks_count]={}", self.bookmarks.to_query_value()))
        }
        add_delim(&mut q, &mut is_first);
        q.push_str(&format!("work_search[sort_column]={}", self.sort_by.to_query_value()));
        add_delim(&mut q, &mut is_first);
        q.push_str(&format!("work_search[sort_direction]={}", self.sort_direction.to_query_value()));
        q
    }

    /// Send query
    pub async  fn send(self) -> Result<String, Box<dyn std::error::Error>> {
        let url = self.create_url();
        let resp = reqwest::get(url).await?.text().await?;

        Ok(resp)
    }
}

impl std::fmt::Display for AO3QueryBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Query:")?;
        if self.title.is_included() {
            writeln!(f, "\ttitle: {}", self.title)?
        }
        if self.authors.is_included() {
            writeln!(f, "\tauthor: {}", self.authors)?
        }
        if self.date.is_included() {
            writeln!(f, "\tdate: {}", self.date)?
        }
        if self.completion_status.is_included() {
            writeln!(f, "\tcompletion_status: {}", self.completion_status)?
        };
        if self.crossover_status.is_included() {
            writeln!(f, "\tcrossover: {}", self.crossover_status)?
        }
        if self.is_single_chapter.is_included() {
            writeln!(f, "\tis single chapter: {}", self.is_single_chapter())?;
        }
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
        if self.categories.is_included() {
            writeln!(f, "\tcategories: {}", self.categories)?
        }
        if self.characters.is_included() {
            writeln!(f, "\tcharacters: {}", self.characters)?
        }
        if self.relationships.is_included() {
            writeln!(f, "\trelationships: {}", self.relationships)?
        }
        if self.additional_tags.is_included() {
            writeln!(f, "\tadditional tags: {}", self.additional_tags)?
        }
        if self.hits.is_included() {
            writeln!(f, "\thits: {}", self.hits)?
        }
        if self.kudos.is_included() {
            writeln!(f, "\tkudos: {}", self.kudos)?
        }
        if self.comments.is_included() {
            writeln!(f, "\tcomments: {}", self.comments)?
        }
        if self.bookmarks.is_included() {
            writeln!(f, "\tbookmarks: {}", self.bookmarks)?
        }
        writeln!(f, "\tSort by: {}", self.sort_by)?;
        writeln!(f, "\tSort direction: {}", self.sort_direction)?;
        std::fmt::Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_query_builder() {
        let q = AO3QueryBuilder::new().set_kudos(NumericalValueRange::LessThan(5)).set_rating(Rating::Explicit);
        println!("{}", q);
        println!("{}", q.send().await.unwrap());
    }
}
