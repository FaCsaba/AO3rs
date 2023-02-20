#[derive(Debug)]
pub enum ParsingErrors {
    CouldNotFind(String)
}

impl std::fmt::Display for ParsingErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingErrors::CouldNotFind(thing) => write!(f, "Could not find: {}", thing),
        }
    }
}

impl std::error::Error for ParsingErrors {}

struct SearchParser;

impl SearchParser {
    fn parse(html_code: &str) {
        
    }
}