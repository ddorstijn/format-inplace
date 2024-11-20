use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar/sql.pest"]
struct SQLParser;

#[derive(Debug, Default)]
struct Line {
    start: String,
    elements: Vec<String>,
}

impl Line {
    pub fn from_start(start: Pair<Rule>) -> Self {
        Self {
            start: start.as_str().to_string(),
            elements: Vec::new(),
        }
    }
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t{}", self.start, self.elements.join(" "))
    }
}

enum FormatElementResult {
    String(String),
    Lines(Vec<Line>),
}

pub struct SQLFormatter;

impl SQLFormatter {
    pub fn format_string(&self, sql: &str) -> Result<String, pest::error::Error<Rule>> {
        let tree = SQLParser::parse(Rule::query, sql)?.next().unwrap();

        let lines = self.format_query(tree);
        Ok(lines
            .iter()
            .map(|line| line.to_string())
            .collect::<Vec<String>>()
            .join("\n"))
    }

    fn format_comment(&self, rule: Pair<Rule>) -> Line {
        Line::from_start(rule)
    }

    fn format_element(&self, rule: Pair<Rule>) -> FormatElementResult {
        let pair = rule.into_inner().next().unwrap();
        match pair.as_rule() {
            Rule::identifier | Rule::operator | Rule::string | Rule::quoted => {
                FormatElementResult::String(pair.as_str().to_string())
            }
            Rule::comma => FormatElementResult::Lines(vec![Line::from_start(pair)]),
            Rule::paren => FormatElementResult::Lines(self.format_paren(pair)),
            Rule::between => todo!(),
            Rule::case => todo!(),
            Rule::join => todo!(),
            _ => unreachable!("Unexpected rule: {:?}", pair.as_node_tag()),
        }
    }

    fn format_paren(&self, rule: Pair<Rule>) -> Vec<Line> {
        let mut lines = Vec::new();

        for pair in rule.into_inner() {
            match pair.as_rule() {
                Rule::open_paren | Rule::close_paren => lines.push(Line::from_start(pair)),
                Rule::block => lines.append(&mut self.format_block(pair)),
                Rule::element => match self.format_element(pair) {
                    FormatElementResult::String(s) => {
                        lines.last_mut().unwrap().elements.push(s);
                    }
                    FormatElementResult::Lines(mut l) => lines.append(&mut l),
                },
                Rule::and_kw | Rule::or_kw => {
                    lines.push(Line::from_start(pair));
                }
                Rule::COMMENT => lines.push(self.format_comment(pair)),
                _ => unreachable!("Unexpected rule: {:?}", pair),
            }
        }

        lines
    }

    fn format_block(&self, rule: Pair<Rule>) -> Vec<Line> {
        let mut lines = Vec::new();

        for pair in rule.into_inner() {
            match pair.as_rule() {
                Rule::block_kw => lines.push(Line::from_start(pair)),
                Rule::element => match self.format_element(pair) {
                    FormatElementResult::String(s) => lines.last_mut().unwrap().elements.push(s),
                    FormatElementResult::Lines(mut l) => lines.append(&mut l),
                },
                Rule::COMMENT => lines.push(self.format_comment(pair)),
                _ => unreachable!("Unexpected rule: {:?}", pair.as_node_tag()),
            }
        }

        lines
    }

    fn format_statement(&self, rule: Pair<Rule>) -> Vec<Line> {
        rule.into_inner()
            .flat_map(|pair| match pair.as_rule() {
                Rule::block => self.format_block(pair),
                Rule::COMMENT => vec![self.format_comment(pair)],
                Rule::delimiter => vec![Line::from_start(pair)],
                _ => unreachable!("Unexpected rule: {:?}", pair),
            })
            .collect()
    }

    fn format_query(&self, rule: Pair<Rule>) -> Vec<Line> {
        println!("Rule: {:?}", rule.as_rule());
        rule.into_inner()
            .flat_map(|pair| match pair.as_rule() {
                Rule::statement => self.format_statement(pair),
                Rule::COMMENT => vec![self.format_comment(pair)],
                Rule::EOI => vec![Line::from_start(pair)],
                _ => unreachable!("Unexpected rule: {:?}", pair),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{read_to_string, OpenOptions},
        io::Write,
    };

    use super::*;

    #[test]
    fn test_format_sql() {
        let sql = read_to_string("docs/robert.sql").unwrap();
        let formatter = SQLFormatter {};
        let formatted = formatter.format_string(&sql);
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
