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

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct AO3Work {
    pub id: String,
    pub url: String,
    pub title: String,
    pub authors: Vec<String>,
    pub date: chrono::NaiveDate,
    pub is_complete: bool,
    pub is_crossover: bool,
    pub word_count: usize,
    pub fandoms: Vec<String>,
    rating: Option<Rating>
}

impl AO3Work {
    fn parse_entire() {

    }

    pub fn get_rating(&mut self) -> Rating {
        todo!()
    }
}