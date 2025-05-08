use std::cmp::max;

use crate::{
    config::{Code, Content, SnapshotConfig},
    utils::code::{calc_max_line_number_length, calc_wh, prepare_code},
};

use super::snapshot_data::SnapshotData;

const SPACE_BOTH_SIDE: usize = 2;

pub struct ASCIISnapshot {
    code: Code,
    has_breadcrumbs: bool,
}

fn optional(component: String, is_view: bool) -> String {
    if is_view {
        component
    } else {
        "".to_string()
    }
}

impl ASCIISnapshot {
    pub fn raw_data(&self) -> Result<SnapshotData, anyhow::Error> {
        Ok(SnapshotData::Text(self.generate_snapshot()))
    }
}

impl ASCIISnapshot {
    pub fn from_config(config: SnapshotConfig) -> anyhow::Result<Self> {
        match config.content {
            Content::Code(ref raw_code) => Ok(ASCIISnapshot {
                code: raw_code.clone(),
                has_breadcrumbs: config.code_config.breadcrumbs.enable,
            }),
            _ => Err(anyhow::anyhow!("The code content is not raw")),
        }
    }

    fn generate_snapshot(&self) -> String {
        let code = prepare_code(&self.code.content);
        let (width, height) = calc_wh(&code, 1., 1.);
        let calc_line_number_width = |start_line_number: u32| {
            calc_max_line_number_length(height as usize, start_line_number)
        };
        let len = self
            .code
            .clone()
            .file_path
            .and_then(|x| Some(x.len()))
            .unwrap_or(0);
        let frame_width = max(width as usize, len + SPACE_BOTH_SIDE);
        let frame_width = match self.code.start_line_number {
            Some(start_line_number) => {
                frame_width + SPACE_BOTH_SIDE + calc_line_number_width(start_line_number)
            }
            None => frame_width,
        };
        let frame_width_with_space = frame_width + SPACE_BOTH_SIDE;
        let line = "─".repeat(frame_width_with_space);
        let breadcurmbs_line = format!("│{}│\n", line);
        let frame_width_with_content = frame_width;
        let top_frame = format!("╭{}╮\n", line);
        let bottom_frame = format!("╰{}╯", line);
        let code = code
            .lines()
            .enumerate()
            .map(|(i, line)| {
                format!(
                    "│ {:1$} │\n",
                    match self.code.start_line_number {
                        Some(ref start_line_number) => format!(
                            "{:1$} {line}",
                            *start_line_number as usize + i,
                            calc_line_number_width(*start_line_number),
                        ),
                        None => line.to_string(),
                    },
                    frame_width_with_content
                )
            })
            .collect::<String>();
        let text_line = |text: &str| format!("│ {:1$} │\n", text, frame_width_with_content);
        let breadcrumbs = optional(
            format!(
                "{}{breadcurmbs_line}",
                text_line(&self.code.file_path.clone().unwrap_or(String::from("")))
            ),
            self.has_breadcrumbs,
        );

        format!("{top_frame}{breadcrumbs}{code}{bottom_frame}")
    }
}
