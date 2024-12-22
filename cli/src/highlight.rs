use anyhow::bail;
use codesnap::config::HighlightLine;

use crate::range::Range;

pub struct HighlightLineRange {
    code_snippet_range: Range<usize>,
    code_snippet: String,
}

impl HighlightLineRange {
    pub fn from(code_snippet_range: Range<usize>, code_snippet: &str) -> anyhow::Result<Self> {
        Ok(HighlightLineRange {
            code_snippet_range,
            code_snippet: code_snippet.to_string(),
        })
    }

    pub fn create_highlight_lines(
        &self,
        raw_range: &str,
        highlight_color: &str,
    ) -> anyhow::Result<Vec<HighlightLine>> {
        let Range(start, end) = Range::from_str(&raw_range)?.parse_range(&self.code_snippet)?;
        let Range(code_snippet_start, code_snippet_end) = self.code_snippet_range;
        let offset_start = code_snippet_start - 1;

        if start < code_snippet_start {
            bail!(
                "The start range should be greater than or equal to {}",
                code_snippet_start
            );
        }

        if end > code_snippet_end {
            bail!(
                "The end range should be less than or equal to {}",
                code_snippet_end
            );
        }

        // Users may specify a range to generate snapshot from a part of the whole code snippet,
        // however the highlight range line number is start from already sliced code snippet
        // so we need to known the start range to calculate the correct highlight range line number
        let start = start - offset_start;
        let end = end - offset_start;

        Ok(vec![HighlightLine::Range(
            start as u32,
            end as u32,
            highlight_color.to_string(),
        )])
    }

    pub fn create_multiple_highlight_lines(
        &self,
        raw_ranges: &Vec<String>,
        highlight_color: &str,
    ) -> anyhow::Result<Vec<HighlightLine>> {
        raw_ranges.iter().try_fold(vec![], |mut acc, range| {
            acc.extend(self.create_highlight_lines(range, highlight_color)?);
            Ok(acc)
        })
    }
}
