use cosmic_text::{Attrs, Family, Metrics, Weight};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    utils::color::parse_hex_to_cosmic_color,
};

pub struct CommandLineHeader {
    children: Vec<Box<dyn Component>>,
    full_command: String,
    metrics: Metrics,
}

impl Component for CommandLineHeader {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self, context: &ComponentContext) -> RawComponentStyle {
        let parsed_line = format!(
            "{} {}",
            context.take_snapshot_params.command_output_config.prompt, self.full_command
        );
        let (w, h) = context
            .font_renderer
            .lock()
            .unwrap()
            .measure_text(self.metrics, parsed_line.as_str());

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
        let command_config = context.take_snapshot_params.command_output_config.clone();
        let command_and_args = self.full_command.split_whitespace().collect::<Vec<&str>>();
        let command_str = command_and_args[0];
        let create_attrs = || {
            Attrs::new().family(Family::Name(
                context
                    .take_snapshot_params
                    .code_config
                    .font_family
                    .as_str(),
            ))
        };
        let with_space = |str: &str| format!("{} ", str);
        let prompt = with_space(&context.take_snapshot_params.command_output_config.prompt);
        let command_str = with_space(command_str);
        let args = command_and_args[1..].join(" ");
        let spans = vec![
            (
                prompt.as_str(),
                create_attrs().color(parse_hex_to_cosmic_color(&command_config.prompt_color)),
            ),
            (
                command_str.as_str(),
                create_attrs()
                    .weight(Weight::BOLD)
                    .color(parse_hex_to_cosmic_color(&command_config.command_color)),
            ),
            (args.as_str(), create_attrs()),
        ];

        context.font_renderer.lock().unwrap().draw_text(
            render_params.x,
            render_params.y,
            self.metrics,
            spans,
            pixmap,
        );

        Ok(())
    }
}

impl CommandLineHeader {
    pub fn from(full_command: &str) -> CommandLineHeader {
        CommandLineHeader {
            full_command: full_command.to_string(),
            children: vec![],
            metrics: Metrics::new(12.5, 20.),
        }
    }
}
