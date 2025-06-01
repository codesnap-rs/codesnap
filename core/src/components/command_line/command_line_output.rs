use cosmic_text::Metrics;

use crate::{
    ansi::ANSI,
    components::interface::{
        component::{self, Component, ComponentContext},
        render_error,
        style::{self, RawComponentStyle, Size, Style},
    },
};

pub struct CommandLineOutput {
    ansi_text: String,
    children: Vec<Box<dyn Component>>,
    metrics: Metrics,
}

impl Component for CommandLineOutput {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, context: &ComponentContext) -> RawComponentStyle {
        let (w, h) = context
            .font_renderer
            .lock()
            .unwrap()
            .measure_text(self.metrics, &self.ansi_text);

        Style::default().size(Size::Num(w), Size::Num(h))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &component::ComponentContext,
        render_params: &component::RenderParams,
        _style: &style::ComponentStyle,
        _parent_style: &style::ComponentStyle,
    ) -> render_error::Result<()> {
        let ansi = ANSI::from(
            &self.ansi_text,
            context.take_snapshot_params.code_config.font_family.clone(),
        );
        let spans = ansi.colorize();

        context.font_renderer.lock().unwrap().draw_text(
            render_params.x,
            render_params.y,
            self.metrics,
            spans.clone(),
            pixmap,
        );

        Ok(())
    }

    fn name(&self) -> &'static str {
        "CommandLineOutput"
    }
}

impl CommandLineOutput {
    pub fn from(ansi_text: &str) -> CommandLineOutput {
        CommandLineOutput {
            ansi_text: ansi_text.to_string(),
            children: vec![],
            metrics: Metrics::new(12.5, 20.),
        }
    }
}
