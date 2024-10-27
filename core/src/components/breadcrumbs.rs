use std::path::MAIN_SEPARATOR_STR;

use cosmic_text::{Attrs, Family};
use regex::Regex;

use crate::{
    config,
    edges::margin::Margin,
    utils::text::FontRenderer,
    utils::{code::calc_wh_with_min_width, color::RgbaColor},
};

use super::interface::{
    component::Component,
    style::{ComponentStyle, RawComponentStyle, Size},
};

const LINE_HEIGHT: f32 = 15.;

pub struct Breadcrumbs {
    children: Vec<Box<dyn Component>>,
    path: Option<String>,
    config: Option<config::Breadcrumbs>,
}

impl Component for Breadcrumbs {
    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self) -> bool {
        self.config.is_some()
    }

    fn style(&self) -> RawComponentStyle {
        let style = RawComponentStyle::default();

        if self.config.is_some() {
            if let Some(path) = &self.path {
                let (w, h) = calc_wh_with_min_width(&path, 8., LINE_HEIGHT);

                return style
                    .size(Size::Num(w), Size::Num(h))
                    .margin(Margin::default());
            }
        }

        style
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::interface::component::ComponentContext,
        render_params: &super::interface::component::RenderParams,
        style: &super::interface::style::ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> super::interface::render_error::Result<()> {
        let config = self.config.clone().unwrap();

        if let Some(ref path) = self.path {
            let path = config
                .separator
                .and_then(|separator| Some(parse_separator(path, &separator)))
                .unwrap_or(path.clone());
            let color: RgbaColor = config.color.as_str().into();
            let attrs = Attrs::new().color(color.into());
            let attrs = match config.font_family {
                Some(ref font_family) => attrs.family(Family::Name(font_family)),
                None => attrs,
            };

            FontRenderer::new(
                12.,
                LINE_HEIGHT,
                context.scale_factor,
                context.take_snapshot_params.fonts_folder.clone(),
            )
            .draw_text(
                render_params.x,
                render_params.y,
                style.width,
                LINE_HEIGHT,
                vec![(&path, attrs)],
                pixmap,
            );
        }

        Ok(())
    }
}

impl Breadcrumbs {
    pub fn from_path(
        file_path: Option<String>,
        config: Option<config::Breadcrumbs>,
    ) -> Breadcrumbs {
        Breadcrumbs {
            children: vec![],
            path: file_path,
            config,
        }
    }
}

fn parse_separator(path_str: &str, separator: &str) -> String {
    Regex::new(MAIN_SEPARATOR_STR)
        .unwrap()
        .replace_all(path_str, separator)
        .to_string()
}
