use std::{fmt::Display, rc::Rc};

use layout::LayoutElement;
use logos::{Lexer, Logos};

mod layout;

#[derive(Debug, Logos, Clone, PartialEq, Eq)]
#[logos(skip r"\s+")]
pub enum Token {
    #[regex(r#"INSERT(?:\s+INTO)"#, |lex| lex.slice().to_owned(), ignore(case))]
    InsertInto(String),

    #[token("UPDATE", |lex| lex.slice().to_owned(), ignore(case))]
    Update(String),

    #[regex(r#"DELETE(?:\s+FROM)"#, |lex| lex.slice().to_owned(), ignore(case))]
    Delete(String),

    #[regex(r#"SELECT\s+(DISTINCT)?(\s+TOP\s+\d+)?"#, |lex| lex.slice().to_owned(), priority = 3, ignore(case))]
    Select(String),

    #[token("FROM", |lex| lex.slice().to_owned(), ignore(case))]
    From(String),

    #[token("WHERE", |lex| lex.slice().to_owned(), ignore(case))]
    Where(String),

    #[regex(r#"ORDER(?:\s+BY)"#, |lex| lex.slice().to_owned(), ignore(case))]
    OrderBy(String),

    #[regex(r#"GROUP(?:\s+BY)"#, |lex| lex.slice().to_owned(), ignore(case))]
    GroupBy(String),

    #[token("HAVING", |lex| lex.slice().to_owned(), ignore(case))]
    Having(String),

    #[token("AND", |lex| lex.slice().to_owned(), ignore(case))]
    And(String),

    #[token("SET", |lex| lex.slice().to_owned(), ignore(case))]
    Set(String),

    #[token("VALUES", |lex| lex.slice().to_owned(), ignore(case))]
    Values(String),

    #[token("OFFSET", |lex| lex.slice().to_owned(), ignore(case))]
    Offset(String),

    #[token("LIMIT", |lex| lex.slice().to_owned(), ignore(case))]
    Limit(String),

    #[token("JOIN", |lex| lex.slice().to_owned(), ignore(case))]
    Join(String),

    #[token("ON", |lex| lex.slice().to_owned(), ignore(case))]
    On(String),

    #[token("CASE", |lex| lex.slice().to_owned(), ignore(case))]
    Case(String),

    #[token("WHEN", |lex| lex.slice().to_owned(), ignore(case))]
    When(String),

    #[token("ELSE", |lex| lex.slice().to_owned(), ignore(case))]
    Else(String),

    #[token("THEN", |lex| lex.slice().to_owned(), ignore(case))]
    Then(String),

    #[token("END", |lex| lex.slice().to_owned(), ignore(case))]
    End(String),

    #[token("OR", |lex| lex.slice().to_owned(), ignore(case))]
    Or(String),

    #[token("BETWEEN", |lex| lex.slice().to_owned(), ignore(case))]
    Between(String),

    #[token("IN", |lex| lex.slice().to_owned(), ignore(case))]
    In(String),

    #[token("NOT", |lex| lex.slice().to_owned(), ignore(case))]
    Not(String),

    #[token("IS", |lex| lex.slice().to_owned(), ignore(case))]
    Is(String),

    #[token("LIKE", |lex| lex.slice().to_owned(), ignore(case))]
    Like(String),

    #[token("NULL", |lex| lex.slice().to_owned(), ignore(case))]
    Null(String),

    #[token("EXISTS", |lex| lex.slice().to_owned(), ignore(case))]
    Exists(String),

    #[token("+", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("-", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("/", |lex| lex.slice().to_owned(), priority = 2)]
    #[token(">", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("<", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("=", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("<>", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("<=", |lex| lex.slice().to_owned(), priority = 2)]
    #[token(">=", |lex| lex.slice().to_owned(), priority = 2)]
    #[token("!=", |lex| lex.slice().to_owned(), priority = 2)]
    Operator(String),

    #[token(",", priority = 2)]
    Comma,

    #[token("(", priority = 2)]
    OpenParen,

    #[token(")", priority = 2)]
    CloseParen,

    #[token(";", priority = 2)]
    Delimiter,

    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#, |lex| lex.slice().to_owned())]
    QuotedIdentifier(String),

    #[regex(r#"'([^'\\]*(\\.[^'\\]*)*)'"#, |lex| lex.slice().to_owned())]
    String(String),

    #[regex(r#"\/\*([^*]|\*[^\/])+\*\/"#, |lex| lex.slice().to_owned())]
    BlockComment(String),

