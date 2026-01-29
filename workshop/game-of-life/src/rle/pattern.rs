use super::ParseError;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Pattern {
    pub name: Option<String>,
    pub author: Option<String>,
    pub comments: Vec<String>,
    pub width: u32,
    pub height: u32,
    pub rule: String,
    pub cells: HashSet<(i64, i64)>,
}

impl Pattern {
    pub fn parse_rle(input: &str) -> Result<Pattern, ParseError> {
        let mut name: Option<String> = None;
        let mut author: Option<String> = None;
        let mut comments: Vec<String> = Vec::new();
        let mut width: Option<u32> = None;
        let mut height: Option<u32> = None;
        let mut rule: String = "B3/S23".to_string();
        let mut pattern_data: String = String::new();

        for line in input.lines() {
            let line = line.trim();

            if let Some(n) = line.strip_prefix("#N ") {
                name = Some(n.trim().to_string());
            } else if let Some(o) = line.strip_prefix("#O ") {
                author = Some(o.trim().to_string());
            } else if let Some(c) = line
                .strip_prefix("#C ")
                .or_else(|| line.strip_prefix("#c "))
            {
                comments.push(c.trim().to_string());
            } else if line.starts_with('#') {
            } else if line.contains("x =") || line.contains("x=") {
                Self::parse_header_line(line, &mut width, &mut height, &mut rule)?;
            } else if !line.is_empty() {
                pattern_data.push_str(line);
            }
        }

        let width: u32 = width.ok_or(ParseError::MissingHeader)?;
        let height: u32 = height.ok_or(ParseError::MissingHeader)?;

        let cells: HashSet<(i64, i64)> = Self::parse_pattern_data(&pattern_data)?;

        Ok(Pattern {
            name,
            author,
            comments,
            width,
            height,
            rule,
            cells,
        })
    }

    fn parse_header_line(
        line: &str,
        width: &mut Option<u32>,
        height: &mut Option<u32>,
        rule: &mut String,
    ) -> Result<(), ParseError> {
        for part in line.split(',') {
            let part: &str = part.trim();

            if let Some(x_part) = part.strip_prefix("x =").or_else(|| part.strip_prefix("x=")) {
                *width = Some(x_part.trim().parse().map_err(|_| {
                    ParseError::InvalidHeader(format!("Invalid x value: {x_part}"))
                })?);
            } else if let Some(y_part) =
                part.strip_prefix("y =").or_else(|| part.strip_prefix("y="))
            {
                *height = Some(y_part.trim().parse().map_err(|_| {
                    ParseError::InvalidHeader(format!("Invalid y value: {y_part}"))
                })?);
            } else if let Some(rule_part) = part
                .strip_prefix("rule =")
                .or_else(|| part.strip_prefix("rule="))
            {
                *rule = rule_part.trim().to_string();
            }
        }

        Ok(())
    }

    fn parse_pattern_data(data: &str) -> Result<HashSet<(i64, i64)>, ParseError> {
        let mut cells: HashSet<(i64, i64)> = HashSet::new();
        let mut x: i64 = 0;
        let mut y: i64 = 0;
        let mut run_count: u32 = 0;

        for ch in data.chars() {
            match ch {
                '0'..='9' => {
                    let digit: u32 = ch.to_digit(10).unwrap_or(0);
                    run_count = run_count * 10 + digit;
                }
                'b' => {
                    let count: u32 = if run_count == 0 { 1 } else { run_count };
                    x += count as i64;
                    run_count = 0;
                }
                'o' => {
                    let count: u32 = if run_count == 0 { 1 } else { run_count };
                    for _ in 0..count {
                        cells.insert((x, y));
                        x += 1;
                    }
                    run_count = 0;
                }
                '$' => {
                    let count: u32 = if run_count == 0 { 1 } else { run_count };
                    y += count as i64;
                    x = 0;
                    run_count = 0;
                }
                '!' => break,
                ' ' | '\n' | '\r' | '\t' => {}
                _ => {
                    return Err(ParseError::InvalidPattern(format!(
                        "Unknown character: '{ch}'"
                    )));
                }
            }
        }

        Ok(cells)
    }
}
