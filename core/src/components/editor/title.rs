use cosmic_text::{Align, Attrs, Color, Weight};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    edges::margin::Margin,
    utils::text::FontRenderer,
};

pub struct Title {
    text: String,
    children: Vec<Box<dyn Component>>,
}

impl Component for Title {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn style(&self) -> RawComponentStyle {
        let calced_title_width = 6. * self.text.len() as f32;

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
        let attrs = Attrs::new()
            .weight(Weight::BOLD)
            .color(Color::rgb(172, 169, 178));

        FontRenderer::new(
            10.,
            10.,
            context.scale_factor,
            &context.take_snapshot_params.fonts_folder,
        )
        .draw_line(
            0.,
            render_params.y,
            pixmap.width() as f32,
            pixmap.height() as f32,
            &self.text,
            attrs,
            Some(Align::Center),
            pixmap,
        );

        Ok(())
    }
}

impl Title {
    pub fn from_text(text: &str) -> Title {
        Title {
            text: text.to_string(),
            children: vec![],
        }
    }
}
