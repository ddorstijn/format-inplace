use logos::Logos;
use std::fmt::Display;

#[derive(Debug, Logos, Clone, PartialEq, Eq)]
#[logos(skip r"\s+")]
pub enum Token<'a> {
    #[regex(r#"INSERT\s+INTO"#, ignore(case))]
    #[token("UPDATE", ignore(case))]
    #[regex(r#"DELETE(?:\s+FROM)"#, ignore(case))]
    #[regex(r#"SELECT\s+(DISTINCT)?(\s+TOP\s+\d+)?"#, priority = 3, ignore(case))]
    #[token("FROM", ignore(case))]
    #[token("WHERE", ignore(case))]
    #[regex(r#"ORDER(?:\s+BY)"#, ignore(case))]
    #[regex(r#"GROUP(?:\s+BY)"#, ignore(case))]
    #[token("HAVING", ignore(case))]
    #[token("AND", ignore(case))]
    #[token("SET", ignore(case))]
    #[token("VALUES", ignore(case))]
    #[token("OFFSET", ignore(case))]
    #[token("LIMIT", ignore(case))]
    #[token("JOIN", ignore(case))]
    #[token("ON", ignore(case))]
    #[token("CASE", ignore(case))]
    #[token("WHEN", ignore(case))]
    #[token("ELSE", ignore(case))]
    #[token("THEN", ignore(case))]
    #[token("END", ignore(case))]
    #[token("OR", ignore(case))]
    BlockKeyword(&'a str),

    #[token("BETWEEN", ignore(case))]
    #[token("IN", ignore(case))]
    #[token("NOT", ignore(case))]
    #[token("IS", ignore(case))]
    #[token("LIKE", ignore(case))]
    #[token("NULL", ignore(case))]
    #[token("EXISTS", ignore(case))]
    InlineKeyword(&'a str),

    #[regex(r#""([^"\\]*(\\.[^"\\]*)*)""#)]
    #[regex(r#"'([^'\\]*(\\.[^'\\]*)*)'"#)]
    #[regex(r#"\S+"#, priority = 1)]
    #[token("+", priority = 2)]
    #[token("-", priority = 2)]
    #[token("/", priority = 2)]
    #[token(">", priority = 2)]
    #[token("<", priority = 2)]
    #[token("=", priority = 2)]
    #[token("<>", priority = 2)]
    #[token("<=", priority = 2)]
    #[token(">=", priority = 2)]
    #[token("!=", priority = 2)]
    Identifier(&'a str),

    #[token(",", priority = 2)]
    Comma,

    #[token("(", priority = 2)]
    OpenParen,

    #[token(")", priority = 2)]
    CloseParen,

    #[token(";", priority = 2)]
    Delimiter,

    #[regex(r#"\/\*([^*]|\*[^\/])+\*\/"#)]
    BlockComment(&'a str),
}

impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Comma => write!(f, ",")?,
            Token::OpenParen => write!(f, "(")?,
            Token::CloseParen => write!(f, ")")?,
            Token::Delimiter => write!(f, ";")?,
            Token::BlockComment(s) => write!(f, "{}", s.trim())?,
            Token::Identifier(s) => write!(f, "{}", s.trim())?,
            Token::InlineKeyword(s) => write!(f, "{}", s.trim().to_lowercase())?,
            Token::BlockKeyword(s) => write!(
                f,
                "{}",
                s.trim()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
                    .join(" ")
                    .to_lowercase()
            )?,
        };

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LineType<'a> {
    Inline(Vec<Token<'a>>),
    Subquery(Vec<Line<'a>>),
    Empty,
}

#[derive(Debug, PartialEq, Eq)]
struct Line<'a> {
    block: Token<'a>,
    tokens: LineType<'a>,
    indent: usize,
}

impl<'a> Line<'a> {
    fn new(block: Token<'a>) -> Self {
        Self {
            block,
            tokens: LineType::Empty,
            indent: 0,
        }
    }
}

impl<'a> Display for Line<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.block)?;
        match self.block {
            Token::BlockKeyword(_) => {
                if self.tokens != LineType::Empty {
                    write!(f, "\n")?;
                }
            }
            _ => (),
        }

        match &self.tokens {
            LineType::Inline(tokens) => {
                if tokens.len() > 0 {
                    write!(f, "\t")?;
                }

                write!(
                    f,
                    "{}",
                    tokens
                        .iter()
                        .map(|token| token.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )?;
            }
            LineType::Subquery(lines) => {
                for (i, line) in lines.iter().enumerate() {
                    if i == 0 {
                        write!(f, "\t")?;
                    }

                    write!(f, "{}", line)?;
                }
            }
            LineType::Empty => (),
        }

        Ok(())
    }
}

pub fn format_sql(sql: &str) -> Result<String, std::ops::Range<usize>> {
    let mut lexer = Token::lexer(&sql);
    let mut lines: Vec<Line> = Vec::new();

    let mut last_line: Option<&mut Line> = None;

    while let Some(token) = lexer.next() {
        match token.clone() {
            Ok(token) => match token {
                Token::BlockKeyword(_) => {
                    let last_line = lines.last_mut();
                    if let Some(last_line) = last_line {
                        if last_line.block == Token::OpenParen
                            && last_line.tokens == LineType::Empty
                        {
                            last_line.tokens = LineType::Subquery(vec![Line::new(token)]);
                            continue;
                        }
                    }

                    lines.push(Line::new(token));
                }
                Token::InlineKeyword(_) | Token::Identifier(_) => {
                    let last_line = last_line.ok_or(lexer.span())?;
                    match &mut last_line.tokens {
                        LineType::Subquery(_) => return Err(lexer.span()),
                        LineType::Inline(v) => v.push(token),
                        LineType::Empty => last_line.tokens = LineType::Inline(vec![token]),
                    }
                }
                Token::Comma => {
                    lines.push(Line::new(token));
                    last_line = Some(lines.last_mut().unwrap());
                }
                Token::OpenParen => {
                    let mut new_line = Line::new(token);
                    lines.push(new_line);
                    last_line = Some(&mut new_line);
                }
                Token::CloseParen => {
                    let mut new_line = Line::new(token);
                    lines.push(new_line);
                    last_line = Some(&mut new_line);
                }
                Token::Delimiter => {
                    let mut new_line = Line::new(token);
                    lines.push(new_line);
                    last_line = Some(&mut new_line);
                }
                Token::BlockComment(_) => {
                    let mut new_line = Line::new(token);
                    lines.push(new_line);
                    last_line = Some(&mut new_line);
                }
            },
            Err(_) => return Err(lexer.span()),
        }
    }

    Ok(lines
        .iter()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n"))
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

        match formatted.clone() {
            Ok(formatted) => file
                .write_all(formatted.as_bytes())
                .expect("Failed to write to file"),
            Err(err_span) => println!(
                "Failed to format SQL, error at {:?}. '{}'",
                err_span.clone(),
                sql.get(err_span).unwrap()
            ),
        }

        assert_eq!(formatted.unwrap(), sql);
    }
}
