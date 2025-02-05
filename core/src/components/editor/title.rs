use cosmic_text::{Align, Attrs, Family, Metrics, Weight};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    edges::margin::Margin,
    utils::color::parse_hex_to_cosmic_color,
};

pub struct Title {
    content: Option<String>,
    children: Vec<Box<dyn Component>>,
}

impl Component for Title {
    fn name(&self) -> &'static str {
        "Title"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self, _context: &ComponentContext) -> bool {
        self.content.is_some()
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        let calced_title_width = 6. * self.content.clone().unwrap().len() as f32;

        RawComponentStyle::default()
            .margin(Margin {
                bottom: 2.,
                ..Default::default()
            })
            .size(Size::Num(calced_title_width), Size::Num(12.))
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &ComponentContext,
        render_params: &RenderParams,
        _style: &ComponentStyle,
        _parent_style: &Style<f32>,
    ) -> render_error::Result<()> {
        let config = context.take_snapshot_params.window.title_config.clone();
        let attrs = Attrs::new()
            .weight(Weight::BOLD)
            .color(parse_hex_to_cosmic_color(&config.color))
            .family(Family::Name(&config.font_family));

        context.font_renderer.lock().unwrap().draw_line(
            0.,
            render_params.y,
            Metrics::new(10., 10.),
            &self.content.clone().unwrap(),
            attrs,
            Some(Align::Center),
            pixmap,
        );

        Ok(())
    }
}

impl Title {
    pub fn from_content(content: Option<String>) -> Title {
        Title {
            content,
            children: vec![],
        }
    }
}
