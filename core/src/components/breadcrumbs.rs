use std::path::MAIN_SEPARATOR_STR;

use cosmic_text::{Attrs, Family, Metrics};
use regex::Regex;

use crate::{
    edges::margin::Margin,
    utils::{code::calc_wh_with_min_width, color::RgbaColor},
};

use super::interface::{
    component::{Component, ComponentContext},
    style::{ComponentStyle, RawComponentStyle, Size},
};

const LINE_HEIGHT: f32 = 15.;

pub struct Breadcrumbs {
    children: Vec<Box<dyn Component>>,
    has_breadcrumbs: bool,
    path: Option<String>,
}

impl Component for Breadcrumbs {
    fn name(&self) -> &'static str {
        "Breadcrumbs"
    }

    fn children(&self) -> &Vec<Box<dyn Component>> {
        &self.children
    }

    fn render_condition(&self, _context: &ComponentContext) -> bool {
        self.has_breadcrumbs
    }

    fn style(&self, _context: &ComponentContext) -> RawComponentStyle {
        let style = RawComponentStyle::default();

        if !self.has_breadcrumbs {
            return style;
        }

        self.path
            .as_ref()
            .and_then(|path| {
                let (w, h) = calc_wh_with_min_width(&path, 8., LINE_HEIGHT);

                return Some(
                    style
                        .clone()
                        .size(Size::Num(w), Size::Num(h))
                        .margin(Margin::default()),
                );
            })
            .unwrap_or(style)
    }

    fn draw_self(
        &self,
        pixmap: &mut tiny_skia::Pixmap,
        context: &super::interface::component::ComponentContext,
        render_params: &super::interface::component::RenderParams,
        _style: &super::interface::style::ComponentStyle,
        _parent_style: &ComponentStyle,
    ) -> super::interface::render_error::Result<()> {
        let config = context.take_snapshot_params.code_config.breadcrumbs.clone();

        if let Some(ref path) = self.path {
            let path = parse_separator(
                path,
                &context
                    .take_snapshot_params
                    .code_config
                    .breadcrumbs
                    .separator,
            );
            let color: RgbaColor = config.color.as_str().into();
            let attrs = Attrs::new().color(color.into());
            let attrs = attrs.family(Family::Name(
                &context.take_snapshot_params.code_config.font_family,
            ));

            context.font_renderer.lock().unwrap().draw_text(
                render_params.x,
                render_params.y,
                Metrics::new(12., LINE_HEIGHT),
                vec![(&path, attrs)],
                pixmap,
            );
        }

        Ok(())
    }
}

impl Breadcrumbs {
    pub fn from(has_breadcrumbs: bool, file_path: Option<String>) -> Breadcrumbs {
        Breadcrumbs {
            children: vec![],
            path: file_path,
            has_breadcrumbs,
        }
    }
}

fn parse_separator(path_str: &str, separator: &str) -> String {
    Regex::new(MAIN_SEPARATOR_STR)
        .unwrap()
        .replace_all(path_str, separator)
        .to_string()
}
