use config::{CodeBuilder, CodeSnap, HighlightLine, WatermarkBuilder};

mod components;
mod config;
mod edges;
mod preset_background;
mod snapshot;
mod utils;

const CODE_SNIPPET: &'static str = r##"if not vim.g.neovide then
	vim.api.nvim_create_autocmd({
		"VimLeave",
	}, {
		pattern = "*",
		callback = function()
			os.execute("kitty @ set-spacing padding=10 margin=0")
		end,
	})

	vim.api.nvim_create_autocmd({
		"VimEnter",
	}, {
		pattern = "*",
		callback = function()
			os.execute("kitty @ set-spacing padding=0 margin=0")
		end,
	})
end"##;

const data: &'static str = r##"
    {
        "window": {
            "macWindowBar": true,
            "shadow": 20,
            "margin": {
                "x": 82,
                "y": 82
            }
        },
        "code": {
            "fontFamily": "CaskaydiaCove Nerd Font",
            "theme": "base16-ocean.light"
        },
        "watermark": {
            "content": "CodeSnap",
            "fontFamily": "Pacifico",
            "color": "#ff0000"
        },
        "background":  {
            "start": {
                "x": 0,
                "y": 0
            },
            "end": {
                "x": "max",
                "y": 0
            },
            "stops": [
                {
                    "position": 0,
                    "color": "#6bcba5"
                },
                {
                    "position": 1,
                    "color": "#caf4c2"
                }
            ]
        }
    }
"##;

pub fn main() -> anyhow::Result<()> {
    CodeSnap::default()
        .code(
            CodeBuilder::default()
                .language("haskell")
                .content(r#"print "Hello, CodeSnap!""#)
                .highlight_lines(vec![HighlightLine::Range(1, 1, "#ff0000".to_string())])
                .build()?,
        )
        .watermark(WatermarkBuilder::default().content("YYM").build()?)
        .build()?
        .create_snapshot()?
        .raw_data()?
        .copy()?;

    Ok(())

    // CodeSnap::default()
    //     .themes_folder("/Users/zhanhaozhao/repositories/codesnap/assets/themes")
    //     .code(
    //         CodeBuilder::default()
    //             .language("rust")
    //             .theme("candy")
    //             .content(CODE_SNIPPET)
    //             .build()?,
    //     )
    //     .watermark(WatermarkBuilder::default().content("CodeSnap").build()?)
    //     .background(BAMBOO.clone())
    //     .build()?
    //     .create_snapshot()?
    //     .copy()
}
