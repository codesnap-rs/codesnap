use syntect::easy::HighlightLines;

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    config::{self, CodeConfig},
    utils::{
        code::{calc_wh_with_min_width, prepare_code, CHAR_WIDTH},
        highlight::Highlight,
        syntax_provider::SyntaxProvider,
        text::{create_file_system_from_binary, FontRenderer},
    },
};

const CASKAYDIA_COVE_NERD_FONT: &[u8] =
    include_bytes!("../../../assets/fonts/CaskaydiaCoveNerdFont-Regular.ttf");
const FONT_SIZE: f32 = 12.5;

pub struct Code {
    children: Vec<Box<dyn Component>>,
    value: String,
    code_content: config::Code,
    line_height: f32,
}

impl Component for Code {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        let (w, h) = calc_wh_with_min_width(&self.value, CHAR_WIDTH, self.line_height);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let highlight = Highlight::new(
            self.value.clone(),
            context.take_snapshot_params.code_config.font_family.clone(),
        );
        let syntax_provider = SyntaxProvider::new();
        let syntax = syntax_provider.guess_syntax(
            self.code_content.language.clone(),
            self.code_content.file_path.clone(),
            &self.value,
        )?;
        let (mut highlight_lines, syntax_set) = (
            HighlightLines::new(&syntax, &context.theme_provider.theme),
            &syntax_provider.syntax_set,
        );
        let highlight_result = highlight.parse(&mut highlight_lines, syntax_set)?;

        FontRenderer::new(
            FONT_SIZE,
            self.line_height,
            context.scale_factor,
            create_file_system_from_binary(
                CASKAYDIA_COVE_NERD_FONT,
                &context.take_snapshot_params.fonts_folder,
            ),
        )
        .draw_text(
            render_params.x,
            render_params.y,
            style.width,
            style.height,
            highlight_result.clone(),
            pixmap,
        );

        Ok(())
    }
}

impl Code {
    pub fn new(code_content: config::Code, line_height: f32) -> Code {
        Code {
            value: prepare_code(&code_content.content),
            code_content,
            children: vec![],
            line_height,
        }
    }
}
