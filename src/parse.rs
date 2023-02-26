use crate::models::AO3Work;

#[derive(Debug)]
pub enum ParsingError<'a> {
    CouldNotFind(&'a str),
}

impl std::fmt::Display for ParsingError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsingError::CouldNotFind(thing) => write!(
                f,
                "Could not find: {}\nThis is a problem with the parsing!",
                thing
            ),
        }
    }
}

impl std::error::Error for ParsingError<'_> {}

pub fn parse_search(html_code: &str) -> Result<Vec<AO3Work>, Box<dyn std::error::Error>> {
    let dom = tl::parse(
        html_code,
        tl::ParserOptions::new().track_classes().track_ids(),
    )?;
    let parser = dom.parser();
    let work_list_nodes = dom
        .query_selector("[role=article]")
        .ok_or(ParsingError::CouldNotFind("the list of works."))?;
    let mut works = vec![];
    for work_node in work_list_nodes {
        works.push(parse_search_single_work(
            parser,
            work_node.get(parser).unwrap(),
        )?);
    }
    Ok(works)
}

fn search_by_attrib<'a, 'b>(
    parser: &'b tl::Parser<'b>,
    node: &tl::Node,
    attrib: &'a str,
    value: &str,
) -> Result<&'b tl::Node<'b>, ParsingError<'a>> {
    Ok(node
        .find_node(parser, &mut |n| {
            n.as_tag().map_or(false, |t| {
                t.attributes()
                    .get(attrib)
                    .flatten()
                    .map_or(false, |a| a == value)
            })
        })
        .ok_or(ParsingError::CouldNotFind(attrib))?
        .get(parser)
        .unwrap())
}

fn get_all_nodes<'a>(parser: &'a tl::Parser, node: &'a tl::Node) -> Vec<&'a tl::Node<'a>> {
    if let Some(children) = node.children() {
        let mut nodes = vec![];
        for ch in children.all(parser) {
            nodes.push(ch);
            nodes.append(&mut get_all_nodes(parser, ch));
        }
        nodes
    } else {
        vec![]
    }
}

fn search_all_by_attrib<'a, 'b>(
    parser: &'b tl::Parser<'b>,
    node: &'b tl::Node,
    attrib: &'a str,
    value: &str,
) -> Result<Vec<&'b tl::Node<'b>>, ParsingError<'a>> {
    let a = get_all_nodes(parser, node);
    Ok(a.into_iter()
        .filter_map(|nh| {
            if nh.as_tag()?.attributes().get(attrib)?? == value {
                Some(nh)
            } else {
                None
            }
        })
        .collect())
}

fn parse_search_single_work(
    parser: &tl::Parser,
    node: &tl::Node,
) -> Result<AO3Work, Box<dyn std::error::Error>> {
    let id = node
        .as_tag()
        .unwrap()
        .attributes()
        .id()
        .unwrap()
        .as_utf8_str()
        .to_string()
        .replace("work_", "");

    let title = search_by_attrib(parser, node, "href", &format!("/works/{id}"))?
        .inner_text(parser)
        .to_string();

    let authors = search_all_by_attrib(
        parser,
        search_by_attrib(parser, node, "class", "fandoms heading")?,
        "rel",
        "author",
    )?
    .into_iter()
    .map(|ch| ch.inner_text(parser).to_string())
    .collect();

    let fandoms = search_all_by_attrib(
        parser,
        search_by_attrib(parser, node, "class", "fandoms heading")?,
        "class",
        "tag",
    )?
    .into_iter()
    .map(|ch| ch.inner_text(parser).to_string())
    .collect();

    let mut work = AO3Work::default();
    work.id = id;
    work.title = title;
    work.authors = authors;
    work.fandoms = fandoms;
    Ok(work)
}

#[cfg(test)]
mod tests {
    use crate::parse::parse_search;

    #[test]
    fn test_query_builder() {
        let html = include_str!("parse_test/search.html");
        println!("{:#?}", parse_search(html));
    }
}
