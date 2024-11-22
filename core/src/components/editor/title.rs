use cosmic_text::{Align, Attrs, Family, Weight};

use crate::{
    components::interface::{
        component::{Component, ComponentContext, RenderParams},
        render_error,
        style::{ComponentStyle, RawComponentStyle, Size, Style},
    },
    config::TitleConfig,
    edges::margin::Margin,
    utils::{
        color::parse_hex_to_cosmic_color,
        text::{create_file_system_by_fonts_folder, FontRenderer},
    },
};

pub struct Title {
    config: Option<TitleConfig>,
    children: Vec<Box<dyn Component>>,
}

impl Component for Title {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self) -> bool {
        self.config.is_some()
    }

    fn style(&self) -> RawComponentStyle {
        let calced_title_width = 6. * self.config.clone().unwrap().title.len() as f32;

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
        let config = self.config.clone().unwrap();
        let attrs = Attrs::new()
            .weight(Weight::BOLD)
            .color(parse_hex_to_cosmic_color(&config.color))
            .family(Family::Name(&config.font_family));

        FontRenderer::new(
            10.,
            10.,
            context.scale_factor,
            create_file_system_by_fonts_folder(&context.take_snapshot_params.fonts_folder),
        )
        .draw_line(
            0.,
            render_params.y,
            pixmap.width() as f32,
            pixmap.height() as f32,
            &config.title,
            attrs,
            Some(Align::Center),
            pixmap,
        );

        Ok(())
    }
}

impl Title {
    pub fn from_config(config: Option<TitleConfig>) -> Title {
        Title {
            config,
            children: vec![],
        }
    }
}
