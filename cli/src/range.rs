use anyhow::bail;

const RANGE_SEPARATOR: &'static str = ":";
pub const DEFAULT_RANGE: &'static str = "start:end";

pub struct Range<T>(pub T, pub T);

impl Range<String> {
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

        // When range is in format "start:end", the length of range_points is 2
        // When range is in format "line_number", the length of range_points is 1
        if range_points.len() != 2 && range_points.len() != 1 {
            bail!("Invalid range format");
        }

        let start = range_points[0].to_string();
        let end = range_points.get(1).unwrap_or(&start.as_str()).to_string();

        Ok(Range(start, end))
    }

    // Parse "start" to 0, "end" to lines.len(), and other values to usize
    pub fn parse_range(&self, code_snippet: &str) -> anyhow::Result<Range<usize>> {
        let Range(start, end) = self;
        let lines = code_snippet.lines().clone().collect::<Vec<&str>>();
        let start = parse_range_point(&start, &lines)?;
        let end = parse_range_point(&end, &lines)?;
        let points = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        Ok(Range(points.0, points.1))
    }
}

impl Range<usize> {
    pub fn cut_code_snippet(&self, code_snippet: &str) -> anyhow::Result<String> {
        let Range(start, end) = self;
        let code_snippet_lines = code_snippet.lines();
        let code_snippet = code_snippet_lines
            .skip(start - 1)
            .take((end + 1) - start)
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
