use clap::ValueEnum;
use std::{fmt::Display, path::Path};

use pest::{iterators::Pair, Parser, Span};
use pest_derive::Parser;

struct Line {
    indent: u8,
    block: String,
    inline: Vec<String>,
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            "\t".repeat(self.indent as usize),
            self.block,
            self.inline.join(" ")
        )
    }
}

#[derive(Parser)]
#[grammar = "grammar/sql.pest"]
struct SQLParser;

#[derive(ValueEnum, Debug, Clone, Copy)]
pub enum IndentationType {
    Standard,
    Tabbed,
}

pub struct SQLFormatter {
    indentation_type: IndentationType,
}

impl SQLFormatter {
    pub fn new(indentation: IndentationType) -> Self {
        Self {
            indentation_type: indentation,
        }
    }

    pub fn format_string(&self, sql: &str) -> Result<String, pest::error::Error<Rule>> {
        let tree = SQLParser::parse(Rule::query, sql)?.next().unwrap();

        Ok(tree
            .into_inner()
            .map(|pair| self.format_rule(pair, 0))
            .collect::<Vec<String>>()
            .join("\n\n"))
    }

    pub fn format_file<P: AsRef<Path>>(&self, path: P) -> Result<String, pest::error::Error<Rule>> {
        let sql = std::fs::read_to_string(path).map_err(|_| {
            pest::error::Error::new_from_span(
                pest::error::ErrorVariant::CustomError {
                    message: "Failed to read file".to_string(),
                },
                Span::new("", 0, 0).unwrap(),
            )
        })?;

        // TODO: Write back to file

        self.format_string(&sql)
    }

    fn format_rule(&self, rule: Pair<Rule>, level: usize) -> String {
        match rule.as_rule() {
            Rule::EOI => String::new(),
            Rule::COMMENT => todo!(),
            Rule::comma => "\t".repeat(level) + rule.as_str(),
            Rule::open_paren | Rule::close_paren => {
                String::from("\n") + &"\t".repeat(level) + rule.as_str()
            }
            Rule::double_quote
            | Rule::single_quote
            | Rule::delimiter
            | Rule::identifier
            | Rule::string
            | Rule::quoted
            | Rule::select_kw
            | Rule::distinct_kw
            | Rule::top_kw
            | Rule::insert_kw
            | Rule::into_kw
            | Rule::values_kw
            | Rule::update_kw
            | Rule::set_kw
            | Rule::delete_kw
            | Rule::from_kw
            | Rule::relate_kw
            | Rule::where_kw
            | Rule::and_kw
            | Rule::or_kw
            | Rule::by_kw
            | Rule::group_kw
            | Rule::order_kw
            | Rule::desc_kw
            | Rule::asc_kw
            | Rule::inline_kw
            | Rule::between_kw
            | Rule::in_kw
            | Rule::not_kw
            | Rule::is_kw
            | Rule::like_kw
            | Rule::null_kw
            | Rule::exists_kw
            | Rule::operator => rule.as_str().to_string(),
            Rule::block => self.format_rule(rule.into_inner().next().unwrap(), level),
            Rule::insert_block
            | Rule::select_block
            | Rule::update_block
            | Rule::delete_block
            | Rule::from_block
            | Rule::relate_block
            | Rule::where_block
            | Rule::and_block
            | Rule::or_block
            | Rule::values_block
            | Rule::set_block
            | Rule::group_by_block
            | Rule::order_by_block => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level))
                .collect::<Vec<String>>()
                .join("\t"),
            Rule::block_kw => {
                "\t".repeat(level) + &self.format_rule(rule.into_inner().next().unwrap(), level)
            }
            Rule::select_compound
            | Rule::insert_into_compound
            | Rule::delete_from_compound
            | Rule::group_by_compound
            | Rule::order_by_compound => {
                let keywords = rule
                    .into_inner()
                    .map(|pair| self.format_rule(pair, level))
                    .collect::<Vec<_>>();

                match self.indentation_type {
                    IndentationType::Tabbed => keywords.join("\n"),
                    IndentationType::Standard => keywords.join(" "),
                }
            }
            Rule::function => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level + 1))
                .collect::<Vec<_>>()
                .join("\t"),
            Rule::inline_item | Rule::table_identifier => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level))
                .collect::<Vec<_>>()
                .join(" "),
            Rule::list => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level))
                .collect::<Vec<_>>()
                .join("\n"),
            Rule::list_line => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level))
                .collect::<Vec<String>>()
                .join("\t"),
            Rule::list_item => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level))
                .collect::<Vec<_>>()
                .join(" "),
            Rule::between => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level))
                .collect::<Vec<_>>()
                .join(" "),
            Rule::subquery => rule
                .into_inner()
                .map(|pair| match pair.as_rule() {
                    Rule::statement => self.format_rule(pair, level + 2),
                    _ => self.format_rule(pair, level + 1),
                })
                .collect::<Vec<_>>()
                .join("\t"),
            Rule::item_list => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level + 1))
                .collect::<Vec<_>>()
                .join("\t"),
            Rule::subclause => rule
                .into_inner()
                .map(|pair| self.format_rule(pair, level + 1))
                .collect::<Vec<_>>()
                .join("\n"),
            Rule::clause => {
                let mut pairs = rule.into_inner().peekable();
                let inner_rule = pairs.peek().unwrap();

                match inner_rule.as_rule() {
                    Rule::subclause => self.format_rule(pairs.next().unwrap(), level),
                    _ => pairs
                        .map(|pair| self.format_rule(pair, level))
                        .collect::<Vec<_>>()
                        .join(" "),
                }
            }
            Rule::statement => rule
                .into_inner()
                .map(|pair| match pair.as_rule() {
                    Rule::block => self.format_rule(pair, level),
                    Rule::delimiter => self.format_rule(pair, level),
                    _ => todo!("Unexpected rule: {:?}", pair.as_rule()),
                })
                .collect::<Vec<_>>()
                .join("\n"),
            _ => todo!("Unexpected rule: {:?}", rule.as_rule()),
        }
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
        let formatter = SQLFormatter::new(IndentationType::Tabbed);
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

        assert_eq!(formatted.unwrap(), sql);
    }
}
