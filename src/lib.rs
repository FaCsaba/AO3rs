/// Rating given to a specific work
enum Rating {
    Mature,
    Explicit,
    NotRated,
    TeenAndUp,

    /// Fan fiction works for general audiences
    General,
}

pub struct Fanfic {
    rating: Rating,
}

pub enum Period {
    Years,
    Weeks,
    Months,
    Days,
    Hours,
}

impl Period {
    pub fn to_string(&self) -> String {
        match self {
            Period::Years => String::from("years"),
            Period::Weeks => String::from("weeks"),
            Period::Months => String::from("months"),
            Period::Days => String::from("days"),
            Period::Hours => String::from("hours"),
        }
    }
}

/// Create a range of time
///
/// AO3 allows you to create a range of time
///
/// Examples form AO3 (taking Wednesday 25th April 2012 as the current day):
/// ```rust
/// use ao3rs::{Date, Period};
///
/// // 7 days ago (this will return all works posted/updated on Wednesday 18th April)
/// assert_eq!(String::from("7 days ago"), Date::Exactly(7, Period::Days).to_string());           
/// // 1 week ago (this will return all works posted/updated in the week starting Monday 16th April and ending Sunday 22nd April)
/// assert_eq!(String::from("1 weeks ago"), Date::Exactly(1, Period::Weeks).to_string());         
/// // 2 months ago (this will return all works posted/updated in the month of February)
/// assert_eq!(String::from("2 months ago"), Date::Exactly(2, Period::Months).to_string());        
/// // 3 years ago (this will return all works posted/updated in 2010)
/// assert_eq!(String::from("3 years ago"), Date::Exactly(3, Period::Years).to_string());         
/// // < 7 days (this will return all works posted/updated within the past seven days)
/// assert_eq!(String::from("< 7 days ago"), Date::LessThan(7, Period::Days).to_string());        
/// // > 8 weeks (this will return all works posted/updated more than eight weeks ago)
/// assert_eq!(String::from("> 8 weeks ago"), Date::MoreThan(8, Period::Weeks).to_string());      
/// // 13-21 months (this will return all works posted/updated between thirteen and twenty-one months ago)
/// assert_eq!(String::from("13-21 months"), Date::Between(13, 21, Period::Months).to_string());  
/// ```
/// "ago" is not required but for aesthetics I put it there
pub enum Date {
    Exactly(usize, Period),
    MoreThan(usize, Period),
    LessThan(usize, Period),
    Between(usize, usize, Period),
}

impl Date {
    pub fn to_string(&self) -> String {
        match self {
            Date::Exactly(time, period) => format!("{} {} ago", time, period.to_string()),
            Date::MoreThan(time, period) => format!("> {} {} ago", time, period.to_string()),
            Date::LessThan(time, period) => format!("< {} {} ago", time, period.to_string()),
            Date::Between(from_time, to_time, period) => {
                format!("{}-{} {}", from_time, to_time, period.to_string())
            }
        }
    }

    /// Create a query value used for the date field in [QueryBuilder](QueryBuilder)
    ///
    /// Synonym for to_string but for a more consistent querying experience
    pub fn to_query_value(&self) -> String {
        self.to_string()
    }
}

/// Completion Status
/// 
/// Wether a fan fiction has been completed or not
/// ```rust
/// use ao3rs::CompletionStatus;
/// 
/// assert_eq!(String::from(""), CompletionStatus::Ignore.to_query_value()); // We don't know if the fic was completed or we don't care like in a query_builder
/// assert_eq!(String::from("T"), CompletionStatus::Completed.to_query_value()); // The work was completed 
/// assert_eq!(String::from("F"), CompletionStatus::Incomplete.to_query_value()); // The work has not been completed
/// ```
pub enum CompletionStatus {
    /// Only used for Querying
    /// 
    /// query value: empty string
    Ignore,

    /// A work has been completed,
    /// unless the author was an asshole and put completed but really they just abandoned it
    ///
    /// query value: T
    Completed,

    /// A work has yet to be completed
    ///
    /// query value: F
    Incomplete,
}

impl CompletionStatus {
    pub fn to_query_value(&self) -> String {
        match self {
            CompletionStatus::Ignore => String::from(""),
            CompletionStatus::Completed => String::from("T"),
            CompletionStatus::Incomplete => String::from("F"),
        }
    }
}

pub struct AO3QueryBuilder {
    any_field: String,
    title: String,
    author: String,

    /// Date on which it was last updated or (if not updated at all) posted, 
    date: Date,

    completion_status: CompletionStatus,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
