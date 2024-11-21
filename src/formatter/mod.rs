use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/sql.pest"]
struct SQLParser;

#[derive(Debug, Default)]
struct Line<'a> {
    start: Option<Pair<'a, Rule>>,
    elements: Vec<Pair<'a, Rule>>,
    indent: usize,
}

impl<'a> Line<'a> {
    pub fn from_start(start: Pair<'a, Rule>, indent: usize) -> Self {
        Self {
            start: Some(start),
            elements: Vec::new(),
            indent,
        }
    }
}

impl<'a> std::fmt::Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}\t{}",
            "\t".repeat(self.indent),
            match &self.start {
                Some(pair) => pair.as_str().to_string(),
                None => String::new(),
            },
            self.elements
                .iter()
                .map(|pair| pair.as_str().to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}

enum FormatElementResult<'a> {
    Rule(Pair<'a, Rule>),
    Lines(Vec<Line<'a>>),
}

pub fn format_string(sql: &str) -> Result<String, pest::error::Error<Rule>> {
    let tree = SQLParser::parse(Rule::query, sql)?.next().unwrap();

    let lines = format_query(tree);
    let mut iter_peek = lines.into_iter().peekable();
    let mut output = String::new();

    while let Some(line) = iter_peek.next() {
        output.push_str(&line.to_string());

        if let Some(next_line) = iter_peek.peek() {
            if line.elements.is_empty()
                && next_line.indent == line.indent + 1
                && next_line.start.as_ref().unwrap().as_rule() != Rule::open_paren
            {
                let mut next_line = iter_peek.next().unwrap();
                next_line.indent = 0;
                output.push_str(&next_line.to_string());
            }
        }

        output.push('\n');
    }

    Ok(output)
}

fn format_comment(rule: Pair<Rule>, indent: usize) -> Line {
    Line::from_start(rule, indent)
}

fn format_case(rule: Pair<Rule>, indent: usize) -> Vec<Line> {
    let mut lines = Vec::new();

    for pair in rule.into_inner() {
        match pair.as_rule() {
            Rule::case_kw | Rule::end_kw => lines.push(Line::from_start(pair.clone(), indent)),
            Rule::element => match format_element(
                pair.clone(),
                indent,
                match lines.last().unwrap().start.as_ref().unwrap().as_rule() {
                    Rule::or_kw | Rule::and_kw => {
                        if lines.last().unwrap().elements.is_empty() {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
            ) {
                FormatElementResult::Rule(p) => lines.last_mut().unwrap().elements.push(p),
                FormatElementResult::Lines(mut l) => lines.append(&mut l),
            },
            Rule::and_kw | Rule::or_kw => {
                lines.last_mut().unwrap().elements.push(pair.clone());
            }
            Rule::when_kw | Rule::else_kw => lines.push(Line::from_start(pair, indent)),
            Rule::COMMENT => lines.push(format_comment(pair, indent)),
            _ => unreachable!("Unexpected rule: {:?}", pair),
        }
    }

    lines
}

fn format_join(rule: Pair<Rule>, indent: usize) -> Vec<Line> {
    let mut lines = Vec::new();

    for pair in rule.into_inner() {
        match pair.as_rule() {
            Rule::left_kw
            | Rule::right_kw
            | Rule::full_kw
            | Rule::inner_kw
            | Rule::outer_kw
            | Rule::join_kw => lines.push(Line::from_start(pair, indent)),
            Rule::element => match format_element(
                pair,
                indent,
                match lines.last().unwrap().start.as_ref().unwrap().as_rule() {
                    Rule::or_kw | Rule::and_kw => {
                        if lines.last().unwrap().elements.is_empty() {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
            ) {
                FormatElementResult::Rule(p) => lines.last_mut().unwrap().elements.push(p),
                FormatElementResult::Lines(mut l) => lines.append(&mut l),
            },
            Rule::on_kw | Rule::and_kw => {
                lines.push(Line::from_start(pair, indent));
            }
            Rule::COMMENT => lines.push(format_comment(pair, indent)),
            _ => unreachable!("Unexpected rule: {:?}", pair),
        }
    }

    lines
}

fn format_between(rule: Pair<Rule>, indent: usize) -> Vec<Line> {
    let mut lines = Vec::new();

    for pair in rule.into_inner() {
        match pair.as_rule() {
            Rule::between_kw => lines.push(Line::from_start(pair, indent)),
            Rule::element => match format_element(
                pair,
                indent,
                match lines.last().unwrap().start.as_ref().unwrap().as_rule() {
                    Rule::or_kw | Rule::and_kw => {
                        if lines.last().unwrap().elements.is_empty() {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
            ) {
                FormatElementResult::Rule(p) => lines.last_mut().unwrap().elements.push(p),
                FormatElementResult::Lines(mut l) => lines.append(&mut l),
            },
            Rule::and_kw => {
                lines.push(Line::from_start(pair, indent));
            }
            Rule::COMMENT => lines.push(format_comment(pair, indent)),
            _ => unreachable!("Unexpected rule: {:?}", pair),
        }
    }

    lines
}

fn format_element(rule: Pair<Rule>, indent: usize, block: bool) -> FormatElementResult {
    let pair = rule.into_inner().next().unwrap();
    match pair.as_rule() {
        Rule::identifier | Rule::operator | Rule::string | Rule::quoted => {
            FormatElementResult::Rule(pair)
        }
        Rule::comma => FormatElementResult::Lines(vec![Line::from_start(pair, indent)]),
        Rule::paren => FormatElementResult::Lines(format_paren(
            pair,
            match block {
                true => indent,
                false => indent + 1,
            },
        )),
        Rule::between => FormatElementResult::Lines(format_between(pair, indent + 1)),
        Rule::case => FormatElementResult::Lines(format_case(pair, indent + 1)),
        Rule::join => FormatElementResult::Lines(format_join(pair, indent + 1)),
        _ => unreachable!("Unexpected rule: {:?}", pair),
    }
}

fn format_paren(rule: Pair<Rule>, indent: usize) -> Vec<Line> {
    let mut lines = Vec::new();

    for pair in rule.into_inner() {
        match pair.as_rule() {
            Rule::open_paren | Rule::close_paren => lines.push(Line::from_start(pair, indent)),
            Rule::block => lines.append(&mut format_block(pair, indent + 1)),
            Rule::element => match format_element(
                pair,
                indent,
                match lines.last().unwrap().start.as_ref().unwrap().as_rule() {
                    Rule::or_kw | Rule::and_kw => {
                        if lines.last().unwrap().elements.is_empty() {
                            true
                        } else {
                            false
                        }
                    }
                    _ => false,
                },
            ) {
                FormatElementResult::Rule(p) => lines.last_mut().unwrap().elements.push(p),
                FormatElementResult::Lines(mut l) => lines.append(&mut l),
            },
            Rule::and_kw | Rule::or_kw => {
                lines.push(Line::from_start(pair, indent));
            }
            Rule::COMMENT => lines.push(format_comment(pair, indent)),
            _ => unreachable!("Unexpected rule: {:?}", pair),
        }
    }

    lines
}

fn format_block(rule: Pair<Rule>, indent: usize) -> Vec<Line> {
    let mut lines = Vec::new();

    for pair in rule.into_inner() {
        match pair.as_rule() {
            Rule::block_kw | Rule::and_kw | Rule::or_kw => {
                lines.push(Line::from_start(pair, indent))
            }
            Rule::element => match format_element(pair, indent, true) {
                FormatElementResult::Rule(p) => lines.last_mut().unwrap().elements.push(p),
                FormatElementResult::Lines(mut l) => lines.append(&mut l),
            },
            Rule::COMMENT => lines.push(format_comment(pair, indent)),
            _ => unreachable!("Unexpected rule: {:?}", pair),
        }
    }

    lines
}

fn format_statement(rule: Pair<Rule>) -> Vec<Line> {
    rule.into_inner()
        .flat_map(|pair| match pair.as_rule() {
            Rule::block => format_block(pair, 0),
            Rule::COMMENT => vec![format_comment(pair, 0)],
            Rule::delimiter => vec![Line::from_start(pair, 0), Line::default()],
            _ => unreachable!("Unexpected rule: {:?}", pair),
        })
        .collect()
}

fn format_query(rule: Pair<Rule>) -> Vec<Line> {
    rule.into_inner()
        .flat_map(|pair| match pair.as_rule() {
            Rule::statement => format_statement(pair),
            Rule::COMMENT => vec![format_comment(pair, 0)],
            Rule::EOI => vec![Line::from_start(pair, 0)],
            _ => unreachable!("Unexpected rule: {:?}", pair),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{read_to_string, OpenOptions},
        io::Write,
    };

    use super::format_string;

    #[test]
    fn test_format_sql() {
        let sql = read_to_string("docs/vincent.sql").unwrap();
        let formatted = format_string(&sql);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("docs/output.sql")
            .expect("Failed to open file");

        match formatted.clone() {
            Ok(formatted) => file
                .write_all(formatted.as_bytes())
                .expect("Failed to write to file"),
            Err(err) => println!("{:?}", err),
        }

        // assert_eq!(formatted.unwrap(), sql);
    }
}