    #[regex(r#"\S+"#, |lex| lex.slice().to_owned(), priority = 1)]
    Identifier(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Identifier(s) => write!(
                f,
                "{}",
                s.trim()
                    .split_whitespace()
                    .collect::<String>()
                    .to_lowercase()
            )?,
            Token::QuotedIdentifier(s) => write!(f, "{}", s.trim())?,
            Token::String(s) => write!(f, "{}", s.trim())?,
            Token::Operator(s) => write!(f, "{}", s.trim())?,
            Token::Comma => write!(f, ",")?,
            Token::OpenParen => write!(f, "(")?,
            Token::CloseParen => write!(f, ")")?,
            Token::BlockComment(s) => write!(f, "{}", s.trim())?,
            Token::Case(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Join(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::On(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Else(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Then(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::End(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Or(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Between(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::In(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Not(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Is(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Like(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Null(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Exists(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::InsertInto(s) => write!(
                f,
                "{}",
                s.trim()
                    .split_whitespace()
                    .collect::<String>()
                    .to_lowercase()
            )?,
            Token::Update(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Delete(s) => write!(
                f,
                "{}",
                s.trim()
                    .split_whitespace()
                    .collect::<String>()
                    .to_lowercase()
            )?,
            Token::Select(s) => write!(
                f,
                "{}",
                s.trim()
                    .split_whitespace()
                    .collect::<String>()
                    .to_lowercase()
            )?,
            Token::From(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Where(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::OrderBy(s) => write!(
                f,
                "{}",
                s.trim().split_whitespace().join(" ").to_lowercase()
            )?,
            Token::GroupBy(s) => write!(
                f,
                "{}",
                s.trim()
                    .split_whitespace()
                    .collect::<String>()
                    .to_lowercase()
            )?,
            Token::Having(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::And(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Set(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Values(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Offset(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Limit(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::When(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::Delimiter => write!(f, ";")?,
        };

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum NodeType {
    Inline(Token),
    Block(Token),
    OpenParenthesis,
    CloseParenthesis,
    Comma,
    BlockComment(String),
    Case(Token),
    Join(Token),
    Delimiter,
}

#[derive(Debug, PartialEq, Eq)]
struct Node {
    node_type: NodeType,
    level: u8,
}

#[derive(Debug, PartialEq, Eq)]
struct Tree {
    tree: Vec<Node>,
    level: u8,
}

impl Tree {
    fn new() -> Self {
        Self {
            tree: Vec::new(),
            level: 0,
        }
    }

    fn add(&mut self, token: Token) {
        match token {
            Token::Select(_)
            | Token::InsertInto(_)
            | Token::Update(_)
            | Token::Delete(_)
            | Token::From(_)
            | Token::Where(_)
            | Token::OrderBy(_)
            | Token::GroupBy(_)
            | Token::And(_)
            | Token::Set(_)
            | Token::Values(_)
            | Token::Offset(_)
            | Token::Limit(_) => {
                if let Some(node) = self.tree.last() {
                    if node.node_type != NodeType::OpenParenthesis && self.level > 0 {
                        self.level -= 1;
                    }
                }

                let node = Node {
                    node_type: NodeType::Block(token),
                    level: self.level,
                };

                self.tree.push(node);
                self.level += 1;
            }
            Token::Case(_) => {
                todo!("Case");
            }
            Token::BlockComment(_) => {
                todo!("BlockComment");
            }
            Token::OpenParen => {
                let node = Node {
                    node_type: NodeType::OpenParenthesis,
                    level: self.level,
                };

                self.tree.push(node);

                self.level += 1;
            }
            Token::CloseParen => {
                let mut iter = self.tree.iter().rev();
                while let Some(node) = iter.next() {
                    println!("{:?}", node);
                    match node.node_type {
                        NodeType::OpenParenthesis => {
                            if node.level < self.level {
                                self.level = node.level;
                                break;
                            }
                        }
                        _ => (),
                    }
                }

                let node = Node {
                    node_type: NodeType::CloseParenthesis,
                    level: self.level,
                };

                self.tree.push(node);
            }
            Token::Comma => {
                let node = Node {
                    node_type: NodeType::Comma,
                    level: self.level,
                };

                self.tree.push(node);
            }
            Token::Delimiter => {
                self.level = 0;
                self.tree.push(Node {
                    node_type: NodeType::Delimiter,
                    level: self.level,
                });
            }
            _ => {
                let node = Node {
                    node_type: NodeType::Inline(token),
                    level: self.level,
                };

                self.tree.push(node);
            }
        }
    }
}

impl Display for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut last_level = 0;
        for node in &self.tree {
            match &node.node_type {
                NodeType::Block(_)
                | NodeType::OpenParenthesis
                | NodeType::CloseParenthesis
                | NodeType::BlockComment(_)
                | NodeType::Comma => {
                    write!(f, "\n")?;
                    for _ in 0..node.level {
                        write!(f, "\t")?;
                    }
                }
                _ => (),
            };

            match &node.node_type {
                NodeType::Inline(token) => {
                    if node.level != last_level {
                        write!(f, "\t")?;
                    }
                    write!(f, "{}", token)?
                }
                NodeType::Block(token) => write!(f, "{}\n", token)?,
                NodeType::OpenParenthesis => write!(f, "(\n")?,
                NodeType::CloseParenthesis => write!(f, ")\n")?,
                NodeType::BlockComment(s) => write!(f, "{}\n", s)?,
                NodeType::Case(token) => write!(f, "{}", token)?,
                NodeType::Join(token) => write!(f, "{}", token)?,
                NodeType::Comma => write!(f, ",\t")?,
                NodeType::Delimiter => write!(f, ";")?,
            }

            last_level = node.level;
        }

        Ok(())
    }
}

fn format_block(
    mut lexer: Lexer<Token>,
    indentation: u8,
) -> Result<String, std::ops::Range<usize>> {
    let str = String::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(token) => tree.add(token),
            Err(_) => return Err(lexer.span()),
        }
    }

    str
}

pub fn format_sql(sql: &str) -> Result<String, std::ops::Range<usize>> {
    let mut tree = Tree::new();

    let mut lexer = Token::lexer(&sql);
    format_block(lexer, 0);

    Ok(tree.to_string())
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
        let formatted = format_sql(&sql);
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("docs/output.sql")
            .expect("Failed to open file");

        file.write_all(formatted.clone().unwrap().as_bytes())
            .expect("Failed to write to file");
        assert_eq!(formatted.unwrap(), sql);
    }
}
