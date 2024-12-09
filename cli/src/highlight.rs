use codesnap::config::HighlightLine;

use crate::range::Range;

pub fn create_highlight_lines_by_range(
    range: &str,
    highlight_color: &str,
    code_snippet: &str,
) -> anyhow::Result<Vec<HighlightLine>> {
    let Range(start, end) = Range::from_str(&range)?.parse_range(code_snippet)?;

    Ok(vec![HighlightLine::Range(
        start as u32,
        end as u32,
        highlight_color.to_string(),
    )])
}

pub fn create_highlight_lines_by_opt_range(
    range: &Option<String>,
    highlight_color: &str,
    code_snippet: &str,
) -> anyhow::Result<Vec<HighlightLine>> {
    match range {
        Some(ref range) => create_highlight_lines_by_range(range, highlight_color, code_snippet),
        None => Ok(vec![]),
    }
}

pub fn create_highlight_lines_by_ranges(
    ranges: &Vec<String>,
    highlight_color: &str,
    code_snippet: &str,
) -> anyhow::Result<Vec<HighlightLine>> {
    ranges.iter().try_fold(vec![], |mut acc, range| {
        acc.extend(create_highlight_lines_by_range(
            range,
            highlight_color,
            code_snippet,
        )?);
        Ok(acc)
    })
}
