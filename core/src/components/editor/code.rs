use cosmic_text::Metrics;
use syntect::{
    easy::HighlightLines,
    parsing::{SyntaxReference, SyntaxSet},
};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    config::{self},
    utils::{code::prepare_code, highlight::Highlight, syntax_provider::SyntaxProvider},
};

const FONT_SIZE: f32 = 12.5;
pub(crate) const CODE_LINE_HEIGHT: f32 = 18.;

pub struct Code {
    children: Vec<Box<dyn Component>>,
    value: String,
    metrics: Metrics,
    syntax: SyntaxReference,
    syntax_set: SyntaxSet,
}

impl Component for Code {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, context: &ComponentContext) -> RawComponentStyle {
        let (w, h) = context
            .font_renderer
            .lock()
            .unwrap()
            .measure_text(self.metrics, &self.value);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> render_error::Result<()> {
        let highlight = Highlight::new(
            self.value.clone(),
            context.take_snapshot_params.code_config.font_family.clone(),
        );
        let (mut highlight_lines, syntax_set) = (
            HighlightLines::new(&self.syntax, &context.theme_provider.theme),
            &self.syntax_set,
        );
        let highlight_result = highlight.parse(&mut highlight_lines, syntax_set)?;

        context.font_renderer.lock().unwrap().draw_text(
            render_params.x,
            render_params.y,
            self.metrics,
            highlight_result.clone(),
            pixmap,
        );

        Ok(())
    }

    fn name(&self) -> &'static str {
        "Code"
    }
}

impl Code {
    pub fn new(code_content: config::Code) -> anyhow::Result<Self> {
        let value = prepare_code(&code_content.content);
        let metrics = Metrics::new(FONT_SIZE, CODE_LINE_HEIGHT);
        let syntax_provider = SyntaxProvider::new();
        let syntax = syntax_provider.guess_syntax(
            code_content.language.clone(),
            code_content.file_path.clone(),
            &value,
        )?;

        Ok(Code {
            value: prepare_code(&code_content.content),
            children: vec![],
            metrics,
            syntax,
            syntax_set: syntax_provider.syntax_set,
        })
    }
}
