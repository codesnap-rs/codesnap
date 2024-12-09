use std::str::Lines;

use anyhow::bail;

const RANGE_SEPARATOR: &'static str = ":";
const DEFAULT_RANGE: &'static str = "start:end";

fn parse_range_point(point: &str, lines: &Vec<&str>) -> anyhow::Result<usize> {
    let point = match point {
        "start" => 0,
        "end" => lines.len(),
        _ => point.parse::<usize>()?,
    };

    Ok(point)
}

pub fn cut_code_snippet_by_range(
    code_snippet: &str,
    range: &Option<String>,
) -> anyhow::Result<String> {
    let range = range.clone().unwrap_or(String::from(DEFAULT_RANGE));
    let range_points = range.split(RANGE_SEPARATOR).collect::<Vec<_>>();

    if range_points.len() != 2 {
        bail!("Invalid range format");
    }

    let code_snippet_lines = code_snippet.lines();
    let (start, end) = parse_range(&range_points, &code_snippet_lines)?;
    let code_snippet = code_snippet_lines
        .skip(start)
        .take(end - start)
        .collect::<Vec<&str>>()
        .join("\n");

    Ok(code_snippet)
}

// Prepare range string by following rules:
//
// If raw_range is equal to "n:", which means x:end
// If raw_range is equal to ":n", which means start:n
pub fn prepare_range(raw_range: &str) -> String {
    if raw_range.starts_with(RANGE_SEPARATOR) {
        format!("{}{}", "start", raw_range)
    } else if raw_range.ends_with(RANGE_SEPARATOR) {
        format!("{}{}", raw_range, "end")
    } else {
        raw_range.to_string()
    }
}

// Parse "start" to 0, "end" to lines.len(), and other values to usize
pub fn parse_range(
    range_points: &Vec<&str>,
    code_snippet_lines: &Lines,
) -> anyhow::Result<(usize, usize)> {
    let lines = code_snippet_lines.clone().collect::<Vec<&str>>();
    let start = parse_range_point(range_points[0], &lines)?;
    let end = parse_range_point(range_points[1], &lines)?;
    let points = if start > end {
        (end, start)
    } else {
        (start, end)
    };

    Ok(points)
}
