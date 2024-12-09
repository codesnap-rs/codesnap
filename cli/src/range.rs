use std::str::Lines;

use anyhow::bail;

const RANGE_SEPARATOR: &'static str = ":";
pub const DEFAULT_RANGE: &'static str = "start:end";

pub struct Range(pub String, pub String);

impl Range {
    // Prepare range string by following rules:
    //
    // If raw_range is equal to "n:", which means x:end
    // If raw_range is equal to ":n", which means start:n
    fn prepare_range(raw_range: &str) -> String {
        if raw_range.starts_with(RANGE_SEPARATOR) {
            format!("{}{}", "start", raw_range)
        } else if raw_range.ends_with(RANGE_SEPARATOR) {
            format!("{}{}", raw_range, "end")
        } else {
            raw_range.to_string()
        }
    }

    pub fn from_opt_string(opt_str: Option<String>) -> anyhow::Result<Self> {
        Range::from_str(&opt_str.unwrap_or(String::from(DEFAULT_RANGE)))
    }

    pub fn from_str(range_str: &str) -> anyhow::Result<Self> {
        let range = Range::prepare_range(range_str);
        let range_points = range.split(RANGE_SEPARATOR).collect::<Vec<_>>();

        if range_points.len() != 2 {
            bail!("Invalid range format");
        }

        Ok(Range(
            range_points[0].to_string(),
            range_points[1].to_string(),
        ))
    }

    // Parse "start" to 0, "end" to lines.len(), and other values to usize
    fn parse_range(&self, code_snippet_lines: &Lines) -> anyhow::Result<(usize, usize)> {
        let Range(start, end) = self;
        let lines = code_snippet_lines.clone().collect::<Vec<&str>>();
        let start = parse_range_point(&start, &lines)?;
        let end = parse_range_point(&end, &lines)?;
        let points = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        Ok(points)
    }

    pub fn cut_code_snippet(&self, code_snippet: String) -> anyhow::Result<String> {
        let code_snippet_lines = code_snippet.lines();
        let (start, end) = self.parse_range(&code_snippet_lines)?;
        let code_snippet = code_snippet_lines
            .skip(start)
            .take(end - start)
            .collect::<Vec<&str>>()
            .join("\n");

        Ok(code_snippet)
    }
}

fn parse_range_point(point: &str, lines: &Vec<&str>) -> anyhow::Result<usize> {
    let point = match point {
        "start" => 1,
        "end" => lines.len(),
        _ => point.parse::<usize>()?,
    };

    Ok(point)
}
